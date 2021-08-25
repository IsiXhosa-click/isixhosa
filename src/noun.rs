use unicode_normalization::UnicodeNormalization;
#[cfg(feature = "with-serde-1")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "with-num_enum-0_5")]
use num_enum::{TryFromPrimitive, IntoPrimitive};

pub fn guess_noun_base(noun: &str, noun_class: Option<NounClass>) -> String {
    // All possible noun prefixes ordered by descending length
    const ALL_PREFIXES: &[&str] = &[
        "izin",

        "iin",
        "iim",
        "isi",
        "izi",
        "ili",
        "ama",
        "imi",
        "aba",
        "ulu",
        "ubu",
        "uku",

        "um",
        "oo",
        "is",
        "iz",
        "in",
        "im",
        "ii",

        "u",
        "i",
    ];

    let normalized: String = noun.nfc().collect();

    let trimmed = match noun_class {
        Some(class) => trim_best_match(&normalized, class.to_prefix().forms),
        None => trim_best_match(&normalized, ALL_PREFIXES)
    };

    trimmed.to_owned()
}

fn trim_best_match<'a>(word: &'a str, prefixes: &[&str]) -> &'a str {
    for prefix in prefixes {
        let trimmed = word.trim_start_matches(prefix);
        if trimmed.len() < word.len() {
            return trimmed;
        }
    }

    word
}

/// A noun class. Variants are named by the longest possible conventional prefix, and the class
/// number when the prefix alone is ambiguous.
///
/// # `#[repr(u8)]`
/// Variants are numbered beginning from 1.
#[cfg_attr(feature = "with-num_enum-0_5", derive(IntoPrimitive, TryFromPrimitive))]
#[cfg_attr(feature = "with-serde-1", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub struct NounClassPrefix {
    /// Possible forms of the noun class prefix in descending order of length
    pub forms: &'static [&'static str],
}

impl NounClass {
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
