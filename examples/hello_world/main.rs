extern crate rustupolis;

use rustupolis::tuple::E;
use rustupolis::tuple::Tuple;
use rustupolis::tuplespace::TupleSpace;

fn main() {

    println!("rustupolis - hello world example");
    let mut t_space = TupleSpace::new();
    println!("creating space and putting three new tuples into it");
    let tup1 = Tuple::new(vec!(E::S("Hello".to_string()), E::S("World!".to_string())), 86567);
    let tup2 = Tuple::new(vec!(E::D(3.14), E::S("bar".to_string()), E::S("foo".to_string())), 12390);
    let tup3 = Tuple::new(vec!(E::S("baz".to_string()), E::D(1.14), E::D(2.14), E::D(3.14)), 12390);

    t_space.put(tup1);
    t_space.put(tup2);
    t_space.put(tup3);

    println!("taking out a tuple that matches the pattern (None, None)");
    let tup4 = t_space.take(Tuple::new(vec!(E::None, E::None), 0));

    println!("printing tuple contents:");
    match tup4 {
        Some(x) => {
            for elem in x.content {
                elem.print();
            }
        },
        None => assert!(false)
    }
    println!();
    println!();

}
