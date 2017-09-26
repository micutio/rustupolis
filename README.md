# rustupolis - Tuple Space for Rust

An easy to use tuple space library written entirely in Rust.

Goals
-----

The goal of this project is to implement a comprehensive tuple space library
that fulfills the following criteria:

1. General Use - the implementation should support the use of any kind of data the user wishes.

2. Distributed - the tuple space should be able to run locally and remotely

3. Parallelized - the most __core__ value, the tuple space should be accessible concurrently

Development
-----------

The tuple space is being implemented iteratively, where each iteration improves upon the previous by adding or refining features to achieve the next milestone. The planned milestones are:

1. Local tuple space for storing tupels and retrieving them via pattern matching

2. Local tuple space with multi-threaded and concurrent access

3. Remote tuple space server, accessible via network sockets

4. Distributed tuple space on multiple servers, accessible via network

5. 'Space of spaces', tuples can be tuple spaces themselves

Current Version
---------------

none
