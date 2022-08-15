//! This example demonstrates insertion and reading of a number of random tuples into the tuple
//! space.

extern crate rand;
extern crate rand_isaac;

#[macro_use]
extern crate rustupolis;

use rand::{Rng, SeedableRng};
use rand_isaac::isaac64::Isaac64Rng;

use rustupolis::error::Result;
use rustupolis::store::{SimpleStore, Store};
use rustupolis::tuple::E;

fn put_and_read(rng: &mut Isaac64Rng, t_store: &mut SimpleStore) -> Result<()> {
    for _i in 0..5 {
        println!("pushing tuple");
        let int = rng.gen::<i32>();
        let dbl = rng.gen::<f64>();
        let tup = tuple![
            E::S("tuple".to_string()),
            E::I(int),
            E::D(dbl),
            E::S("more content...".to_string()),
        ];
        println!("{:?}", tup);
        t_store.out(tup)?;
    }

    for _i in 0..5 {
        println!("reading tuple");
        let tup = t_store.rdp(&tuple![E::Any, E::Any, E::Any, E::Any])?;
        println!("{:?}", tup);
    }

    Ok(())
}

fn main() {
    let seed: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6,
        7, 8,
    ];
    let mut rng = Isaac64Rng::from_seed(seed);

    println!("rustupolis - single threaded example");
    let mut t_store = SimpleStore::new();

    put_and_read(&mut rng, &mut t_store).unwrap();

    println!();
    println!();
}
