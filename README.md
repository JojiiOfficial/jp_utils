# jp_utils
Rust crate providing some handy tools for working with Japanese text

# Usage
```
jp_utils = "0.1.4"
```

# Examples
```rust
use jp_utils::furigana::{segment::SegmentRef, Furigana}; // Feature: "furigana"
use jp_utils::hiragana::Syllable; // Feature: "hiragana"
use jp_utils::JapaneseExt;
use jp_utils::{alphabet::Alphabet, counter::is_counter};

// Basic string functions on japanese alphabet using the `jp_utils::JapaneseExt` trait
assert!("あ".is_kana());
assert!("あ".is_hiragana());
assert!("日本語".is_kanji());
assert!("日ほん語".has_kanji());
assert!("日本語".is_japanese());
assert!("例です".starts_with_alphabet(Alphabet::Kanji));
assert!("ょ".is_small_kana());
assert!(!"よ".is_small_kana());
assert!("、".is_symbol());
assert_eq!("１".to_halfwidth(), "1");

// Hiragana hacks (requires feature: "hiragana")
assert_eq!(
    Syllable::from_char('く').to_dakuten(),
    Syllable::from_char('ぐ')
);

// Furigana parsing (requires feature "furigana"!)
let furigana = Furigana("[日本|に|ほん]が[好|す]きです");
assert_eq!(furigana.kanji_str(), "日本が好きです");
assert_eq!(furigana.kana_str(), "にほんがすきです");

let mut iter = furigana.segments(); // and even iterate over each part
assert_eq!(
    iter.next(),
    Some(SegmentRef::new_kanji_mult("日本", &["に", "ほん"]))
);
assert_eq!(iter.next(), Some(SegmentRef::new_kana("が")));
assert_eq!(iter.next(), Some(SegmentRef::new_kanji("好", "す")));
assert_eq!(iter.next(), Some(SegmentRef::new_kana("きです")));
assert_eq!(iter.next(), None);

// Counter
assert!(is_counter("人"));
assert!(!is_counter("楽"));

```
