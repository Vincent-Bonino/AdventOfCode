use std::fmt::Display;

use proc_macro2::{Ident, Span};

/// Supported char ranges.
#[derive(Clone, Copy, Debug)]
pub enum CharRange {
    Alphabetic,
    Alphanumeric,
    Digit,
    Hexdigit,
    Lowercase,
    Punctuation,
    Uppercase,
}

impl CharRange {
    pub fn get_match_function(&self) -> Ident {
        Ident::new(&format!("is_ascii_{}", self), Span::call_site())
    }

    // Compare with other chars / CharRanges

    pub fn match_char(&self, value: &char) -> bool {
        match self {
            CharRange::Alphabetic => value.is_ascii_alphabetic(),
            CharRange::Alphanumeric => value.is_ascii_alphanumeric(),
            CharRange::Digit => value.is_ascii_digit(),
            CharRange::Hexdigit => value.is_ascii_hexdigit(),
            CharRange::Lowercase => value.is_ascii_lowercase(),
            CharRange::Punctuation => value.is_ascii_punctuation(),
            CharRange::Uppercase => value.is_ascii_uppercase(),
        }
    }

    #[inline]
    pub fn has_overlap_with(&self, other: &CharRange) -> bool {
        let lhs: u8 = self._to_match_mask();
        let rhs: u8 = other._to_match_mask();
        lhs & rhs != 0
    }

    /// Build a match mask for the char ranges.
    ///
    /// ```text,ignore
    /// 0000 XXXX  // u8 (ascii) value
    ///      │││└──── punctuation (see [1])     = 1
    ///      ││└───── digits (0-9)              = 2
    ///      │└────── lowercase letters (a-z)   = 4
    ///      └─────── uppercase letters (A-Z)   = 8
    /// ```
    ///
    /// \[1\]: [`char::is_ascii_punctuation`]
    #[inline]
    fn _to_match_mask(&self) -> u8 {
        match self {
            CharRange::Alphabetic => 4 + 8,
            CharRange::Alphanumeric => 2 + 4 + 8,
            CharRange::Digit => 2,
            CharRange::Hexdigit => 2 + 4 + 8,
            CharRange::Lowercase => 4,
            CharRange::Punctuation => 1,
            CharRange::Uppercase => 8,
        }
    }
}

impl TryFrom<String> for CharRange {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "alphabetic" => Ok(Self::Alphabetic),
            "alphanumeric" => Ok(Self::Alphanumeric),
            "digit" => Ok(Self::Digit),
            "hexdigit" => Ok(Self::Hexdigit),
            "lowercase" => Ok(Self::Lowercase),
            "punctuation" => Ok(Self::Punctuation),
            "uppercase" => Ok(Self::Uppercase),
            _ => Err(format!("Unsupported char range '{}'", value)),
        }
    }
}

impl Display for CharRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharRange::Alphabetic => write!(f, "alphabetic"),
            CharRange::Alphanumeric => write!(f, "alphanumeric"),
            CharRange::Digit => write!(f, "digit"),
            CharRange::Hexdigit => write!(f, "hexdigit"),
            CharRange::Lowercase => write!(f, "lowercase"),
            CharRange::Punctuation => write!(f, "punctuation"),
            CharRange::Uppercase => write!(f, "uppercase"),
        }
    }
}
