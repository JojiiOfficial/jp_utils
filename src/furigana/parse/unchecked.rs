use super::super::segment::SegmentRef;
use super::gen::FuriParserGen;
use crate::reading::Reading;

/// Iterator over encoded furigana which returns ReadingPartRef's of all parts without checking for
/// validity.
pub struct UncheckedFuriParser<'a> {
    gen_parser: FuriParserGen<'a>,
}

impl<'a> UncheckedFuriParser<'a> {
    /// Creates a new furigana parser for the given string.
    #[inline]
    pub(super) fn new(gen_parser: FuriParserGen<'a>) -> Self {
        Self { gen_parser }
    }

    /// Parses the furigana to a vec of segments.
    #[inline]
    pub fn to_vec(self) -> Vec<SegmentRef<'a>> {
        self.collect()
    }

    /// Parses a string to a [`Reading`].
    #[inline]
    pub fn to_reading(self) -> Reading {
        self.collect()
    }

    /// Parses a single string segment.
    #[inline]
    pub fn from_seg_str(txt: &'a str, kanji: bool) -> SegmentRef {
        SegmentRef::parse_str(txt, kanji, false).unwrap()
    }
}

impl<'a> Iterator for UncheckedFuriParser<'a> {
    type Item = SegmentRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.gen_parser
            .next()
            .map(|(txt, kanji)| Self::from_seg_str(txt, kanji))
    }
}

#[cfg(test)]
mod test {
    use crate::furigana::parse::FuriParser;
    use test_case::test_case;

    #[test_case("[おんがく]"; "single kana in kanji brackets")]
    #[test_case("[おんがく|]";"Kana in kanji with space")]
    #[test_case("[音楽]";"kanji")]
    #[test_case("[音楽|]";"kanji2")]
    #[test_case("[]";"empty kanji")]
    fn test_err(furi: &str) {
        // Check that it doesn't unwrap or panic
        FuriParser::new(furi).unchecked().to_vec();
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
}
