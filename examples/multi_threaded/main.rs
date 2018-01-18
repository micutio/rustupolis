extern crate rustupolis;
extern crate rand;

use rustupolis::tuple::E;
use rustupolis::tuple::Tuple;
use rustupolis::tuplespace::TupleSpace;

use rand::{Rng, Isaac64Rng, SeedableRng};

use std::thread;

fn put_and_read(mut rng: Isaac64Rng, id: String, t_space: &TupleSpace) {

    for _i in 0..5 {
        println!("pushing tuple");
        let mut strg = "tuple from ".to_string();
        strg.push_str(&id);
        let int = rng.gen::<i32>();
        let dbl = rng.gen::<f64>();
        let tup = Tuple::new(
            vec![
                E::S(strg),
                E::I(int),
                E::D(dbl),
                E::S("more content...".to_string()),
            ],
            99999,
        );
        // tup.print();
        println!("{:?}", tup);
        &mut t_space.put(tup);
    }

    for _i in 0..5 {
        println!("reading tuple");
        let tup = t_space.read(Tuple::new(vec![E::None, E::None, E::None, E::None], 0));
        println!("{:?}", tup);
    }

}

fn main() {

    // let mut rng = match OsRng::new() {
    //     Ok(g) => g,
    //     Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    // };

    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::Isaac64Rng::new_unseeded();
    rng.reseed(seed);

    println!("rustupolis - hello world example");
    let mut t_space = TupleSpace::new();

    let handle_a = thread::spawn(move || put_and_read(rng, "a".to_string(), &t_space));

    // let handle_b = thread::spawn(|| {
    //     put_and_read(rng, "b".to_string(), &t_space)
    // });

    handle_a.join();
    // handle_b.join();

    println!();
    println!();

}
