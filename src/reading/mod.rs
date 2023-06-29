mod r_ref;
pub mod traits;

pub use r_ref::ReadingRef;

use self::traits::AsReadingRef;

#[cfg(feature = "furigana")]
use crate::furi::segment::kanji::as_kanji::AsKanjiSegment;
#[cfg(feature = "furigana")]
use crate::furi::{parse::reading::FuriToReadingParser, Furigana};
#[cfg(feature = "furigana")]
use crate::furi::{segment::AsSegment, segment::Segment, seq::FuriSequence};

/// Represents a Japanese 'reading' which always consists of a kana reading and sometimes an
/// equivalent way to write that word with kanji. This is an owned variant. For a borrowed variant
/// see [`ReadingRef`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reading {
    kana: String,
    kanji: Option<String>,
}

impl Reading {
    /// Create a new kana reading.
    #[inline]
    pub fn new(kana: String) -> Self {
        Self { kana, kanji: None }
    }

    /// Create a new reading with a kanji.
    #[inline]
    pub fn new_with_kanji(kana: String, kanji: String) -> Self {
        Self {
            kana,
            kanji: Some(kanji),
        }
    }

    /// Create a new reading where you can pass an `Option` for kanji.
    #[inline]
    pub fn new_raw(kana: String, kanji: Option<String>) -> Self {
        Self { kana, kanji }
    }

    /// Returns `true` if the ReadingRef has a kanji reading.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.kanji.is_some()
    }

    /// Returns the kanji reading if available or uses kana as fallback.
    #[inline]
    pub fn kanji_or_kana(&self) -> &str {
        self.kanji.as_deref().unwrap_or(&self.kana)
    }

    /// Returns the kanji reading if exists.
    #[inline]
    pub fn kanji(&self) -> Option<&str> {
        self.kanji.as_deref()
    }

    /// Returns the readings kana reading
    #[inline]
    pub fn kana(&self) -> &str {
        &self.kana
    }
}

impl AsReadingRef for Reading {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_raw(&self.kana, self.kanji.as_deref())
    }
}

impl PartialEq<ReadingRef<'_>> for Reading {
    #[inline]
    fn eq(&self, other: &ReadingRef) -> bool {
        self.kana.as_str() == other.kana() && self.kanji.as_deref() == other.kanji()
    }
}

#[cfg(feature = "furigana")]
impl From<&FuriSequence<Segment>> for Reading {
    #[inline]
    fn from(value: &FuriSequence<Segment>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl<T: AsRef<str>> From<&Furigana<T>> for Reading {
    #[inline]
    fn from(value: &Furigana<T>) -> Self {
        let (kana, kanji) = FuriToReadingParser::parse_kanji_and_kana(value.raw());
        Self::new_raw(kana, kanji)
    }
}

#[cfg(feature = "furigana")]
impl<T: AsRef<str>> From<Furigana<T>> for Reading {
    #[inline]
    fn from(value: Furigana<T>) -> Self {
        let (kana, kanji) = FuriToReadingParser::parse_kanji_and_kana(value.raw());
        Self::new_raw(kana, kanji)
    }
}

#[cfg(feature = "furigana")]
impl From<FuriSequence<Segment>> for Reading {
    #[inline]
    fn from(value: FuriSequence<Segment>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl<S: AsSegment> FromIterator<S> for Reading {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Reading {
        let mut kana = String::with_capacity(20);
        let mut kanji = String::new();
        let mut has_kanji = false;

        for i in iter {
            if let Some(k) = i.as_kanji() {
                if !has_kanji {
                    // lazy initialize kanji reading
                    kanji = kana.clone();
                    has_kanji = true;
                }

                let readings = k.readings();
                if readings.is_empty() || readings[0].as_ref().is_empty() {
                    kana.push_str(k.literals().as_ref());
                    kanji.push_str(k.literals().as_ref());
                    continue;
                }

                kanji.push_str(k.literals().as_ref());
                for r in readings {
                    kana.push_str(r.as_ref());
                }
            } else if let Some(k) = i.as_kana() {
                let k = k.as_ref();
                kana.push_str(k);
                if has_kanji {
                    kanji.push_str(k);
                }
            }
        }
        let kanji = has_kanji.then_some(kanji);
        Reading::new_raw(kana, kanji)
    }
}

#[cfg(feature = "furigana")]
impl<A> From<A> for Reading
where
    A: AsSegment,
{
    #[inline]
    fn from(value: A) -> Self {
        let kana = value.get_kana_reading();
        let kanji = value.as_kanji().map(|i| i.literals().as_ref().to_string());
        Self { kana, kanji }
    }
}
