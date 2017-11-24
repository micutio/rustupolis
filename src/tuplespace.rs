//! Module TSpace
//!
//! TSpace is the actual tuple space

use tuple::Tuple;
use std::collections::HashSet;

struct TupleSpace {
    // Naive implementation for now: keep all tuples in a set.
    data : HashSet<Tuple>,
}

impl TupleSpace {

    pub fn new() -> TupleSpace {
        TupleSpace { data: HashSet::new(), }
    }

    pub fn put(&mut self, tup: Tuple) {
        trace!("[TupleSpace] put tuple into space");
        self.data.insert(tup);
    }

    pub fn take(&self, id: String) -> Option<Tuple> {
        trace!("[TupleSpace] taking tuple from space");
        for t in self.data.iter() {
            if t.id == id {
                let result_tuple = Tuple::new(id);
                return Some(result_tuple);
            }
        }
        None
    }

    pub fn read(&self, tup: Tuple) -> Option<Tuple> {
        trace!("[TupleSpace] reading tuple from space");
        unimplemented!();
    }

}
