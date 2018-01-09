extern crate rustupolis;

use rustupolis::tuplespace::TupleSpace;

#[test]
fn test_start() {
    println!("testing startup");
    let t_space = TupleSpace::new();
    println!("tuple space current size: {}", t_space.len());
    println!("startup done");
}
