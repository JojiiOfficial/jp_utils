/// Compare furigana segments
pub mod compare;
/// Parses encoded furigana.
pub mod parse;
/// A single segment of an encoded furigana string.
pub mod segment;
/// Sequence of parsed segments.
pub mod seq;

use self::{
    parse::{reading::FuriToReadingParser, unchecked::UncheckedFuriParser, FuriParserGen},
    segment::{AsSegment, Segment, SegmentRef},
    seq::FuriSequence,
};
use crate::reading::Reading;
use parse::FuriParser;
use std::{borrow::Borrow, fmt::Display, ops::Deref};

/// A struct that holds encoded furigana data in a string. Such an element can be created by directly wrapping around
/// a [`String`] or using the `new()` function which has the benefit that the furigana gets validated.
/// Valid encoded furigana looks like this: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]です。`
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Furigana<T>(pub T);

impl<T> Furigana<T>
where
    T: AsRef<str>,
{
    /// Create a new Furigana value with a given encoded furi string as value which gets checked.
    #[inline]
    pub fn new(furi: T) -> Result<Self, ()> {
        if !FuriParser::check(&furi) {
            return Err(());
        }
        Ok(Self::new_unchecked(furi))
    }

    /// Returns `true` if the Furigana is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.raw().is_empty()
    }

    /// Returns the raw (encoded) furigana string.
    #[inline]
    pub fn raw(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns a generalized furigana parser over the furigana data.
    #[inline]
    pub fn gen_parser(&self) -> FuriParserGen {
        FuriParserGen::new(self.raw())
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
        self.gen_parser().any(|i| i.1)
    }

    /// Returns `true` if the furgiana has a given kanji literal.
    #[inline]
    pub fn contains_kanji(&self, kanji: char) -> bool {
        self.raw().contains(kanji)
    }

    /// Returns a [`Reading`] of the furigana.
    #[inline]
    pub fn to_reading(&self) -> Reading {
        Reading::from(self)
    }

    /// Returns an Iterator over all segments of the furigana.
    #[inline]
    pub fn segments(&self) -> UncheckedFuriParser {
        FuriParser::new(self.raw()).unchecked()
    }

    /// Returns the amount of reading segments.
    #[inline]
    pub fn segment_count(&self) -> usize {
        self.gen_parser().count()
    }

    /// Converts the sequence into a Vec of its segments.
    #[inline]
    pub fn as_segments(&self) -> Vec<Segment> {
        self.segments().map(|i| i.to_owned()).collect()
    }

    /// Returns the segment at `pos` or None if out of bounds.
    #[inline]
    pub fn segment_at(&self, pos: usize) -> Option<SegmentRef> {
        self.gen_parser()
            .nth(pos)
            .map(|i| UncheckedFuriParser::from_seg_str(i.0, i.1))
    }

    /// Converts the sequence into a Vec of its segments.
    #[inline]
    pub fn as_segments_ref(&self) -> Vec<SegmentRef> {
        self.segments().collect()
    }
}

impl<T> Furigana<T> {
    /// Returns a new Furgiana block with the inner type dereferenced.
    #[inline]
    pub fn as_deref(&self) -> Furigana<&T::Target>
    where
        T: Deref,
        T::Target: AsRef<str>,
    {
        Furigana(&self.0)
    }

    /// Returns a new furigana wrapper with the current furiganas data as reference.
    #[inline]
    pub fn as_ref(&self) -> Furigana<&T> {
        Furigana(&self.0)
    }

    /// Returns the inner string of the furigana value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Create a new Furigana value with a given encoded furi string as value which doesn't get checked.
    #[inline]
    pub fn new_unchecked(furi: T) -> Self {
        Self(furi)
    }
}

impl Furigana<String> {
    /// Pushes a segment to the end of the furigana sequence.
    #[inline]
    pub fn push_segment<S>(&mut self, seg: S)
    where
        S: AsSegment,
    {
        self.0.push_str(&seg.encode());
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
}

impl<T> From<FuriSequence<T>> for Furigana<String>
where
    T: AsSegment,
{
    #[inline]
    fn from(value: FuriSequence<T>) -> Self {
        value.encode()
    }
}

impl<'a, T> Into<FuriSequence<SegmentRef<'a>>> for &'a Furigana<T>
where
    T: AsRef<str>,
{
    fn into(self) -> FuriSequence<SegmentRef<'a>> {
        FuriSequence::from(FuriParser::new(self.raw()).unchecked().to_vec())
    }
}

impl<T> From<T> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<Furigana<T>> for Vec<Segment>
where
    T: AsRef<str>,
{
    #[inline]
    fn from(value: Furigana<T>) -> Self {
        value.as_segments()
    }
}

impl<S> FromIterator<S> for Furigana<String>
where
    S: AsRef<str>,
{
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        iter.into_iter().fold(Furigana::default(), |mut i, f| {
            i.0.push_str(f.as_ref());
            i
        })
    }
}

impl<S> Extend<S> for Furigana<String>
where
    S: AsSegment,
{
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for s in iter {
            self.push_segment(s);
        }
    }
}

impl<T> Display for Furigana<T>
where
    T: AsRef<str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl<T> Borrow<str> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> Borrow<T> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<str> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn as_ref(&self) -> &str {
        self.raw()
    }
}

impl<T> AsRef<T> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> PartialEq<String> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.raw() == other
    }
}

impl<T> PartialEq<&str> for Furigana<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.raw() == *other
    }
}

#[cfg(test)]
mod test {
    use crate::reading::ReadingRef;

    use super::*;

    #[test]
    fn test_furigana() {
        let furi = Furigana::new_unchecked("[音楽|おん|がく]が[大好|だい|す]きです");
        assert_eq!(furi.kanji().to_string(), "音楽が大好きです");
        assert_eq!(furi.kana().to_string(), "おんがくがだいすきです");
        assert_eq!(
            furi.to_reading(),
            ReadingRef::new_with_kanji("おんがくがだいすきです", "音楽が大好きです")
        );
    }

    #[test]
    fn test_furigana2() {
        let furi = Furigana::new("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。").unwrap();
        assert_eq!(
            furi.kanji().to_string(),
            "2x+1の定義域がA=[1,2]のとき、fの値域はf(A) = [3,5]となる。"
        );
    }

    #[test]
    fn test_furigana3() {
        let furi = Furigana("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。".to_string());
        let furi = furi.as_deref();
        assert_eq!(
            furi.kanji().to_string(),
            "2x+1の定義域がA=[1,2]のとき、fの値域はf(A) = [3,5]となる。"
        );
    }
}
