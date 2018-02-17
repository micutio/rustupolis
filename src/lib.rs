//! # Rustupolis
//!
//! A tuple space implementation for Rust.
//!
//! TODO: Add more information
//!
//! This Library does ...

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate rand;

pub mod error;
pub mod store;
pub mod tuple;
