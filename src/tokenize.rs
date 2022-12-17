use crate::{alphabet::Alphabet, JapaneseExt};
use std::iter;

/// Returns an iterator over all kanji / kana. If `kana_same` is `true` hiragana won't be split
/// from katakana
pub fn by_alphabet<'a>(kanji: &'a str, kana_same: bool) -> impl Iterator<Item = &'a str> {
    let mut kanji_indices = kanji.char_indices().peekable();

    iter::from_fn(move || {
        let (curr_c_pos, curr_char) = kanji_indices.next()?;

        while let Some((pos, c)) = kanji_indices.peek() {
            if (!kana_same && curr_char.get_alphabet() != c.get_alphabet())
                || (kana_same && !curr_char.get_alphabet().eq_both_kana(&c.get_alphabet()))
            {
                return Some(&kanji[curr_c_pos..*pos]);
            }

            kanji_indices.next();
        }

        Some(&kanji[curr_c_pos..])
    })
}

/// Returns an iterator over all substrings of `inp` that have the given alphabet
pub fn words_with_alphabet<'a>(inp: &'a str, alphabet: Alphabet) -> impl Iterator<Item = &'a str> {
    let inp = inp.trim();

    let mut char_iter = inp.char_indices();

    iter::from_fn(move || {
        if inp.is_empty() {
            return None;
        }

        let mut found_next = false;
        let mut start = 0;

        loop {
            let (i, c) = match char_iter.next() {
                Some(i) => i,
                None => return found_next.then(|| &inp[start..]),
            };

            if c.get_alphabet() != alphabet {
                if found_next {
                    return Some(&inp[start..i]);
                }

                continue;
            }

            if !found_next {
                found_next = true;
                start = i;
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("これは漢字で書いたテキストです", &["これは", "漢字", "で", "書", "いたテキストです"]; "Simple")]
    #[test_case("このテキストはかなだけでかいた", &["このテキストはかなだけでかいた"]; "Kana only")]
    #[test_case("朝に道を聞かば、夕べに死すとも可なり", &["朝", "に", "道", "を", "聞", "かば","、", "夕", "べに", "死", "すとも", "可", "なり"]; "Special char")]
    fn test_by_alphabet(inp: &str, exp: &[&str]) {
        let pairs: Vec<_> = by_alphabet(inp, true).collect();
        let exp: Vec<_> = exp.iter().map(|i| i.to_string()).collect();
        assert_eq!(pairs, exp);
    }

    #[test_case("朝に道を聞かば、夕べに死すとも可なり", Alphabet::Kanji, &["朝", "道", "聞", "夕", "死", "可"]; "Kanji")]
    #[test_case("朝に道を聞かば、夕べに死すとも可なり", Alphabet::kana(), &["に", "を", "かば", "べに", "すとも", "なり"]; "Hiragana")]
    fn test_words_with_alphabet(inp: &str, alphabet: Alphabet, exp: &[&str]) {
        let collected: Vec<&str> = words_with_alphabet(inp, alphabet).collect();
        assert_eq!(collected, exp);
    }
}
