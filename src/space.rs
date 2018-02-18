use std::collections::BTreeMap;

extern crate futures;
use futures::prelude::{Async, Future, Stream};
use futures::sync::mpsc::{channel, Receiver, Sender};

use error::Error;
use store::Store;
use tuple::Tuple;

pub enum Match {
    Ready(Option<Tuple>),
    Pending(Receiver<Tuple>),
    Fail(Option<Error>),
}

impl Future for Match {
    type Item = Tuple;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Tuple>, Error> {
        match self {
            &mut Match::Ready(ref mut tup) => Ok(Async::Ready(tup.take().unwrap())),
            &mut Match::Pending(ref mut rx) => match rx.poll() {
                Ok(Async::Ready(ref mut tup)) => Ok(Async::Ready(tup.take().unwrap())),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(_) => bail!("unexpected channel error!"),
            },
            &mut Match::Fail(ref mut err) => Err(err.take().unwrap()),
        }
    }
}

pub struct Space<T: Store> {
    store: T,
    pending: PendingStore,
}

impl<T> Space<T>
where
    T: Store,
{
    pub fn new(store: T) -> Space<T> {
        Space {
            store: store,
            pending: PendingStore::new(),
        }
    }

    pub fn in_(&mut self, tup: Tuple) -> Match {
        match self.store.inp(&tup) {
            Ok(Some(have)) => Match::Ready(Some(have)),
            Ok(None) => {
                let (tx, rx) = channel(0);
                self.pending.insert(tup.clone(), tx);
                Match::Pending(rx)
            }
            Err(e) => Match::Fail(Some(e)),
        }
    }

    pub fn rd(&mut self, tup: Tuple) -> Match {
        match self.store.rdp(&tup) {
            Ok(Some(have)) => Match::Ready(Some(have)),
            Ok(None) => {
                let (tx, rx) = channel(0);
                self.pending.insert(tup.clone(), tx);
                Match::Pending(rx)
            }
            Err(e) => Match::Fail(Some(e)),
        }
    }

    pub fn out(&mut self, tup: Tuple) -> Box<Future<Item = (), Error = Error>> {
        panic!("todo");
    }
}

type PendingStore = BTreeMap<Tuple, Sender<Tuple>>; // FIXME
