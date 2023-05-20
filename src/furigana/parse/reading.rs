use super::gen::FuriParserGen;
use std::fmt::Debug;

/// Parses an encoded furigana string into its kana or kanji reading efficiently.
#[derive(Clone, Copy)]
pub struct FuriToReadingParser<'a> {
    str: &'a str,
    to_kana: bool,
    kanji_fallback: bool,
}

impl<'a> FuriToReadingParser<'a> {
    /// Create a new Furigana parse iterator that parses the given `inp` string
    #[inline]
    pub fn new(str: &'a str, to_kana: bool) -> Self {
        Self {
            str,
            to_kana,
            kanji_fallback: true,
        }
    }

    /// Disables kanji fallback for the parser. Kanji fallback means that the kanji reading is used
    /// if there is no kana reading.
    pub fn no_kanji_fallback(mut self) -> Self {
        self.kanji_fallback = false;
        self
    }

    /// Returns `true` if the parser would return an empty string.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the count of chars of the string that would be returned when parsing.
    pub fn char_count(&self) -> usize {
        if self.str.is_empty() {
            return 0;
        }
        let mut len = 0;
        self.run(|s| len += s.chars().count());
        len
    }

    /// Returns the length of the bytes of the string that gets returned after parsing. This
    /// function is slow, so try caching if possible. However its faster than parsing and checking
    /// length as it doesn't allocate any string.
    pub fn len(&self) -> usize {
        if self.str.is_empty() {
            return 0;
        }
        let mut len = 0;
        self.run(|s| len += s.len());
        len
    }

    /// Parses the furigana to either kana or kanji.
    pub fn parse(&self) -> String {
        let mut buf = String::with_capacity(self.str.len().saturating_sub(10));
        self.run(|s| buf.push_str(s));
        buf
    }

    /// Parses furigana to kanji and kana at the same time. If you need both kana and kanji, use
    /// this function instead of calling parsing twice.
    pub fn parse_kanji_and_kana(furi: &str) -> (String, Option<String>) {
        let mut kana_buf = String::with_capacity(furi.len());
        let mut kanji_buf = String::new();

        let mut has_kanji = false;

        for (txt, is_kanji) in FuriParserGen::new(furi) {
            if is_kanji {
                if !has_kanji {
                    has_kanji = true;
                    kanji_buf = String::with_capacity(furi.len().saturating_sub(10));
                    kanji_buf.push_str(&kana_buf);
                }
                let mut part = txt[1..txt.len() - 1].split('|');

                // Safety
                // split always returns at least one element
                let kanji = unsafe { &part.next().unwrap_unchecked() };
                let first_kana = part.next().unwrap();

                if first_kana.trim().is_empty() {
                    kana_buf.push_str(kanji);
                    kanji_buf.push_str(kanji);
                    // kana_buf.push_str(first_kana);
                    /* kanji_buf.push_str(kanji);
                    kana_buf.push_str(first_kana);
                    kana_buf.extend(part); */
                } else {
                    kanji_buf.push_str(kanji);
                    kana_buf.push_str(first_kana);
                    kana_buf.extend(part);
                }
            } else {
                kana_buf.push_str(txt);
                if has_kanji {
                    kanji_buf.push_str(txt);
                }
            }
        }

        (kana_buf, has_kanji.then_some(kanji_buf))
    }

    /// Runs the parser and writes all sub strings into `w`.
    fn run<W>(&self, mut w: W)
    where
        W: FnMut(&str),
    {
        for (txt, kanji) in FuriParserGen::new(self.str) {
            if kanji {
                self.accept_kanji(txt, &mut w);
            } else {
                self.accept_kana(txt, &mut w);
            }
        }
    }

    /// Parses the given block as kana.
    #[inline]
    fn accept_kana<W>(&self, block: &str, mut w: W)
    where
        W: FnMut(&str),
    {
        w(block)
    }

    /// Parses the given block as kanji.
    fn accept_kanji<W>(&self, block: &str, w: W)
    where
        W: FnMut(&str),
    {
        let block_inner = &block[1..block.len() - 1];

        if self.to_kana {
            self.parse_kana_part(block_inner, w);
        } else {
            self.parse_kanji(block_inner, w);
        }
    }

    /// Parses a kanji from a kanji block without brackets.
    fn parse_kanji<W>(&self, kanji_inner: &str, mut w: W)
    where
        W: FnMut(&str),
    {
        let mut block = kanji_inner.split('|');
        w(block.next().unwrap());
    }

    /// Parses the kana part from a kanji block without brackets.
    fn parse_kana_part<W>(&self, kanji_inner: &str, mut w: W)
    where
        W: FnMut(&str),
    {
        let mut block = kanji_inner.split('|');
        let mut pushed = false;

        let kanji = block.next().unwrap();

        for b in block {
            if !b.is_empty() {
                pushed = true;
            }
            w(b);
        }

        // Apply kanji fallback if we didn't modify the string
        if self.kanji_fallback && !pushed {
            w(kanji);
        }
    }
}

impl ToString for FuriToReadingParser<'_> {
    #[inline]
    fn to_string(&self) -> String {
        self.parse()
    }
}

impl Debug for FuriToReadingParser<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.parse())
    }
}

#[cfg(test)]
mod test {
    use super::FuriToReadingParser;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]が[好|す]き","おんがくがすき"; "parse to kana1")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ]が[A|えい]=[[1|],[2|]] = [[3|],[5|]]","2えっくす+1のていぎがえい=[1,2] = [3,5]"; "with brackets")]
    fn test_parse_to_kana(furi: &str, out: &str) {
        let parsed = FuriToReadingParser::new(furi, true).parse();
        assert_eq!(parsed, out);
    }

    #[test_case("[音楽|おん|がく]が[好|す]き","音楽が好き"; "parse to kanji1")]
    #[test_case("[[3|],[5|]]ああ","[3,5]ああ"; "parse to kanji2")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ]が[A|えい]=[[1|],[2|]] = [[3|],[5|]]","2x+1の定義がA=[1,2] = [3,5]"; "with brackets")]
    fn test_parse_to_kanji(furi: &str, out: &str) {
        let parsed = FuriToReadingParser::new(furi, false).parse();
        assert_eq!(parsed, out);
    }

    #[test]
    fn test_empty_kanji_block() {
        let s =
            "[高校生|こう|こう|せい]の[時|とき]は[毎朝|まい|あさ][6|][時|じ]に[起|お]きていた。";
        let (kana, _kanji) = FuriToReadingParser::parse_kanji_and_kana(s);
        assert_eq!(kana, "こうこうせいのときはまいあさ6じにおきていた。");
    }
}
