use super::{
    kanji::{as_kanji::AsKanjiRef, Kanji},
    s_ref::SegmentRef,
    traits::{AsSegment, AsSegmentRef},
};
use std::str::FromStr;

/// A single segment of a Furigana formatted string. Either holds a Kana or Kanji part.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Segment {
    Kana(String),
    Kanji(Kanji),
}

impl Segment {
    /// Create a new kana Segment.
    #[inline]
    pub fn new_kana(kana: String) -> Self {
        Self::Kana(kana)
    }

    /// Create a new kanji Segment.
    #[inline]
    pub fn new_kanji(lits: String, readings: &[String]) -> Self {
        Self::Kanji(Kanji::new(lits, readings))
    }
}

impl<'a> AsSegmentRef<'a> for &'a Segment {
    #[inline]
    fn as_seg_ref(&self) -> SegmentRef<'a> {
        match self {
            Segment::Kana(k) => SegmentRef::Kana(k),
            Segment::Kanji(k) => SegmentRef::Kanji(k.as_kanji_ref()),
        }
    }
}

impl AsSegment for Segment {
    type StrType = String;
    type KanjiType = Kanji;

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

impl FromStr for Segment {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: find a better way to do this
        SegmentRef::from_str_checked(s).map(|i| i.to_owned())
    }
}

impl<'a> PartialEq<SegmentRef<'a>> for Segment {
    fn eq(&self, other: &SegmentRef<'a>) -> bool {
        match (self, other) {
            (Segment::Kana(k1), SegmentRef::Kana(k2)) => k1 == k2,
            (Segment::Kanji(k1), SegmentRef::Kanji(k2)) => k1 == k2,
            _ => false,
        }
    }
}

impl<'a> PartialEq<SegmentRef<'a>> for &'a Segment {
    #[inline]
    fn eq(&self, other: &SegmentRef<'a>) -> bool {
        (*self).eq(other)
    }
}
