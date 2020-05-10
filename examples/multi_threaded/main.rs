//! This example demonstrates insertion and reading of a number of random tuples into the tuple
//! space. It is done multi-threaded to show the thread-safety of the tuple space.

extern crate rand;
extern crate rand_isaac;

#[macro_use]
extern crate rustupolis;

#[macro_use]
use std::sync::{Arc, Mutex};
use std::thread;

use rand::{Rng, SeedableRng};
use rand_isaac::isaac64::Isaac64Rng;

use rustupolis::error::Result;
use rustupolis::store::{SimpleStore, Store};
use rustupolis::tuple::E;

fn put_and_read(
    rng: &mut rand_isaac::isaac64::Isaac64Rng,
    id: &str,
    t_store: std::sync::Arc<std::sync::Mutex<rustupolis::store::SimpleStore>>,
) -> Result<()> {
    let mut t_store = t_store.lock().unwrap();
    for _i in 0..5 {
        println!("{0} pushing tuple", id);
        let mut strg = "tuple from ".to_string();
        strg.push_str(&id);
        let int = rng.gen::<i32>();
        let dbl = rng.gen::<f64>();
        let tup = tuple![
            E::S(strg),
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
    let mut rng1 = Isaac64Rng::from_seed(seed);
    let mut rng2 = Isaac64Rng::from_seed(seed);

    // rng.reseed(seed);

    println!("rustupolis - multi threaded example");

    let t_store = Arc::new(Mutex::new(SimpleStore::new()));
    let ts1 = t_store.clone();
    let handle_a = thread::spawn(move || {
        put_and_read(&mut rng1, "a", ts1).unwrap();
    });

    let ts2 = t_store;
    let handle_b = thread::spawn(move || {
        put_and_read(&mut rng2, "b", ts2).unwrap();
    });

    let res_a = handle_a.join();
    let res_b = handle_b.join();

    println!("{:?}", res_a);
    println!("{:?}", res_b);
}
