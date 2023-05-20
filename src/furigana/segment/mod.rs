pub mod as_segment;
pub mod encode;
pub mod iter;
mod seg_ref;

pub use as_segment::AsSegment;
pub use iter::{
    flatten::{FlattenIter, FlattenKajiIter},
    ReadingIter,
};
pub use seg_ref::SegmentRef;

use self::as_segment::AsSegmentMut;
use std::str::FromStr;
use tinyvec::{tiny_vec, TinyVec};

/// Represents a single segment of a furigana string. This can be a kana or kanji segment. Kanji
/// segments also save the assigned kana readings.
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Segment {
    // Kana reading
    Kana(String),

    // Kanji reading with assigned kana readings
    Kanji {
        kanji: String,
        readings: TinyVec<[String; 1]>,
    },
}

impl Segment {
    /// Create a new `SentencePart` with kana only
    #[inline]
    pub fn new_kana(kana: String) -> Self {
        Self::Kana(kana)
    }

    /// Create a new `SentencePart` with kanji value
    #[inline]
    pub fn new_kanji(kanji: String, kana: String) -> Self {
        Self::Kanji {
            kanji,
            readings: tiny_vec!([String; 1] => kana),
        }
    }

    /// Returns the reading part as a reference
    #[inline]
    pub fn as_ref(&self) -> SegmentRef {
        self.into()
    }

    /// Parses a ReadingPart from string
    #[inline]
    pub fn from_str_unchecked(s: &str) -> Segment {
        // TODO: find a better way to do this
        SegmentRef::from_str_unchecked(s).to_owned()
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

impl ToString for Segment {
    #[inline]
    fn to_string(&self) -> String {
        self.encode()
    }
}

impl AsSegment for Segment {
    type StrType = String;

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            Segment::Kana(k) => k.is_empty(),
            Segment::Kanji { kanji, readings } => readings.is_empty() || kanji.is_empty(),
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
    fn as_kana(&self) -> Option<&String> {
        match self {
            Segment::Kana(k) => Some(k),
            Segment::Kanji { .. } => None,
        }
    }

    #[inline]
    fn kana_reading(&self) -> String {
        match self {
            Segment::Kana(k) => k.to_string(),
            Segment::Kanji { kanji: _, readings } => readings.join(""),
        }
    }

    /// Returns the kanji reading if exists
    #[inline]
    fn as_kanji(&self) -> Option<&String> {
        match self {
            Segment::Kana(_) => None,
            Segment::Kanji { kanji, readings: _ } => Some(kanji),
        }
    }

    /// Returns the kanji readings
    #[inline]
    fn readings(&self) -> Option<&TinyVec<[Self::StrType; 1]>> {
        match self {
            Segment::Kana(_) => None,
            Segment::Kanji { kanji: _, readings } => Some(readings),
        }
    }
}

impl AsSegmentMut for Segment {
    /// Sets the kanji reading or converts it to one
    fn set_kanji(&mut self, new_kanji: String) {
        match self {
            Segment::Kana(k) => {
                let kana = std::mem::take(k);
                *self = Self::new_kanji(new_kanji, kana);
            }
            Segment::Kanji { kanji, readings: _ } => *kanji = new_kanji,
        }
    }

    #[inline]
    fn set_kana(&mut self, s: String) {
        if let Segment::Kana(k) = self {
            *k = s
        }
    }

    #[inline]
    fn add_reading(&mut self, r: String) {
        if let Segment::Kanji { kanji: _, readings } = self {
            readings.push(r);
        }
    }
}

impl From<String> for Segment {
    #[inline]
    fn from(s: String) -> Self {
        Self::new_kana(s)
    }
}

impl From<(String, String)> for Segment {
    /// (Kanji, kana)
    #[inline]
    fn from(s: (String, String)) -> Self {
        Self::new_kanji(s.0, s.1)
    }
}

impl<S> From<(S, Vec<S>)> for Segment
where
    S: AsRef<str>,
{
    /// (Kanji, readings) when |v| > 0
    /// (Kana, _) when |v| == 0
    #[inline]
    fn from(s: (S, Vec<S>)) -> Self {
        if s.1.is_empty() {
            Self::new_kana(s.0.as_ref().to_string())
        } else {
            let readings = s.1.into_iter().map(|i| i.as_ref().to_string()).collect();
            Self::Kanji {
                kanji: s.0.as_ref().to_string(),
                readings,
            }
        }
    }
}

impl From<(String, Option<String>)> for Segment {
    /// (Kanji, reading) when Some()
    /// (Kana, _) when None
    #[inline]
    fn from(s: (String, Option<String>)) -> Self {
        if let Some(kana) = s.1 {
            Self::new_kanji(s.0, kana)
        } else {
            Self::Kana(s.0)
        }
    }
}

impl From<&str> for Segment {
    /// Kana
    #[inline]
    fn from(s: &str) -> Self {
        Self::new_kana(s.to_string())
    }
}

impl From<(&str, &str)> for Segment {
    /// (Kanji, kana)
    #[inline]
    fn from(s: (&str, &str)) -> Self {
        Self::new_kanji(s.0.to_string(), s.1.to_string())
    }
}

impl From<(&str, Option<&str>)> for Segment {
    /// (Kanji, reading) when Some()
    /// (Kana, _) when None
    #[inline]
    fn from(s: (&str, Option<&str>)) -> Self {
        if let Some(kana) = s.1 {
            Self::new_kanji(s.0.to_string(), kana.to_string())
        } else {
            Self::Kana(s.0.to_string())
        }
    }
}

impl<'a> PartialEq<SegmentRef<'a>> for Segment {
    fn eq(&self, other: &SegmentRef) -> bool {
        match (self, other) {
            (Segment::Kana(s_kana), SegmentRef::Kana(o_kana)) => s_kana == o_kana,
            (
                Segment::Kana(_),
                SegmentRef::Kanji {
                    kanji: _,
                    readings: _,
                },
            )
            | (
                Segment::Kanji {
                    kanji: _,
                    readings: _,
                },
                SegmentRef::Kana(_),
            ) => false,
            (
                Segment::Kanji {
                    kanji: self_k,
                    readings: self_r,
                },
                SegmentRef::Kanji {
                    kanji: other_k,
                    readings: other_r,
                },
                // ) => self_k == other_k && self_r == other_r,
            ) => self_k == other_k && self_r.iter().zip(other_r.iter()).all(|i| i.0 == i.1),
        }
    }
}

impl<'a> PartialEq<SegmentRef<'a>> for &Segment {
    #[inline]
    fn eq(&self, other: &SegmentRef) -> bool {
        (*self).eq(other)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Segment {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encode())
    }
}

#[cfg(feature = "serde")]
impl<'a, 'de: 'a> serde::Deserialize<'de> for Segment {
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
    type Value = Segment;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected string in furigana format!")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Segment::from_str(v).unwrap())
    }
}
