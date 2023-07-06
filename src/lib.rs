use proc_macro::{TokenStream, TokenTree};

fn bad_symbol_error() -> TokenStream {
    return "compile_error!(\"s!() takes a single ident, constrained to a maximum of 25 characters long using an\
            alphabet of lowercase a-z as well as `_`. No other characters are allowed, and you must \
            specify at least one character.\")".parse().unwrap();
}

#[proc_macro]
pub fn s(tokens: TokenStream) -> TokenStream {
    let mut backing: u128 = 0;
    let mut iter = tokens.into_iter();
    let Some(TokenTree::Ident(ident)) = iter.next() else { return bad_symbol_error() };
    let ident = ident.to_string();
    if ident.is_empty() {
        return bad_symbol_error();
    }
    if ident.len() > 25 {
        return bad_symbol_error();
    }
    for c in ident.chars() {
        let val = match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            'i' => 8,
            'j' => 9,
            'k' => 10,
            'l' => 11,
            'm' => 12,
            'n' => 13,
            'o' => 14,
            'p' => 15,
            'q' => 16,
            'r' => 17,
            's' => 18,
            't' => 19,
            'u' => 20,
            'v' => 21,
            'w' => 22,
            'x' => 23,
            'y' => 24,
            'z' => 25,
            '_' => 26,
            _ => return bad_symbol_error(),
        };
        backing *= 26;
        backing += val;
    }
    format!("{backing}u128").as_str().parse().unwrap()
}
