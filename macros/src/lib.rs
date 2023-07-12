//! This crate houses the [`s!`] macro, used to create `Symbol`s at compile-time from a
//! provided ident.

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Ident, Token, TypePath};

fn _bad_symbol_error() -> TokenStream {
    return "compile_error!(\"s!() takes a single ident, constrained to a maximum of 25 characters long using an \
            alphabet of lowercase a-z as well as `_`. No other characters are allowed, and you must specify at \
            least one character.\")".parse().unwrap();
}

#[derive(Parse)]
struct SymbolInput {
    ident: Ident,
    _comma: Option<Token![,]>,
    #[parse_if(_comma.is_some())]
    alphabet_path: Option<TypePath>,
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
    let input = parse_macro_input!(tokens as SymbolInput);
    let ident = input.ident.to_string();
    let chars = ident.chars();
    let alphabet_path = input
        .alphabet_path
        .unwrap_or_else(|| parse_quote!(::smol_symbol::DefaultAlphabet));
    quote! {
        #alphabet_path::parse_chars_panic(&[#(#chars),*])
    }
    .into()
}

#[derive(Parse)]
struct CustomAlphabetInput {
    name: Ident,
    _comma: Token![,],
    alphabet: Ident,
}

#[proc_macro]
pub fn custom_alphabet(tokens: TokenStream) -> TokenStream {
    let crate_path = match std::env::var("CARGO_PKG_NAME") {
        Ok(crate_path) => match crate_path.as_str() {
            "smol-symbol" => quote!(crate),
            _ => quote!(::smol_symbol),
        },
        _ => quote!(::smol_symbol),
    };
    let input = parse_macro_input!(tokens as CustomAlphabetInput);
    let name = input.name;
    let alphabet = input.alphabet.to_string().chars().collect::<Vec<char>>();
    let alphabet_len = alphabet.len();
    let alphabet_map = alphabet.iter().enumerate().map(|(i, c)| {
        let i = i + 1;
        quote!(#c => #i)
    });
    let alphabet_map_u128 = alphabet.iter().enumerate().map(|(i, c)| {
        let i = i + 1;
        let i = i as u128;
        quote!(#c => #i)
    });
    quote! {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct #name {}

        impl #crate_path::Alphabet<#alphabet_len> for #name {
            const ALPHABET: [char; #alphabet_len] = [#(#alphabet),*];

            const ALPHABET_INVERTED: #crate_path::__private::Map<char, usize> = #crate_path::__private::phf_map! {
                #(#alphabet_map),*
            };
        }

        impl #name {
            pub const fn invert_char(c: char) -> core::result::Result<u128, #crate_path::SymbolParsingError> {
                let i = match c {
                    #(#alphabet_map_u128),*,
                    _ => return Err(#crate_path::SymbolParsingError {}),
                };
                Ok(i as u128)
            }

            pub const fn parse_chars(chars: &[char]) -> core::result::Result<
                #crate_path::CustomSymbol<#alphabet_len, #name>,
                SymbolParsingError
            > {
                let mut i = 0;
                let mut data: u128 = 0;
                while i < chars.len() {
                    let c = chars[i];
                    let inverted = Self::invert_char(c);
                    data *= CustomSymbol::<#alphabet_len, #name>::LEN_U218 + 1;
                    data += match inverted {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    };
                    i += 1;
                }
                Ok(CustomSymbol {
                    _alphabet: PhantomData,
                    data,
                })
            }

            pub const fn parse_chars_panic(chars: &[char]) -> #crate_path::CustomSymbol<#alphabet_len, #name> {
                match Self::parse_chars(chars) {
                    Ok(sym) => sym,
                    Err(err) => panic!("invalid symbol!"),
                }
            }
        }
    }
    .into()
}
