use super::{
    encode::FuriEncoder,
    iter::{flatten::FlattenIter, SegmentIter},
    kanji::as_kanji::AsKanjiSegment,
    s_ref::SegmentRef,
};
use crate::reading::{traits::AsReadingRef, Reading};

/// Defines shared behaivor segments.
pub trait AsSegment {
    type StrType: AsRef<str> + Default;
    type KanjiType: AsKanjiSegment<StrType = Self::StrType>;

    /// Returns `true` if the segment is a kana segment.
    fn is_kana(&self) -> bool;

    /// Returns `true` if the segment is a kanji segment.
    fn is_kanji(&self) -> bool;

    /// Returns the segment as kana.
    fn as_kana(&self) -> Option<&Self::StrType>;

    /// Returns the segment as kanji.
    fn as_kanji(&self) -> Option<&Self::KanjiType>;

    /// Returns the kana reading of the segment. The output is equal to `as_kana()` for kana
    /// segments and `as_kanji().full_reading()` for kanji segments.
    fn get_kana_reading(&self) -> String {
        if let Some(kana) = self.as_kana() {
            return kana.as_ref().to_string();
        }

        // Safe as there can only be kanji or kana and in case of kana this function had early
        // returned.
        unsafe { self.as_kanji().unwrap_unchecked() }.full_reading()
    }

    /// Returns a ReadingOwned representing the reading of the part.
    fn to_reading(&self) -> Reading {
        if let Some(kana) = self.as_kana() {
            return Reading::new(kana.as_ref().to_string());
        }

        // Safe as there can only be kanji or kana and in case of kana this function had early
        // returned.
        let kanji = unsafe { self.as_kanji().unwrap_unchecked() };
        Reading::new_with_kanji(kanji.full_reading(), kanji.literals().as_ref().to_string())
    }

    /// Returns an Iterator over all readings of the Segment.
    #[inline]
    fn reading_iter(&self) -> SegmentIter<Self>
    where
        Self: Sized,
    {
        SegmentIter::new(self)
    }

    /// Returns `true` if the segment is empty.
    fn is_empty(&self) -> bool {
        if let Some(kana) = self.as_kana() {
            return kana.as_ref().is_empty();
        }

        // Safe as there can only be kanji or kana and in case of kana this function had early
        // returned.
        unsafe { self.as_kanji().unwrap_unchecked() }.is_empty()
    }

    /// Encodes the segment into a buffer.
    fn encode_into(&self, buf: &mut String) {
        let mut enc = FuriEncoder::new(buf);

        if let Some(s) = self.as_kana() {
            enc.write_kana(s.as_ref());
            return;
        }

        // Safe as there can only be kanji or kana and in case of kana this function had early
        // returned.
        enc.write_kanji(unsafe { self.as_kanji().unwrap_unchecked() });
    }

    /// Encodes the segment into a newly allocated String. This shouldn't be used in loops or
    /// situations where `encode_into` would work too as this does less allocations.
    fn encode(&self) -> String {
        let mut buf = String::with_capacity(8);
        self.encode_into(&mut buf);
        buf
    }

    /// Returns the main reading of the part. This is the Kanji reading if the part is a kanji or
    /// the kana reading if the part is a kana part.
    fn main_reading(&self) -> &Self::StrType {
        if let Some(kana) = self.as_kana() {
            return &kana;
        }

        // Safe as there can only be kanji or kana and in case of kana this function had early
        // returned.
        unsafe { self.as_kanji().unwrap_unchecked().literals() }
    }

    /// Returns an iterator over flattened readings
    #[inline]
    fn reading_flattened(&self) -> FlattenIter<'_, Self::StrType, Self::KanjiType>
    where
        Self: Sized,
    {
        FlattenIter::new(self)
    }

    /// Returns `true` if the segment holds equal reading data as `reading`.
    fn eq_reading<R>(&self, reading: R) -> bool
    where
        R: AsReadingRef,
    {
        let reading = reading.as_reading_ref();

        if let Some(kana) = self.as_kana() {
            return kana.as_ref() == reading.kana() && !reading.has_kanji();
        }

        if !reading.has_kanji() {
            return false;
        }
        let reading_kanji = match reading.kanji() {
            Some(k) => k,
            None => return false,
        };

        // Safety:
        // A reading is either a kanji or kana. This is unreachable if its not kanji.
        let kanji = unsafe { self.as_kanji().unwrap_unchecked() };
        kanji.literals().as_ref() == reading_kanji && self.get_kana_reading() == reading.kana()
    }
}

impl<T> AsSegment for &T
where
    T: AsSegment,
{
    type StrType = T::StrType;
    type KanjiType = T::KanjiType;

    #[inline]
    fn is_kana(&self) -> bool {
        (*self).is_kana()
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        (*self).is_kanji()
    }

    #[inline]
    fn as_kana(&self) -> Option<&T::StrType> {
        (*self).as_kana()
    }

    #[inline]
    fn as_kanji(&self) -> Option<&Self::KanjiType> {
        (*self).as_kanji()
    }
}

pub trait AsSegmentRef<'a> {
    fn as_seg_ref(&self) -> SegmentRef<'a>;
}
