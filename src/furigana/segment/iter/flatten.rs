use itertools::Itertools;
use std::str::CharIndices;
use tinyvec::TinyVec;

use crate::furigana::segment::{AsSegment, Segment};

/// Iterator over reading parts that flattenes readings which means that a part with multiple
/// kanji-kana readings will yield multiple Parts each holding a single kanji-kana reading.
pub enum FlattenIter<'a, S: Default> {
    Kanji(FlattenKajiIter<'a, S>),
    Kana((&'a S, bool)),
}

impl<'a, S> FlattenIter<'a, S>
where
    S: AsRef<str> + Default,
{
    pub(crate) fn new<P>(part: &'a P) -> Self
    where
        P: AsSegment<StrType = S>,
    {
        if part.is_kanji() {
            FlattenIter::Kanji(FlattenKajiIter::new(part).unwrap())
        } else {
            let kana = part.as_kana().unwrap();
            FlattenIter::Kana((kana, false))
        }
    }
}

impl<'a, S> Iterator for FlattenIter<'a, S>
where
    S: AsRef<str> + Default,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FlattenIter::Kanji(k) => k.next(),
            FlattenIter::Kana((s, did)) => {
                if *did {
                    return None;
                }
                *did = true;
                Some(Segment::Kana(s.as_ref().to_string()))
            }
        }
    }
}

pub struct FlattenKajiIter<'a, S: Default> {
    // readings: &'a Vec<S>,
    readings: &'a TinyVec<[S; 1]>,
    kanji: &'a S,
    kanji_char_iter: CharIndices<'a>,
    curr_pos: usize,
    exact_reading: bool,
}

impl<'a, S> FlattenKajiIter<'a, S>
where
    S: AsRef<str> + Default,
{
    pub(crate) fn new<P>(part: &'a P) -> Option<Self>
    where
        P: AsSegment<StrType = S>,
    {
        let readings = part.readings()?;
        let kanji = part.as_kanji()?;
        let kanji_char_iter = kanji.as_ref().char_indices();
        let exact_reading = kanji.as_ref().chars().count() == readings.len();
        Some(Self {
            readings,
            kanji,
            kanji_char_iter,
            curr_pos: 0,
            exact_reading,
        })
    }
}

impl<'a, P> Iterator for FlattenKajiIter<'a, P>
where
    P: AsRef<str> + Default,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.exact_reading {
            if self.curr_pos > 0 {
                return None;
            }

            self.curr_pos += 1;
            let reading = self.readings.iter().map(|i| i.as_ref()).join("");
            return Some(Segment::new_kanji(self.kanji.as_ref().to_string(), reading));
        }

        let (k_idx, k_char) = self.kanji_char_iter.next()?;
        let reading = self.readings.get(self.curr_pos)?;

        let start = k_idx;
        let end = start + k_char.len_utf8();

        let kanji_sub = self.kanji.as_ref()[start..end].to_string();

        self.curr_pos += 1;
        Some(Segment::new_kanji(kanji_sub, reading.as_ref().to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::FlattenIter;
    use crate::furigana::{segment::AsSegment, segment::Segment};
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", vec![("音","おん"), ("楽","がく")])]
    #[test_case("[大学|だいがく]", vec![("大学","だいがく")])]
    #[test_case("[四字熟語|よ|じ|じゅく|ご]", vec![("四","よ"), ("字", "じ"), ("熟","じゅく"), ("語","ご")])]
    pub fn test_flatten_kanji_iter(p: &str, readings: Vec<(&str, &str)>) {
        let part = Segment::from_str(p).unwrap();
        FlattenIter::new(&part).zip(readings).for_each(|(i, ex)| {
            let kanji = i.as_kanji().unwrap().to_string();
            let reading = i.kana_reading();
            assert_eq!((kanji.as_str(), reading.as_str()), ex);
        });
    }

    #[test_case("おんがく", vec![("おんがく")])]
    #[test_case("へんたい", vec![("へんたい")])]
    pub fn test_flatten_kana_iter(p: &str, readings: Vec<&str>) {
        let part = Segment::from_str(p).unwrap();
        FlattenIter::new(&part).zip(readings).for_each(|(i, ex)| {
            let kana = i.as_kana().unwrap().to_string();
            let reading = i.kana_reading();
            assert_eq!(kana, ex);
            assert_eq!(reading, ex);
        });
    }
}
