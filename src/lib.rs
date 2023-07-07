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
