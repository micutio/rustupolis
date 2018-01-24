//! Module TSpace
//!
//! TSpace is the actual tuple space

use tuple::Tuple;

use rand::{Rng, thread_rng};

pub struct TupleSpace {
    // Naive implementation for now: keep all tuples in a set.
    data: Vec<Tuple>,
}

impl TupleSpace {
    pub fn new() -> TupleSpace {
        TupleSpace { data: Vec::new() }
    }

    pub fn put(&mut self, tup: Tuple) {
        trace!("[TupleSpace] put tuple into space");
        self.data.push(tup);
    }

    pub fn read(&self, tup: Tuple) -> Option<Tuple> {
        trace!("[TupleSpace] reading tuple from space");

        let mut index = self.data.len();
        let mut index_vec: Vec<usize> = Vec::new();
        for i in 0..self.data.len() {
            if tup.content == self.data[i].content {
                index = i;
                index_vec.push(i);
            }
        }

        if index < self.data.len() {
            let mut rng = thread_rng();
            let i: usize;
            i = *rng.choose_mut(index_vec.as_mut_slice()).unwrap();
            let return_tup = self.data[i].clone();
            Some(return_tup)
        } else {
            None
        }
    }

    pub fn take(&mut self, tup: Tuple) -> Option<Tuple> {
        trace!("[TupleSpace] taking tuple from space");

        let mut index = self.data.len();
        let mut index_vec: Vec<usize> = Vec::new();
        for i in 0..self.data.len() {
            if tup.content == self.data[i].content {
                index = i;
                index_vec.push(i);
            }
        }

        if index < self.data.len() {
            let mut rng = thread_rng();
            let i: usize;
            i = *rng.choose_mut(index_vec.as_mut_slice()).unwrap();
            let return_tup = self.data.remove(i);
            Some(return_tup)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
