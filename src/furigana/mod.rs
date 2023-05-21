/// Transcodes furigana codes into various different styles.
pub mod cformat;
/// Compare furigana segments
pub mod compare;
/// Parses encoded furigana.
pub mod parse;
/// A single segment of an encoded furigana string.
pub mod segment;
/// Sequence of parsed segments.
pub mod seq;

use crate::reading::{traits::AsReadingRef, Reading};
use parse::{
    reading::FuriToReadingParser, unchecked::UncheckedFuriParser, FuriParser, FuriParserGen,
};
use segment::{AsSegment, Segment, SegmentRef};
use seq::FuriSequence;
use std::{
    borrow::Borrow,
    fmt::Display,
    ops::{Deref, Range},
};

use self::{cformat::CodeFormatter, segment::encoder::FuriEncoder};

/// A struct that holds encoded furigana data in a string. Such an element can be created by directly wrapping around
/// a [`String`] or using the `new()` function which has the benefit that the furigana gets validated.
/// Valid encoded furigana looks like this: `[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]です。`
#[derive(Clone, Copy, Hash, Default, Debug)]
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

    /// Returns `true` if the Furigana has at least one kana segment.
    #[inline]
    pub fn has_kana(&self) -> bool {
        self.gen_parser().any(|i| !i.1)
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

    /// Returns a `Reading` of the furigana.
    #[inline]
    pub fn to_reading(&self) -> Reading {
        self.into()
    }

    /// Converts the furigana to a `FuriSequence`.
    #[inline]
    pub fn to_seq(&self) -> FuriSequence<SegmentRef> {
        self.into()
    }

    /// Returns an iterator over all kana segments.
    #[inline]
    pub fn kana_segments(&self) -> impl Iterator<Item = SegmentRef> {
        self.gen_parser()
            .filter(|i| !i.1)
            .map(|i| SegmentRef::Kana(i.0))
    }

    /// Returns an iterator over all kanji segments.
    #[inline]
    pub fn kanji_segments(&self) -> impl Iterator<Item = SegmentRef> {
        // FuriParser::new(self.raw()).unchecked()
        self.gen_parser()
            .filter(|i| i.1)
            .map(|i| UncheckedFuriParser::from_seg_str(i.0, i.1))
    }

    /// Returns an iterator over all segments of the furigana.
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

    /// Returns the segment at `pos` or None if out of bounds. This is faster than
    /// `self.segments().nth(pos)` as it only encodes the value at `pos`.
    #[inline]
    pub fn segment_at(&self, pos: usize) -> Option<SegmentRef> {
        self.gen_parser()
            .nth(pos)
            .map(|i| UncheckedFuriParser::from_seg_str(i.0, i.1))
    }

    #[inline]
    pub fn segment_range(&self, r: Range<usize>) -> impl Iterator<Item = SegmentRef> {
        let start = r.start;
        let len = r.len();
        self.gen_parser()
            .skip(start)
            .take(len)
            .map(|i| UncheckedFuriParser::from_seg_str(i.0, i.1))
    }

    /// Converts the sequence into a Vec of its segments.
    #[inline]
    pub fn as_segments_ref(&self) -> Vec<SegmentRef> {
        self.segments().collect()
    }

    /// Replaces all occurring `src_seg` with the given `with` segment.
    pub fn replace_seg<SR, WR>(&self, src: SR, with: WR) -> Furigana<String>
    where
        SR: AsReadingRef,
        WR: AsReadingRef,
    {
        let src = src.as_reading_ref();

        // Don't encode `with` more than once in cases there are two segments to replace.
        let mut with_str: Option<String> = None;

        let mut out_buf = String::with_capacity(self.raw().len());

        for seg_str in self.gen_parser() {
            let seg = UncheckedFuriParser::from_seg_str(seg_str.0, seg_str.1);

            if seg.eq_reading(src) {
                let with = with_str.get_or_insert_with(|| with.encode().into_inner());
                out_buf.push_str(with);
            } else {
                out_buf.push_str(seg_str.0);
            }
        }

        out_buf.shrink_to_fit();

        Furigana(out_buf)
    }

    /// Converts the furigana to a Furigana<String>
    #[inline]
    pub fn as_owned(&self) -> Furigana<String> {
        Furigana(self.raw().to_string())
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

    /// Returns an object that can work on the format of the furigana object.
    #[inline]
    pub fn code_formatter(&self) -> CodeFormatter<T> {
        CodeFormatter::new(self)
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

    /// Pushes an already encoded string to the furigana. Returns an error if `seg` is no valid furigana and can't
    /// be pushed.
    pub fn push_str<S>(&mut self, seg: S) -> Result<(), ()>
    where
        S: AsRef<str>,
    {
        if !FuriParser::check(&seg) {
            return Err(());
        }
        self.push_str_unchecked(seg);
        Ok(())
    }

    /// Pushes a string to the furigana without checking if `seg` is valid furigana. The caller
    /// has to ensure that only valid furigana strings will be pushed.
    #[inline]
    pub fn push_str_unchecked<S>(&mut self, seg: S)
    where
        S: AsRef<str>,
    {
        self.0.push_str(seg.as_ref());
    }
}

impl<T: AsSegment> From<FuriSequence<T>> for Furigana<String> {
    #[inline]
    fn from(value: FuriSequence<T>) -> Self {
        value.encode()
    }
}

impl<'a, T: AsRef<str>> Into<FuriSequence<SegmentRef<'a>>> for &'a Furigana<T> {
    #[inline]
    fn into(self) -> FuriSequence<SegmentRef<'a>> {
        FuriSequence::from(self.segments().to_vec())
    }
}

impl<T: AsRef<str>> From<T> for Furigana<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: AsRef<str>> From<Furigana<T>> for Vec<Segment> {
    #[inline]
    fn from(value: Furigana<T>) -> Self {
        value.as_segments()
    }
}

impl<S: AsSegment> FromIterator<S> for Furigana<String> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut buf = String::new();
        FuriEncoder::new(&mut buf).extend(iter);
        Furigana(buf)
    }
}

impl<S: AsSegment> Extend<S> for Furigana<String> {
    #[inline]
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.0.reserve(iter.size_hint().0 * 6);
        for s in iter {
            self.push_segment(s);
        }
    }
}

impl Into<String> for Furigana<String> {
    #[inline]
    fn into(self) -> String {
        self.0
    }
}

impl<T> Eq for Furigana<T> where T: AsRef<str> + PartialEq<T> {}

impl<T, S> PartialEq<Furigana<S>> for Furigana<T>
where
    T: AsRef<str>,
    S: AsRef<str>,
    S: PartialEq<T>,
{
    #[inline]
    fn eq(&self, other: &Furigana<S>) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

impl<T: AsRef<str>> Display for Furigana<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw())
    }
}

impl<T: AsRef<str>> Borrow<str> for Furigana<T> {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: AsRef<str>> Borrow<T> for Furigana<T> {
    #[inline]
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: AsRef<str>> AsRef<str> for Furigana<T> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.raw()
    }
}

impl<T: AsRef<str>> AsRef<T> for Furigana<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: AsRef<str>> PartialEq<String> for Furigana<T> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.raw() == other
    }
}

impl<T: AsRef<str>> PartialEq<&str> for Furigana<T> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.raw() == *other
    }
}

impl<'a, T> IntoIterator for &'a Furigana<T>
where
    T: AsRef<str>,
{
    type Item = SegmentRef<'a>;
    type IntoIter = UncheckedFuriParser<'a>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.segments()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::reading::ReadingRef;
    use criterion::black_box;

    #[test]
    fn test_furigana() {
        let furi = Furigana::new_unchecked("[音楽|おん|がく]が[大好|だい|す]きです");
        assert_eq!(furi.kanji().to_string(), "音楽が大好きです");
        assert_eq!(furi.kana().to_string(), "おんがくがだいすきです");
        assert_eq!(furi.kana_str(), furi.kana().to_string());
        assert_eq!(furi.kanji_str(), furi.kanji().to_string());
        assert_eq!(
            furi.to_reading(),
            ReadingRef::new_with_kanji("おんがくがだいすきです", "音楽が大好きです")
        );

        assert!(furi.has_kana());
        assert!(furi.has_kanji());
        assert!(furi.contains_kanji('音'));
        assert!(!furi.contains_kanji('弱'));
        assert_eq!(furi.segment_count(), 4);
        assert_eq!(furi.segment_at(black_box(2)), furi.segments().nth(2));
        assert_eq!(furi.segment_at(1), Some(SegmentRef::new_kana("が")));
        assert_eq!(
            furi.as_segments(),
            vec![
                SegmentRef::new_kanji_mult("音楽", &["おん", "がく"]),
                SegmentRef::new_kana("が"),
                SegmentRef::new_kanji_mult("大好", &["だい", "す"]),
                SegmentRef::new_kana("きです")
            ]
        );
        assert_eq!(furi.as_segments(), furi.as_segments_ref());
        assert_eq!(furi.as_ref().raw(), furi.raw());

        let mut furi2 = furi.as_owned();
        furi2.push_segment(SegmentRef::new_kana("よ"));
        assert_eq!(
            furi2.as_segments(),
            vec![
                SegmentRef::new_kanji_mult("音楽", &["おん", "がく"]),
                SegmentRef::new_kana("が"),
                SegmentRef::new_kanji_mult("大好", &["だい", "す"]),
                SegmentRef::new_kana("きですよ")
            ]
        );
        furi2.push_str("ね").unwrap();
        assert_eq!(
            furi2.as_segments(),
            vec![
                SegmentRef::new_kanji_mult("音楽", &["おん", "がく"]),
                SegmentRef::new_kana("が"),
                SegmentRef::new_kanji_mult("大好", &["だい", "す"]),
                SegmentRef::new_kana("きですよね")
            ]
        );

        assert_eq!(
            furi2.segment_range(1..3).collect::<Vec<_>>(),
            vec![
                SegmentRef::new_kana("が"),
                SegmentRef::new_kanji_mult("大好", &["だい", "す"]),
            ]
        );

        assert_eq!(
            furi.kanji_segments().collect::<Vec<_>>(),
            furi.segments().filter(|i| i.is_kanji()).collect::<Vec<_>>()
        );
        assert_eq!(
            furi.kana_segments().collect::<Vec<_>>(),
            furi.segments().filter(|i| i.is_kana()).collect::<Vec<_>>()
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

    #[test]
    fn test_replace_seg() {
        let new = Furigana::new_unchecked("[音楽|おん|がく]が[大好|だい|す]きです")
            .replace_seg(("おんがく", "音楽"), "セックス");
        assert_eq!(new, Furigana("セックスが[大好|だい|す]きです"))
    }

    #[test]
    fn test_is_empty() {
        assert!(Furigana("").is_empty())
    }
}
