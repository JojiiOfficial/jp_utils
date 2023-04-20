use super::part::ReadingPartRef;
use std::{iter::Peekable, str::CharIndices};

/// Parses an encoded furigana string into its kana or kanji reading efficiently.
pub struct FuriToReadingParser<'a> {
    str: &'a str,
    char_iter: Peekable<CharIndices<'a>>,
    out: String,
    to_kana: bool,
}

impl<'a> FuriToReadingParser<'a> {
    /// Create a new Furigana parse iterator that parses the given `inp` string
    #[inline]
    pub fn new(str: &'a str, to_kana: bool) -> Self {
        let char_iter = str.char_indices().peekable();
        Self {
            str: str.trim(),
            char_iter,
            out: String::with_capacity(str.len()),
            to_kana,
        }
    }

    /// Parses the furigana to either kana or kanji.
    pub fn parse(mut self) -> String {
        while let Some(curr) = self.char_iter.next() {
            if curr.1 == '[' {
                self.accept_kanji(curr);
            } else {
                self.accept_kana(curr);
            }
        }

        self.out
    }

    /// Parses the next part as kana.
    fn accept_kana(&mut self, curr: (usize, char)) {
        let (start, first_c) = curr;

        // Get position of window until the next occurence of `[`.
        let end = match take_while(&mut self.char_iter, |i| i.1 != '[').last() {
            Some(end) => end.0 + end.1.len_utf8(),
            None => start + first_c.len_utf8(),
        };

        self.out.push_str(&self.str[start..end]);
    }

    /// Parses the next part as kanji.
    fn accept_kanji(&mut self, curr: (usize, char)) {
        let (start, _) = curr;

        // TODO: find full window or something idk
        // Advance the internal char iter to the position of the last character before the kanji
        // window ends.
        take_while(&mut self.char_iter, |i| i.1 != ']').last();

        // Also include the last `]`. This fails if a kanji part is not properly closed and thus
        // retruns an error in that case.
        let end = match self.char_iter.next() {
            Some(s) => s.0 + s.1.len_utf8(),
            None => {
                self.out.push_str(&self.str[start..]);
                return;
            }
        };

        let block = &self.str[start..end];
        // let block_inner = block.trim_matches(|c| c == '[' || c == ']');
        let block_inner = my_trim(block, '[', ']');
        // println!("{block_inner}");

        // Removes []-brackets and splits at '|'
        let mut split = block_inner.split('|');

        if self.to_kana {
            // Skip kanji reading
            for s in split.skip(1) {
                self.out.push_str(s);
            }
        } else {
            // Push only first kanji reading
            self.out.push_str(split.next().unwrap());
        }
    }
}

fn my_trim(inp: &str, l: char, r: char) -> &str {
    let mut sub = inp;
    println!("{inp}");
    // sub.trim_matches(|c| c == '[' || c == ']')
    sub
}

/// Iterator over encoded furigana which returns ReadingPartRef's of all parts
/// Encoded furigana: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]`
pub struct FuriParser<'a> {
    str: &'a str,
    checked: bool,
    char_iter: Peekable<CharIndices<'a>>,
}

impl<'a> FuriParser<'a> {
    /// Create a new Furigana parse iterator that parses the given `inp` string
    #[inline]
    pub fn new(str: &'a str) -> Self {
        let char_iter = str.char_indices().peekable();
        Self {
            str: str.trim(),
            char_iter,
            checked: true,
        }
    }

    /// Returns `true` if the given furigana is parsable.
    pub fn check<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        let r = s.as_ref();
        FuriParser::new(r).all(|i| i.is_ok())
    }

    /// Don't checke content just parse
    pub fn unchecked(mut self) -> Self {
        self.checked = false;
        self
    }

    /// Parses the next part as kana.
    fn accept_kana(&mut self, curr: (usize, char)) -> ReadingPartRef<'a> {
        let (start, first_c) = curr;

        // Get position of window until the next occurence of `[`.
        let end = match take_while(&mut self.char_iter, |i| i.1 != '[').last() {
            Some(end) => end.0 + end.1.len_utf8(),
            None => start + first_c.len_utf8(),
        };

        ReadingPartRef::Kana(&self.str[start..end])
    }

    /// Parses the next part as kanji.
    fn accept_kanji(
        &mut self,
        curr: (usize, char),
    ) -> Result<ReadingPartRef<'a>, ReadingPartRef<'a>> {
        let (start, _) = curr;

        // Advance the internal char iter to the position of the last character before the kanji
        // window ends.
        take_while(&mut self.char_iter, |i| i.1 != ']').last();

        // Also include the last `]`. This fails if a kanji part is not properly closed and thus
        // retruns an error in that case.
        let end = match self.char_iter.next() {
            Some(s) => s.0 + s.1.len_utf8(),
            None => return Err(ReadingPartRef::Kana(&self.str[start..])),
        };

        let sub = &self.str[start..end];
        ReadingPartRef::from_str_checked(sub).map_err(|_| ReadingPartRef::from_str_unchecked(sub))
    }
}

impl<'a> Iterator for FuriParser<'a> {
    type Item = Result<ReadingPartRef<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.char_iter.next()?;

        // Check if we need to parse kanji for this part.
        if curr.1 == '[' {
            let kanji = match self.accept_kanji(curr) {
                Ok(k) => k,
                Err(k) => {
                    if self.checked {
                        return Some(Err(()));
                    } else {
                        k
                    }
                }
            };
            return Some(Ok(kanji));
        }

        // Parse kana instead
        Some(Ok(self.accept_kana(curr)))
    }
}

fn take_while<'a, U, P>(
    iter: &'a mut Peekable<U>,
    mut pred: P,
) -> impl Iterator<Item = U::Item> + 'a
where
    U: Iterator,
    P: FnMut(&U::Item) -> bool + 'a,
{
    std::iter::from_fn(move || {
        if pred(iter.peek()?) {
            iter.next()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::furigana::part::{encode, AsPart, ReadingPart};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("おんがくが[好|す]"; "End_kanji")]
    #[test_case("おんがくが[好|す]きです")]
    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[音楽|おん|がく]が[好|す]きです")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "with brackets")]
    fn test_parse_furigana2(furi: &str) {
        let parsed = FuriParser::new(furi)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let encoded = encode::sequence(&parsed);
        assert_eq!(furi, encoded);
    }

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
    #[test_case("[音楽おん|がく]が[好す")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎは[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
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
        let e = ReadingPart::from_str("").unwrap();
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

    #[test_case("[音楽|おん|がく]が[好|す]き","おんがくがすき"; "parse to kana1")]
    fn test_parse_to_kana(furi: &str, out: &str) {
        let parsed = FuriToReadingParser::new(furi, true).parse();
        assert_eq!(parsed, out);
    }

    #[test_case("[音楽|おん|がく]が[好|す]き","音楽が好き"; "parse to kanji1")]
    #[test_case("[[3|],[5|]]ああ",""; "parse to kanji2")]
    fn test_parse_to_kanji(furi: &str, out: &str) {
        let parsed = FuriToReadingParser::new(furi, false).parse();
        assert_eq!(parsed, out);
        panic!("");
    }
}
