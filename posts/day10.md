Good day Rustaceans!

Day 10. Final day. We made it.

Ten days ago, on day 1, you wrote `let x = 5`.

Today you're building a word frequency counter. `HashMap`, iterators, custom sorting, string processing. A real little tool that takes a sentence in and returns `["the (3)", "fox (2)"]` out.

I love the learning arc of challenges.

You don't feel it day to day. You write `let mut`. You fight the borrow checker once. You learn `match`. You discover `Option`. 

Each day feels small. Then on day 10 you compose three functions into something that looks like Python's `Counter`, and you realize you can read and write some basic Rust.

This line will be a lot to unpack today:

```rust
*map.entry(word).or_insert(0) += 1;
```

The Rust version of `Counter`. No "if key in map" check, Rust has its helpers too. 

Ten days ago this looked like noise. Today, after 9 days of coding, you can come up with this and understand it.

Day 1: `let x = 5`
Day 5: ownership and the borrow checker
Day 7: `Option` and `Result`
Day 9: closures and iterator chains
Day 10: `HashMap` composition

Ten days of Rust, now onto bigger things.

Which brings me to the next round; I'm thinking we build a small project together over a couple of weeks. Something you can actually ship.

If you've been doing the exercises, drop a comment with what surprised you most. The comments are how I shape what comes next.

Like and follow if you want to be there for it. 🦀
