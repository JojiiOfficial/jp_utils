pub mod iter;
pub mod reading;

use self::{
    iter::{IterItem, SeqIter},
    reading::Reading,
};
use super::as_part::AsPart;
use std::slice::Iter;

/// Sequence of multiple furigana reading parts.
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

    /// Returns an iter over borrowed items of the parts
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.parts.iter()
    }

    /// Converts the sequence into a Vec of its parts
    #[inline]
    pub fn into_parts(self) -> Vec<T> {
        self.parts
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

#[cfg(test)]
mod tests {
    use crate::furigana::{parse::parse_seq_ref, reading_part::ReadingPart};
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
}
