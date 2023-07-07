//! This crate provides the ability to create globally unique (based on input value),
//! human-readable [`Symbol`]s at compile-time as well as at run-time, meant to be reminiscent
//! of the `Symbol` type in the Crystal programming language.
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
//! ### Example
#![doc = docify::embed_run!("tests/tests.rs", symbol_type_example)]
//!
//! See the docs for [`Symbol`] and [`s!`] for more detailed information.

#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
pub use compile_symbol_macros::*;
use core::fmt::{Debug, Display, Formatter, Result};

pub const ALPHABET: [char; 28] = [
    '-', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_',
];
const ALPHABET_LEN: u128 = ALPHABET.len() as u128;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Symbol {
    data: u128,
}

impl Symbol {
    #[inline]
    pub const fn from_raw(data: u128) -> Self {
        Symbol { data }
    }

    pub fn to_string(&self) -> String {
        self.into()
    }
}

impl From<&Symbol> for String {
    fn from(value: &Symbol) -> Self {
        (*value).into()
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        let mut n = value.data;
        let mut chars: Vec<char> = Vec::new();
        loop {
            let i = n % ALPHABET_LEN;
            n -= i;
            n /= ALPHABET_LEN;
            chars.push(ALPHABET[i as usize]);
            if n == 0 {
                break;
            }
        }
        chars.into_iter().rev().collect()
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Symbol")
            .field("data", &self.data)
            .field("symbol", &String::from(*self))
            .finish()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.to_string())
    }
}
