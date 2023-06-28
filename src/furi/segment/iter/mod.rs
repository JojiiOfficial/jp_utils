pub mod flatten;

use super::{kanji::as_kanji::AsKanjiSegment, AsSegment};
use crate::reading::Reading;
use std::str::Chars;

/// Iterator over all readings of a `Segment`
pub struct SegmentIter<'a, S> {
    segment: &'a S,
    pos: usize,
    lit_iter: Option<Chars<'a>>,
}

impl<'a, S> SegmentIter<'a, S>
where
    S: AsSegment,
{
    #[inline]
    pub(crate) fn new(segment: &'a S) -> Self {
        Self {
            segment,
            pos: 0,
            lit_iter: None,
        }
    }
}

impl<'a, S> Iterator for SegmentIter<'a, S>
where
    S: AsSegment,
{
    type Item = Reading;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle kana
        if let Some(kana) = self.segment.as_kana() {
            if self.pos > 0 {
                return None;
            }
            self.pos = 1;
            return Some(Reading::new(kana.as_ref().to_string()));
        }

        // We checked for kana before and always early return for kana segments.
        let kanji = unsafe { self.segment.as_kanji().unwrap_unchecked() };

        // Handle non detailed kanji
        if !kanji.is_detailed() {
            if self.pos > 0 {
                return None;
            }
            self.pos = 1;
            let kana = self.segment.get_kana_reading();
            let kanji = kanji.literals().as_ref().to_string();
            return Some(Reading::new_with_kanji(kana, kanji));
        }

        if self.lit_iter.is_none() {
            self.lit_iter = Some(kanji.literals().as_ref().chars());
        }

        let lit = unsafe { self.lit_iter.as_mut().unwrap_unchecked() }.next()?;
        let reading = kanji.readings()[self.pos].as_ref().to_string();

        self.pos += 1;

        Some(Reading::new_with_kanji(reading, lit.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{furi::segment::SegmentRef, reading::ReadingRef};
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", &[
        ReadingRef::new_with_kanji("おん", "音"),
        ReadingRef::new_with_kanji("がく", "楽"),
    ]; "Normal Part")]
    #[test_case("[音楽|おんがく]", &[
        ReadingRef::new_with_kanji("おんがく", "音楽"),
    ]; "merged multi kanji")]
    #[test_case("かな", &[
        ReadingRef::new("かな")
    ]; "Kana only")]
    #[test_case("", &[]; "Empty")]
    #[test_case("[音楽|お|ん|がく]", &[
        ReadingRef::new_with_kanji("おんがく", "音楽")
    ]; "Malformed kanji")]
    fn test_reading_iter(part: &str, expected: &[ReadingRef]) {
        let part = SegmentRef::from_str_unchecked(part);
        let iter = SegmentIter::new(&part);
        for (got, expect) in iter.zip(expected) {
            assert_eq!(got, *expect);
        }
    }
}
