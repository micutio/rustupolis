extern crate futures;

use futures::prelude::*;

use error::Error;
use store::Store;
use tuple::Tuple;

pub type Match = Future<Item = Tuple, Error = Error>;

pub struct Space<T: Store> {
    store: T,
}

impl<T> Space<T>
where
    T: Store,
{
    pub fn new(store: T) -> Space<T> {
        Space { store: store }
    }

    pub fn in_(&mut self, tup: Tuple) -> Box<Match> {
        panic!("todo");
    }

    pub fn rd(&mut self, tup: Tuple) -> Box<Match> {
        panic!("todo");
    }

    pub fn out(&mut self, tup: Tuple) -> Box<Future<Item = (), Error = Error>> {
        panic!("todo");
    }
}
