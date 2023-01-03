use super::reading_part_ref::ReadingPartRef;
use std::{iter::Peekable, str::CharIndices};

/// Returns an iterator over all parsed ReadingParts of the given input string
/// Encoded furigana: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
#[inline]
pub fn from_str(input: &str) -> FuriParseIter {
    FuriParseIter::new(input)
}

/// Similar to `from_str` but returns a Result with the already parsed parts instead of an
/// iterator and an Error when the input couldn't be parsed properly
#[inline]
pub fn full(input: &str) -> Result<Vec<ReadingPartRef>, ()> {
    from_str(input).collect::<Result<Vec<_>, _>>()
}

/// Similar to `full` but ignores parts that contain errors
#[inline]
pub fn unchecked(input: &str) -> Vec<ReadingPartRef> {
    from_str(input).filter_map(|i| i.ok()).collect()
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
    fn advance_chars(&mut self, is_kanji_block: bool) -> Result<Option<usize>, ()> {
        let mut last = self.char_iter.peek().map(|i| i.0);

        loop {
            let Some((_, nchar)) = self.char_iter.peek().copied() else {
                // We reached the end since we can't peek to the next character. This means we want
                // the substring include all chars until the end of the string
                return Ok(Some(self.inp.len()));
            };

            // We don't want to advance on '[' since this is needed for the next call
            if nchar == '[' {
                if is_kanji_block {
                    return Err(());
                }

                return Ok(last);
            }

            last = self.char_iter.next().map(|i| i.0);

            if nchar == ']' {
                if !is_kanji_block {
                    return Err(());
                }

                // Include ']' in returned position
                return Ok(last.map(|i| i + 1));
            }
        }
    }
}

impl<'a> Iterator for FuriParseIter<'a> {
    type Item = Result<ReadingPartRef<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let (nidx, nchar) = self.char_iter.next()?;
        let is_kanji_block = nchar == '[';
        let end = match self.advance_chars(is_kanji_block) {
            Ok(o) => o,
            Err(e) => return Some(Err(e)),
        }
        .unwrap_or_else(|| self.inp.len());

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
    #[test_case("[楽|たの]しい")]
    fn test_parse_furigana(furi: &str) {
        let parsed = from_str(furi).collect::<Result<Vec<_>, _>>().unwrap();
        let encoded = encode::sequence(&parsed);
        assert_eq!(furi, encoded);
    }

    #[test_case("[音楽|おん|がく]が[好す]き")]
    #[test_case("[音楽おん|がく]が[好す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎは[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|も|ん|だい]"; "other")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]]は[問題|も|ん|だい]"; "other2")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ|e]は[問題|もん|だい]")]
    #[test_case("[拝金主義|はい|]")]
    fn test_parse_furigana_error(furi: &str) {
        let parsed = from_str(furi).collect::<Result<Vec<_>, _>>();
        assert_eq!(parsed, Err(()));
    }
}
