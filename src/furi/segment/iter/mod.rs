pub mod flatten;
pub mod lit_readings;

use self::lit_readings::LitReadingsIter;
use super::{kanji::as_kanji::AsKanjiSegment, AsSegment};
use crate::reading::Reading;

/// Iterator over all readings of a `Segment`
pub struct SegmentIter<'a, S>
where
    S: AsSegment,
{
    segment: &'a S,
    did_kana: bool,
    kanji_lits: Option<LitReadingsIter<'a, S::KanjiType>>,
}

impl<'a, S> SegmentIter<'a, S>
where
    S: AsSegment,
{
    #[inline]
    pub(crate) fn new(segment: &'a S) -> Self {
        let kanji_lits = segment.as_kanji().map(|i| i.literal_readings());

        Self {
            segment,
            did_kana: false,
            kanji_lits,
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
            if self.did_kana {
                return None;
            }
            self.did_kana = true;
            return Some(Reading::new(kana.as_ref().to_string()));
        }

        // We checked for kana before and always early return for kana segments.
        let (kanji, kana) = unsafe { self.kanji_lits.as_mut().unwrap_unchecked() }.next()?;
        Some(Reading::new_with_kanji(kana, kanji))
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
