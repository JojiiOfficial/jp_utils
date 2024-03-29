use super::FuriSequence;
use crate::furi::segment::AsSegment;

/// A `reading` view over `FuriSequence` that targets a given reading (kanji or kana) of the
/// furigana sequence
pub struct SReading<'a, T> {
    r: &'a FuriSequence<T>,
    kana: bool,
}

impl<'a, T> SReading<'a, T>
where
    T: AsSegment,
{
    #[inline]
    pub fn new(r: &'a FuriSequence<T>, kana: bool) -> Self {
        Self { r, kana }
    }

    /// Returns the length in bytes of string
    pub fn len(&self) -> usize {
        if self.kana {
            self.r.iter().map(|i| i.get_kana_reading().len()).sum()
        } else {
            self.r.iter().map(|i| i.main_reading().as_ref().len()).sum()
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.r.is_empty()
    }

    /// Returns the amount of characters of the reading
    pub fn str_len(&self) -> usize {
        if self.kana {
            self.r
                .iter()
                .map(|i| i.get_kana_reading().chars().count())
                .sum()
        } else {
            self.r
                .iter()
                .map(|i| i.main_reading().as_ref().chars().count())
                .sum()
        }
    }
}

impl<'a, T> ToString for SReading<'a, T>
where
    T: AsSegment,
{
    fn to_string(&self) -> String {
        if self.kana {
            self.r.iter().map(|i| i.get_kana_reading()).collect()
        } else {
            self.r.iter().map(|i| i.main_reading().as_ref()).collect()
        }
    }
}
