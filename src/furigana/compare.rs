use super::{as_part::AsPart, seq::FuriSequence};

/// Comparator for furigana blocks
pub struct FuriComparator {
    /// Whether the kanji literals have to match the readings exactly.
    lit_match: bool,
}

impl FuriComparator {
    /// Creates a new comparator for furigana parts.
    #[inline]
    pub fn new(lit_match: bool) -> Self {
        Self { lit_match }
    }

    /// `[音楽|おんがく]`
    /// `[音楽|おん|がく]`
    /// `[音|おん][楽|がく]`
    pub fn eq_seq<L: AsPart, R: AsPart>(
        &self,
        left: &FuriSequence<L>,
        right: &FuriSequence<R>,
    ) -> bool {
        if self.lit_match {
            self.eq_seq_lit_match(left, right)
        } else {
            self.eq_seq_no_lit_match(left, right)
        }
    }

    pub fn eq_seq_no_lit_match<L: AsPart, R: AsPart>(
        &self,
        left: &FuriSequence<L>,
        right: &FuriSequence<R>,
    ) -> bool {
        /* let mut l_iter = left.iter().map(|i| i.reading_iter()).flatten();
        let mut r_iter = right.iter().map(|i| i.reading_iter()).flatten();
        loop {
            let (l, r) = match (l_iter.next(), r_iter.next()) {
                (None, None) => break,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(l), Some(r)) => {
                    if l == r {
                        continue;
                    }
                    (l, r)
                }
            };

            //
        } */
        true
    }

    pub fn eq_seq_lit_match<L: AsPart, R: AsPart>(
        &self,
        left: &FuriSequence<L>,
        right: &FuriSequence<R>,
    ) -> bool {
        let mut l_iter = left.iter().map(|i| i.reading_iter()).flatten();
        let mut r_iter = right.iter().map(|i| i.reading_iter()).flatten();
        loop {
            match (l_iter.next(), r_iter.next()) {
                (None, None) => break,
                (None, Some(_)) | (Some(_), None) => return false,
                (Some(l), Some(r)) => {
                    if l != r {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn eq<L: AsPart, R: AsPart>(&self, left: &L, right: &R) -> bool {
        true
    }
}
