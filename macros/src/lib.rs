//! This crate houses the [`s!`] macro, used to create `Symbol`s at compile-time from a
//! provided ident.

use proc_macro::{TokenStream, TokenTree};

fn bad_symbol_error() -> TokenStream {
    return "compile_error!(\"s!() takes a single ident, constrained to a maximum of 25 characters long using an \
            alphabet of lowercase a-z as well as `_`. No other characters are allowed, and you must specify at \
            least one character.\")".parse().unwrap();
}

/// Generates a `Symbol` at compile-time from the provided ident.
///
/// Your ident should be constrained to a minimum of one character and a maximum of 25
/// characters long, and may only use an alphabet of lowercase a-z as well as `_`. No other
/// characters are allowed, and specifying other characters or breaking any of these rules will
/// result in a compile error.
///
/// At runtime, each unique`Symbol` is represented internally as a unique [`u128`] that encodes
/// the bits of the symbol (5 bits per character), and enough information is preserved in this
/// representation that the [`u128`] can be converted back into a [`String`] during at runtime,
/// if desired.
///
/// These are great for scenarios where you need a human-readable globally unique identifier.
/// The `Symbol` type is intended to be similar to the `Symbol` type in the Crystal programming
/// language, with the additional capability that `Symbol`s can be created and runtime in
/// addition to compile-time.
#[proc_macro]
pub fn s(tokens: TokenStream) -> TokenStream {
    let mut backing: u128 = 0;
    let mut iter = tokens.into_iter();
    let Some(TokenTree::Ident(ident)) = iter.next() else { return bad_symbol_error() };
    let ident = ident.to_string();
    if ident.is_empty() || ident.len() > 25 {
        return bad_symbol_error();
    }
    for c in ident.chars() {
        let val = match c {
            '-' => 0, // not used
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            '_' => 27,
            _ => return bad_symbol_error(),
        };
        backing *= 28;
        backing += val;
    }
    format!("::compile_symbol::Symbol::from_raw({backing}u128)")
        .as_str()
        .parse()
        .unwrap()
}
