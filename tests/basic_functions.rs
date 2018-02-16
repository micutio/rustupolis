extern crate rustupolis;

use rustupolis::tuple::{E, Tuple};
use rustupolis::tuplespace::{SimpleSpace, Space};

#[macro_use]
extern crate log;

extern crate error_chain;

use error_chain::ChainedError;

#[test]
fn test_out() {
    info!("testing basic tuplespace function out()");
    let mut t_space = SimpleSpace::new();
    let content = E::S("test content".to_string());
    let tup = Tuple::new(&[content]);

    t_space.out(tup).unwrap()
}

#[test]
fn test_rd() {
    info!{"testing basic tuplespace function rd()"}
    let mut t_space = SimpleSpace::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

    t_space.out(tup1).unwrap();
    let tup4 = t_space.rd(tup2);

    match tup4.unwrap() {
        Some(x) => assert!(tup3 == x),
        None => assert!(false),
    }
}

#[test]
fn test_in() {
    info!{"testing basic tuplespace function in_()"}

    let mut t_space = SimpleSpace::new();
    let tup1 = Tuple::new(&[E::D(3.14), E::I(123)]);
    let tup2 = Tuple::new(&[E::D(3.14), E::Any]);
    let tup3 = Tuple::new(&[E::D(3.14), E::Any]);

    t_space.out(tup1).unwrap();
    assert_eq!(t_space.len(), 1);
    let tup4 = t_space.in_(tup2);
    assert_eq!(t_space.len(), 0);

    match tup4.unwrap() {
        Some(x) => assert!(tup3 == x),
        None => assert!(false),
    }
}
