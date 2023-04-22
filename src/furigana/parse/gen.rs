use std::str::MatchIndices;

/// Generic parser for furigana segments that only returns the parts as strings.
pub struct FuriParserGen<'a> {
    // Input
    str: &'a str,

    // Tmp
    iter: MatchIndices<'a, [char; 2]>,
    kana_start: usize,
    block_start: Option<usize>,
    buf: Option<(&'a str, bool)>,
}

impl<'a> FuriParserGen<'a> {
    /// Create a new generalized furigana parser.
    #[inline]
    pub fn new(str: &'a str) -> Self {
        Self {
            str,
            kana_start: 0,
            block_start: None,
            buf: None,
            iter: str.match_indices(['[', ']']),
        }
    }

    fn advance(&mut self) -> Option<(&'a str, bool)> {
        loop {
            let (cur_bracket, c) = match self.iter.next() {
                Some(k) => k,
                None => {
                    if self.kana_start < self.str.len() {
                        let kana_text = &self.str[self.kana_start..];
                        self.kana_start = self.str.len();
                        return Some((kana_text, false));
                    }

                    return None;
                }
            };

            // Hack to check if current bracket is a '[' bracket
            if unsafe { *c.as_bytes().get_unchecked(0) } == 91 {
                self.block_start = Some(cur_bracket);
                continue;
            }

            let Some(prev_bracket) = self.block_start.take() else { continue };

            let kanji = &self.str[prev_bracket..cur_bracket + 1];

            let mut to_return = Some((kanji, true));

            if self.kana_start < prev_bracket {
                self.buf = to_return.take();
                let kana_text = &self.str[self.kana_start..prev_bracket];
                to_return = Some((kana_text, false));
            }

            self.kana_start = cur_bracket + 1;
            return to_return;
        }
    }
}

impl<'a> Iterator for FuriParserGen<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.buf.take() {
            return Some(t);
        }
        self.advance()
    }
}
