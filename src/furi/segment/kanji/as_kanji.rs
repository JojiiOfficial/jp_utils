use crate::{furigana::segment::encoder::FuriEncoder, reading::Reading};
use itertools::Itertools;

use super::KanjiRef;

/// Trait sharing behaivor of kanji segments.
pub trait AsKanjiSegment {
    type StrType: AsRef<str>;

    /// Returns the kanji literals of the kanji segment.
    fn literals(&self) -> &Self::StrType;

    /// Returns the readings for the literals.
    fn readings(&self) -> &[Self::StrType];

    /// Returns the full kanji reading as `String`.
    fn full_reading(&self) -> String {
        if self.reading_count() == 1 {
            return self.readings()[0].as_ref().to_string();
        }

        self.readings().iter().map(|i| i.as_ref()).join("")
    }

    /// Returns a ReadingOwned representing the kanji segment.
    #[inline]
    fn to_reading(&self) -> Reading {
        Reading::new_with_kanji(self.full_reading(), self.literals().as_ref().to_string())
    }

    /// Returns `true` if the amount of kanji literals is equal to the amount of readings.
    #[inline]
    fn is_detailed(&self) -> bool {
        self.lit_count() == self.reading_count()
    }

    /// Returns `true` if the kanji segment doesn't have any literals.
    #[inline]
    fn is_empty(&self) -> bool {
        self.literals().as_ref().is_empty()
    }

    /// Returns the amount of literals.
    #[inline]
    fn lit_count(&self) -> usize {
        self.literals().as_ref().chars().count()
    }

    /// Returns the amount of literals of the kanji segment.
    #[inline]
    fn reading_count(&self) -> usize {
        self.readings().len()
    }

    /// Returns `true` if the kanji segment holds a single kanji with its reading.
    #[inline]
    fn is_single(&self) -> bool {
        self.lit_count() == 1
    }

    /// Returns `true` if the there is either one reading for each kanji literal or there is one
    /// reading for all kanji literals.
    #[inline]
    fn is_valid(&self) -> bool {
        !self.is_empty() && (self.is_detailed() || self.reading_count() == 1)
    }

    /// Encodes the kanji segment into a String.
    #[inline]
    fn encode_into(&self, buf: &mut String)
    where
        Self: Sized,
    {
        FuriEncoder::new(buf).write_kanji(self)
    }
}

impl<T> AsKanjiSegment for &T
where
    T: AsKanjiSegment,
{
    type StrType = T::StrType;

    #[inline]
    fn literals(&self) -> &Self::StrType {
        (*self).literals()
    }

    #[inline]
    fn readings(&self) -> &[Self::StrType] {
        (*self).readings()
    }
}

pub trait AsKanjiRef<'a> {
    fn as_kanji_ref(&self) -> KanjiRef<'a>;
}
