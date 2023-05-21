use super::{
    parse::unchecked::UncheckedFuriParser,
    segment::{encoder::FuriEncoder, AsSegment},
    Furigana,
};
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
    #[inline]
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
    /// Applies all formattings.
    #[inline]
    pub fn apply_all(self) -> Furigana<String> {
        self.merge_kanji_parts()
            .remove_empty_kanji()
            .fix_kanji_blocks()
            .finish()
    }

    /// Fixes kanji blocks with invalid reading kanji count.
    /// eg. [音楽大|おんがく|だい] => [音楽大|おんがくだい]
    pub fn fix_kanji_blocks(mut self) -> Self {
        let (str, buf) = self.get_src();
        let mut enc = FuriEncoder::new(buf);

        for seg in &Furigana(str) {
            if let Some(kana) = seg.as_kana() {
                enc.write_kana(kana);
                continue;
            }

            let readings = seg.readings().unwrap();
            let kanji = seg.as_kanji().unwrap();
            if seg.detailed_readings().unwrap() || readings.len() == 1 {
                enc.write_kanji_seg(&seg, kanji);
                continue;
            }

            let new_reading = readings.iter().fold(String::new(), |mut init, i| {
                init.push_str(i);
                init
            });

            enc.write_block(kanji, &new_reading);
        }

        self
    }

    /// Converts kanji blocks without readings to kana.
    pub fn remove_empty_kanji(mut self) -> Self {
        let (str, buf) = self.get_src();
        let mut enc = FuriEncoder::new(buf);

        for seg in &Furigana(str) {
            if let Some(kana) = seg.as_kana() {
                enc.write_kana(kana);
                continue;
            }

            let kanji = seg.as_kanji().unwrap();
            let readings = seg.readings().unwrap();
            if readings.is_empty() || readings[0].is_empty() {
                enc.write_kana(kanji);
                continue;
            }

            enc.write_kanji_seg(&seg, kanji);
        }

        self
    }

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
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。",
    "[2|][x|えっくす]+[1|]の[定義域|てい|ぎ|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "special")]
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

    #[test_case("[Wi|ワイ][-|][Fi|ファイ] って", "[Wi|ワイ]-[Fi|ファイ] って"; "1")]
    #[test_case("[毎朝|まい|あさ][6|][時|じ]に", "[毎朝|まい|あさ]6[時|じ]に";"2")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。",
    "2[x|えっくす]+1の[定義|てい|ぎ][域|いき]が[A|えい]=[1,2]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [3,5]となる。"; "special")]
    fn test_remove_empty_kanji(s: &str, exp: &str) {
        let furi = Furigana(s);
        let out = CodeFormatter::new(&furi).remove_empty_kanji().finish();
        assert_eq!(out, exp);
    }

    #[test_case("[音楽大|おんがく|だい]", "[音楽大|おんがくだい]"; "1")]
    #[test_case("おんがくが[好|す]","おんがくが[好|す]"; "End_kanji")]
    #[test_case("おんがくが[好|す]きです", "おんがくが[好|す]きです")]
    #[test_case("[音楽|おん|がく]が[好|す]き", "[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]","[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい", "[楽|たの]しい")]
    #[test_case("[音楽おん|がく]が[好す", "[音楽おん|がく]が[好す")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。", "この[人|ひと]が[嫌|きら]いです。")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。", "[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "with brackets")]
    fn test_fix_kanji_blocks(s: &str, exp: &str) {
        let furi = Furigana(s);
        let out = CodeFormatter::new(&furi).fix_kanji_blocks().finish();
        assert_eq!(out, exp);
    }
}
