//! This crate houses the [`s!`] macro, used to create `Symbol` / `CustomSymbol` instances at
//! const-eval time from a provided ident and (if applicable) `Alphabet`.

use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Ident, Token, TypePath};

#[derive(Parse)]
struct SymbolInput {
    ident: Ident,
    _comma: Option<Token![,]>,
    #[parse_if(_comma.is_some())]
    alphabet_path: Option<TypePath>,
}

/// Generates a `Symbol` or `CustomSymbol` at const-eval time based on the provided ident and
/// (optional) path to a custom `Alphabet`., e.g.:
///
/// ```ignore
/// let my_sym = s!(hello_world); // uses Symbol / DefaultAlphabet
/// let my_custom_sym = s!(OtHeR, MyCustomAlphabet); // uses the custom alphabet `MyCustomAlphabet`
/// ```
///
/// Your symbol ident should be constrained to a minimum of one character and should be no
/// longer than the `MAX_SYMBOL_LEN` for your chosen alphabet (this is 25 for `DefaultAlphabet`).
///
/// At runtime, each unique`Symbol` is represented internally as a unique [`u128`] that encodes
/// the bits of the symbol (5 bits per character when using `DefaultAlphabet`), and enough
/// information is preserved in this representation that the [`u128`] can be converted back
/// into a [`String`] during at runtime, if desired. In other words, encoding your symbol as a
/// [`u128`] is a non-destructive action that can be reversed.
///
/// These are great for scenarios where you need a human-readable globally unique identifier.
/// The `Symbol` / `CustomSymbol` type is intended to be very loosely similar to the `Symbol`
/// type in the Crystal programming language, though it is strictly much more powerful, with
/// the additional capability that `Symbol`s can be created and runtime in addition to
/// compile-time, and can be directly sorted, hashed, etc., in lexically consistent way.
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

/// Used to parse input to [`custom_alphabet`].
#[derive(Parse)]
struct CustomAlphabetInput {
    name: Ident,
    _comma: Token![,],
    alphabet: Ident,
}

/// Allows you to define a custom alphabet for use with `CustomSymbol` and the [`s!`] macro.
/// The macro takes two idents separated by a comma as input. The first ident should be the
/// name of the alphabet you would like to create, and the second ident should contain all of
/// the characters you would like to use in your alphabet (symbols must be comprised only of
/// characters that are valid in an
/// [ident](https://doc.rust-lang.org/reference/identifiers.html).
///
/// For example, this would define `MyAlphabet` to consist of uppercase A-Z, lowercase a-z, and
/// digits, and would have a resulting `MAX_SYMBOL_LEN` of 21 characters long:
///
/// ```ignore
/// custom_alphabet!(MyAlphabet, abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789);
///
/// let my_sym = s!(SoMeThInG33, MyAlphabet);
/// ```
///
/// It is worth noting that in general, the longer an alphabet is, the lower the
/// `MAX_SYMBOL_LEN` bound will be for that alphabet, since a [`u128`] is always used as the
/// backing for `CustomSymbol`.
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
    let alphabet_map_u128 = alphabet.iter().enumerate().map(|(i, c)| {
        let i = i + 1;
        let i = i as u128;
        quote!(#c => #i)
    });
    let alphabet_map_u128_clone = alphabet_map_u128.clone();
    quote! {
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub struct #name;

        impl #crate_path::Alphabet<#alphabet_len> for #name {
            const ALPHABET: [char; #alphabet_len] = [#(#alphabet),*];

            fn invert_char(c: char) -> core::result::Result<u128, #crate_path::SymbolParsingError> {
                let i = match c {
                    #(#alphabet_map_u128),*,
                    _ => return Err(#crate_path::SymbolParsingError),
                };
                Ok(i as u128)
            }
        }

        impl #name {
            pub const fn invert_char(c: char) -> core::result::Result<u128, #crate_path::SymbolParsingError> {
                let i = match c {
                    #(#alphabet_map_u128_clone),*,
                    _ => return Err(#crate_path::SymbolParsingError),
                };
                Ok(i as u128)
            }

            pub const fn parse_chars(chars: &[char]) -> core::result::Result<
                #crate_path::CustomSymbol<#alphabet_len, #name>,
                #crate_path::SymbolParsingError
            > {
                let mut i = chars.len() - 1;
                let mut data: u128 = 0;
                loop {
                    let c = chars[i];
                    let inverted = Self::invert_char(c);
                    data *= #name::LEN_U218 + 1;
                    data += match inverted {
                        Ok(val) => val,
                        Err(err) => return Err(err),
                    };
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
                Ok(#crate_path::CustomSymbol::from_raw(data))
            }

            pub const fn parse_chars_panic(chars: &[char]) -> #crate_path::CustomSymbol<#alphabet_len, #name> {
                match Self::parse_chars(chars) {
                    Ok(sym) => sym,
                    Err(err) => panic!("{}", #crate_path::PARSING_ERROR_MSG),
                }
            }
        }
    }
    .into()
}
