/// Alphabet type of text
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alphabet {
    Kana(Kana),
    Kanji,
    Symbol,
    Romaji,
    Other,
}

impl Alphabet {
    /// Returns `true` if the alphabet is Hiragana.
    #[inline]
    pub fn is_hiragana(&self) -> bool {
        match self {
            Alphabet::Kana(k) => k.is_hiragana(),
            _ => false,
        }
    }

    /// Returns `true` if the alphabet is Katakana.
    #[inline]
    pub fn is_katakana(&self) -> bool {
        match self {
            Alphabet::Kana(k) => k.is_katakana(),
            _ => false,
        }
    }

    /// Returns `true` if the alphabet is kana
    pub fn is_kana(&self) -> bool {
        self.is_hiragana() || self.is_katakana()
    }

    /// Returns `true` if the alphabet is [`Kanji`].
    ///
    /// [`Kanji`]: Alphabet::Kanji
    #[inline]
    pub fn is_kanji(&self) -> bool {
        matches!(self, Self::Kanji)
    }

    /// Returns `true` if the alphabet is japanese
    pub fn is_japanese(&self) -> bool {
        self.is_kana() || self.is_kanji()
    }

    /// Returns `true` if the alphabet is [`Symbol`].
    ///
    /// [`Symbol`]: Alphabet::Symbol
    #[inline]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol)
    }

    /// Returns `true` if the alphabet is [`Romaji`].
    ///
    /// [`Romaji`]: Alphabet::Romaji
    #[inline]
    pub fn is_romaji(&self) -> bool {
        matches!(self, Self::Romaji)
    }

    /// Returns `true` if the alphabet is [`Other`].
    ///
    /// [`Other`]: Alphabet::Other
    #[inline]
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other)
    }

    /// Returns alphabet of type hiragana
    #[inline]
    pub fn hiragana() -> Self {
        Self::Kana(Kana::Hiragana)
    }

    /// Returns alphabet of type katakana
    #[inline]
    pub fn katakana() -> Self {
        Self::Kana(Kana::Katakana)
    }

    /// Returns alphabet of type kana (both hiragana and katakana)
    #[inline]
    pub fn kana() -> Self {
        Self::Kana(Kana::Both)
    }

    #[inline]
    pub fn eq_both_kana(&self, other: &Self) -> bool {
        match self {
            Alphabet::Kana(_) => other.is_kana(),
            _ => self == other,
        }
    }
}

/// Type of Kana
#[derive(Clone, Copy, Debug, Eq)]
pub enum Kana {
    Hiragana,
    Katakana,
    Both,
}

impl PartialEq for Kana {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.is_both()
            || other.is_both()
            || core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Kana {
    /// Returns `true` if the kana is [`Hiragana`].
    ///
    /// [`Hiragana`]: Kana::Hiragana
    #[must_use]
    pub fn is_hiragana(&self) -> bool {
        matches!(self, Self::Hiragana)
    }

    /// Returns `true` if the kana is [`Katakana`].
    ///
    /// [`Katakana`]: Kana::Katakana
    #[must_use]
    pub fn is_katakana(&self) -> bool {
        matches!(self, Self::Katakana)
    }

    /// Returns `true` if the kana is [`Both`].
    ///
    /// [`Both`]: Kana::Both
    #[must_use]
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both)
    }
}
