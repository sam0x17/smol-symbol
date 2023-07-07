use compile_symbol::s;

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
        "Symbol { data: 103472738014991221645200, symbol: \"this_is_a_symbol\" }"
    );
}
