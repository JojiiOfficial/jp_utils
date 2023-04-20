/// Compare furigana
pub mod compare;
/// Parses encoded furigana
pub mod parse;
/// Single parts of an furigana string.
pub mod part;
/// Furigana sequence
pub mod seq;

use self::{
    parse::FuriToReadingParser,
    part::{AsPart, ReadingPart, ReadingPartRef},
};
use parse::FuriParser;
use std::fmt::Display;

/// A struct that holds encoded furigana data in a string. Such an element can be created by directly wrapping around
/// a [`String`] or using the `new()` function which has the benefit that the furigana
/// gets validated.
/// Valid encoded furigana looks like this: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]です。`
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Furigana(pub String);

impl Furigana {
    /// Create a new Furigana value with a given encoded furi string as value which gets checked.
    #[inline]
    pub fn new<S: AsRef<str>>(furi: S) -> Result<Self, ()> {
        if !FuriParser::check(&furi) {
            return Err(());
        }
        Ok(Self::new_unchecked(furi))
    }

    /// Create a new Furigana value with a given encoded furi string as value which doesn't get checked.
    #[inline]
    pub fn new_unchecked<S: AsRef<str>>(furi: S) -> Self {
        Self(furi.as_ref().to_string())
    }

    /// Returns an Iterator over all parts of the furigana.
    pub fn iter(&self) -> impl Iterator<Item = ReadingPartRef> {
        FuriParser::new(self.raw()).unchecked().map(|i| i.unwrap())
    }

    /// Returns the amount of reading parts.
    #[inline]
    pub fn part_count(&self) -> usize {
        self.iter().count()
    }

    /// Returns `true` if the Furigana is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the raw (encoded) furigana string.
    #[inline]
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Push a part to the end of the furigana sequence.
    #[inline]
    pub fn push_part<T>(&mut self, part: T)
    where
        T: AsPart,
    {
        self.0.push_str(&part.encode());
    }

    /// Converts the sequence into a Vec of its parts
    #[inline]
    pub fn as_parts(&self) -> Vec<ReadingPart> {
        self.iter().map(|i| i.to_owned()).collect()
    }

    /// Returns the part at `pos` or None if out of bounds.
    #[inline]
    pub fn part_at(&self, pos: usize) -> Option<ReadingPartRef> {
        self.iter().nth(pos)
    }

    /// Returns the kana reading of the Furigana.
    pub fn kana(&self) -> String {
        FuriToReadingParser::new(self.raw(), true).parse()
    }

    /// Returns the kanji reading of the Furigana.
    pub fn kanji(&self) -> String {
        FuriToReadingParser::new(self.raw(), false).parse()
    }

    /// Returns `true` if the Furigana has at least one kanji part.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.iter().any(|i| i.is_kanji())
    }

    /// Converts the sequence into a Vec of its parts
    #[inline]
    pub fn as_parts_ref(&self) -> Vec<ReadingPartRef> {
        self.iter().collect()
    }

    /// Returns the inner string of the furigana value.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for Furigana {
    #[inline]
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<Furigana> for Vec<ReadingPart> {
    #[inline]
    fn from(value: Furigana) -> Self {
        value.as_parts()
    }
}

impl<S: AsRef<str>> FromIterator<S> for Furigana {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        iter.into_iter().fold(Furigana::default(), |mut i, f| {
            i.0.push_str(f.as_ref());
            i
        })
    }
}

impl<T: AsPart> Extend<T> for Furigana {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for s in iter {
            self.push_part(s);
        }
    }
}

impl Display for Furigana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_furigana() {
        let furi = Furigana::new_unchecked("[音楽|おん|がく]が[大好|だい|す]きです");
        assert_eq!(furi.kanji(), "音楽が大好きです");
        assert_eq!(furi.kana(), "おんがくがだいすきです");
    }

    #[test]
    fn test_furigana2() {
        let furi = Furigana::new("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。").unwrap();
        assert_eq!(furi.kanji(), "");
    }
}
