use super::AsPart;
use itertools::Itertools;

/// Encodes a reading. Returns a kanji block if `kanji` is Some() and `kana` as owned if not.
pub fn reading<A, B>(kana: A, kanji: Option<B>) -> String
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    if let Some(kanji) = kanji {
        single_block(kanji, kana)
    } else {
        kana.as_ref().to_string()
    }
}

/// Encodes a single furigana block
#[inline]
pub fn single_block(kanji: impl AsRef<str>, kana: impl AsRef<str>) -> String {
    format!("[{}|{}]", kanji.as_ref(), kana.as_ref())
}

/// Encodes a set of kanji with their own assigned readings. Requires `readings` to drop at least
/// one element which would output the same as a `single_block(..)` call
pub fn multi_block<S>(kanji: impl AsRef<str>, readings: &[S]) -> String
where
    S: AsRef<str>,
{
    let readings = readings.iter().map(|i| i.as_ref()).join("|");
    single_block(kanji, readings)
}

/// Encodes a sequence of ReadingParts as a single furigana string
pub fn sequence<'a, I, P>(iter: I) -> String
where
    I: IntoIterator<Item = &'a P>,
    P: AsPart + 'a,
{
    iter.into_iter().map(|i| i.encode()).join("")
}
