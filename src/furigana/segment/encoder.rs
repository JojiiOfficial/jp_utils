use crate::reading::traits::AsReadingRef;

use super::AsSegment;

/// An encoder fur furigana.
pub struct FuriEncoder<'a> {
    out: &'a mut String,
}

impl<'a> FuriEncoder<'a> {
    /// Create a new furigana encoder with a buf as output.
    #[inline]
    pub fn new(out: &'a mut String) -> Self {
        Self { out }
    }

    /// Encodes a segment
    pub fn write_seg<S: AsSegment>(&mut self, segment: S) {
        if let Some(kanji) = segment.as_kanji() {
            let kanji = kanji.as_ref();
            self.write_kanji_seg(&segment, kanji);
        } else if let Some(kana) = segment.as_kana() {
            self.write_kana(kana.as_ref());
        }
    }

    /// Writes kana to the buffer.
    #[inline]
    pub fn write_kana(&mut self, kana: &str) {
        self.out.push_str(kana);
    }

    /// Writes a single block of `[kanji|kana]` to the buffer.
    pub fn write_block(&mut self, kanji: &str, kana: &str) {
        self.out.push('[');
        self.out.push_str(kanji);
        self.out.push('|');
        self.out.push_str(kana);
        self.out.push(']');
    }

    /// Writes a [`jp_utils::reading::Reading`] into the furi encoder.
    ///
    /// Note that `readings` can contain kana characters in their kanji strings.
    pub fn write_reading<R: AsReadingRef>(&mut self, r: R) {
        let r = r.as_reading_ref();
        if let Some(kanji) = r.kanji() {
            self.write_block(kanji, r.kana());
        } else {
            self.write_kana(r.kana());
        }
    }

    /// Writes a kanji segment to the buffer.
    pub(crate) fn write_kanji_seg<S: AsSegment>(&mut self, segment: S, kanji: &str) {
        let readings = segment.readings().unwrap();
        let detailed = segment.detailed_readings().unwrap();

        self.out.push('[');
        self.out.push_str(kanji);
        self.out.push('|');

        for (pos, reading) in readings.iter().enumerate() {
            if pos > 0 && detailed {
                self.out.push('|');
            }
            self.out.push_str(reading.as_ref());
        }

        self.out.push(']');
    }
}

impl<'a, S> Extend<S> for FuriEncoder<'a>
where
    S: AsSegment,
{
    #[inline]
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();
        self.out.reserve(lower * 15);
        for i in iter {
            self.write_seg(i);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::furigana::Furigana;

    use super::*;
    use test_case::test_case;

    #[test_case("";"empty")]
    #[test_case("おんがくが[好|す]"; "End_kanji")]
    #[test_case("おんがくが[好|す]きです")]
    #[test_case("[音楽|おん|がく]が[好|す]き")]
    #[test_case("[拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい][拝金主義|はい|きん|しゅ|ぎ]は[問題|もん|だい]")]
    #[test_case("[楽|たの]しい")]
    #[test_case("[音楽おん|がく]が[好す")]
    #[test_case("この[人|ひと]が[嫌|きら]いです。")]
    #[test_case("[2|][x|えっくす]+[1|]の[定義|てい|ぎ][域|いき]が[A|えい]=[[1|],[2|]]のとき、[f|えふ]の[値域|ち|いき]は[f|えふ]([A|えい]) = [[3|],[5|]]となる。"; "with brackets")]
    fn test_furi_enc_new(furi: &str) {
        let mut buf = String::new();
        let mut encoder = FuriEncoder::new(&mut buf);

        for seg in &Furigana(furi) {
            encoder.write_seg(seg);
        }

        assert_eq!(buf, furi);

        let mut buf2 = String::new();
        let mut encoder = FuriEncoder::new(&mut buf2);
        encoder.extend(&Furigana(furi));
        assert_eq!(buf2, furi);
    }
}
