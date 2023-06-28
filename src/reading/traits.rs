use super::r_ref::ReadingRef;

pub trait AsReadingRef {
    fn as_reading_ref(&self) -> ReadingRef<'_>;

    /// Encodes the reading to furigana.
    #[cfg(feature = "furigana")]
    #[inline]
    fn encode(&self) -> crate::furi::Furigana<String> {
        self.as_reading_ref().encode()
    }
}

impl<R> AsReadingRef for &R
where
    R: AsReadingRef,
{
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef<'_> {
        (*self).as_reading_ref()
    }
}

impl AsReadingRef for (&String, Option<&String>) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_raw(self.0, self.1.map(|i| i.as_str()))
    }
}

impl AsReadingRef for (&String, &String) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_with_kanji(self.0, self.1)
    }
}

impl AsReadingRef for (&str, Option<&str>) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_raw(self.0, self.1)
    }
}

impl AsReadingRef for (&str, &str) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_with_kanji(self.0, self.1)
    }
}

impl AsReadingRef for &str {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new(self)
    }
}

impl AsReadingRef for &String {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new(self)
    }
}

impl AsReadingRef for (&String, &Option<String>) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_raw(self.0, self.1.as_deref())
    }
}

impl AsReadingRef for (&String, &Option<&String>) {
    #[inline]
    fn as_reading_ref(&self) -> ReadingRef {
        ReadingRef::new_raw(self.0, self.1.as_deref().map(|i| i.as_str()))
    }
}
