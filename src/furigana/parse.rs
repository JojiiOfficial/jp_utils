use super::reading_part_ref::ReadingPartRef;
use std::{iter::Peekable, str::CharIndices};

/// Returns an iterator over all parsed ReadingParts of the given input string
/// Encoded furigana: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
#[inline]
pub fn from_str(input: &str) -> FuriParseIter {
    FuriParseIter::new(input)
}

/// Iterator over encoded furigana which returns ReadingPartRef's of all parts
/// Encoded furigana: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
pub struct FuriParseIter<'a> {
    inp: &'a str,
    char_iter: Peekable<CharIndices<'a>>,
}

impl<'a> FuriParseIter<'a> {
    /// Create a new Furigana parse iterator that parses the given `inp` string
    #[inline]
    pub fn new(inp: &'a str) -> Self {
        let char_iter = inp.char_indices().peekable();
        Self { inp, char_iter }
    }

    /// Finds the last position of the item that is currently being parsed
    fn advance_chars(&mut self) -> Option<usize> {
        let mut last = self.char_iter.peek().map(|i| i.0);

        loop {
            let Some((_, nchar)) = self.char_iter.peek().copied() else {
                return last;
            };

            // We don't want to advance on '[' since this is needed for the next call
            if nchar == '[' {
                return last;
            }

            last = self.char_iter.next().map(|i| i.0);

            if nchar == ']' {
                // Include ']' in returned position
                return last.map(|i| i + 1);
            }
        }
    }
}

impl<'a> Iterator for FuriParseIter<'a> {
    type Item = ReadingPartRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (nidx, _) = self.char_iter.next()?;
        let end = self.advance_chars().unwrap_or_else(|| self.inp.len());
        let furi_part = &self.inp[nidx..end];
        Some(ReadingPartRef::from_str(furi_part))
    }
}

#[cfg(test)]
mod test {
    use super::super::encode;
    use super::*;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    fn test_parse_furigana2(furi: &str) {
        let parsed: Vec<_> = from_str(furi).collect();
        let encoded = encode::sequence(&parsed);
        assert_eq!(furi, encoded);
    }
}
