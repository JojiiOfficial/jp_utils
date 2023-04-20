mod r_ref;
pub mod traits;

pub use r_ref::ReadingRef;

use self::traits::AsReadingRef;

#[cfg(feature = "furigana")]
use crate::furigana::{part::AsPart, part::ReadingPart, seq::FuriSequence};

/// Represents a Japanese 'reading' which always consists of a kana reading and sometimes an
/// equivalent way to write that word with kanji. This is an owned variant. For a borrowed variant
/// see [`ReadingRef`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reading {
    kana: String,
    kanji: Option<String>,
}

impl Reading {
    #[inline]
    pub fn new(kana: String) -> Self {
        Self { kana, kanji: None }
    }

    #[inline]
    pub fn new_with_kanji(kana: String, kanji: String) -> Self {
        Self {
            kana,
            kanji: Some(kanji),
        }
    }

    #[inline]
    pub fn new_raw(kana: String, kanji: Option<String>) -> Self {
        Self { kana, kanji }
    }

    /// Returns `true` if the ReadingRef has a kanji reading.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.kanji.is_some()
    }

    /// Returns the kanji reading if exists.
    #[inline]
    pub fn kanji(&self) -> Option<&String> {
        self.kanji.as_ref()
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

#[cfg(feature = "furigana")]
impl From<&FuriSequence<ReadingPart>> for Reading {
    #[inline]
    fn from(value: &FuriSequence<ReadingPart>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl From<FuriSequence<ReadingPart>> for Reading {
    #[inline]
    fn from(value: FuriSequence<ReadingPart>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl<A> From<A> for Reading
where
    A: AsPart,
{
    #[inline]
    fn from(value: A) -> Self {
        let kana = value.kana_reading();
        let kanji = value.as_kanji().map(|i| i.as_ref().to_string());
        Self { kana, kanji }
    }
}
