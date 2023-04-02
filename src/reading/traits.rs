use super::r_ref::ReadingRef;

pub trait AsReadingRef {
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a>;
}

impl AsReadingRef for (&String, Option<&String>) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_raw(self.0, self.1.map(|i| i.as_str()))
    }
}

impl AsReadingRef for (&String, &String) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_with_kanji(self.0, self.1)
    }
}

impl AsReadingRef for (&str, Option<&str>) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_raw(self.0, self.1)
    }
}

impl AsReadingRef for (&str, &str) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_with_kanji(self.0, self.1)
    }
}

impl AsReadingRef for &str {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new(self)
    }
}

impl AsReadingRef for &String {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new(self)
    }
}

impl AsReadingRef for (&String, &Option<String>) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_raw(self.0, self.1.as_deref())
    }
}

impl AsReadingRef for (&String, &Option<&String>) {
    #[inline]
    fn as_reading_ref<'a>(&'a self) -> ReadingRef<'a> {
        ReadingRef::new_raw(self.0, self.1.as_deref().map(|i| i.as_str()))
    }
}
