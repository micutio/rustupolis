extern crate rustupolis;

use rustupolis::tuple::{Tuple, E};
use rustupolis::space::{SimpleStore, Store};

#[macro_use]
extern crate log;

extern crate error_chain;

use error_chain::ChainedError;

#[test]
fn test_len() {
    let t_space = SimpleSpace::new();
    assert_eq!(t_space.len(), 0);
    for i in 0..42 {
        assert_ok!(t_space.out(Tuple::new(&[E::I(i)])));
    }
    assert_eq!(t_space.len(), 42);
}

#[test]
fn test_out() {
    let mut t_space = SimpleStore::new();
    let content = E::S("test content".to_string());
    let tup = Tuple::new(&[content]);

    t_space.out(tup).unwrap()
}

#[test]
fn test_rdp() {
    let mut t_space = SimpleStore::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

    t_space.out(tup1).unwrap();
    let tup4 = t_space.rdp(tup2);

    match tup4.unwrap() {
        Some(x) => assert!(tup3 == x),
        None => assert!(false),
    }
}

#[test]
fn test_inp() {
    let mut t_space = SimpleStore::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

    t_space.out(tup1).unwrap();
    assert_eq!(t_space.len(), 1);
    let tup4 = t_space.inp(tup2);
    assert_eq!(t_space.len(), 0);

    match tup4.unwrap() {
        Some(x) => assert!(tup3 == x),
        None => assert!(false),
    }
}
