#[macro_use]
extern crate rustupolis;
extern crate pretty_env_logger;
use rustupolis::tuple::E;
use rustupolis::wildcard::Tree;

#[test]
fn give_and_take() {
    // env_logger::init();
    let mut t = Tree::new();
    t.insert(tuple![E::Any], 1).unwrap();
    // Wrong shape
    assert_eq!(t.take(tuple![E::I(42), E::S("foo".to_string())]), None);
    // Matched and consumed
    assert_eq!(t.take(tuple![E::I(42)]), Some(1));
    // Gone
    assert_eq!(t.take(tuple![E::I(42)]), None);
    assert_eq!(t.take(tuple![E::Any]), None);
}

#[test]
fn give_more() {
    // env_logger::init();
    let mut t = Tree::new();
    t.insert(tuple![E::Any], 1).unwrap();
    t.insert(tuple![E::Any], 1).unwrap();
    t.insert(tuple![E::Any], 1).unwrap();
    for i in 0..3 {
        assert_eq!(t.take(tuple![E::I(i)]), Some(1));
    }
    assert_eq!(t.take(tuple![E::I(0)]), None);
}

#[test]
fn nested_matching() {
    // env_logger::init();
    let mut t = Tree::new();
    t.insert(
        tuple![E::T(tuple![E::str("command"), E::Any]), E::T(tuple![])],
        "doit".to_string(),
    )
    .unwrap();
    assert_eq!(t.take(tuple![E::str("nonsense")]), None);
    assert_eq!(
        t.take(tuple![
            E::T(tuple![E::str("nonsense"), E::str("gibberish")]),
            E::T(tuple![]),
        ]),
        None
    );
    assert_eq!(
        t.take(tuple![
            E::T(tuple![E::str("command"), E::str("wobble")]),
            E::T(tuple![E::I(10)]),
        ]),
        None
    );
    assert_eq!(
        t.take(tuple![
            E::T(tuple![E::str("command"), E::str("bobble")]),
            E::T(tuple![]),
        ]),
        Some("doit".to_string())
    );
}
