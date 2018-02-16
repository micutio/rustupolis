extern crate rustupolis;
extern crate rand;

use rustupolis::error::Result;
use rustupolis::tuple::{E, Tuple};
use rustupolis::tuplespace::{SimpleSpace, Space};

use rand::{Rng, Isaac64Rng, SeedableRng};

fn put_and_read(mut rng: Isaac64Rng, mut t_space: SimpleSpace) -> Result<()> {
    for _i in 0..5 {
        println!("pushing tuple");
        let int = rng.gen::<i32>();
        let dbl = rng.gen::<f64>();
        let tup = Tuple::new(&[
                E::S("tuple".to_string()),
                E::I(int),
                E::D(dbl),
                E::S("more content...".to_string()),
            ]);
        println!("{:?}", tup);
        t_space.out(tup)?;
    }

    for _i in 0..5 {
        println!("reading tuple");
        let tup = t_space.rd(Tuple::new(&[E::Any, E::Any, E::Any, E::Any]))?;
        println!("{:?}", tup);
    }
    
    Ok(())
}

fn main() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::Isaac64Rng::new_unseeded();
    rng.reseed(seed);

    println!("rustupolis - single threaded example");
    let t_space = SimpleSpace::new();

    put_and_read(rng, t_space).unwrap();

    println!();
    println!();
}
