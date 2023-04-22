pub mod iter;
pub mod reading;

use self::{
    iter::{IterItem, SeqIter},
    reading::SReading,
};
use super::{
    parse::FuriParser,
    segment::{encode, AsSegment, Segment, SegmentRef},
    Furigana,
};
use crate::reading::Reading;
use std::{slice::Iter, str::FromStr};

/// Sequence of parsed furigana segments. This type can be helpful if you access the inner parts a
/// lot. Otherwise you should use [`crate::furigana::Furigana`] instead as its memory efficient and
/// most operations are faster and without allocation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuriSequence<T> {
    parts: Vec<T>,
}

impl<T> FuriSequence<T>
where
    T: AsSegment,
{
    /// Create a new empty sequence of furigana parts
    #[inline]
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    #[inline]
    pub fn new_with_parts<I>(parts: I) -> Self
    where
        I: Into<Vec<T>>,
    {
        Self {
            parts: parts.into(),
        }
    }

    /// Create a new empty sequence of furigana parts with a given capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parts: Vec::with_capacity(capacity),
        }
    }

    /// Returns the amount of parts the sequence holds.
    #[inline]
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Returns `true` if there is no part in the furigana sequence.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Returns the reading as kana
    #[inline]
    pub fn kana_reading(&self) -> SReading<T> {
        SReading::new(self, true)
    }

    /// Returns the whole sequence as kana string. Eg `[音楽|おん|がく]が[好|す]き` will return `おんがくがすき`
    pub fn as_kana(&self) -> String {
        self.kana_reading().to_string()
    }

    /// Returns the reading as kanji
    #[inline]
    pub fn kanji_reading(&self) -> SReading<T> {
        SReading::new(self, false)
    }

    /// Returns the whole sequence as kanji string. Eg `[音楽|おん|がく]が[好|す]き` will return `音楽が好き`
    pub fn as_kanji(&self) -> String {
        self.kanji_reading().to_string()
    }

    /// Returns the part at `pos` or None if out of bounds.
    #[inline]
    pub fn part_at(&self, pos: usize) -> Option<&T> {
        self.parts.get(pos)
    }

    /// Push a part to the end of the sequence
    #[inline]
    pub fn push_part(&mut self, part: T) {
        self.parts.push(part);
    }

    /// Returns an iter over borrowed items of the parts
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.parts.iter()
    }

    /// Returns an iterator over all reading parts with kanji readings split into separate
    /// ReadingParts.
    #[inline]
    pub fn flattened_iter(&self) -> impl Iterator<Item = Segment> + '_ {
        self.parts.iter().flat_map(|i| i.reading_flattened())
    }

    /// Converts the sequence into a Vec of its parts
    #[inline]
    pub fn into_parts(self) -> Vec<T> {
        self.parts
    }

    /// Encodes the sequence to a parsable furigana string.
    #[inline]
    pub fn encode(&self) -> Furigana<String> {
        encode::sequence(self.iter())
    }

    /// Returns `true` if the FuriSequence has at least one kanji part.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.parts.iter().any(|i| i.is_kanji())
    }

    /// Returns a ReadingOwned representing the reading of the sequence.
    pub fn to_reading(&self) -> Reading {
        if self.has_kanji() {
            Reading::new_with_kanji(
                self.kana_reading().to_string(),
                self.kanji_reading().to_string(),
            )
        } else {
            Reading::new(self.kana_reading().to_string())
        }
    }
}

impl<'a> FuriSequence<SegmentRef<'a>> {
    /// Parse a referencd FuriSequence from a `str`
    #[inline]
    pub fn parse_ref(s: &'a str) -> Result<FuriSequence<SegmentRef<'a>>, ()> {
        FuriParser::new(s).collect()
    }
}

impl<'a> FuriSequence<SegmentRef<'a>> {
    #[inline]
    pub fn to_owned(&self) -> FuriSequence<Segment> {
        self.iter().map(|i| i.to_owned()).collect()
    }
}

impl FromStr for FuriSequence<Segment> {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FuriParser::new(s)
            .map(|i| i.map(|i| i.to_owned()))
            .collect::<Result<_, _>>()
    }
}

impl<T: AsSegment> ToString for FuriSequence<T> {
    #[inline]
    fn to_string(&self) -> String {
        self.encode().into_inner()
    }
}

impl<'s, T> IntoIterator for &'s FuriSequence<T>
where
    T: AsSegment,
{
    type Item = IterItem<'s, T>;
    type IntoIter = SeqIter<'s, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SeqIter::new(self)
    }
}

impl<T> FromIterator<T> for FuriSequence<T>
where
    T: AsSegment,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            parts: Vec::from_iter(iter),
        }
    }
}

impl<T> From<Vec<T>> for FuriSequence<T>
where
    T: AsSegment,
{
    #[inline]
    fn from(parts: Vec<T>) -> Self {
        Self { parts }
    }
}

impl<T: Default> Default for FuriSequence<T>
where
    T: AsSegment,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for FuriSequence<T>
where
    T: AsSegment,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.parts.extend(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]が[好|す]き", "おんがくがすき"; "seq_to_kana1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]","はいきんしゅぎはもんだいはいきんしゅぎはもんだいはいきんしゅぎはもんだい"; "seq_to_kana2")]
    fn test_to_kana(furi: &str, expc: &str) {
        let seq = FuriSequence::parse_ref(furi).unwrap();
        let kana = seq.as_kana();
        assert_eq!(kana, expc);
    }

    #[test_case("[音楽|おん|がく]が[好|す]き", "音楽が好き"; "seq_to_kanji1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]","拝金主義は問題拝金主義は問題拝金主義は問題"; "seq_to_kanji2")]
    fn test_to_kanji(furi: &str, expc: &str) {
        let seq = FuriSequence::parse_ref(furi).unwrap();
        let kana = seq.as_kanji();
        assert_eq!(kana, expc);
    }

    #[test_case("[音楽|おんがく]が[好|す]き", vec![("音楽",Some("おんがく")), ("が",None), ("好", Some("す")), ("き",None)]; "seq_to_kanji1")]
    #[test_case("[音楽|おん|がく]が[好|す]き", vec![("音楽",vec!["おん","がく"]), ("が",vec![]), ("好", vec!["す"]), ("き",vec![])]; "seq_to_kanji2")]
    fn test_iter(furi: &str, parts: Vec<impl Into<Segment>>) {
        let seq = FuriSequence::parse_ref(furi).unwrap();
        for (s_item, exp_item) in (&seq).into_iter().zip(parts.into_iter()) {
            let exp_item = exp_item.into();
            assert_eq!(&*s_item, exp_item);
        }
    }

    #[test_case("[音楽|おんがく]が[好|す]き"; "serde1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]"; "serde2")]
    fn test_serde(furi: &str) {
        let seq = FuriSequence::parse_ref(furi).unwrap();
        let json = serde_json::to_string(&seq).unwrap();
        let parsed: FuriSequence<Segment> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, seq.to_owned());
        let parsed_ref: FuriSequence<SegmentRef> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed_ref, seq);
    }
}
