//! # smol-symbol 💠
//!
//! [![Crates.io](https://img.shields.io/crates/v/smol-symbol)](https://crates.io/crates/smol-symbol)
//! [![docs.rs](https://img.shields.io/docsrs/smol-symbol?label=docs)](https://docs.rs/smol-symbol/latest/smol_symbol/)
//! [![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/smol-symbol/ci.yaml)](https://github.com/sam0x17/smol-symbol/actions/workflows/ci.yaml?query=branch%3Amain)
//! [![MIT License](https://img.shields.io/github/license/sam0x17/smol-symbol)](https://github.com/sam0x17/smol-symbol/blob/main/LICENSE)
//!
//! This crate provides the ability to create globally unique (per input value),
//! human-readable [`Symbol`]s at compile-time as well as at run-time, that are meant to be
//! reminiscent of the `Symbol` type in the Crystal programming language.
//!
//! Where this crate differs is the alphabet and length of our [`Symbol`] is a bit more
//! restrictive, allowing us to encode the entire text of each [`Symbol`] as a [`u128`]
//! internally. The only caveat is we are limited to 25 characters of length and an alphabet
//! consisting of lowercase a-z as well as `_`.
//!
//! The [`Symbol`] type can be created at compile-time using the convenient [`s!`] macro, and
//! can also be created using the [`From<Into<String>>`] impl at runtime, though this is not as
//! efficient as using the [`s!`] macro.
//!
//! The [`Symbol`] type can also be turned into a [`String`] via a convenient [`Into<String>`].
//!
//! We also provide the ability to define custom alphabets that use the more general
//! [`CustomSymbol`] type via a handy [`custom_alphabet!`] macro, allowing you to alter these
//! restrictions directly (smaller alphabet = larger max length for a symbol) and add support
//! for other languages or less restrictive character sets. The only invariant that can't be
//! customized at the moment is [`CustomSymbol`] will always use a [`u128`] as its backing data
//! store.
//!
//! ### Example
#![doc = docify::embed_run!("tests/tests.rs", symbol_type_example)]
//!
//! See the docs for [`Symbol`] and [`s!`] for more detailed information.

#![no_std]

#[cfg(all(doc, feature = "generate-readme"))]
docify::compile_markdown!("README.docify.md", "README.md");

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::{
    fmt::{Debug, Display, Formatter, Result},
    hash::Hash,
    marker::PhantomData,
};

pub use smol_symbol_macros::*;

/// A compact representation for a (maximum of) 25-character identifier consisting of only
/// lowercase a-z as well as `_`. Internally this data is converted to a [`u128`], allowing for
/// trivial comparison operations between symbols.
///
/// [`Symbol`]s can be created _at compile time_ using the powerful [`s!`] macro. This is the
/// preferred way of creating symbols as it incurs zero overhead at runtime.
///
/// [`Symbol`]s can also be created at runtime, albeit slower than using the [`s!`] macro, via
/// a convenient [`From<AsRef<str>>`] impl on [`Symbol`].
///
/// The [`Symbol`] struct itself impls many useful traits, including [`Copy`], [`Clone`],
/// [`Eq`], [`Ord`], [`Hash`], [`Display`], [`Debug`], [`Send`], and [`Sync`], allowing for a
/// variety of scenarios and use-cases.
///
/// ### Example
#[doc = docify::embed_run!("tests/tests.rs", test_basics)]
pub type Symbol = CustomSymbol<{ DefaultAlphabet::LEN }, DefaultAlphabet>;

/// Represents a custom alphabet for use with [`CustomSymbol`]. To create one of these you
/// should use the [`custom_alphabet!`] macro, as there are several functions you need to
/// define in addition to implementing the trait.
pub trait Alphabet<const N: usize>: Copy + Clone + PartialEq + Eq {
    /// An array of [`char`]'s representing the raw UTF-8 characters that are allowed in this
    /// [`Alphabet`]. All characters in this array should be unique and should be valid
    /// characters that could appear in an [identifier](https://doc.rust-lang.org/reference/identifiers.html).
    const ALPHABET: [char; N];

    /// Auto-generated constant that provides easy access to the size/length of this [`Alphabet`].
    const LEN: usize = N;

    /// Auto-generated constant that provides easy access to the size/length of this
    /// [`Alphabet`] as a [`u128`], for performance reasons.
    const LEN_U218: u128 = Self::LEN as u128;

    /// Auto-generated constant that determines the maximum length a [`CustomSymbol`] using
    /// this [`Alphabet`] could be, based on the number of bits used per symbol character.
    const MAX_SYMBOL_LEN: usize = 128 / ceil_log2(Self::LEN + 1);

    /// Returns the 1-based (0 is reserved) index of this [`char`] in this [`Alphabet`]. An
    /// automatic implementation of this is provided by the [`custom_alphabet!`] macro.
    fn invert_char(c: char) -> core::result::Result<u128, SymbolParsingError>;
}

custom_alphabet!(DefaultAlphabet, abcdefghijklmnopqrstuvwxyz_);

/// The base type used for [`Symbol`] and any custom [`Alphabet`]'s that have been created
/// using [`custom_alphabet!`].
///
/// Typically to create a [`Symbol`] or [`CustomSymbol`], you will want to use the [`s!`] macro.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct CustomSymbol<const N: usize, A: Alphabet<N>> {
    _alphabet: PhantomData<A>,
    data: u128,
}

impl<const N: usize, A: Alphabet<N>> CustomSymbol<N, A> {
    /// Used internally by the [`s!`] macro to create a [`Symbol`] or [`CustomSymbol`] from a
    /// raw [`u128`] generated by the macro's interaction with some const fns.
    pub const fn from_raw(data: u128) -> Self {
        CustomSymbol {
            _alphabet: PhantomData,
            data,
        }
    }

    /// Converts this [`Symbol`] or [`CustomSymbol`] into a human-readable [`String`]
    /// representation. This is only possible because the [`u128`] used as the backing for
    /// [`CustomSymbol`] encodes all bits of information for each character in the
    /// [`CustomSymbol`].
    pub fn to_string(&self) -> String {
        self.into()
    }
}

impl<const N: usize, A: Alphabet<N>> PartialEq for CustomSymbol<N, A> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}
impl<const N: usize, A: Alphabet<N>> Eq for CustomSymbol<N, A> {}
impl<const N: usize, A: Alphabet<N>> Hash for CustomSymbol<N, A> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}
impl<const N: usize, A: Alphabet<N>> PartialOrd for CustomSymbol<N, A> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize, A: Alphabet<N>> Ord for CustomSymbol<N, A> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.data.cmp(&other.data)
    }
}

impl<const N: usize, A: Alphabet<N>> From<CustomSymbol<N, A>> for u128 {
    fn from(value: CustomSymbol<N, A>) -> Self {
        value.data
    }
}

/// Thrown when an attempt was made to parse an invalid [`CustomSymbol`] / [`Symbol`]. This can
/// occur when the underlying ident or string is too long, too short, or contains invalid
/// character (characters not in the specified [`Alphabet`]).
pub struct SymbolParsingError;

pub const PARSING_ERROR_MSG: &'static str =
    "To be a valid `Symbol` or `CustomSymbol`, the provided ident or string must be at least one \
    character long, at most `Alphabet::MAX_SYMBOL_LEN` characters long, and consist only of \
    characters that are included in the `Alphabet`. No other characters are permitted, nor is \
    whitespace of any kind.";

impl Debug for SymbolParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(PARSING_ERROR_MSG)
    }
}

impl<const N: usize, A: Alphabet<N>> TryFrom<&str> for CustomSymbol<N, A> {
    type Error = SymbolParsingError;

    /// Attempts to interpret the provided string as a valid [`Symbol`] / [`CustomSymbol`]. The usual parsing
    /// rules for [`CustomSymbol`] apply, namely:
    /// - At least one character
    /// - At most `Alphabet::MAX_SYMBOL_LEN` characters
    /// - Only characters that are contained in the [`Alphabet`].
    ///
    /// If any of these requirements are violated, a generic [`SymbolParsingError`] is returned
    /// and parsing will abort.
    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        if value.is_empty() || value.len() > A::MAX_SYMBOL_LEN {
            return Err(SymbolParsingError {});
        }
        let mut data: u128 = 0;
        for c in value.chars() {
            data *= A::LEN_U218 + 1;
            data += A::invert_char(c)?;
        }
        Ok(CustomSymbol {
            _alphabet: PhantomData,
            data,
        })
    }
}

impl<const N: usize, A: Alphabet<N>> TryFrom<String> for CustomSymbol<N, A> {
    type Error = SymbolParsingError;

    fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
        CustomSymbol::try_from(value.as_str())
    }
}

impl<const N: usize, A: Alphabet<N>> TryFrom<&String> for CustomSymbol<N, A> {
    type Error = SymbolParsingError;

    fn try_from(value: &String) -> core::result::Result<Self, Self::Error> {
        CustomSymbol::try_from(value.as_str())
    }
}

impl<const N: usize, A: Alphabet<N>> From<CustomSymbol<N, A>> for String {
    fn from(value: CustomSymbol<N, A>) -> Self {
        let mut n = value.data;
        let mut chars: Vec<char> = Vec::new();
        let len = (A::ALPHABET.len() + 1) as u128;
        loop {
            let i = n % len;
            n -= i;
            n /= len;
            chars.push(A::ALPHABET[i as usize - 1]);
            if n == 0 {
                break;
            }
        }
        chars.into_iter().rev().collect()
    }
}

impl<const N: usize, A: Alphabet<N>> From<&CustomSymbol<N, A>> for String {
    fn from(value: &CustomSymbol<N, A>) -> Self {
        (*value).into()
    }
}

impl<const N: usize, A: Alphabet<N>> Debug for CustomSymbol<N, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Symbol")
            .field("data", &self.data)
            .field("symbol", &String::from(*self))
            .finish()
    }
}

impl<const N: usize, A: Alphabet<N>> Display for CustomSymbol<N, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.to_string())
    }
}

/// Internal function used to calculate the `ceil(log2(x))` when determining the
/// `MAX_SYMBOL_LEN` of an [`Alphabet`].
const fn ceil_log2(x: usize) -> usize {
    let mut n = x;
    let mut log = 0;
    while n > 1 {
        n = (n + 1) / 2; // ceil division
        log += 1;
    }
    log
}
