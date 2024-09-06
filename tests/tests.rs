use smol_symbol::{s, Symbol};
use smol_symbol_macros::custom_alphabet;

#[docify::export]
#[test]
fn symbol_example() {
    // Symbols can be stored in variables
    let sym1 = s!(hello_world);

    // Symbols can be used in const contexts
    const SYM2: Symbol = s!(goodnight);

    // Symbols can be compared with each other
    let sym3 = s!(hello_world);
    assert_eq!(sym1, sym3);
    assert_ne!(sym1, SYM2);
    assert_ne!(s!(this_is_a_triumph), s!(im_making_a_note_here));

    // Symbols are 16 bytes
    assert_eq!(std::mem::size_of_val(&sym1), 16);
    assert_eq!(std::mem::size_of_val(&sym1), std::mem::size_of::<u128>());

    // Symbols can even be created dynamically at runtime!
    let some_string = String::from("some_random_string");
    let dynamic_sym = Symbol::try_from(some_string).unwrap();
    assert_eq!(dynamic_sym, s!(some_random_string));

    // Can't be longer than 25 characters
    assert!(Symbol::try_from("this_is_too_long_to_store_").is_err());
    assert!(Symbol::try_from("this_is_just_short_enough").is_ok());

    // Character alphabet is limited to lowercase a-z and _
    assert!(Symbol::try_from("this-is-invalid").is_err());
    assert!(Symbol::try_from("this is_invalid").is_err());
    assert!(Symbol::try_from("this.is.invalid").is_err());
}

#[docify::export]
#[test]
fn symbol_type_example() {
    // Symbols can be stored in variables and compared
    let sym1 = s!(hello_world);
    assert_eq!(s!(hello_world), sym1);
    assert_ne!(s!(goodbye), s!(hello));

    // Symbols can be used in const contexts
    const MY_SYM: Symbol = s!(this_is_a_triumph);
    assert_eq!(MY_SYM, s!(this_is_a_triumph));

    // Symbols can be converted directly to Strings
    assert_eq!(sym1.to_string().as_str(), "hello_world");
}

#[docify::export]
#[test]
fn test_basics() {
    assert_eq!(s!(hello).to_string().as_str(), "hello");
    assert_eq!(s!(hello_world).to_string().as_str(), "hello_world");
    let sym1 = s!(my_symbol);
    let sym2 = s!(my_symbol_);
    assert_ne!(sym1, sym2);
    assert_eq!(s!(hello), s!(hello));
    assert_ne!(s!(hello), s!(world));
    assert_eq!(
        s!(symbols_are_really_coolok).to_string().as_str(),
        "symbols_are_really_coolok"
    );
    assert_eq!(
        s!(symbols_are_really_cool_o).to_string().as_str(),
        "symbols_are_really_cool_o"
    );
}

#[test]
fn test_roundtrip() {
    assert_eq!(s!(a_what).to_string().as_str(), "a_what");
    assert_eq!(
        s!(abcdefghijklmnopqrstuvwxy).to_string().as_str(),
        "abcdefghijklmnopqrstuvwxy"
    );
    assert_eq!(
        s!(cdefghijklmnopqrstuvwxyz_).to_string().as_str(),
        "cdefghijklmnopqrstuvwxyz_"
    );
    assert_eq!(
        s!(_________________________).to_string().as_str(),
        "_________________________"
    );
}

#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", s!(this_is_a_symbol)),
        "Symbol { data: 63918987372445988790468, symbol: \"this_is_a_symbol\" }"
    );
}

use smol_symbol::*;

custom_alphabet!(
    Ferris,
    ABCDEFGHIJKLMNOPQRSTVWXYZ東京_abcdefghijklmnopqrstuvwxyz12345678910
);
custom_alphabet!(Short, hello_world);

#[test]
fn test_custom_alphabets() {
    assert_eq!(Ferris::MAX_SYMBOL_LEN, 18);
    assert_eq!(Short::MAX_SYMBOL_LEN, 32);
    let sym1 = s!(hello東_world京_33, Ferris);
    let sym2 = s!(hello_world, Ferris);
    let sym3 = s!(hello_world, Short);
    let sym4 = s!(we_hold_world, Short);
    assert_eq!(sym1, s!(hello東_world京_33, Ferris));
    assert_ne!(sym1, sym2);
    assert_eq!(sym3, s!(hello_world, Short));
    assert_ne!(sym3, sym4);
    let sym2_u128: u128 = sym2.into();
    let sym3_u128: u128 = sym3.into();
    assert_ne!(sym2_u128, sym3_u128);
    assert!(sym3_u128 < sym2_u128);
    let sym5 = s!(HELLO_WORLD, Ferris);
    assert_ne!(sym2, sym5);
}
