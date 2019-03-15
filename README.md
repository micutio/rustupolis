# rustupolis - Tuple Space for Rust [![travis CI build status](https://travis-ci.org/Micutio/rustupolis.svg?branch=master)](https://travis-ci.org/Micutio/rustupolis)

An easy to use tuple space library written in Rust.

Also check out rustupolis' sister project [goTupolis](https://github.com/Micutio/goTupolis), a tuple space implementation in Go. Both projects are developed in tandem to learn about the Rust and Go languages, as well as the differences in implementing a similar library.

## Why Tuple Spaces

Tuple spaces are a very cool application (and enabler) of decentralized computing. Even though not widely used, they are nevertheless useful to facilitate asynchronous and distributed communcation and data exchange and a nice to have tool for any programming language that cares about these things.

Additionally tuple spaces are a great way to explore a programming language. Their underlying concept is rather straight-forward and easy to grasp, the implementation touches on many prominent concepts in programming:

- data structures and generics
- multi-threading and concurrency
- sockets and network communication

and various more.

## Goals

The goal of this project is to implement a comprehensive tuple space library that fulfills the following criteria:

1. General Use - the implementation should support the use of any kind of data the user wishes.
2. Distributed - the tuple space should be able to run locally and remotely on multiple nodes.
3. Parallelized - the **core** value, the tuple space should be accessible concurrently.

## Development

The tuple space is being implemented iteratively, where each iteration improves upon the previous by adding or refining features to achieve the next milestone.

### Milestones

- [x] local tuple space for storing tuples and retrieving them via pattern matching
- [x] local tuple space with multi-threaded and concurrent access
- [ ] remote tuple space server, accessible via network sockets
- [ ] distributed tuple space on multiple servers, accessible via network
- [ ] 'space of spaces', tuples can be tuple spaces themselves

## Current Version

alpha 0.0.2
