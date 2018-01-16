//! Module Tuple
//!
//! Tuple is the basis for everything that can be put into the tuple space.

/// Enum, which is used to represent the tuple elements.
#[derive(Clone)]
pub enum E {
    /// Integer data type.
    ///
    /// Implemented as 32-bit integer (i32).
    I(i32),
    /// Floating point data type.
    ///
    /// Implemented as double precision (f62).
    D(f64),
    /// String data type.
    ///
    /// Implemented as String.
    S(String),
    /// Tuple data type.
    ///
    /// Implemented as vector of tuple types (Vec<E>).
    T(Vec<E>),
    /// None data type.
    ///
    /// In context of this tuple, None stands for the wild card that is used
    /// for pattern matching when querying the tuple space for certain tuples.
    None,
}

impl E {
	/// Prints an element to the standard output
	pub fn print(&self) {
		match self {
			&E::I(ref i) => print!("Int: {}, ", i),
			&E::D(ref d) => print!("Double: {}, ", d),
			&E::S(ref s) => print!("String: {}, ", s),
			&E::T(ref v) => {
				print!("Tuple: [");
				for e in v {
					e.print();
				}
				print!("], ");
			}
			&E::None => println!("Wildcard"),
		}
	}
}

/// Allow tuples to be equal to identical tuples with wildcards.
impl PartialEq for E {
    fn eq(&self, other: &E) -> bool {
        match (self, other) {
            (&E::I(ref a), &E::I(ref b)) => a == b,
            (&E::D(ref a), &E::D(ref b)) => a == b,
            (&E::S(ref a), &E::S(ref b)) => a == b,
            (&E::T(ref a), &E::T(ref b)) => a == b,
            (&E::I(ref _a), &E::None) => true,
            (&E::D(ref _a), &E::None) => true,
            (&E::S(ref _a), &E::None) => true,
            (&E::T(ref _a), &E::None) => true,
            (&E::None, _) => true,
            _ => false,
        }
    }
}

/// Base tuple for the tuple space.
///
/// Properties:
/// - identifier
/// - lifetime
/// - generic number of fiels of generic types
#[derive(PartialEq, Clone)]
pub struct Tuple {
   pub content: Vec<E>,
   pub lifetime: u64,
}

impl Tuple {

    /// Create a tuple.
    pub fn new(ct: Vec<E>, lt: u64) -> Tuple {
        Tuple { content: ct, lifetime: lt }
    }

    // pub fn print(&self) {
    //     for elem in self.content {
    //         elem.print();
    //     }
    // }

}
