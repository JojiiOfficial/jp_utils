use super::{as_part::AsPart, reading_part_ref::ReadingPartRef};

/// Represents a single part of a reading that can either be a kana only reading or a kanji reading
/// with a kana part that describes the kanjis reading
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadingPart {
    // Kana reading
    Kana(String),

    // Kanji reading with assigned kana readings
    Kanji {
        kanji: String,
        readings: Vec<String>,
    },
}

impl ReadingPart {
    /// Create a new `SentencePart` with kana only
    #[inline]
    fn new_kana(kana: String) -> Self {
        Self::Kana(kana)
    }

    /// Create a new `SentencePart` with kanji value
    #[inline]
    fn new_kanji(kanji: String, kana: String) -> Self {
        Self::Kanji {
            kanji,
            readings: vec![kana],
        }
    }

    /// Returns the reading part as a reference
    #[inline]
    pub fn as_ref_part(&self) -> ReadingPartRef {
        self.into()
    }

    /// Parses a ReadingPart from string
    #[inline]
    pub fn from_str(s: &str) -> Result<ReadingPart, ()> {
        // TODO: find a better way to do this
        Ok(ReadingPartRef::from_str(s)?.to_owned())
    }
}

impl AsPart for ReadingPart {
    type StrType = String;

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            ReadingPart::Kana(k) => k.is_empty(),
            ReadingPart::Kanji { kanji, readings } => readings.is_empty() || kanji.is_empty(),
        }
    }

    /// Returns `true` if the reading part ref is a kana reading.
    #[inline]
    fn is_kana(&self) -> bool {
        matches!(self, Self::Kana(..))
    }

    /// Returns `true` if the reading part ref is a kanji reading.
    #[inline]
    fn is_kanji(&self) -> bool {
        matches!(self, Self::Kanji { .. })
    }

    /// Returns the kana reading
    #[inline]
    fn get_kana<'a>(&'a self) -> Option<&'a String> {
        match self {
            ReadingPart::Kana(k) => Some(k),
            ReadingPart::Kanji { .. } => None,
        }
    }

    /// Returns the kanji reading if exists
    #[inline]
    fn get_kanji<'a>(&'a self) -> Option<&'a String> {
        match self {
            ReadingPart::Kana(_) => None,
            ReadingPart::Kanji { kanji, readings: _ } => Some(kanji),
        }
    }

    /// Returns the kanji readings
    #[inline]
    fn readings(&self) -> Option<&Vec<String>> {
        match self {
            ReadingPart::Kana(_) => None,
            ReadingPart::Kanji { kanji: _, readings } => Some(readings),
        }
    }
}

impl From<String> for ReadingPart {
    #[inline]
    fn from(s: String) -> Self {
        Self::new_kana(s)
    }
}

impl From<(String, String)> for ReadingPart {
    #[inline]
    fn from(s: (String, String)) -> Self {
        Self::new_kanji(s.0, s.1)
    }
}

impl<S> From<(S, Vec<S>)> for ReadingPart
where
    S: AsRef<str>,
{
    #[inline]
    fn from(s: (S, Vec<S>)) -> Self {
        let readings = s.1.into_iter().map(|i| i.as_ref().to_string()).collect();
        Self::Kanji {
            kanji: s.0.as_ref().to_string(),
            readings,
        }
    }
}

impl From<(String, Option<String>)> for ReadingPart {
    #[inline]
    fn from(s: (String, Option<String>)) -> Self {
        if let Some(kanji) = s.1 {
            Self::new_kanji(kanji, s.0)
        } else {
            Self::Kana(s.0)
        }
    }
}

impl From<&str> for ReadingPart {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new_kana(s.to_string())
    }
}

impl From<(&str, &str)> for ReadingPart {
    #[inline]
    fn from(s: (&str, &str)) -> Self {
        Self::new_kanji(s.0.to_string(), s.1.to_string())
    }
}

impl From<(&str, Option<&str>)> for ReadingPart {
    #[inline]
    fn from(s: (&str, Option<&str>)) -> Self {
        if let Some(kanji) = s.1 {
            Self::new_kanji(s.0.to_string(), kanji.to_string())
        } else {
            Self::Kana(s.0.to_string())
        }
    }
}
