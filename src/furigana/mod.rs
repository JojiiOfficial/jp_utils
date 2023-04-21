/// Compare furigana
pub mod compare;
/// Parses encoded furigana
pub mod parse;
/// A single segment of an encoded furigana string.
pub mod segment;
/// Furigana sequence
pub mod seq;

use self::{
    parse::reading::FuriToReadingParser,
    segment::{AsSegment, Segment, SegmentRef},
};
use parse::FuriParser;
use std::fmt::Display;

/// A struct that holds encoded furigana data in a string. Such an element can be created by directly wrapping around
/// a [`String`] or using the `new()` function which has the benefit that the furigana gets validated.
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

    /// Returns the kana reading of the Furigana.
    #[inline]
    pub fn kana(&self) -> FuriToReadingParser {
        FuriToReadingParser::new(self.raw(), true)
    }

    /// Returns the kana reading as string. If you want more customizability, use `kana()`.
    #[inline]
    pub fn kana_str(&self) -> String {
        self.kana().to_string()
    }

    /// Returns the kanji reading of the Furigana.
    #[inline]
    pub fn kanji(&self) -> FuriToReadingParser {
        FuriToReadingParser::new(self.raw(), false)
    }

    /// Returns the kanji reading as string. If you want more customizability, use `kanji()`.
    #[inline]
    pub fn kanji_str(&self) -> String {
        self.kanji().to_string()
    }

    /// Returns `true` if the Furigana has at least one kanji segment.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.segments().any(|i| i.is_kanji())
    }

    /// Returns an Iterator over all segments of the furigana.
    #[inline]
    pub fn segments(&self) -> impl Iterator<Item = SegmentRef> {
        FuriParser::new(self.raw()).unchecked().map(|i| i.unwrap())
    }

    /// Returns the amount of reading segments.
    #[inline]
    pub fn segment_count(&self) -> usize {
        self.segments().count()
    }

    /// Pushes a strting to the furigana. Returns an error if `seg` is no valif furigana and can't
    /// be pushed.
    pub fn push_str<S>(&mut self, seg: S) -> Result<(), ()>
    where
        S: AsRef<str>,
    {
        let seg = seg.as_ref();
        if !FuriParser::check(seg) {
            return Err(());
        }
        self.0.push_str(seg);
        Ok(())
    }

    /// Pushes a strting to the furigana without checking if `seg` is valid furigana. The caller
    /// has to ensure that only valid furigana strings will be pushed.
    #[inline]
    pub fn push_str_unchecked<S>(&mut self, seg: S)
    where
        S: AsRef<str>,
    {
        self.0.push_str(seg.as_ref());
    }

    /// Pushes a segment to the end of the furigana sequence.
    #[inline]
    pub fn push_segment<T>(&mut self, seg: T)
    where
        T: AsSegment,
    {
        self.0.push_str(&seg.encode());
    }

    /// Converts the sequence into a Vec of its segments.
    #[inline]
    pub fn as_segments(&self) -> Vec<Segment> {
        self.segments().map(|i| i.to_owned()).collect()
    }

    /// Returns the segment at `pos` or None if out of bounds.
    #[inline]
    pub fn segment_at(&self, pos: usize) -> Option<SegmentRef> {
        self.segments().nth(pos)
    }

    /// Converts the sequence into a Vec of its segments.
    #[inline]
    pub fn as_segments_ref(&self) -> Vec<SegmentRef> {
        self.segments().collect()
    }

    /// Returns the inner string of the furigana value.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<S> From<S> for Furigana
where
    S: AsRef<str>,
{
    #[inline]
    fn from(value: S) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Into<String> for Furigana {
    #[inline]
    fn into(self) -> String {
        self.into_inner()
    }
}

impl From<Furigana> for Vec<Segment> {
    #[inline]
    fn from(value: Furigana) -> Self {
        value.as_segments()
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

impl<T: AsSegment> Extend<T> for Furigana {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for s in iter {
            self.push_segment(s);
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
        assert_eq!(furi.kanji().to_string(), "音楽が大好きです");
        assert_eq!(furi.kana().to_string(), "おんがくがだいすきです");
    }

    #[test]
    fn test_furigana2() {
        let furi = Furigana::new("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。").unwrap();
        assert_eq!(
            furi.kanji().to_string(),
            "2x+1の定義域がA=[1,2]のとき、fの値域はf(A) = [3,5]となる。"
        );
    }
}
