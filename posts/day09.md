Bob's 10-Day Rust 🦀 Beginner Challenge - Day 9

Good morning Rustaceans!

Getting into some more advanced and powerful territory here: closures and iterator chains.
This is where Rust starts to feel as expressive as Python.

In Python, you'd write a list comprehension.
In Rust, we use a "chain".
Once you know the pattern, it reads beautifully from top-to-bottom:
parse → filter → sort → format.

What you're learning today:

* `filter_map`:
This combines filtering and transforming into one pass, an important iterator.

* Closures:
Rust's version of lambdas (inline functions).

```rust
apply_bonus(&scores, |s| s + 5)
```

The `|s|` is your argument, and what follows is the logic.

* Higher-Order Functions:
You'll write a function that accepts `impl Fn(u32) -> u32`.
This tells Rust: "I don't care exactly what this is, as long as I can call it like a function".

You'll see the `?` operator again. We used it on `Result` in day 08; today, you'll see how it works with `Option`.
Same operator, same idea: "If this is a failure/nothing, bail out early".

Code is in the repo. We're almost at the finish line.
Have fun!

Tomorrow is final **Day 10 Capstone**:
A word frequency counter using `HashMap`. 🚀
