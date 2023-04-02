use super::{r_ref::ReadingRef, traits::AsReadingRef};

#[cfg(feature = "furigana")]
use crate::furigana::{as_part::AsPart, reading_part::ReadingPart, seq::FuriSequence};

/// Represents a Japanese 'reading' which always consists of a kana reading and sometimes an
/// equivalent way to write that word with kanji. This is an owned variant. For a borrowed variant
/// see [`ReadingRef`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadingOwned {
    kana: String,
    kanji: Option<String>,
}

impl ReadingOwned {
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

impl AsReadingRef for ReadingOwned {
    #[inline]
    fn as_reading_ref<'b>(&'b self) -> ReadingRef<'b> {
        ReadingRef::new_raw(&self.kana, self.kanji.as_ref().map(|i| i.as_str()))
    }
}

#[cfg(feature = "furigana")]
impl From<&FuriSequence<ReadingPart>> for ReadingOwned {
    #[inline]
    fn from(value: &FuriSequence<ReadingPart>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl From<FuriSequence<ReadingPart>> for ReadingOwned {
    #[inline]
    fn from(value: FuriSequence<ReadingPart>) -> Self {
        let kana = value.kana_reading().to_string();
        let kanji = value.has_kanji().then(|| value.kanji_reading().to_string());
        Self { kana, kanji }
    }
}

#[cfg(feature = "furigana")]
impl<A> From<A> for ReadingOwned
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
