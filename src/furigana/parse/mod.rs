pub mod check;
mod gen;
pub mod reading;
pub mod unchecked;

pub use gen::FuriParserGen;

use self::unchecked::UncheckedFuriParser;
use super::segment::SegmentRef;
use crate::reading::Reading;

/// Iterator over encoded furigana which returns ReadingPartRef's of all parts.
/// Encoded furigana format: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
pub struct FuriParser<'a> {
    gen_parser: FuriParserGen<'a>,
}

impl<'a> FuriParser<'a> {
    /// Creates a new furigana parser for the given string.
    #[inline]
    pub fn new(str: &'a str) -> Self {
        Self {
            gen_parser: FuriParserGen::new(str),
        }
    }

    /// Returns an iterator over all parsed segments without doing any checks. Unparsable segments
    /// may be parsed as kana part as fallback.
    #[inline]
    pub fn unchecked(self) -> UncheckedFuriParser<'a> {
        UncheckedFuriParser::new(self.gen_parser)
    }

    /// Parses a single string segment
    #[inline]
    pub fn from_seg_str(txt: &'a str, kanji: bool) -> Result<SegmentRef, ()> {
        SegmentRef::parse_str(txt, kanji, true)
    }

    /// Returns `true` if the given furigana is parsable.
    pub fn check<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        FuriParser::new(s.as_ref()).all(|i| i.is_ok())
    }

    /// Parses the furigana to a vec of segments.
    #[inline]
    pub fn to_vec(self) -> Result<Vec<SegmentRef<'a>>, ()> {
        self.collect()
    }

    /// Parses a string to a [`Reading`]. This is slower than the unchecked version as it does
    /// checks and allocates each segment before allocating the reading.
    #[inline]
    pub fn to_reading(self) -> Result<Reading, ()> {
        self.collect()
    }
}

impl<'a> Iterator for FuriParser<'a> {
    type Item = Result<SegmentRef<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let (txt, kanji) = self.gen_parser.next()?;
        Some(Self::from_seg_str(txt, kanji))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::furigana::segment::{AsSegment, Segment};
    use crate::furigana::seq::FuriSequence;
    use crate::furigana::Furigana;
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
        let parsed = FuriParser::new(furi).to_vec().unwrap();
        // let encoded = encode::sequence(&parsed);
        let encoded = Furigana::from_iter(parsed.iter());
        assert_eq!(encoded, furi);
        let seq = FuriSequence::from(parsed);
        assert_eq!(Furigana(furi).to_reading(), seq.to_reading());
    }

    #[test_case("おんがくが[好|す]"; "End_kanji")]
    #[test_case("おんがくが[好|す]きです")]
    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい")]
    #[test_case("[音楽おん|がく]が[好す")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "with brackets")]
    fn test_to_reading(furi: &str) {
        let reading = FuriParser::new(furi).to_reading().unwrap();
        let exp = FuriSequence::parse_ref(furi).unwrap().to_reading();
        assert_eq!(reading, exp);
        let furigana = Furigana(furi);
        assert_eq!(furigana.to_reading(), reading);
    }

    #[test]
    fn test_empty() {
        let e = Segment::from_str("").unwrap();
        assert!(e.is_empty());

        let mut p = FuriParser::new("");
        assert_eq!(p.next(), None);
    }

    #[test]
    fn test_all_sentences() {
        let data = File::open("./furigana.csv").expect(
            "No furigana file found! Place tatoebas furigana file converted in ./furigana.csv",
        );
        let reader = BufReader::new(data);
        for line in reader.lines() {
            let line = line.unwrap();
            let parsed = FuriParser::new(&line).to_vec();
            if let Err(err) = parsed {
                println!("Error: {err:?} at line {:?}", line);
                continue;
            }
            let parsed = parsed.unwrap();
            let encoded = Furigana::from_iter(parsed.iter());
            // let encoded = encode::sequence(&parsed);
            assert_eq!(encoded, line);

            let seq = FuriSequence::from(parsed);
            assert_eq!(seq.to_reading(), Furigana(&line).to_reading());

            assert_eq!(
                Furigana(&line).to_reading().kana(),
                Furigana(&line).kana_str()
            );

            let reading = Furigana(&line).code_formatter().apply_all().to_reading();
            if reading.kana() != Furigana(&line).kana_str() {
                println!("furi: {:?}", Furigana(&line).raw());
                println!("old: {:?}", Furigana(&line).kana_str());
                println!("new: {:?}", reading.kana());
                panic!("err: {line:?}");
            }
            // assert_eq!(reading.kana(), Furigana(&line).kana_str());
            if reading.has_kanji() {
                assert_eq!(reading.kanji().unwrap(), Furigana(&line).kanji_str());
            }
        }
    }
}
