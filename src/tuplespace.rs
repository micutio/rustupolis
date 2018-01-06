//! Module TSpace
//!
//! TSpace is the actual tuple space

use tuple::Tuple;

pub struct TupleSpace {
    // Naive implementation for now: keep all tuples in a set.
    data : Vec<Tuple>,
}

impl TupleSpace {

    pub fn new() -> TupleSpace {
        TupleSpace { data: Vec::new(), }
    }

    pub fn put(&mut self, tup: Tuple) {
        trace!("[TupleSpace] put tuple into space");
        self.data.push(tup);
    }

    pub fn read(&self, tup: Tuple) -> Option<Tuple> {
        trace!("[TupleSpace] reading tuple from space");

        let mut index = self.data.len();
        for i in 0..self.data.len() {
            if tup.content == self.data[i].content {
                index = i;
                break;
            }
        }

        if index < self.data.len() {
            Some(tup)
        } else {
            None
        }
    }

    pub fn take(&mut self, tup: Tuple) -> Option<Tuple> {
        trace!("[TupleSpace] taking tuple from space");
        let mut index = self.data.len();
        for i in 0..self.data.len() {
            if tup.content == self.data[i].content {
                index = i;
                break;
            }
        }

        if index < self.data.len() {
            self.data.remove(index);
            Some(tup)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
