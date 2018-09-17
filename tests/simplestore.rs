#[macro_use]
extern crate rustupolis;

use rustupolis::tuple::E;
use rustupolis::store::{SimpleStore, Store};

#[test]
fn test_len() {
    let mut ss = SimpleStore::new();
    assert_eq!(ss.len(), 0);
    for i in 0..42 {
        assert!(ss.out(tuple![E::I(i)]).is_ok());
    }
    assert_eq!(ss.len(), 42);
}

#[test]
fn test_out() {
    let mut ss = SimpleStore::new();
    let tup = tuple![E::S("test content".to_string())];
    assert!(ss.out(tup).is_ok())
}

#[test]
fn test_rdp() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![E::D(3.143), E::I(123)];
    let tup2 = tuple![E::D(3.143), E::Any];
    let tup3 = tuple![E::D(3.143), E::Any];

    ss.out(tup1).unwrap();
    let tup4 = ss.rdp(&tup2).unwrap().unwrap();

    assert!(tup3.matches(&tup4));
}

#[test]
fn test_inp() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![E::D(3.143), E::I(123)];
    let tup2 = tuple![E::D(3.143), E::Any];
    let tup3 = tuple![E::D(3.143), E::Any];

    ss.out(tup1).unwrap();
    assert_eq!(ss.len(), 1);
    let tup4 = ss.inp(&tup2);
    assert_eq!(ss.len(), 0);

    match tup4.unwrap() {
        Some(ref x) => assert!(tup3.matches(x)),
        None => assert!(false),
    }

    assert_eq!(ss.len(), 0);
}

#[test]
fn test_empty_tuple() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![];
    ss.out(tup1.clone()).unwrap();
    assert_eq!(ss.inp(&tup1).unwrap(), Some(tup1.clone()));
}

#[test]
fn test_contains_empty_tuple() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![E::T(tuple![])];
    ss.out(tup1.clone()).unwrap();
    assert_eq!(ss.inp(&tuple![E::Any]).unwrap(), Some(tup1.clone()));
    ss.out(tup1.clone()).unwrap();
    assert_eq!(ss.inp(&tuple![E::T(tuple![])]).unwrap(), Some(tup1.clone()));
}

#[test]
fn test_match_defined() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![E::I(123)];
    ss.out(tup1.clone()).unwrap();
    assert_eq!(ss.inp(&tup1).unwrap(), Some(tup1.clone()));
}

#[test]
fn test_match_nested() {
    let mut ss = SimpleStore::new();
    let tup1 = tuple![E::I(123), E::T(tuple![E::str("hello")])];
    ss.out(tup1.clone()).unwrap();
    assert_eq!(ss.inp(&tup1).unwrap(), Some(tup1.clone()));
}
