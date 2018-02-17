extern crate futures;

use futures::prelude::*;

use error::Error;
use store::Store;
use tuple::Tuple;

pub struct Match {
    want: Tuple,
    have: Option<Tuple>,
}

impl Future for Match {
    type Item = Tuple;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Tuple>, Error> {
        match self.have {
            Some(_) => Ok(Async::Ready(self.have.take().unwrap())),
            None => Ok(Async::NotReady),
        }
    }
}

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

    pub fn in_(&mut self, tup: Tuple) -> Match {
        panic!("todo");
    }

    pub fn rd(&mut self, tup: Tuple) -> Match {
        panic!("todo");
    }

    pub fn out(&mut self, tup: Tuple) -> Box<Future<Item = (), Error = Error>> {
        panic!("todo");
    }
}
