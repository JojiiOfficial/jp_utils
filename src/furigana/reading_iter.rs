use super::as_part::AsPart;

/// Iterator over all readings of a `ReadingPartRef`
pub struct ReadingIter<'a, P> {
    part: &'a P,
    pos: usize,
    multi_reading: bool,
}

impl<'a, P> ReadingIter<'a, P>
where
    P: AsPart,
{
    #[inline]
    pub fn new(part: &'a P) -> Self {
        let multi_reading = part.is_kanji()
            && part.readings().unwrap().len() == part.as_kanji().unwrap().as_ref().chars().count();
        Self {
            part,
            pos: 0,
            multi_reading,
        }
    }
}

impl<'a, P> Iterator for ReadingIter<'a, P>
where
    P: AsPart,
{
    type Item = (String, Option<String>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(kana) = self.part.as_kana() {
            if self.pos == 0 {
                self.pos += 1;
                return Some((kana.as_ref().to_string(), None));
            }

            return None;
        }

        let kanji = self.part.as_kanji().unwrap().as_ref();
        let readings = self.part.readings().unwrap();

        if !self.multi_reading {
            if self.pos == 0 && readings.len() == 1 {
                self.pos += 1;
                return Some((kanji.to_string(), Some(readings[0].as_ref().to_string())));
            }

            return None;
        }

        let item = readings.get(self.pos).and_then(|r| {
            let k = kanji.chars().nth(self.pos).unwrap();
            Some((k.to_string(), Some(r.as_ref().to_string())))
        })?;

        self.pos += 1;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::furigana::reading_part_ref::ReadingPartRef;
    use test_case::test_case;

    #[test_case("[音楽|おん|がく]", &[("音", Some("おん")), ("楽", Some("がく"))]; "Normal Part")]
    #[test_case("[音楽|おんがく]", &[("音楽", Some("おんがく"))]; "merged multi kanji")]
    #[test_case("かな", &[("かな", None)]; "Kana only")]
    #[test_case("", &[]; "Empty")]
    #[test_case("[音楽|お|ん|がく]", &[("音楽", Some("おんがく"))]; "Malformed kanji")]
    fn test_reading_iter(part: &str, expected: &[(&str, Option<&str>)]) {
        let part = ReadingPartRef::from_str(part);
        let iter = ReadingIter::new(&part);
        for (got, expect) in iter.zip(expected) {
            assert_eq!(got.0, expect.0);
            assert_eq!(got.1.as_deref(), expect.1);
        }
    }
}
