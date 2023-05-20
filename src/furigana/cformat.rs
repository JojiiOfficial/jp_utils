use super::{parse::unchecked::UncheckedFuriParser, segment::AsSegment, Furigana};
use std::mem::swap;

/// Transcodes underlying furigana data without changing the furigana text itself. This can be used
/// to convert encoded furigana strings to different styles, eg all kanjis in separate parts or
/// merging kanji parts into a single.
pub struct CodeFormatter<'a, T> {
    // Furigana that should be transcoded.
    src_furi: &'a Furigana<T>,

    // Buffer to write changes to.
    buf: String,

    // Src to work on in case a change already has been applied.
    // This way we can reuse the buffer.
    src: String,

    /// Whether operations can be lossy or not.
    lossy: bool,
}

impl<'a, T> CodeFormatter<'a, T> {
    #[inline]
    pub fn new(furi: &'a Furigana<T>) -> Self {
        Self {
            buf: String::new(),
            src: String::new(),
            src_furi: furi,
            lossy: false,
        }
    }

    /// Whether operations can be lossy or not.
    pub fn lossy(mut self) -> Self {
        self.lossy = true;
        self
    }

    /// Finishes the formatting process and returns the resulting furigana value.
    #[inline]
    pub fn finish(self) -> Furigana<String> {
        Furigana(self.buf)
    }
}

impl<'a, T> CodeFormatter<'a, T>
where
    T: AsRef<str>,
{
    /// Merges all kanji segments with detailed readings which are located next to each other into a single
    /// with kanji segment with assigned literal readings.
    pub fn merge_kanji_parts(mut self) -> Self {
        let lossy = self.lossy;

        let (str, buf) = self.get_src();

        let mut merge_buf = String::new();
        let mut pushed_kanji = false;
        let mut has_undetailed = false;

        // Merges a merge buffer buffer into `buf`
        #[inline(always)]
        fn merge(buf: &mut String, merge_buf: &mut String, has_undetailed: bool) {
            merge_buf.pop();
            buf.push('|');
            if !has_undetailed {
                buf.push_str(&merge_buf);
            } else {
                buf.push_str(&merge_buf.replace('|', ""));
            }
            buf.push(']');
            merge_buf.clear();
        }

        for (sub, is_kanji) in Furigana(str).gen_parser() {
            let i = UncheckedFuriParser::from_seg_str(sub, is_kanji);

            // Handle merge with reading data after a sequence of kanji parts.
            let detailed = i.detailed_readings().unwrap_or_default();

            let empty_kanji = i
                .readings()
                .map(|i| i.is_empty() || i[0].is_empty())
                .unwrap_or_default();

            let exit_undetailed = detailed || lossy;
            if (!is_kanji || !exit_undetailed || empty_kanji) && !merge_buf.is_empty() {
                merge(buf, &mut merge_buf, has_undetailed);
                pushed_kanji = false;
                has_undetailed = false;
            }

            // Handle kana
            if let Some(kana) = i.as_kana() {
                buf.push_str(kana);
                continue;
            }

            // Only kanji parts from here!
            let kanji = i.as_kanji().unwrap();

            // Push empty kanjis as new empty kanji.
            if empty_kanji {
                buf.push('[');
                buf.push_str(kanji);
                buf.push_str("|]");
                continue;
            }

            if !detailed {
                // Treat non detailed kanji as separate parts when not in lossy mode.
                if !lossy {
                    buf.push_str(sub);
                    continue;
                }
                has_undetailed = true;
            }

            // Push beginning of kanji block to out buf
            if !pushed_kanji {
                pushed_kanji = true;
                buf.push('[');
            }

            // Push the current segments kanji literals to out buf
            buf.push_str(kanji);

            // Extend the reading buffer with the current kanjis readings.
            for reading in i.readings().into_iter().flatten() {
                merge_buf.push_str(reading);
                merge_buf.push('|');
            }
        }

        // Apply one more merge for the last kanji element that probably hasn't been pushed yet.
        if merge_buf.len() > 0 {
            merge(buf, &mut merge_buf, has_undetailed);
        }

        self
    }

    /// Returns the src furigana string that should be used to work with. This prefers using
    /// the buffer by setting `self.src` to `self.buf`. To not break this types invariant you have
    /// to fill `self.buf` again with some furigana.
    fn get_src(&mut self) -> (&str, &mut String) {
        if self.buf.is_empty() {
            (self.src_furi.raw(), &mut self.buf)
        } else {
            swap(&mut self.buf, &mut self.src);
            self.buf.clear();
            (&self.src, &mut self.buf)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("[大|だい][丈|じょう][夫|ぶ]", "[大丈夫|だい|じょう|ぶ]"; "AllKanji")]
    #[test_case("それは[大|だい][丈|じょう][夫|ぶ]", "それは[大丈夫|だい|じょう|ぶ]"; "KanaBefore")]
    #[test_case(
        "それは[大|だい][丈|じょう]です",
        "それは[大丈|だい|じょう]です"; "kanaBeforeAndAfter"
    )]
    #[test_case("それは[大|だい][丈夫|じょうぶ]だよ", "それは[大|だい][丈夫|じょうぶ]だよ"; "NonDetailed")]
    #[test_case("それは[音|おん][楽|がく][大学|だいがく]です", "それは[音楽|おん|がく][大学|だいがく]です"; "NonDetailed2")]
    #[test_case("それは[音|おん][楽|がく][大学|だい|がく]です", "それは[音楽大学|おん|がく|だい|がく]です"; "NonDetailed3")]
    #[test_case("[Wi|ワイ][-|][Fi|ファイ] ってフランス[語|ご]ではどう[発音|はつ|おん]するんですか？", "[Wi|ワイ][-|][Fi|ファイ] ってフランス[語|ご]ではどう[発音|はつ|おん]するんですか？"; "EmptyKanjiPart")]
    #[test_case(
        "[高校生|こう|こう|せい]の[時|とき]は[毎朝|まい|あさ][6|][時|じ]に[起|お]きていた。",
        "[高校生|こう|こう|せい]の[時|とき]は[毎朝|まい|あさ][6|][時|じ]に[起|お]きていた。"
    )]
    fn test_merge_parts(src: &str, dst: &str) {
        let furi = Furigana(src);
        let res = furi.code_formatter().merge_kanji_parts().finish();
        assert_eq!(res.raw(), dst);

        let r = res.to_reading();
        assert_eq!(furi.kana_str(), r.kana());
        assert_eq!(furi.kanji_str(), r.kanji().unwrap());

        assert_eq!(Furigana(dst).kana_str(), res.kana_str());
        assert_eq!(Furigana(dst).kanji_str(), res.kanji_str());

        assert_eq!(Furigana(dst).kana_str(), furi.kana_str());
        assert_eq!(Furigana(dst).kanji_str(), furi.kanji_str());
    }

    #[test_case("[大|だい][丈|じょう][夫|ぶ]", "[大丈夫|だい|じょう|ぶ]"; "AllKanji")]
    #[test_case("それは[大|だい][丈|じょう][夫|ぶ]", "それは[大丈夫|だい|じょう|ぶ]"; "KanaBefore")]
    #[test_case(
        "それは[大|だい][丈|じょう]です",
        "それは[大丈|だい|じょう]です"; "kanaBeforeAndAfter"
    )]
    #[test_case("それは[大|だい][丈夫|じょうぶ]だよ", "それは[大丈夫|だいじょうぶ]だよ"; "lossy1")]
    #[test_case("それは[音|おん][楽|がく][大学|だいがく]です", "それは[音楽大学|おんがくだいがく]です"; "lossy2")]
    #[test_case("それは[音|おん][楽|がく][大学|だい|がく]です", "それは[音楽大学|おん|がく|だい|がく]です"; "Detailed")]
    fn test_merge_parts_lossy(src: &str, dst: &str) {
        let furi = Furigana(src);
        let res = CodeFormatter::new(&furi)
            .lossy()
            .merge_kanji_parts()
            .finish();
        assert_eq!(res.raw(), dst);
    }
}
