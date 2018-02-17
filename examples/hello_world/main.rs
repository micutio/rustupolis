extern crate rustupolis;

use rustupolis::tuple::{Tuple, E};
use rustupolis::store::{SimpleStore, Store};

fn main() {
    println!("rustupolis - hello world example");
    let mut t_store = SimpleStore::new();
    let tup1 = Tuple::new(&[E::S("Hello".to_string()), E::S("World!".to_string())]);
    let tup2 = Tuple::new(&[E::D(3.14), E::S("bar".to_string()), E::S("foo".to_string())]);
    let tup3 = Tuple::new(&[E::S("baz".to_string()), E::D(1.14), E::D(2.14), E::D(3.14)]);

    t_store.out(tup1).unwrap();
    t_store.out(tup2).unwrap();
    t_store.out(tup3).unwrap();

    print!("inp(Any, Any) -> ");
    let tup4 = t_store.inp(Tuple::new(&[E::Any, E::Any]));

    match tup4.unwrap() {
        Some(x) => {
            println!("{:?}", x);
        }
        None => assert!(false),
    }
}
