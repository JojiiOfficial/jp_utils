use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vowel {
    A,
    E,
    I,
    O,
    U,
}

impl Vowel {
    #[inline]
    pub fn to_romaji(&self) -> char {
        match *self {
            Vowel::A => 'a',
            Vowel::E => 'e',
            Vowel::I => 'i',
            Vowel::O => 'o',
            Vowel::U => 'u',
        }
    }
}

impl Into<Vowel> for char {
    #[inline]
    fn into(self) -> Vowel {
        match self {
            'a' => Vowel::A,
            'e' => Vowel::E,
            'i' => Vowel::I,
            'o' => Vowel::O,
            'u' => Vowel::U,
            _ => panic!("Not a vowel"),
        }
    }
}

pub fn to_romaji(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        out.push_str(&Syllable::from_char(c).get_splitted()?.to_romaji_char());
    }
    Some(out)
}

/// One single syllable within the a kana alphabet
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Syllable(char);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyllableSplit {
    pub consonant: Option<Consonant>,
    pub vowel: Option<Vowel>,
}

impl SyllableSplit {
    /// Returns (consonant, vowel) in romaji
    #[inline]
    pub fn in_romaji(&self) -> (Option<char>, Option<char>) {
        let vowel = self.vowel.map(|i| i.to_romaji());
        let consonant = self.consonant.and_then(|i| i.to_romaji());
        (consonant, vowel)
    }

    pub fn to_romaji_char(&self) -> String {
        let (consonant, vowel) = self.in_romaji();
        let mut out = String::with_capacity(2);
        if let Some(c) = consonant {
            out.push(c);
        }
        if let Some(v) = vowel {
            out.push(v);
        }
        out
    }

    /// Get the syllable split's consonant.
    pub fn consonant(&self) -> Option<Consonant> {
        self.consonant
    }

    /// Get the syllable split's vowel.
    pub fn vowel(&self) -> Option<Vowel> {
        self.vowel
    }
}

/// A kana row
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Consonant {
    /// あ,え,い,お,う
    Vowels,

    /// ん
    NSpecial,

    K,
    G,
    S,
    Z,
    T,
    D,
    N,
    H,
    B,
    P,
    M,
    R,
    Y,
    W,
}

impl Consonant {
    /// Converts the consonant (row) into its representing romaji character. Does not work for Vowels since
    /// they are being treated different from consonants
    #[inline]
    pub fn to_romaji(&self) -> Option<char> {
        Some(match *self {
            Consonant::Vowels => return None,
            Consonant::K => 'k',
            Consonant::G => 'g',
            Consonant::S => 's',
            Consonant::Z => 'z',
            Consonant::T => 't',
            Consonant::D => 'd',
            Consonant::N | Consonant::NSpecial => 'n',
            Consonant::H => 'h',
            Consonant::B => 'b',
            Consonant::P => 'p',
            Consonant::M => 'm',
            Consonant::R => 'r',
            Consonant::Y => 'y',
            Consonant::W => 'w',
        })
    }
}

impl From<char> for Syllable {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl Into<char> for Syllable {
    fn into(self) -> char {
        self.get_char()
    }
}

impl From<&str> for Syllable {
    fn from(s: &str) -> Self {
        s.chars().next().unwrap().into()
    }
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

impl Syllable {
    pub fn from_char(c: char) -> Self {
        let s: Self = c.into();
        s
    }

    pub fn to_romaji_char(&self) -> Option<String> {
        let split = self.get_splitted()?;
        Some(split.to_romaji_char())
    }

    pub fn get_splitted(&self) -> Option<SyllableSplit> {
        let c = self.0;

        if c == 'ん' {
            return Some(SyllableSplit {
                consonant: Some(Consonant::NSpecial),
                vowel: None,
            });
        }

        for (row, letters) in HIRAGANA_SYLLABLES {
            for (character, vowel) in *letters {
                if *character == c {
                    if *row == Consonant::Vowels {
                        return Some(SyllableSplit {
                            vowel: Some(*vowel),
                            consonant: None,
                        });
                    }
                    return Some(SyllableSplit {
                        vowel: Some(*vowel),
                        consonant: Some(*row),
                    });
                }
            }
        }

        None
    }

    /// Returns the character with dakuten
    #[inline]
    pub fn to_dakuten(&self) -> Self {
        match self.get_char() {
            'た' => Self::from('だ'),
            'て' => Self::from('で'),
            'ち' => Self::from('ぢ'),
            'と' => Self::from('ど'),
            'つ' => Self::from('づ'),
            'か' => Self::from('が'),
            'け' => Self::from('げ'),
            'き' => Self::from('ぎ'),
            'こ' => Self::from('ご'),
            'く' => Self::from('ぐ'),
            'は' => Self::from('ば'),
            'へ' => Self::from('べ'),
            'ひ' => Self::from('び'),
            'ほ' => Self::from('ぼ'),
            'ふ' => Self::from('ぶ'),
            'さ' => Self::from('ざ'),
            'せ' => Self::from('ぜ'),
            'し' => Self::from('じ'),
            'そ' => Self::from('ぞ'),
            'す' => Self::from('ず'),
            _ => *self,
        }
    }

    /// Returns the character hold by [`self`]
    pub fn get_char(&self) -> char {
        self.0
    }

    /// Returns true if the syllable is a valid (hiragana) character
    pub fn is_valid(&self) -> bool {
        self.get_splitted().is_some()
    }
}

/// All (single) hiragana syllables
pub const HIRAGANA_SYLLABLES: &[(Consonant, &[(char, Vowel)])] = &[
    (
        Consonant::Vowels,
        &[
            ('あ', Vowel::A),
            ('え', Vowel::E),
            ('い', Vowel::I),
            ('お', Vowel::O),
            ('う', Vowel::U),
        ],
    ),
    (
        Consonant::K,
        &[
            ('か', Vowel::A),
            ('け', Vowel::E),
            ('き', Vowel::I),
            ('こ', Vowel::O),
            ('く', Vowel::U),
        ],
    ),
    (
        Consonant::G,
        &[
            ('が', Vowel::A),
            ('げ', Vowel::E),
            ('ぎ', Vowel::I),
            ('ご', Vowel::O),
            ('ぐ', Vowel::U),
        ],
    ),
    (
        Consonant::S,
        &[
            ('さ', Vowel::A),
            ('せ', Vowel::E),
            ('し', Vowel::I),
            ('そ', Vowel::O),
            ('す', Vowel::U),
        ],
    ),
    (
        Consonant::Z,
        &[
            ('ざ', Vowel::A),
            ('ぜ', Vowel::E),
            ('じ', Vowel::I),
            ('ぞ', Vowel::O),
            ('ず', Vowel::U),
        ],
    ),
    (
        Consonant::T,
        &[
            ('た', Vowel::A),
            ('て', Vowel::E),
            ('ち', Vowel::I),
            ('と', Vowel::O),
            ('つ', Vowel::U),
        ],
    ),
    (
        Consonant::D,
        &[
            ('だ', Vowel::A),
            ('で', Vowel::E),
            ('ぢ', Vowel::I),
            ('ど', Vowel::O),
            ('づ', Vowel::U),
        ],
    ),
    (
        Consonant::N,
        &[
            ('な', Vowel::A),
            ('ね', Vowel::E),
            ('に', Vowel::I),
            ('の', Vowel::O),
            ('ぬ', Vowel::U),
        ],
    ),
    (
        Consonant::H,
        &[
            ('は', Vowel::A),
            ('へ', Vowel::E),
            ('ひ', Vowel::I),
            ('ほ', Vowel::O),
            ('ふ', Vowel::U),
        ],
    ),
    (
        Consonant::B,
        &[
            ('ば', Vowel::A),
            ('べ', Vowel::E),
            ('び', Vowel::I),
            ('ぼ', Vowel::O),
            ('ぶ', Vowel::U),
        ],
    ),
    (
        Consonant::P,
        &[
            ('ぱ', Vowel::A),
            ('ぺ', Vowel::E),
            ('ぴ', Vowel::I),
            ('ぽ', Vowel::O),
            ('ぷ', Vowel::U),
        ],
    ),
    (
        Consonant::M,
        &[
            ('ま', Vowel::A),
            ('め', Vowel::E),
            ('み', Vowel::I),
            ('も', Vowel::O),
            ('む', Vowel::U),
        ],
    ),
    (
        Consonant::R,
        &[
            ('ら', Vowel::A),
            ('れ', Vowel::E),
            ('り', Vowel::I),
            ('ろ', Vowel::O),
            ('る', Vowel::U),
        ],
    ),
    (
        Consonant::Y,
        &[
            ('や', Vowel::A),
            ('よ', Vowel::O),
            ('ょ', Vowel::O),
            ('ゆ', Vowel::U),
        ],
    ),
    (Consonant::W, &[('わ', Vowel::A), ('を', Vowel::O)]),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_split() {
        assert_eq!(Syllable::from_char('a').get_splitted(), None);
        assert_eq!(Syllable::from_char('a').is_valid(), false);
        assert_eq!(Syllable::from_char('ん').is_valid(), true);
        assert_eq!(Syllable::from_char('こ').is_valid(), true);
        assert_eq!(Syllable::from_char('お').is_valid(), true);

        assert_eq!(
            Syllable::from_char('ん').to_romaji_char().unwrap(),
            "n".to_string()
        );

        assert_eq!(
            Syllable::from_char('そ')
                .to_dakuten()
                .to_romaji_char()
                .unwrap(),
            "zo".to_string()
        );

        assert_eq!(
            Syllable::from_char('ぞ').to_romaji_char().unwrap(),
            "zo".to_string()
        );

        assert_eq!(
            Syllable::from_char('か').to_romaji_char().unwrap(),
            "ka".to_string()
        );

        assert_eq!(
            Syllable::from_char('あ')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (None, Some('a'))
        );

        assert_eq!(
            Syllable::from_char('ん')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('n'), None)
        );

        assert_eq!(
            Syllable::from_char('ず')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('z'), Some('u'))
        );

        assert_eq!(
            Syllable::from_char('か')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('k'), Some('a'))
        );

        assert_eq!(
            Syllable::from_char('づ')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('d'), Some('u'))
        );

        assert_eq!(
            Syllable::from_char('じ')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('z'), Some('i'))
        );

        assert_eq!(
            Syllable::from_char('ち')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('t'), Some('i'))
        );

        assert_eq!(
            Syllable::from_char('る')
                .get_splitted()
                .unwrap()
                .in_romaji(),
            (Some('r'), Some('u'))
        );
    }
}
