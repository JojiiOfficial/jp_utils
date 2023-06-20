use super::{kanji::KanjiRef, traits::AsSegment};

#[derive(Clone, Debug)]
pub enum SegmentRef<'a> {
    Kana(&'a str),
    Kanji(KanjiRef<'a>),
}

impl<'a> SegmentRef<'a> {
    /// Create a new kana Segment.
    #[inline]
    pub fn new_kana(kana: &'a str) -> Self {
        Self::Kana(kana)
    }

    /// Create a new kanji Segment.
    #[inline]
    pub fn new_kanji(lits: &'a str, readings: &[&'a str]) -> Self {
        Self::Kanji(KanjiRef::new(lits, readings))
    }
}

impl<'a> AsSegment for SegmentRef<'a> {
    type StrType = &'a str;
    type KanjiType = KanjiRef<'a>;

    #[inline]
    fn is_kana(&self) -> bool {
        matches!(self, Self::Kana(..))
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        matches!(self, Self::Kanji(..))
    }

    #[inline]
    fn as_kana(&self) -> Option<&Self::StrType> {
        if let Self::Kana(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    fn as_kanji(&self) -> Option<&Self::KanjiType> {
        if let Self::Kanji(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
