use super::{encoder::FuriEncoder, FlattenIter, ReadingIter};
use crate::reading::{traits::AsReadingRef, Reading};
use tinyvec::TinyVec;

/// Trait defining common behavior for ReadingParts
pub trait AsSegment {
    type StrType: AsRef<str> + Default;

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    fn is_empty(&self) -> bool;

    /// Returns `true` if the reading part ref is a kana reading.
    fn is_kana(&self) -> bool;

    /// Returns `true` if the reading part ref is a kanji reading.
    fn is_kanji(&self) -> bool;

    /// Returns the kana reading
    fn as_kana(&self) -> Option<&Self::StrType>;

    /// Returns the kanji reading if exists
    fn as_kanji(&self) -> Option<&Self::StrType>;

    /// Returns the kana reading of the reading part. This is equal to .get_kana() for kana reading
    /// parts and equal to all kanji readings merged to one
    fn kana_reading(&self) -> String;

    /// Returns the kanji readings
    fn readings(&self) -> Option<&TinyVec<[Self::StrType; 1]>>;

    /// Returns a list of kanjis assigned to their readings.
    fn literal_readings(&self) -> Option<Vec<(String, String)>> {
        let readings = self.readings()?;

        let res = if self.detailed_readings()? {
            self.as_kanji()?
                .as_ref()
                .chars()
                .zip(readings.iter())
                .map(|(lit, r)| (lit.to_string(), r.as_ref().to_string()))
                .collect()
        } else {
            let kanji = self.as_kanji()?.as_ref().to_string();
            let reading = self.kana_reading();
            vec![(kanji, reading)]
        };

        Some(res)
    }

    /// Returns `Some(true)` if each kanji has its own reading assigned. Returns `None` if reading
    /// is not a kanji reading
    fn detailed_readings(&self) -> Option<bool> {
        let kanji = self.as_kanji()?.as_ref();
        let readings = self.readings()?;
        Some(kanji.chars().count() == readings.len())
    }

    /// Encodes the part into a string. This shouldn't be used in loops as it allocates a new
    /// string each time. Use [`FuriEncoder`] [`encode_into`] if you want to encode multiple segments or
    fn encode(&self) -> String
    where
        Self: Sized,
    {
        let mut buf = String::new();
        self.encode_into(&mut buf);
        buf
    }

    /// Encocdes the part into a buffer.
    #[inline]
    fn encode_into(&self, buf: &mut String)
    where
        Self: Sized,
    {
        FuriEncoder::new(buf).write_seg(self);
    }

    /// Returns an iterator over flattened readings
    #[inline]
    fn reading_flattened(&self) -> FlattenIter<'_, Self::StrType>
    where
        Self: Sized,
    {
        FlattenIter::new(self)
    }

    /// Returns an iterator over all readings of the part
    #[inline]
    fn reading_iter(&self) -> ReadingIter<Self>
    where
        Self: Sized,
    {
        ReadingIter::new(self)
    }

    /// Returns the main reading of the part. This is the Kanji reading if the part is a kanji or
    /// the kana reading if the part is a kana part.
    fn main_reading(&self) -> &str {
        if let Some(kanji) = self.as_kanji() {
            return kanji.as_ref();
        }

        self.as_kana().unwrap().as_ref()
    }

    /// Returns a ReadingOwned representing the reading of the part.
    fn to_reading(&self) -> Reading {
        if let Some(kanji) = self.as_kanji() {
            Reading::new_with_kanji(self.kana_reading(), kanji.as_ref().to_string())
        } else {
            Reading::new(self.kana_reading())
        }
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
        let kanji = unsafe { self.as_kanji().unwrap_unchecked().as_ref() };
        kanji == reading_kanji && self.kana_reading() == reading.kana()
    }
}

pub trait AsSegmentMut: AsSegment {
    /// Sets the kanji. Converts a Kana reading to a kanji reading
    fn set_kanji(&mut self, s: Self::StrType);

    /// Sets the kana text to `s`. Does nothing on a kanji reading
    fn set_kana(&mut self, s: Self::StrType);

    /// Adds a new reading to a kanji reading. Does nothing on a kana reading
    fn add_reading(&mut self, r: Self::StrType);
}

impl<'a, S> AsSegment for &'a S
where
    S: AsSegment,
{
    type StrType = S::StrType;

    #[inline]
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }

    #[inline]
    fn is_kana(&self) -> bool {
        (*self).is_kana()
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        (*self).is_kanji()
    }

    #[inline]
    fn as_kana(&self) -> Option<&Self::StrType> {
        (*self).as_kana()
    }

    #[inline]
    fn as_kanji(&self) -> Option<&Self::StrType> {
        (*self).as_kanji()
    }

    #[inline]
    fn kana_reading(&self) -> String {
        (*self).kana_reading()
    }

    #[inline]
    fn readings(&self) -> Option<&TinyVec<[Self::StrType; 1]>> {
        (*self).readings()
    }
}

#[cfg(test)]
mod test {
    use crate::furigana::segment::Segment;

    use super::AsSegment;
    use test_case::test_case;

    #[test_case(("私", vec!["わたし"]), "[私|わたし]"; "Kanji")]
    #[test_case("は", "は"; "SingleHiragana")]
    #[test_case("ハ", "ハ"; "SingleKatakana")]
    #[test_case(("音楽", vec!["おん","がく"]), "[音楽|おん|がく]"; "MultipleKanji")]
    #[test_case(("大学生", vec!["だい","がくせい"]), "[大学生|だいがくせい]"; "Malformed kanji readings")]
    fn test_encode(part: impl Into<Segment>, exp: &str) {
        assert_eq!(part.into().encode(), exp);
    }
}
