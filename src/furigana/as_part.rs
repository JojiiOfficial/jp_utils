use super::encode;
use itertools::Itertools;

/// Trait defining common behavior for ReadingParts
pub trait AsPart {
    type StrType: AsRef<str>;

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    fn is_empty(&self) -> bool;

    /// Returns `true` if the reading part ref is a kana reading.
    fn is_kana(&self) -> bool;

    /// Returns `true` if the reading part ref is a kanji reading.
    fn is_kanji(&self) -> bool;

    /// Returns the kana reading
    fn get_kana<'a>(&'a self) -> Option<&'a Self::StrType>;

    /// Returns the kanji reading if exists
    fn get_kanji<'a>(&'a self) -> Option<&'a Self::StrType>;

    /// Returns the kanji readings
    fn readings(&self) -> Option<&Vec<Self::StrType>>;

    /// Returns `Some(true)` if each kanji has its own reading assigned. Returns `None` if reading
    /// is not a kanji reading
    fn detailed_readings(&self) -> Option<bool> {
        let kanji = self.get_kanji()?.as_ref();
        let readings = self.readings()?;
        Some(kanji.chars().count() == readings.len())
    }

    /// Sets the kanji. Converts a Kana reading to a kanji reading
    fn set_kanji(&mut self, s: Self::StrType);

    /// Sets the kana text to `s`. Does nothing on a kanji reading
    fn set_kana(&mut self, s: Self::StrType);

    /// Adds a new reading to a kanji reading. Does nothing on a kana reading
    fn add_reading(&mut self, r: Self::StrType);

    /// Encodes the part into a string
    fn encode(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }

        if let Some(kanji) = self.get_kanji() {
            let kanji = kanji.as_ref();
            let readings = self.readings().unwrap();

            if self.detailed_readings().unwrap() {
                Some(encode::multi_block(kanji, readings))
            } else {
                let readings_combined = readings.iter().map(|i| i.as_ref()).join("");
                Some(encode::single_block(kanji, readings_combined))
            }
        } else if let Some(kana) = self.get_kana() {
            Some(kana.as_ref().to_string())
        } else {
            // A part is always either a kanji or a kana part
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::reading_part::ReadingPart;
    use super::AsPart;
    use test_case::test_case;

    #[test_case(("私", vec!["わたし"]), "[私|わたし]"; "Kanji")]
    #[test_case("は", "は"; "SingleHiragana")]
    #[test_case("ハ", "ハ"; "SingleKatakana")]
    #[test_case(("音楽", vec!["おん","がく"]), "[音楽|おん|がく]"; "MultipleKanji")]
    #[test_case(("大学生", vec!["だい","がくせい"]), "[大学生|だいがくせい]"; "Malformed kanji readings")]
    fn test_encode(part: impl Into<ReadingPart>, exp: &str) {
        assert_eq!(part.into().encode().unwrap(), exp);
    }
}
