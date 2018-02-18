extern crate rustupolis;

use rustupolis::tuple::{Tuple, E};
use rustupolis::store::{SimpleStore, Store};

#[macro_use]
extern crate log;

extern crate error_chain;

use error_chain::ChainedError;

#[test]
fn test_len() {
    let mut ss = SimpleStore::new();
    assert_eq!(ss.len(), 0);
    for i in 0..42 {
        assert!(ss.out(Tuple::new(&[E::I(i)])).is_ok());
    }
    assert_eq!(ss.len(), 42);
}

#[test]
fn test_out() {
    let mut ss = SimpleStore::new();
    let content = E::S("test content".to_string());
    let tup = Tuple::new(&[content]);

    assert!(ss.out(tup).is_ok())
}

#[test]
fn test_rdp() {
    let mut ss = SimpleStore::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

    ss.out(tup1).unwrap();
    let tup4 = ss.rdp(&tup2).unwrap().unwrap();

    assert!(tup3.matches(&tup4));
}

#[test]
fn test_inp() {
    let mut ss = SimpleStore::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

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
