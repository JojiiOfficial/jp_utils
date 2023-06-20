use super::as_kanji::{AsKanjiRef, AsKanjiSegment};
use tinyvec::TinyVec;

#[derive(Clone, Debug)]
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
