use super::{
    as_kanji::{AsKanjiRef, AsKanjiSegment},
    k_kref::KanjiRef,
};
use tinyvec::TinyVec;

#[derive(Clone, Debug)]
pub struct Kanji {
    lit: String,
    readings: TinyVec<[String; 1]>,
}

impl Kanji {
    #[inline]
    pub fn new(lit: String, readings: &[String]) -> Self {
        let readings: TinyVec<[String; 1]> = readings.into();
        Self { lit, readings }
    }

    #[inline]
    pub fn as_ref(&self) -> KanjiRef {
        let readings = self.readings.iter().map(|i| i.as_str()).collect();
        KanjiRef::new_raw(&self.lit, readings)
    }
}

impl AsKanjiSegment for Kanji {
    type StrType = String;

    #[inline]
    fn literals(&self) -> &Self::StrType {
        &self.lit
    }

    #[inline]
    fn readings(&self) -> &[Self::StrType] {
        self.readings.as_slice()
    }
}

impl<'a> AsKanjiRef<'a> for &'a Kanji {
    #[inline]
    fn as_kanji_ref(&self) -> KanjiRef<'a> {
        self.as_ref()
    }
}
