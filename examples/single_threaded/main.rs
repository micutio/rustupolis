extern crate rustupolis;
extern crate rand;

use rustupolis::tuple::E;
use rustupolis::tuple::Tuple;
use rustupolis::tuplespace::TupleSpace;

use rand::Rng;

fn put_and_read(mut t_space : TupleSpace) {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        println!("pushing tuple");
        let int = rng.gen::<i32>();
        let dbl = rng.gen::<f64>();
        let tup = Tuple::new(vec!(E::S("tuple".to_string()), E::I(int), E::D(dbl), E::S("more content...".to_string())), 99999);
        // tup.print();
        for elem in tup.content {
            elem.print();
        }
        t_space.put(tup)
    }

    for _i in 0..10 {
        println!("reading tuple");
        let tup = t_space.take(Tuple::new(vec!(E::None, E::None, E::None, E::None), 0));
        // tup.print();
        match tup {
            Some(x) => {
                for elem in x.content {
                    elem.print();
                }
            },
            None => assert!(false)
        }
    }

}

fn main() {

    println!("rustupolis - hello world example");
    let t_space = TupleSpace::new();

    put_and_read(t_space);

    println!();
    println!();

}
