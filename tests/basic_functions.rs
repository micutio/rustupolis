extern crate rustupolis;

use rustupolis::tuple::Tuple;
use rustupolis::tuplespace::TupleSpace;

#[macro_use]
extern crate log;

#[test]
fn test_put() {
    info!("testing basic tuplespace functions");
    let mut t_space = TupleSpace::new();
    let tup = Tuple::new("tuple1");

    t_space.put(tup);
}
