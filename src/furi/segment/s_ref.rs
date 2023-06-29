use super::{kanji::KanjiRef, traits::AsSegment, Segment};
use tinyvec::TinyVec;

/// A single segment of a Furigana formatted string. Either holds a Kana or Kanji part.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Create a new kanji Segment.
    #[inline]
    pub fn new_kanji_raw(lits: &'a str, readings: TinyVec<[&'a str; 1]>) -> Self {
        Self::Kanji(KanjiRef::new_raw(lits, readings))
    }

    /// Parses a ReadingPart from string
    pub fn from_str_checked(str: &'a str) -> Result<SegmentRef, ()> {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, true).ok_or(())
        } else {
            Ok(SegmentRef::Kana(str))
        }
    }

    /// Parses a `SegmentRef` from string
    pub fn from_str_unchecked(str: &'a str) -> SegmentRef {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, false).unwrap()
        } else {
            SegmentRef::Kana(str)
        }
    }

    /// Parses an encoded Kanji furigana segment eg: `[音楽|おん|がく]`.
    /// Multiple kanji literals with a single reading are allowed.
    /// Is `check` == `true` the literals and kanji readings have to match up (except if there is only
    /// one reading) and there has to be at least a single reading. If `check` == `false` no
    /// checks a made and a parsed Segment will always be returned.
    fn parse_kanji_str(s: &'a str, checked: bool) -> Option<SegmentRef> {
        // Strip [ and ] and split at the |
        let mut split = s[1..s.len() - 1].split('|');

        // First item is the kanji reading
        let kanji = split.next()?;

        let readings = split.collect::<TinyVec<[&str; 1]>>();
        if readings.is_empty() && checked {
            return None;
        }

        if readings.len() == 1 {
            // Fallback where all kanji get the first reading assigned
            return Some(SegmentRef::new_kanji_raw(kanji, readings));
        } else if checked && kanji.chars().count() != readings.len() {
            // Malformed kanji string
            return None;
        }

        Some(SegmentRef::Kanji(KanjiRef::new_raw(kanji, readings)))
    }

    /// Converts the SegmentRef to a Segment.
    #[inline]
    pub fn to_owned(&self) -> Segment {
        match self {
            SegmentRef::Kana(k) => Segment::Kana(k.to_string()),
            SegmentRef::Kanji(k) => Segment::Kanji(k.to_owned()),
        }
    }

    /// Parses a ReadingPart from string with `kanji` as parameter to give a hint whether its a
    /// kanji or kana segment. This avoids additional checks.
    pub(crate) fn parse_str(str: &'a str, kanji: bool, checked: bool) -> Result<SegmentRef, ()> {
        if kanji {
            Self::parse_kanji_str(str, checked).ok_or(())
        } else {
            Ok(SegmentRef::Kana(str))
        }
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

impl<'a> PartialEq<Segment> for SegmentRef<'a> {
    #[inline]
    fn eq(&self, other: &Segment) -> bool {
        other.eq(self)
    }
}

impl<'a> PartialEq<Segment> for &'a SegmentRef<'a> {
    #[inline]
    fn eq(&self, other: &Segment) -> bool {
        other.eq(*self)
    }
}
