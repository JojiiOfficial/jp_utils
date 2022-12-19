use crate::{
    alphabet::Alphabet,
    constants::{NORMAL_ALPHANUMERIC, WIDE_ALPHANUMERIC},
    counter,
    radicals::RADICALS,
};
use std::ops::Range;

pub trait JapaneseExt {
    /// Returns true if self is of the given alphabet
    fn is_in_alphabet(&self, a: Alphabet) -> bool;

    /// Get the Alphabet of a character
    fn get_alphabet(&self) -> Alphabet;

    /// Returns true if self contains at least one kana character
    fn has_kana(&self) -> bool;

    /// Returns true if self is entirely written in kana
    fn is_kana(&self) -> bool;

    /// Returns true if inp is entirely written with kanji
    fn is_kanji(&self) -> bool;

    /// Returns true if inp has at least one kanji
    fn has_kanji(&self) -> bool;

    /// Returns true if inp is build with kanji and kana only
    fn is_japanese(&self) -> bool;

    /// Returns true if inp contains japanese characters
    fn has_japanese(&self) -> bool;

    /// Returns true if self is written in katakana
    fn is_katakana(&self) -> bool;

    /// Returns true if self is written in katakana
    fn has_katakana(&self) -> bool;

    /// Returns true if self is written in hiragana
    fn is_hiragana(&self) -> bool;

    /// Returns the amount of kanji self has
    fn kanji_count(&self) -> usize;

    /// Returns true if self is a (cjk) symbol
    fn is_symbol(&self) -> bool;

    /// Returns true if self is a (cjk) symbol
    fn has_symbol(&self) -> bool;

    /// Returns true if self has a roman letter
    fn has_roman_letter(&self) -> bool;

    /// Returns true if self is a roman letter
    fn is_roman_letter(&self) -> bool;

    /// Returns true if self is a small katakana letter
    fn is_small_katakana(&self) -> bool;

    /// Returns true if self is a small hiragana letter
    fn is_small_hiragana(&self) -> bool;

    /// Returns true if self is a small hiragana letter
    fn is_small_kana(&self) -> bool;

    /// Returns `true` if self is a radical
    fn is_radical(&self) -> bool;

    /// Returns `true` if self is a particle
    fn is_particle(&self) -> bool;

    /// Returns `true` if self is a counter
    fn is_counter(&self) -> bool;

    /// Returns `true` if self starts with a character of a given alphabet
    fn starts_with_alphabet(&self, a: Alphabet) -> bool;

    /// Convert Wide-alphanumeric into normal ASCII  [Ａ -> A]
    fn to_halfwidth(&self) -> String;

    /// Convert normal ASCII into Wide-alphanumeric [ A -> Ａ]
    fn to_fullwidth(&self) -> String;

    /// Returns the real length of the string. This is the amount of characters
    fn real_len(&self) -> usize;
}

impl JapaneseExt for char {
    #[inline]
    fn is_in_alphabet(&self, a: Alphabet) -> bool {
        self.get_alphabet() == a
    }

    #[inline]
    fn get_alphabet(&self) -> Alphabet {
        if self.is_symbol() {
            Alphabet::Symbol
        } else if self.is_katakana() {
            Alphabet::katakana()
        } else if self.is_hiragana() {
            Alphabet::hiragana()
        } else if self.is_kanji() || self.is_roman_letter() {
            Alphabet::Kanji
        } else {
            Alphabet::Other
        }
    }

    #[inline]
    fn has_kana(&self) -> bool {
        self.is_kana()
    }

    #[inline]
    fn is_kana(&self) -> bool {
        self.is_hiragana() || self.is_katakana()
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        ((*self) >= '\u{3400}' && (*self) <= '\u{4DBF}')
            || ((*self) >= '\u{4E00}' && (*self) <= '\u{9FFF}')
            || ((*self) >= '\u{F900}' && (*self) <= '\u{FAFF}')
            || ((*self) >= '\u{FF10}' && (*self) <= '\u{FF19}')
            || ((*self) >= '\u{20000}' && (*self) <= '\u{2A6DF}')
            || (*self) == '\u{29E8A}'
            || (*self) == '\u{3005}'
    }

    #[inline]
    fn has_kanji(&self) -> bool {
        self.is_kanji()
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        self.is_kana() || self.is_kanji() || self.is_symbol() || self.is_roman_letter()
    }

    #[inline]
    fn has_japanese(&self) -> bool {
        self.is_japanese()
    }

    #[inline]
    fn is_katakana(&self) -> bool {
        (*self) >= '\u{30A0}' && (*self) <= '\u{30FF}'
    }

    #[inline]
    fn has_katakana(&self) -> bool {
        self.is_katakana()
    }

    #[inline]
    fn is_hiragana(&self) -> bool {
        (*self) >= '\u{3040}' && (*self) <= '\u{309F}'
    }

    #[inline]
    fn kanji_count(&self) -> usize {
        if self.is_kanji() {
            1
        } else {
            0
        }
    }

    #[inline]
    fn is_symbol(&self) -> bool {
        // https://www.htmlsymbols.xyz/ascii-symbols/fullwidth-ascii-variants
        ((*self) >= '\u{3000}' && (*self) <= '\u{303F}' && (*self) != '\u{3005}')
            || ((*self) >= '\u{0370}' && (*self) <= '\u{03FF}')
            || ((*self) >= '\u{25A0}' && (*self) <= '\u{25FF}')
            || ((*self) >= '\u{FF01}' && (*self) <= '\u{FF0F}')
            || ((*self) >= '\u{FF1A}' && (*self) <= '\u{FF20}')
            || ((*self) >= '\u{FF3B}' && (*self) <= '\u{FF40}')
            || ((*self) >= '\u{FF5B}' && (*self) <= '\u{FF5E}')
            || (*self) == '\u{002D}'
            || (*self) == '\u{00D7}'
    }

    #[inline]
    fn has_symbol(&self) -> bool {
        self.is_symbol()
    }

    #[inline]
    fn has_roman_letter(&self) -> bool {
        self.is_roman_letter()
    }

    #[inline]
    fn is_roman_letter(&self) -> bool {
        (*self) >= '\u{FF01}' && (*self) <= '\u{FF5A}'
            || ((*self) >= '\u{2000}' && (*self) <= '\u{206F}')
            || ((*self) >= '\u{20000}' && (*self) <= '\u{2A6DF}')
            || (*self) == '\u{2010}'
            || (*self) == '\u{2212}'
    }

    #[inline]
    fn is_small_katakana(&self) -> bool {
        *self == '\u{30E3}'
            || *self == '\u{30E5}'
            || *self == '\u{30E7}'
            || *self == '\u{30A1}'
            || *self == '\u{30A3}'
            || *self == '\u{30A5}'
            || *self == '\u{30A7}'
            || *self == '\u{30A9}'
    }

    #[inline]
    fn is_small_hiragana(&self) -> bool {
        *self == '\u{3083}'
            || *self == '\u{3085}'
            || *self == '\u{3087}'
            || *self == '\u{3041}'
            || *self == '\u{3043}'
            || *self == '\u{3045}'
            || *self == '\u{3047}'
            || *self == '\u{3049}'
    }

    #[inline]
    fn is_small_kana(&self) -> bool {
        self.is_small_katakana() || self.is_small_hiragana()
    }

    #[inline]
    fn is_radical(&self) -> bool {
        self.is_kanji() || RADICALS.iter().any(|i| *i == *self)
    }

    #[inline]
    fn is_particle(&self) -> bool {
        matches!(
            self,
            'を' | 'の' | 'に' | 'と' | 'が' | 'か' | 'は' | 'も' | 'で' | 'へ' | 'や'
        )
    }

    #[inline]
    fn is_counter(&self) -> bool {
        counter::is_counter(&self.to_string())
    }

    #[inline]
    fn starts_with_alphabet(&self, a: Alphabet) -> bool {
        self.is_in_alphabet(a)
    }

    #[inline]
    fn to_halfwidth(&self) -> String {
        map_char(*self, WIDE_ALPHANUMERIC, |x| x - 0xfee0).to_string()
    }

    #[inline]
    fn to_fullwidth(&self) -> String {
        map_char(*self, NORMAL_ALPHANUMERIC, |x| x + 0xfee0).to_string()
    }

    #[inline]
    fn real_len(&self) -> usize {
        1
    }
}

impl JapaneseExt for str {
    #[inline]
    fn is_in_alphabet(&self, ct: Alphabet) -> bool {
        self.get_alphabet() == ct
    }

    #[inline]
    fn get_alphabet(&self) -> Alphabet {
        if self.is_symbol() {
            Alphabet::Symbol
        } else if self.is_kanji() {
            Alphabet::Kanji
        } else if self.is_hiragana() {
            Alphabet::hiragana()
        } else if self.is_katakana() {
            Alphabet::katakana()
        } else {
            Alphabet::Other
        }
    }

    #[inline]
    fn has_kana(&self) -> bool {
        self.chars().any(|s| s.is_kana())
    }

    #[inline]
    fn is_kana(&self) -> bool {
        self.chars().all(|s| s.is_kana())
    }

    #[inline]
    fn is_kanji(&self) -> bool {
        self.chars().all(|s| s.is_kanji())
    }

    #[inline]
    fn has_kanji(&self) -> bool {
        self.chars().any(|s| s.is_kanji())
    }

    #[inline]
    fn is_japanese(&self) -> bool {
        let mut buf = [0; 16];
        !self.chars().any(|c| {
            let s = c.encode_utf8(&mut buf);
            !s.is_kana() && !s.is_kanji() && !s.is_symbol() && !s.is_roman_letter()
        })
    }

    #[inline]
    fn has_japanese(&self) -> bool {
        let mut buf = [0; 16];
        self.chars().any(|c| {
            let s = c.encode_utf8(&mut buf);
            s.is_kana() || s.is_kanji() || s.is_symbol() || s.is_roman_letter()
        })
    }

    #[inline]
    fn is_katakana(&self) -> bool {
        self.chars().all(|s| s.is_katakana())
    }

    #[inline]
    fn has_katakana(&self) -> bool {
        self.chars().any(|s| s.is_katakana())
    }

    #[inline]
    fn is_hiragana(&self) -> bool {
        self.chars().all(|s| s.is_hiragana())
    }

    #[inline]
    fn kanji_count(&self) -> usize {
        self.chars().filter(|i| i.is_kanji()).count()
    }

    #[inline]
    fn is_symbol(&self) -> bool {
        self.chars().all(|s| s.is_symbol())
    }

    #[inline]
    fn has_symbol(&self) -> bool {
        self.chars().any(|s| s.is_symbol())
    }

    #[inline]
    fn has_roman_letter(&self) -> bool {
        self.chars().any(|s| s.is_roman_letter())
    }

    #[inline]
    fn is_roman_letter(&self) -> bool {
        self.chars().all(|s| s.is_roman_letter())
    }

    #[inline]

    fn is_small_katakana(&self) -> bool {
        self.chars().all(|s| s.is_small_katakana())
    }

    #[inline]
    fn is_small_hiragana(&self) -> bool {
        self.chars().all(|s| s.is_small_hiragana())
    }

    #[inline]
    fn is_small_kana(&self) -> bool {
        self.is_small_katakana() || self.is_small_hiragana()
    }

    #[inline]
    fn is_radical(&self) -> bool {
        self.chars().all(|s| s.is_radical())
    }

    #[inline]
    fn is_particle(&self) -> bool {
        self.chars().all(|s| s.is_particle())
    }

    #[inline]
    fn starts_with_alphabet(&self, a: Alphabet) -> bool {
        let first = self.chars().nth(0);
        match first {
            Some(s) => s.is_in_alphabet(a),
            None => false,
        }
    }

    #[inline]
    fn is_counter(&self) -> bool {
        counter::is_counter(self)
    }

    #[inline]
    fn to_halfwidth(&self) -> String {
        shift_unicode(self, WIDE_ALPHANUMERIC, |x| x - 0xfee0)
    }

    #[inline]
    fn to_fullwidth(&self) -> String {
        shift_unicode(self, NORMAL_ALPHANUMERIC, |x| x + 0xfee0)
    }

    #[inline]
    fn real_len(&self) -> usize {
        self.chars().count()
    }
}

fn shift_unicode<D, S: AsRef<str>>(s: S, range: Range<u32>, conv: D) -> String
where
    D: Fn(u32) -> u32,
{
    s.as_ref()
        .chars()
        .map(|c| map_char(c, range.clone(), &conv))
        .collect()
}

fn map_char<D>(c: char, range: Range<u32>, conv: D) -> char
where
    D: FnOnce(u32) -> u32,
{
    let n = c as u32;
    if range.contains(&n) {
        char::from_u32(conv(n)).unwrap()
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("音",true; "音")]
    #[test_case("々", true)]
    #[test_case("あ",false; "Kana 'a'")]
    #[test_case("、",false; "Special japanese char")]
    fn is_kanji(inp: &str, expcected: bool) {
        assert_eq!(inp.is_kanji(), expcected);
    }

    #[test_case("、",true; "Symbol")]
    #[test_case("音",false; "Kanji")]
    #[test_case("々", false)]
    #[test_case("あ",false; "Kana")]
    fn is_symbol(inp: &str, expcected: bool) {
        assert_eq!(inp.is_symbol(), expcected);
    }

    #[test_case("1234","１２３４"; "To fullwidth")]
    fn test_to_fullwidth(inp: &str, exp: &str) {
        assert_eq!(inp.to_fullwidth().as_str(), exp);
    }

    #[test_case("１２３４","1234"; "To halfwidth")]
    #[test_case("５日","5日"; "With kanji")]
    fn test_to_halfwidth(inp: &str, exp: &str) {
        assert_eq!(inp.to_halfwidth().as_str(), exp);
    }

    #[test_case("音楽", Alphabet::Kanji)]
    #[test_case("、", Alphabet::Symbol)]
    #[test_case("お", Alphabet::hiragana())]
    #[test_case("お", Alphabet::kana())]
    fn test_alphabet_eq(inp: &str, a: Alphabet) {
        assert_eq!(inp.get_alphabet(), a)
    }

    #[test_case("よ", false)]
    #[test_case("ょ", true)]
    #[test_case("ゃ", true)]
    #[test_case("ゅ", true)]
    #[test_case("ョ", true)]
    #[test_case("ャ", true)]
    #[test_case("ュ", true)]
    fn test_small_kana(inp: &str, is_small: bool) {
        assert!(inp.is_small_kana() == is_small);
    }
}
