use super::{AsSegment, Segment};
use tinyvec::{tiny_vec, TinyVec};

/// Same as [`Segment`] but borrowed
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum SegmentRef<'a> {
    // Kana reading
    Kana(&'a str),

    // Kanji reading with assigned kana readings
    Kanji {
        kanji: &'a str,
        readings: TinyVec<[&'a str; 1]>,
    },
}

impl<'a> SegmentRef<'a> {
    /// Creates a new ReadingPartRef
    #[inline]
    pub fn new_kana(kana: &'a str) -> Self {
        Self::Kana(kana)
    }

    /// Creates a new ReadingPartRef with a value for kanji
    #[inline]
    pub fn new_kanji(kana: &'a str, kanji: &'a str) -> Self {
        Self::Kanji {
            kanji,
            readings: tiny_vec!([&'a str;1] => kana),
        }
    }

    /// Returns an owned ReadingPart
    #[inline]
    pub fn to_owned(&self) -> Segment {
        match self {
            SegmentRef::Kana(k) => Segment::Kana(k.to_string()),
            SegmentRef::Kanji { kanji, readings } => {
                let readings: TinyVec<[String; 1]> =
                    readings.iter().map(|i| i.to_string()).collect();
                Segment::Kanji {
                    kanji: kanji.to_string(),
                    readings,
                }
            }
        }
    }

    /// Parses a ReadingPart from string
    pub fn parse_str(str: &'a str, kanji: bool, checked: bool) -> Result<SegmentRef, ()> {
        if kanji {
            Self::parse_kanji_str(str, checked).ok_or(())
        } else {
            Ok(SegmentRef::Kana(str))
        }
    }

    /// Parses a ReadingPart from string
    pub fn from_str_checked(str: &'a str) -> Result<SegmentRef, ()> {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, true).ok_or(())
        } else {
            Ok(SegmentRef::Kana(str))
        }
    }

    /// Parses a ReadingPart from string
    pub fn from_str_unchecked(str: &'a str) -> SegmentRef {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, false).unwrap()
        } else {
            SegmentRef::Kana(str)
        }
    }

    /// Parses a ReadingPart from string
    pub fn from_str_2(str: &'a str) -> SegmentRef {
        Self::parse_kanji_str(str, false).unwrap()
    }

    /// Parses an encoded Kanji furigana string eg: `[音楽|おん|がく]` thus `s` has to start with
    /// `[` and end  with `]`. If the readings don't line up with the kanji literal count and has
    /// are more than 1 (fallback) the function returns None.
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
            return Some(SegmentRef::new_kanji(readings[0], kanji));
        } else if checked && kanji.chars().count() != readings.len() {
            // Malformed kanji string
            return None;
        }

        Some(SegmentRef::Kanji { kanji, readings })
    }
}

impl<'a> ToString for SegmentRef<'a> {
    #[inline]
    fn to_string(&self) -> String {
        self.encode()
    }
}

impl<'a> AsSegment for SegmentRef<'a> {
    type StrType = &'a str;

    /// Returns `true` if ReadingPart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            SegmentRef::Kana(k) => k.is_empty(),
            SegmentRef::Kanji { kanji, readings } => readings.is_empty() || kanji.is_empty(),
        }
    }

    /// Returns `true` if the reading part ref is a kana reading.
    #[inline]
    fn is_kana(&self) -> bool {
        matches!(self, Self::Kana(..))
    }

    /// Returns `true` if the reading part ref is a kanji reading.
    #[inline]
    fn is_kanji(&self) -> bool {
        matches!(self, Self::Kanji { .. })
    }

    /// Returns the kana reading
    #[inline]
    fn as_kana(&self) -> Option<&Self::StrType> {
        match self {
            SegmentRef::Kana(k) => Some(k),
            SegmentRef::Kanji { .. } => None,
        }
    }

    /// Returns the kanji reading if exists
    #[inline]
    fn as_kanji(&self) -> Option<&Self::StrType> {
        match self {
            SegmentRef::Kana(_) => None,
            SegmentRef::Kanji { kanji, readings: _ } => Some(kanji),
        }
    }

    /// Returns the kanji readings
    #[inline]
    fn readings(&self) -> Option<&TinyVec<[Self::StrType; 1]>> {
        match self {
            SegmentRef::Kana(_) => None,
            SegmentRef::Kanji { kanji: _, readings } => Some(readings),
        }
    }

    /// Sets the kanji reading or converts it to one
    fn set_kanji(&mut self, s: Self::StrType) {
        match self {
            SegmentRef::Kana(k) => {
                *self = Self::new_kanji(s, k);
            }
            SegmentRef::Kanji { kanji, readings: _ } => *kanji = s,
        }
    }

    #[inline]
    fn set_kana(&mut self, s: Self::StrType) {
        if let SegmentRef::Kana(k) = self {
            *k = s
        }
    }

    #[inline]
    fn add_reading(&mut self, r: Self::StrType) {
        if let SegmentRef::Kanji { kanji: _, readings } = self {
            readings.push(r);
        }
    }

    #[inline]
    fn kana_reading(&self) -> String {
        match self {
            SegmentRef::Kana(k) => k.to_string(),
            SegmentRef::Kanji { kanji: _, readings } => readings.join(""),
        }
    }
}

impl<'a> From<&'a Segment> for SegmentRef<'a> {
    #[inline]
    fn from(r: &'a Segment) -> Self {
        match r {
            Segment::Kana(k) => Self::Kana(k),
            Segment::Kanji { kanji, readings } => {
                let readings: TinyVec<[&str; 1]> = readings.iter().map(|i| i.as_str()).collect();
                Self::Kanji { kanji, readings }
            }
        }
    }
}

impl<'a> From<&'a str> for SegmentRef<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::new_kana(s)
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for SegmentRef<'a> {
    #[inline]
    fn from(s: (&'a str, Option<&'a str>)) -> Self {
        if let Some(kanji) = s.1 {
            Self::new_kanji(s.0, kanji)
        } else {
            Self::Kana(s.0)
        }
    }
}

impl<'a> From<(&'a str, Vec<&'a str>)> for SegmentRef<'a> {
    #[inline]
    fn from(s: (&'a str, Vec<&'a str>)) -> Self {
        Self::Kanji {
            kanji: s.0,
            readings: s.1.into_iter().collect(),
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for SegmentRef<'a> {
    #[inline]
    fn from(s: (&'a str, &'a str)) -> Self {
        Self::new_kanji(s.0, s.1)
    }
}

impl<'a> PartialEq<Segment> for SegmentRef<'a> {
    #[inline]
    fn eq(&self, other: &Segment) -> bool {
        other.eq(self)
    }
}

impl<'a> PartialEq<Segment> for &SegmentRef<'a> {
    #[inline]
    fn eq(&self, other: &Segment) -> bool {
        other.eq(*self)
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for SegmentRef<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encode())
    }
}

#[cfg(feature = "serde")]
impl<'a, 'de: 'a> serde::Deserialize<'de> for SegmentRef<'a>
where
    'a: 'de,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(RpDeser)
    }
}

#[cfg(feature = "serde")]
struct RpDeser;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for RpDeser {
    type Value = SegmentRef<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected string in furigana format!")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SegmentRef::from_str_unchecked(v))
    }
}
