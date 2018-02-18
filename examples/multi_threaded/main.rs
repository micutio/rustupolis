extern crate rand;
extern crate rustupolis;

use std::thread;
use std::sync::{Arc, Mutex};

use rand::{Rng, SeedableRng};

use rustupolis::error::Result;
use rustupolis::tuple::{Tuple, E};
use rustupolis::store::{SimpleStore, Store};

fn put_and_read(
    rng: &mut rand::Isaac64Rng,
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
        let tup = Tuple::new(&[
            E::S(strg),
            E::I(int),
            E::D(dbl),
            E::S("more content...".to_string()),
        ]);
        println!("{:?}", tup);
        &mut t_store.out(tup)?;
    }

    for _i in 0..5 {
        println!("reading tuple");
        let tup = t_store.rdp(&Tuple::new(&[E::Any, E::Any, E::Any, E::Any]))?;
        println!("{:?}", tup);
    }

    Ok(())
}

fn main() {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::Isaac64Rng::new_unseeded();
    rng.reseed(seed);

    println!("rustupolis - multi threaded example");

    let t_store = Arc::new(Mutex::new(SimpleStore::new()));
    let ts1 = t_store.clone();
    let handle_a = thread::spawn(move || {
        put_and_read(&mut rng, "a", ts1).unwrap();
    });

    let ts2 = t_store.clone();
    let handle_b = thread::spawn(move || {
        put_and_read(&mut rng, "b", ts2).unwrap();
    });

    let res_a = handle_a.join();
    let res_b = handle_b.join();

    println!("{:?}", res_a);
    println!("{:?}", res_b);
}
