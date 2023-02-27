use super::{as_part::AsPart, reading_part::ReadingPart};

/// Same as ReadingPart but borrowed
#[derive(Clone, PartialEq, Debug, Eq)]
pub enum ReadingPartRef<'a> {
    // Kana reading
    Kana(&'a str),

    // Kanji reading with assigned kana readings
    Kanji {
        kanji: &'a str,
        readings: Vec<&'a str>,
    },
}

impl<'a> ReadingPartRef<'a> {
    /// Creates a new ReadingPartRef
    #[inline]
    pub fn new_kana(kana: &'a str) -> Self {
        Self::Kana(kana)
    }

    /// Creates a new ReadingPartRef with a value for kanji
    #[inline]
    pub fn new_kanji(kana: &'a str, kanji: &'a str) -> Self {
        Self::Kanji {
            kanji,
            readings: vec![kana],
        }
    }

    /// Returns an owned ReadingPart
    #[inline]
    pub fn to_owned(&self) -> ReadingPart {
        match self {
            ReadingPartRef::Kana(k) => ReadingPart::Kana(k.to_string()),
            ReadingPartRef::Kanji { kanji, readings } => {
                let readings: Vec<String> = readings.iter().map(|i| i.to_string()).collect();
                ReadingPart::Kanji {
                    kanji: kanji.to_string(),
                    readings,
                }
            }
        }
    }

    /// Parses a ReadingPart from string
    pub fn from_str_checked(str: &'a str) -> Result<ReadingPartRef, ()> {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, true).ok_or(())
        } else {
            Ok(ReadingPartRef::Kana(str))
        }
    }

    /// Parses a ReadingPart from string
    pub fn from_str(str: &'a str) -> ReadingPartRef {
        if str.starts_with('[') && str.ends_with(']') {
            Self::parse_kanji_str(str, false).unwrap()
        } else {
            ReadingPartRef::Kana(str)
        }
    }

    /// Parses an encoded Kanji furigana string eg: `[音楽|おん|がく]` thus `s` has to start with
    /// `[` and end  with `]`. If the readings don't line up with the kanji literal count and has
    /// are more than 1 (fallback) the function returns None.
    fn parse_kanji_str(s: &'a str, checked: bool) -> Option<ReadingPartRef> {
        // Strip [ and ] and split at the |
        let mut split = (&s[1..s.len() - 1]).split('|');

        // First item is the kanji reading
        let kanji = split.next()?;

        let readings = split.collect::<Vec<_>>();
        if readings.is_empty() && checked {
            return None;
        }

        if readings.len() == 1 {
            // Fallback where all kanji get the first reading assigned
            return Some(ReadingPartRef::new_kanji(readings[0], kanji));
        } else if kanji.chars().count() != readings.len() && checked {
            // Malformed kanji string
            return None;
        }

        Some(ReadingPartRef::Kanji { kanji, readings })
    }
}

impl<'a> ToString for ReadingPartRef<'a> {
    #[inline]
    fn to_string(&self) -> String {
        self.encode().unwrap_or_default()
    }
}

impl<'a> AsPart for ReadingPartRef<'a> {
    type StrType = &'a str;

    /// Returns `true` if ReadingPart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            ReadingPartRef::Kana(k) => k.is_empty(),
            ReadingPartRef::Kanji { kanji, readings } => readings.is_empty() || kanji.is_empty(),
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
    fn as_kana<'b>(&'b self) -> Option<&'b Self::StrType> {
        match self {
            ReadingPartRef::Kana(k) => Some(k),
            ReadingPartRef::Kanji { .. } => None,
        }
    }

    /// Returns the kanji reading if exists
    #[inline]
    fn as_kanji<'b>(&'b self) -> Option<&'b Self::StrType> {
        match self {
            ReadingPartRef::Kana(_) => None,
            ReadingPartRef::Kanji { kanji, readings: _ } => Some(kanji),
        }
    }

    /// Returns the kanji readings
    #[inline]
    fn readings(&self) -> Option<&Vec<Self::StrType>> {
        match self {
            ReadingPartRef::Kana(_) => None,
            ReadingPartRef::Kanji { kanji: _, readings } => Some(readings),
        }
    }

    /// Sets the kanji reading or converts it to one
    fn set_kanji(&mut self, s: Self::StrType) {
        match self {
            ReadingPartRef::Kana(k) => {
                *self = Self::new_kanji(s, *k);
            }
            ReadingPartRef::Kanji { kanji, readings: _ } => *kanji = s,
        }
    }

    #[inline]
    fn set_kana(&mut self, s: Self::StrType) {
        if let ReadingPartRef::Kana(k) = self {
            *k = s
        }
    }

    #[inline]
    fn add_reading(&mut self, r: Self::StrType) {
        if let ReadingPartRef::Kanji { kanji: _, readings } = self {
            readings.push(r);
        }
    }

    #[inline]
    fn kana_reading(&self) -> String {
        match self {
            ReadingPartRef::Kana(k) => k.to_string(),
            ReadingPartRef::Kanji { kanji: _, readings } => readings.join(""),
        }
    }
}

impl<'a> From<&'a ReadingPart> for ReadingPartRef<'a> {
    #[inline]
    fn from(r: &'a ReadingPart) -> Self {
        match r {
            ReadingPart::Kana(k) => Self::Kana(k),
            ReadingPart::Kanji { kanji, readings } => {
                let readings: Vec<&str> = readings.iter().map(|i| i.as_str()).collect();
                Self::Kanji { kanji, readings }
            }
        }
    }
}

impl<'a> From<&'a str> for ReadingPartRef<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::new_kana(s)
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for ReadingPartRef<'a> {
    #[inline]
    fn from(s: (&'a str, Option<&'a str>)) -> Self {
        if let Some(kanji) = s.1 {
            Self::new_kanji(s.0, kanji)
        } else {
            Self::Kana(s.0)
        }
    }
}

impl<'a> From<(&'a str, Vec<&'a str>)> for ReadingPartRef<'a> {
    #[inline]
    fn from(s: (&'a str, Vec<&'a str>)) -> Self {
        Self::Kanji {
            kanji: s.0,
            readings: s.1,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for ReadingPartRef<'a> {
    #[inline]
    fn from(s: (&'a str, &'a str)) -> Self {
        Self::new_kanji(s.0, s.1)
    }
}

impl<'a> PartialEq<ReadingPart> for ReadingPartRef<'a> {
    #[inline]
    fn eq(&self, other: &ReadingPart) -> bool {
        other.eq(self)
    }
}

impl<'a> PartialEq<ReadingPart> for &ReadingPartRef<'a> {
    #[inline]
    fn eq(&self, other: &ReadingPart) -> bool {
        other.eq(*self)
    }
}
