use crate::furi::segment::kanji::as_kanji::AsKanjiSegment;
use std::str::Chars;

/// Iterator over kanji literals and their readings.
pub struct LitReadingsIter<'a, K> {
    kanji: &'a K,
    lit_iter: Option<Chars<'a>>,
    pos: usize,
    detailed: bool,
}

impl<'a, K> LitReadingsIter<'a, K>
where
    K: AsKanjiSegment,
{
    /// Returns a new LitReadingsIter if the passed segment was a kanji segment.
    #[inline]
    pub(crate) fn new(kanji: &'a K) -> Self {
        let detailed = kanji.is_detailed();
        let lit_iter = detailed.then(|| kanji.literals().as_ref().chars());
        Self {
            kanji,
            lit_iter,
            pos: 0,
            detailed,
        }
    }
}

impl<'a, S> Iterator for LitReadingsIter<'a, S>
where
    S: AsKanjiSegment,
{
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        // Handle non detailed kanji
        if !self.detailed {
            if self.pos > 0 {
                return None;
            }

            self.pos = 1;
            let kana = self.kanji.full_reading();
            let kanji = self.kanji.literals().as_ref().to_string();
            return Some((kanji, kana));
        }

        // Safety:
        // For detailed kanji we always set this iterator in the constructor.
        let lit = unsafe { self.lit_iter.as_mut().unwrap_unchecked() }.next()?;

        let reading = self.kanji.readings()[self.pos].as_ref().to_string();

        self.pos += 1;
        Some((lit.to_string(), reading))
    }
}

#[cfg(test)]
mod test {
    use crate::furi::segment::{AsSegment, SegmentRef};

    use super::*;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", &[("音","おん"), ("楽","がく")])]
    #[test_case("[大学|だい|がく]", &[("大","だい"), ("学","がく")])]
    #[test_case("[大学|だいがく]", &[("大学","だいがく")])]
    fn test_lit_readings_iter(s: &str, exp: &[(&str, &str)]) {
        let seg = SegmentRef::from_str_checked(s).unwrap();
        assert!(seg.is_kanji());

        let mut iter = LitReadingsIter::new(seg.as_kanji().unwrap());
        for (e_lit, e_read) in exp {
            let (got_lit, got_read) = iter.next().unwrap();
            assert_eq!(*e_lit, got_lit);
            assert_eq!(*e_read, got_read);
        }

        assert_eq!(iter.next(), None);
    }
}
