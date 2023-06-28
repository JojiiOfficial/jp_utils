use super::{
    as_kanji::{AsKanjiRef, AsKanjiSegment},
    Kanji,
};
use tinyvec::TinyVec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KanjiRef<'a> {
    lit: &'a str,
    readings: TinyVec<[&'a str; 1]>,
}

impl<'a> KanjiRef<'a> {
    #[inline]
    pub fn new(lit: &'a str, readings: &[&'a str]) -> Self {
        let r = readings.into();
        Self { lit, readings: r }
    }

    #[inline]
    pub(crate) fn new_raw(lit: &'a str, readings: TinyVec<[&'a str; 1]>) -> Self {
        Self { lit, readings }
    }

    /// Converts the Kanji reference to an owned [`Kanji`].
    pub fn to_owned(&self) -> Kanji {
        let readings: Vec<_> = self.readings.iter().map(|i| i.to_string()).collect();
        Kanji::new(self.lit.to_string(), &readings)
    }
}

impl<'a> AsKanjiSegment for KanjiRef<'a> {
    type StrType = &'a str;

    #[inline]
    fn literals(&self) -> &Self::StrType {
        &self.lit
    }

    #[inline]
    fn readings(&self) -> &[Self::StrType] {
        self.readings.as_slice()
    }
}

impl<'a> AsKanjiRef<'a> for KanjiRef<'a> {
    #[inline]
    fn as_kanji_ref(&self) -> KanjiRef<'a> {
        self.clone()
    }
}

impl<'a> AsKanjiRef<'a> for &KanjiRef<'a> {
    #[inline]
    fn as_kanji_ref(&self) -> KanjiRef<'a> {
        (*self).as_kanji_ref()
    }
}

impl<'a> PartialEq<Kanji> for KanjiRef<'a> {
    #[inline]
    fn eq(&self, other: &Kanji) -> bool {
        self.literals() == other.literals()
            && self
                .readings()
                .iter()
                .zip(other.readings().iter())
                .all(|i| i.0 == i.1)
    }
}

impl<'a> PartialEq<Kanji> for &'a KanjiRef<'a> {
    #[inline]
    fn eq(&self, other: &Kanji) -> bool {
        (*self).eq(other)
    }
}
