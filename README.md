# Compile Symbol 💠

[![Crates.io](https://img.shields.io/crates/v/compile-symbol)](https://crates.io/crates/compile-symbol)
[![docs.rs](https://img.shields.io/docsrs/compile-symbol?label=docs)](https://docs.rs/compile-symbol/latest/compile-symbol/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/compile-symbol/ci.yaml)](https://github.com/sam0x17/compile-symbol/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/compile-symbol)](https://github.com/sam0x17/compile-symbol/blob/main/LICENSE)

This crate provides the ability to create globally unique (based on input value),
human-readable `Symbol`s at compile-time as well as at run-time, that are meant to be
reminiscent of the `Symbol` type in the Crystal programming language.

Where this crate differs is the alphabet and length of our `Symbol` is a bit more restrictive,
allowing us to encode the entire text of each `Symbol` as a `u128` internally. The only caveat
is we are limited to 25 characters of length and an alphabet consisting of lowercase a-z as
well as `_`. No other characters are permitted.

The `Symbol` type can be created at compile-time using the convenient `s!` macro, and can also
be created using the `TryFrom<AsRef<str>>` impl at runtime, though this is not as efficient as
doing this at compile-time using the `s!` macro.

The `Symbol` type can also be turned into a `String` via a convenient `Into<String>` impl.

### Example
```rust
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
```

See the docs for `Symbol` and `s!` for more detailed information.
