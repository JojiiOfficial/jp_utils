use super::{
    segment::{kanji::as_kanji::AsKanjiSegment, AsSegment},
    seq::FuriSequence,
};

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

    /// Check if two FuriSequences are equal
    pub fn eq_seq<L: AsSegment, R: AsSegment>(
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

    pub fn eq<L: AsSegment, R: AsSegment>(&self, left: &L, right: &R) -> bool
    where
        L::StrType: PartialEq<R::StrType>,
    {
        if self.lit_match {
            left.as_kanji().map(|i| i.literals().as_ref())
                == right.as_kanji().map(|i| i.literals().as_ref())
                && left.as_kana().map(|i| i.as_ref()) == right.as_kana().map(|i| i.as_ref())
        } else {
            left.main_reading() == right.main_reading()
                && left.get_kana_reading() == right.get_kana_reading()
        }
    }

    #[inline]
    fn eq_seq_no_lit_match<L: AsSegment, R: AsSegment>(
        &self,
        left: &FuriSequence<L>,
        right: &FuriSequence<R>,
    ) -> bool {
        left.as_kana() == right.as_kana() && left.as_kanji() == right.as_kanji()
    }

    fn eq_seq_lit_match<L: AsSegment, R: AsSegment>(
        &self,
        left: &FuriSequence<L>,
        right: &FuriSequence<R>,
    ) -> bool {
        let mut l_iter = left.iter().flat_map(|i| i.reading_iter());
        let mut r_iter = right.iter().flat_map(|i| i.reading_iter());
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::furi::seq::FuriSequence;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", "[音|おん][楽|がく]", true)]
    #[test_case("[音楽|おん|がく]", "[音|おん][楽|がく]", false)]
    #[test_case("[音楽|おん|がく]", "[音楽|おんがく]", false)]
    fn test_eq(a: &str, b: &str, lit_match: bool) {
        let a = FuriSequence::from_str(a).unwrap();
        let b = FuriSequence::from_str(b).unwrap();
        assert!(FuriComparator::new(lit_match).eq_seq(&a, &b));
    }

    #[test_case("[音楽|おん|がく]", "[音楽|おんがく]", true)]
    fn test_not_eq(a: &str, b: &str, lit_match: bool) {
        let a = FuriSequence::from_str(a).unwrap();
        let b = FuriSequence::from_str(b).unwrap();
        assert!(!FuriComparator::new(lit_match).eq_seq(&a, &b));
    }
}
