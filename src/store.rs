use tuple::Tuple;
use error::Result;

/// A Store is an associative memory which stores and retrieves tuples.
pub trait Store {
    /// Read a matching tuple and remove it atomically.
    fn inp(&mut self, tup: Tuple) -> Result<Option<Tuple>>;
    /// Read a matching tuple.
    fn rdp(&mut self, tup: Tuple) -> Result<Option<Tuple>>;
    /// Write a tuple.
    fn out(&mut self, tup: Tuple) -> Result<()>;
}

/// A simple, naive implementation of a Store backed by a Vec.
pub struct SimpleStore(Vec<Tuple>);

impl SimpleStore {
    pub fn new() -> SimpleStore {
        SimpleStore(Vec::new())
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
        self.0.push(tup);
        Ok(())
    }

    fn rdp(&mut self, tup: Tuple) -> Result<Option<Tuple>> {
        for i in 0..self.0.len() {
            if tup == self.0[i] {
                return Ok(Some(self.0[i].clone()));
            }
        }
        Ok(None)
    }

    fn inp(&mut self, tup: Tuple) -> Result<Option<Tuple>> {
        for i in 0..self.0.len() {
            if tup == self.0[i] {
                return Ok(Some(self.0.remove(i)));
            }
        }
        Ok(None)
    }
}
