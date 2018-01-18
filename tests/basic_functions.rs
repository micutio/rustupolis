extern crate rustupolis;

use rustupolis::tuple::E;
use rustupolis::tuple::Tuple;
use rustupolis::tuplespace::TupleSpace;

#[macro_use]
extern crate log;

#[test]
fn test_put() {
    info!("testing basic tuplespace function put()");
    let mut t_space = TupleSpace::new();
    let content = E::S("test content".to_string());
    let lifetime = 387465;
    let tup = Tuple::new(vec![content], lifetime);

    t_space.put(tup);
}

#[test]
fn test_read() {
    info!{"testing basic tuplespace function read()"}
    let mut t_space = TupleSpace::new();
    let tup1 = Tuple::new(vec![E::D(3.14), E::I(123)], 86567);
    let tup2 = Tuple::new(vec![E::D(3.14), E::None], 12390);
    let tup3 = Tuple::new(vec![E::D(3.14), E::None], 12390);

    t_space.put(tup1);
    let tup4 = t_space.read(tup2);

    match tup4 {
        Some(x) => assert!(tup3.content == x.content),
        None => assert!(false),
    }
}

#[test]
fn test_take() {
    info!{"testing basic tuplespace function take()"}

    let mut t_space = TupleSpace::new();
    let tup1 = Tuple::new(vec![E::D(3.14), E::I(123)], 86567);
    let tup2 = Tuple::new(vec![E::D(3.14), E::None], 12390);
    let tup3 = Tuple::new(vec![E::D(3.14), E::None], 12390);

    t_space.put(tup1);
    assert_eq!(t_space.len(), 1);
    let tup4 = t_space.take(tup2);
    assert_eq!(t_space.len(), 0);

    match tup4 {
        Some(x) => assert!(tup3.content == x.content),
        None => assert!(false),
    }
}
