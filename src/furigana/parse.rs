use super::{reading_part::ReadingPart, reading_part_ref::ReadingPartRef, seq::FuriSequence};
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

/// Similar to `full` but returns a Furigana sequence
#[inline]
pub fn parse_seq(input: &str) -> Result<FuriSequence<ReadingPart>, ()> {
    from_str(input)
        .map(|i| i.map(|i| i.to_owned()))
        .collect::<Result<_, _>>()
}

/// Similar to `parse_seq` but returns borrowed items
#[inline]
pub fn parse_seq_ref(input: &str) -> Result<FuriSequence<ReadingPartRef>, ()> {
    from_str(input).collect::<Result<_, _>>()
}

/// Similar to `parse_seq` but returns borrowed items
#[inline]
pub fn parse_seq_ref_unchecked(input: &str) -> FuriSequence<ReadingPartRef> {
    FuriParseIter::new(input)
        .unchecked()
        .map(|i| i.unwrap())
        .collect()
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
    checked: bool,
    char_iter: Peekable<CharIndices<'a>>,
}

impl<'a> FuriParseIter<'a> {
    /// Create a new Furigana parse iterator that parses the given `inp` string
    #[inline]
    pub fn new(inp: &'a str) -> Self {
        let char_iter = inp.char_indices().peekable();
        Self {
            inp,
            char_iter,
            checked: true,
        }
    }

    /// Don't checke content just parse
    pub fn unchecked(mut self) -> Self {
        self.checked = false;
        self
    }

    /// Finds the last position of the item that is currently being parsed
    fn advance_chars(&mut self, is_kanji_block: bool) -> Result<usize, ()> {
        loop {
            let Some((curr_pos, peek_char)) = self.char_iter.peek().copied() else {
                // We reached the end since we can't peek to the next character. This means we want
                // the substring include all chars until the end of the string
                return Ok(self.inp.len());
            };

            // We don't want to advance on '[' since this is needed for the next call
            if peek_char == '[' {
                if is_kanji_block {
                    return Err(());
                }

                return Ok(curr_pos);
            }

            let last = self.char_iter.next().map(|i| i.0);

            if peek_char == ']' {
                if !is_kanji_block {
                    return Err(());
                }

                // Include ']' in returned position
                return Ok(last.unwrap() + 1);
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
        };

        let furi_part = &self.inp[nidx..end];
        if self.checked {
            Some(ReadingPartRef::from_str_checked(furi_part))
        } else {
            Some(Ok(ReadingPartRef::from_str(furi_part)))
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use super::super::encode;
    use super::*;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。")]
    fn test_parse_furigana(furi: &str) {
        let parsed = from_str(furi)
            .unchecked()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
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

    #[test]
    fn test_all_sentences() {
        let data = File::open("./furigana.csv").unwrap();
        let reader = BufReader::new(data);
        for line in reader.lines() {
            let line = line.unwrap();
            let parsed = from_str(&line).collect::<Result<Vec<_>, _>>();
            if let Err(err) = parsed {
                println!("Error: {err:?} at line {:?}", line);
                continue;
            }
            let encoded = encode::sequence(&parsed.unwrap());
            assert_eq!(line, encoded);
        }
    }
}
