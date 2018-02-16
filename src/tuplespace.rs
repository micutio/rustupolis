//! Module TSpace
//!
//! TSpace is the actual tuple space

use std::marker::Sized;

use rand::{Rng, Isaac64Rng};

use tuple::Tuple;
use error::Result;

pub type EvalFn = fn(sp: &mut Space) -> Result<()>;

/// A Space is an associative memory which stores and retrieves tuples.
pub trait Space {
    /// Atomically read and remove -- consume -- a matching tuple.
    fn in_(&mut self, tup: Tuple) -> Result<Option<Tuple>>;
    /// Non-destructively read a matching tuple.
    fn rd(&mut self, tup: Tuple) -> Result<Option<Tuple>>;
    /// Produce a tuple.
    fn out(&mut self, tup: Tuple) -> Result<()>;
    /// Evaluate a function that may perform the above operations.
    fn eval(&mut self, f: EvalFn) -> Result<()> where Self: Sized {
        f(self)
    }
}

/// A simple, naive implementation of a Space that just stores tuples in a Vec.
pub struct SimpleSpace {
    data: Vec<Tuple>,
    rng: Box<Isaac64Rng>,
}

impl SimpleSpace {
    pub fn new() -> SimpleSpace {
        SimpleSpace { data: Vec::new(), rng: Box::new(Isaac64Rng::new_unseeded()) }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Space for SimpleSpace {
    fn out(&mut self, tup: Tuple) -> Result<()> {
        trace!("[SimpleSpace] write tuple into space");
        if !tup.is_defined() {
            bail!("cannot write an undefined tuple");
        }
        self.data.push(tup);
        Ok(())
    }

    fn rd(&mut self, tup: Tuple) -> Result<Option<Tuple>> {
        trace!("[TupleSpace] reading tuple from space");

        let mut index = self.data.len();
        let mut index_vec: Vec<usize> = Vec::new();
        for i in 0..self.data.len() {
            if tup == self.data[i] {
                index = i;
                index_vec.push(i);
            }
        }

        if index < self.data.len() {
            let i: usize;
            i = *self.rng.choose_mut(index_vec.as_mut_slice()).unwrap();
            let return_tup = self.data[i].clone();
            Ok(Some(return_tup))
        } else {
            Ok(None)
        }
    }

    fn in_(&mut self, tup: Tuple) -> Result<Option<Tuple>> {
        trace!("[TupleSpace] taking tuple from space");

        let mut index = self.data.len();
        let mut index_vec: Vec<usize> = Vec::new();
        for i in 0..self.data.len() {
            if tup == self.data[i] {
                index = i;
                index_vec.push(i);
            }
        }

        if index < self.data.len() {
            let i: usize;
            i = *self.rng.choose_mut(index_vec.as_mut_slice()).unwrap();
            let return_tup = self.data.remove(i);
            Ok(Some(return_tup))
        } else {
            Ok(None)
        }
    }
}
