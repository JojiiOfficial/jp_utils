pub mod check;
mod gen;
pub mod reading;

use super::segment::SegmentRef;
use gen::FuriParserGen;

/// Iterator over encoded furigana which returns ReadingPartRef's of all parts.
/// Encoded furigana format: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
pub struct FuriParser<'a> {
    gen_parser: FuriParserGen<'a>,
    checked: bool,
}

impl<'a> FuriParser<'a> {
    /// Creates a new furigana parser for the given string.
    #[inline]
    pub fn new(str: &'a str) -> Self {
        Self {
            checked: true,
            gen_parser: FuriParserGen::new(str),
        }
    }

    /// Don't checke content, just parse.
    pub fn unchecked(mut self) -> Self {
        self.checked = false;
        self
    }

    /// Returns `true` if the given furigana is parsable.
    pub fn check<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        let r = s.as_ref();
        FuriParser::new(r).all(|i| i.is_ok())
    }

    /// Parses the furigana to a vec of segments.
    pub fn to_vec(self) -> Result<Vec<SegmentRef<'a>>, ()> {
        self.collect()
    }

    /// Parses the furigana to a vec of segments without checking the input for valid furigana
    /// format.
    pub fn to_vec_unchecked(self) -> Vec<SegmentRef<'a>> {
        self.map(|i| i.unwrap()).collect()
    }
}

impl<'a> Iterator for FuriParser<'a> {
    type Item = Result<SegmentRef<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let (txt, kanji) = self.gen_parser.next()?;
        Some(SegmentRef::parse_str(txt, kanji, self.checked))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::furigana::segment::{encode, AsSegment, Segment};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("おんがくが[好|す]"; "End_kanji")]
    #[test_case("おんがくが[好|す]きです")]
    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい")]
    #[test_case("[音楽おん|がく]が[好す")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "with brackets")]
    fn test_parse_furigana(furi: &str) {
        let parsed = FuriParser::new(furi)
            .unchecked()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let encoded = encode::sequence(&parsed);
        assert_eq!(furi, encoded);
    }

    #[test_case("[音楽|おん|がく]が[好す]き")]
    #[test_case("[音楽おん|がく]が[好す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|も|ん|だい]"; "other")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]]は[問題|も|ん|だい]"; "other2")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ|e]は[問題|もん|だい]")]
    #[test_case("[拝金主義|はい|]")]
    fn test_parse_furigana_error(furi: &str) {
        let parsed = FuriParser::new(furi).collect::<Result<Vec<_>, _>>();
        assert_eq!(parsed, Err(()));
    }

    #[test]
    fn test_empty() {
        let e = Segment::from_str("").unwrap();
        assert!(e.is_empty());
    }

    #[test]
    fn test_all_sentences() {
        let data = File::open("./furigana.csv").unwrap();
        let reader = BufReader::new(data);
        for line in reader.lines() {
            let line = line.unwrap();
            let parsed = FuriParser::new(&line).collect::<Result<Vec<_>, _>>();
            if let Err(err) = parsed {
                println!("Error: {err:?} at line {:?}", line);
                continue;
            }
            let encoded = encode::sequence(&parsed.unwrap());
            assert_eq!(line, encoded);
        }
    }
}
