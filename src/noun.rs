#[cfg(feature = "with-num_enum-0_7")]
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[cfg(feature = "with-serde-1")]
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

pub fn guess_noun_base(noun: &str, noun_class: Option<NounClass>) -> String {
    // All possible noun prefixes ordered by descending length
    const ALL_PREFIXES: &[&str] = &[
        // 4-len prefixes
        "izin", "izim", // 4-len prefixes without initial vowels
        "zin", "zim", // 3-len prefixes
        "iin", "iim", "isi", "izi", "ili", "ama", "imi", "aba", "ulu", "ulw", "ubu", "uku",
        // 3-len prefixes without initial vowels
        "si", "zi", "li", "ma", "mi", "ba", "lu", "lw", "bu", "ku", // 2-len prefixes
        "um", "oo", "is", "iz", "in", "im", "ii", // 2-len prefixes without initial vowels
        "m", "s", "z", "n", // 1-len prefixes
        "u", "i",
    ];

    let normalized: String = noun.nfc().collect();
    let normalized = normalized.trim_start_matches('-');

    let trimmed = match noun_class {
        Some(class) => trim_best_match(normalized, class.to_prefix().forms),
        None => trim_best_match(normalized, ALL_PREFIXES),
    };

    trimmed.to_owned()
}

fn trim_best_match<'a>(word: &'a str, prefixes: &[&str]) -> &'a str {
    for prefix in prefixes {
        let lower = word.to_lowercase();
        let trimmed = lower.trim_start_matches(prefix);
        if trimmed.len() < word.len() {
            let mut start = word.len() - trimmed.len();

            if word.as_bytes().get(start) == Some(&b'-') {
                start += 1;
            }

            // It should be ok to use bytes here, because we trim by unicode
            return &word[start..];
        }
    }

    word
}

/// A noun class. Variants are named by the longest possible conventional prefix, and the class
/// number when the prefix alone is ambiguous.
///
/// # `#[repr(u8)]`
/// Variants are numbered beginning from 1.
#[cfg_attr(feature = "with-num_enum-0_7", derive(IntoPrimitive, TryFromPrimitive))]
#[cfg_attr(feature = "with-serde-1", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NounClass {
    Class1Um = 1,
    Aba,

    U,
    Oo,

    Class3Um,
    Imi,

    Ili,
    Ama,

    Isi,
    Izi,

    In,
    Izin,

    Ulu,
    Ubu,
    Uku,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NounClassPrefix {
    /// Possible forms of the noun class prefix in descending order of length
    pub forms: &'static [&'static str],
}

impl NounClass {
    /// Returns all possible forms of the prefix of this class.
    pub fn to_prefix(&self) -> NounClassPrefix {
        macro_rules! to_prefix {
            ($on:expr => {
                $($variant:ident => [$($string:expr),*],)*
            }) => {
                #[allow(unused_imports)]
                use crate::noun::NounClass::*;

                match $on {
                    $($variant => NounClassPrefix { forms: &[$($string,)*] }),*
                }
            };
        }

        to_prefix! {
            self => {
                Class1Um => ["um"],
                Aba => ["aba"],

                U => ["u"],
                Oo => ["oo"],

                Class3Um => ["um"],
                Imi => ["imi"],

                Ili => ["ili"],
                Ama => ["ama"],

                Isi => ["isi", "is"],
                Izi => ["izi", "iz"],

                In => ["in", "im", "i"],
                Izin => ["izin", "iin", "iim", "ii"],

                Ulu => ["ulu", "u"],
                Ubu => ["ubu"],
                Uku => ["uku"],
            }
        }
    }
}
