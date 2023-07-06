use compile_symbol::s;

#[test]
fn test_basics() {
    assert_eq!(s!(hello), 3276872);
    assert_eq!(s!(hello_world), 1012277775959765);
    assert_eq!(s!(hello), s!(hello));
    assert_ne!(s!(hello), s!(world));
    assert_eq!(
        s!(symbols_are_really_coolok),
        172488978592315878732056772488150354
    );
    assert_eq!(
        s!(symbols_are_really_cool_o),
        172488978592315878732056772488150670
    );
}
