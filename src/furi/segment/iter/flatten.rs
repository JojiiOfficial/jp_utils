use crate::furi::segment::{kanji::as_kanji::AsKanjiSegment, s_owned::Segment, traits::AsSegment};
use std::str::Chars;

/// Flattened iterator over segments.
pub enum FlattenIter<'a, S, K> {
    Kana((&'a S, bool)),
    Kanji(FlattenKanjiIter<'a, K>),
}

impl<'a, S, K> FlattenIter<'a, S, K>
where
    S: AsRef<str>,
    K: AsKanjiSegment,
{
    pub fn new<A: AsSegment<KanjiType = K, StrType = S>>(seg: &'a A) -> Self
    where
        A::StrType: 'a,
    {
        if let Some(kanji) = seg.as_kanji() {
            return Self::Kanji(FlattenKanjiIter::new(kanji));
        }

        let kana = seg.as_kana().unwrap();
        Self::Kana((kana, false))
    }
}

impl<'a, S, K> Iterator for FlattenIter<'a, S, K>
where
    S: AsRef<str>,
    K: AsKanjiSegment,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FlattenIter::Kana((s, did)) => {
                if *did {
                    return None;
                }
                *did = true;
                return Some(Segment::new_kana(s.as_ref().to_string()));
            }
            FlattenIter::Kanji(k) => k.next(),
        }
    }
}

pub struct FlattenKanjiIter<'a, K> {
    kanji: &'a K,
    is_detailed: bool,
    chars: Chars<'a>,
    pos: usize,
}

impl<'a, K> FlattenKanjiIter<'a, K>
where
    K: AsKanjiSegment,
{
    #[inline]
    fn new(kanji: &'a K) -> Self {
        let detailed = kanji.is_detailed();
        let chars = kanji.literals().as_ref().chars();
        Self {
            kanji,
            is_detailed: detailed,
            chars,
            pos: 0,
        }
    }
}

impl<'a, K> Iterator for FlattenKanjiIter<'a, K>
where
    K: AsKanjiSegment,
{
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.is_detailed {
            if self.pos > 0 {
                return None;
            }
            self.pos += 1;
            let kana = self.kanji.full_reading();
            let kanji = self.kanji.literals().as_ref().to_string();
            // return Some(Reading::new_with_kanji(kana, kanji));
            let seg = Segment::new_kanji(kanji, &[kana]);
            return Some(seg);
        }

        let lit = self.chars.next()?;
        let reading = self.kanji.readings()[self.pos].as_ref().to_string();
        self.pos += 1;
        Some(Segment::new_kanji(lit.to_string(), &[reading]))
    }
}

#[cfg(test)]
mod test {
    use crate::furi::segment::{kanji::as_kanji::AsKanjiSegment, s_owned::Segment, AsSegment};

    use super::FlattenIter;
    use std::str::FromStr;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", vec![("音","おん"), ("楽","がく")])]
    #[test_case("[大学|だいがく]", vec![("大学","だいがく")])]
    #[test_case("[四字熟語|よ|じ|じゅく|ご]", vec![("四","よ"), ("字", "じ"), ("熟","じゅく"), ("語","ご")])]
    pub fn test_flatten_kanji_iter(p: &str, readings: Vec<(&str, &str)>) {
        let part = Segment::from_str(p).unwrap();
        FlattenIter::new(&part).zip(readings).for_each(|(i, ex)| {
            let kanji = i.as_kanji().unwrap().literals();
            let reading = i.get_kana_reading();
            assert_eq!((kanji.as_str(), reading.as_str()), ex);
        });
    }

    #[test_case("おんがく", vec![("おんがく")])]
    #[test_case("へんたい", vec![("へんたい")])]
    pub fn test_flatten_kana_iter(p: &str, readings: Vec<&str>) {
        let part = Segment::from_str(p).unwrap();
        FlattenIter::new(&part).zip(readings).for_each(|(i, ex)| {
            let kana = i.as_kana().unwrap().to_string();
            let reading = i.get_kana_reading();
            assert_eq!(kana, ex);
            assert_eq!(reading, ex);
        });
    }
}
