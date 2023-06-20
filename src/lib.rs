#![allow(clippy::result_unit_err)]

#[cfg(feature = "hiragana")]
pub mod hiragana;

#[cfg(feature = "furigana")]
pub mod furigana;

pub mod alphabet;
pub mod constants;
pub mod counter;
pub mod furi;
pub mod radicals;
pub mod reading;
pub mod tokenize;
pub mod trait_ext;

pub use trait_ext::JapaneseExt;
