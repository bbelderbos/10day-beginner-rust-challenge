Bob's 10-Day Rust 🦀 Beginner Challenge - Day 6

Happy Monday Rustaceans and welcome back!

Have you been coding Rust so far in this challenge? Comment below and fork + push (repo below).

Today two new concepts: Enums & Option.

Python uses `None` to mean "nothing here". Rust makes that explicit with `Option<T>`: a value is either `Some(thing)` or `None`.

Think of Option as a wrapper or a "box" that might be empty or might contain your data. You can't get to the data inside until you safely open the box.

This is convenient because in Python it can be easy to miss checking for a `None` value, leading to bugs. In Rust, the type system forces you to handle the possibility of "no value" explicitly.

Custom enums make invalid states unrepresentable.

In this exercise you define a `Direction` enum and use `match` to handle the variants of the Option. You also use it to return the opposite of a direction.

A nice intro to both constructs which are elegant and used a lot in Rust.

Note that the `#[derive(Debug, PartialEq)]` on enum is already given. This is similar to enriching Python objects with dunder methods: `Debug` is like `__repr__` (developer-facing representation), `PartialEq` is like `__eq__` (equality comparison). We'll go into this more deeply in future lessons.

Good luck and comment below how it went. Tomorrow Vectors & Iteration ... 
