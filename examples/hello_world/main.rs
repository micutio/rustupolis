extern crate rustupolis;

use rustupolis::tuple::{E, Tuple};
use rustupolis::tuplespace::{SimpleSpace, Space};

fn main() {
    println!("rustupolis - hello world example");
    let mut t_space = SimpleSpace::new();
    println!("creating space and putting three new tuples into it");
    let tup1 = Tuple::new(&[E::S("Hello".to_string()), E::S("World!".to_string())]);
    let tup2 = Tuple::new(&[E::D(3.14), E::S("bar".to_string()), E::S("foo".to_string())]);
    let tup3 = Tuple::new(&[E::S("baz".to_string()), E::D(1.14), E::D(2.14), E::D(3.14)]);

    t_space.out(tup1).unwrap();
    t_space.out(tup2).unwrap();
    t_space.out(tup3).unwrap();

    println!("taking out a tuple that matches the pattern (Any, Any)");
    let tup4 = t_space.in_(Tuple::new(&[E::Any, E::Any]));

    println!("printing tuple contents:");
    match tup4.unwrap() {
        Some(x) => {
            println!("{:?}", x);
        }
        None => assert!(false),
    }
    println!();
    println!();
}
