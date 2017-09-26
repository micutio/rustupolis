//! Module TSpace
//! 
//! TSpace is the actual tuple space

use tuple::Tuple;

struct TupleSpace {

}

impl TupleSpace {

    pub fn put(&self, tup: &Tuple) {
        println!("[TupleSpace] put tuple into space");
    }

    pub fn take(&self, tup: &Tuple) -> Tuple {
        println!("[TupleSpace] taking tuple from space");
        let result = Tuple{ };
        result
    }

    pub fn read(&self, tup: Tuple) -> Tuple {
        println!("[TupleSpace] reading tuple from space");
        let result = Tuple{ };
        result
    }

}
