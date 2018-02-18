use std::collections::BTreeSet;

use tuple::Tuple;
use error::Result;

/// A Store is an associative memory which stores and retrieves tuples.
/// Implementors should only store _defined_ tuples.
pub trait Store {
    /// Read a matching tuple and remove it atomically.
    fn inp(&mut self, tup: &Tuple) -> Result<Option<Tuple>>;
    /// Read a matching tuple.
    fn rdp(&mut self, tup: &Tuple) -> Result<Option<Tuple>>;
    /// Write a tuple.
    fn out(&mut self, tup: Tuple) -> Result<()>;
}

/// A simple, naive in-memory implementation of a Store.
pub struct SimpleStore(BTreeSet<Tuple>);

impl SimpleStore {
    pub fn new() -> SimpleStore {
        SimpleStore(BTreeSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Store for SimpleStore {
    fn out(&mut self, tup: Tuple) -> Result<()> {
        if !tup.is_defined() {
            bail!("cannot write an undefined tuple");
        }
        self.0.insert(tup);
        Ok(())
    }

    fn rdp(&mut self, tup: &Tuple) -> Result<Option<Tuple>> {
        if let Some(m) = self.0.range(tup.range()).next() {
            Ok(Some(m.clone()))
        } else {
            Ok(None)
        }
    }

    fn inp(&mut self, tup: &Tuple) -> Result<Option<Tuple>> {
        let m = match self.0.range(tup.range()).next() {
            Some(m) => m.clone(),
            None => return Ok(None),
        };
        Ok(self.0.take(&m))
    }
}
