use super::traits::AsReadingRef;

#[cfg(feature = "furigana")]
use crate::furi::Furigana;

/// A borrowed version of [`super::Reading`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadingRef<'a> {
    kana: &'a str,
    kanji: Option<&'a str>,
}

impl<'a> ReadingRef<'a> {
    #[inline]
    pub fn new(kana: &'a str) -> Self {
        Self { kana, kanji: None }
    }

    #[inline]
    pub fn new_with_kanji(kana: &'a str, kanji: &'a str) -> Self {
        Self {
            kana,
            kanji: Some(kanji),
        }
    }

    #[inline]
    pub fn new_raw(kana: &'a str, kanji: Option<&'a str>) -> Self {
        Self { kana, kanji }
    }

    /// Returns `true` if the ReadingRef has a kanji reading.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.kanji.is_some()
    }

    /// Returns the kanji reading if exists.
    #[inline]
    pub fn kanji(&self) -> Option<&str> {
        self.kanji
    }

    /// Returns the readings kana reading
    #[inline]
    pub fn kana(&self) -> &str {
        self.kana
    }

    /// Encodes the reading to furigana.
    #[cfg(feature = "furigana")]
    pub fn encode(&self) -> Furigana<String> {
        use crate::furi::segment::encode::FuriEncoder;

        if let Some(kanji) = self.kanji() {
            let mut buf = String::new();
            FuriEncoder::new(&mut buf).write_block(kanji, self.kana);
            Furigana(buf)
        } else {
            Furigana(self.kana.to_string())
        }
    }
}

impl<'a> AsReadingRef for ReadingRef<'a> {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        *self
    }
}
