//! # Rustupolis
//!
//! A tuple space implementation for Rust.
//!
//! TODO: Add more information
//!
//! This library does ...

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

extern crate futures;
extern crate indextree_ng;

#[macro_use]
pub mod tuple;

pub mod error;
pub mod space;
pub mod store;
pub mod wildcard;
