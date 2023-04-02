pub mod iter;
pub mod reading;

use crate::reading::r_owned::ReadingOwned;

use self::{
    iter::{IterItem, SeqIter},
    reading::Reading,
};
use super::{as_part::AsPart, encode, reading_part::ReadingPart, reading_part_ref::ReadingPartRef};
use std::{slice::Iter, str::FromStr};

/// Sequence of multiple furigana reading parts.
#[cfg_attr(feature = "with_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FuriSequence<T> {
    parts: Vec<T>,
}

impl<T> FuriSequence<T>
where
    T: AsPart,
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
    pub fn kana_reading(&self) -> Reading<T> {
        Reading::new(self, true)
    }

    /// Returns the whole sequence as kana string. Eg `[音楽|おん|がく]が[好|す]き` will return `おんがくがすき`
    pub fn as_kana(&self) -> String {
        self.kana_reading().to_string()
    }

    /// Returns the reading as kanji
    #[inline]
    pub fn kanji_reading(&self) -> Reading<T> {
        Reading::new(self, false)
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
    pub fn flattened_iter(&self) -> impl Iterator<Item = ReadingPart> + '_ {
        self.parts.iter().map(|i| i.reading_flattened()).flatten()
    }

    /// Converts the sequence into a Vec of its parts
    #[inline]
    pub fn into_parts(self) -> Vec<T> {
        self.parts
    }

    /// Encodes the sequence to a parsable furigana string.
    #[inline]
    pub fn encode(&self) -> String {
        encode::sequence(self.iter())
    }

    /// Returns `true` if the FuriSequence has at least one kanji part.
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.parts.iter().any(|i| i.is_kanji())
    }

    /// Returns a ReadingOwned representing the reading of the sequence.
    pub fn to_reading(&self) -> ReadingOwned {
        if self.has_kanji() {
            ReadingOwned::new_with_kanji(
                self.kana_reading().to_string(),
                self.kanji_reading().to_string(),
            )
        } else {
            ReadingOwned::new(self.kana_reading().to_string())
        }
    }
}

impl<'a> FuriSequence<ReadingPartRef<'a>> {
    #[inline]
    pub fn to_owned(&self) -> FuriSequence<ReadingPart> {
        self.iter().map(|i| i.to_owned()).collect()
    }
}

impl FromStr for FuriSequence<ReadingPart> {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(super::parse::parse_seq(s)?)
    }
}

impl<T: AsPart> ToString for FuriSequence<T> {
    #[inline]
    fn to_string(&self) -> String {
        self.encode()
    }
}

impl<'s, T> IntoIterator for &'s FuriSequence<T>
where
    T: AsPart,
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
    T: AsPart,
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
    T: AsPart,
{
    #[inline]
    fn from(parts: Vec<T>) -> Self {
        Self { parts }
    }
}

impl<T: Default> Default for FuriSequence<T>
where
    T: AsPart,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Extend<T> for FuriSequence<T>
where
    T: AsPart,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.parts.extend(iter)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for FuriSequence<T>
where
    T: AsPart,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encode())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FuriSequence<ReadingPart> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(RPDeser)
    }
}

#[cfg(feature = "serde")]
struct RPDeser;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for RPDeser {
    type Value = FuriSequence<ReadingPart>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected string in furigana format!")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let furi = super::parse::parse_seq(v)
            .map_err(|_| serde::de::Error::custom("Invalid Furigana format"))?;
        Ok(furi)
    }
}

#[cfg(feature = "serde")]
impl<'a, 'de: 'a> serde::Deserialize<'de> for FuriSequence<ReadingPartRef<'a>> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(RPDeserRef)
    }
}

#[cfg(feature = "serde")]
struct RPDeserRef;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for RPDeserRef {
    type Value = FuriSequence<ReadingPartRef<'de>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Expected string in furigana format!")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let furi = super::parse::parse_seq_ref(v)
            .map_err(|_| serde::de::Error::custom("Invalid Furigana format"))?;
        Ok(furi)
    }
}

#[cfg(test)]
mod tests {
    use crate::furigana::{
        parse::parse_seq_ref, reading_part::ReadingPart, reading_part_ref::ReadingPartRef,
        seq::FuriSequence,
    };
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]が[好|す]き", "おんがくがすき"; "seq_to_kana1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]","はいきんしゅぎはもんだいはいきんしゅぎはもんだいはいきんしゅぎはもんだい"; "seq_to_kana2")]
    fn test_to_kana(furi: &str, expc: &str) {
        let seq = parse_seq_ref(furi).unwrap();
        let kana = seq.as_kana();
        assert_eq!(kana, expc);
    }

    #[test_case("[音楽|おん|がく]が[好|す]き", "音楽が好き"; "seq_to_kanji1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]","拝金主義は問題拝金主義は問題拝金主義は問題"; "seq_to_kanji2")]
    fn test_to_kanji(furi: &str, expc: &str) {
        let seq = parse_seq_ref(furi).unwrap();
        let kana = seq.as_kanji();
        assert_eq!(kana, expc);
    }

    #[test_case("[音楽|おんがく]が[好|す]き", vec![("音楽",Some("おんがく")), ("が",None), ("好", Some("す")), ("き",None)]; "seq_to_kanji1")]
    #[test_case("[音楽|おん|がく]が[好|す]き", vec![("音楽",vec!["おん","がく"]), ("が",vec![]), ("好", vec!["す"]), ("き",vec![])]; "seq_to_kanji2")]
    fn test_iter(furi: &str, parts: Vec<impl Into<ReadingPart>>) {
        let seq = parse_seq_ref(furi).unwrap();
        for (s_item, exp_item) in (&seq).into_iter().zip(parts.into_iter()) {
            let exp_item = exp_item.into();
            assert_eq!(&*s_item, exp_item);
        }
    }

    #[test_case("[音楽|おんがく]が[好|す]き"; "serde1")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]"; "serde2")]
    fn test_serde(furi: &str) {
        let seq = parse_seq_ref(furi).unwrap();
        let json = serde_json::to_string(&seq).unwrap();
        let parsed: FuriSequence<ReadingPart> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, seq.to_owned());
        let parsed_ref: FuriSequence<ReadingPartRef> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed_ref, seq);
    }
}