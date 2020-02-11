#[macro_use]
extern crate rustupolis;

extern crate rand;

use rustupolis::error::Result;
use rustupolis::store::{SimpleStore, Store};
use rustupolis::tuple::E;

use rand::{Rng, SeedableRng};

fn put_and_read(rng: &mut rand::Isaac64Rng, t_store: &mut SimpleStore) -> Result<()> {
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
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::Isaac64Rng::new_unseeded();
    rng.reseed(seed);

    println!("rustupolis - single threaded example");
    let mut t_store = SimpleStore::new();

    put_and_read(&mut rng, &mut t_store).unwrap();

    println!();
    println!();
}
