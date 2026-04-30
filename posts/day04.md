Bob's 10-Day Rust 🦀 Beginner Challenge - Day 4

Good morning Rustaceans!

A new day, let's write some more Rust.

How did Day 3 go? Any takeaways from `if` / `match` as expressions?

Today we hit one of the concepts that defines Rust: ownership and borrowing. We'll meet it through strings, where the contrast with Python is most visible.

In Rust, strings come in two main flavors:

- `&str`: a borrowed, read-only view into existing string data.
- `String`: heap-allocated, owned data you can build and hand around.

The function signature tells you which one you're dealing with, and that's a key part of Rust's memory model.

Coming from Python, this is a real shift. `s[0:5]` in Python allocates a fresh string (slicing copies). `&str` does not. It's literally a pointer + length pointing into the existing buffer, zero copy.

A quick word on ownership before the functions: in Rust, every value has exactly one owner, and when that owner goes out of scope the value is dropped. Borrowing (`&`) lets other code look at the value without taking ownership. The compiler tracks all of this at compile time, no garbage collector needed.

So onto the functions. `first_word(s: &str) -> &str` returns a slice borrowed from the input. No new allocation. The returned reference is only valid as long as `s` is alive (enforced by the compiler).

Secondly `shout(s: &str) -> String` takes a borrowed view, then builds new owned data. The caller gets a `String` back and takes ownership of it.

Why bother, when Python is already memory-safe? Two reasons:

- Performance: no GC pauses, no runtime overhead. The compiler knows exactly when each value is dropped, and there's no bookkeeping at runtime.
- Mutability discipline: Python lets you mutate shared state freely, and that's where the subtle bugs hide: a global counter incremented from multiple threads, a function quietly mutating a list its caller still holds, a list mutated while another part of the code is iterating over it. Rust's borrowing rules say you can have either one mutable reference or many read-only references, never both at the same time. The compiler enforces it. Whole classes of aliasing and data-race bugs simply don't compile.

Ownership is the foundation everything else in Rust builds on: lifetimes, `Box`, `Rc`, threads. Today's exercise is a start.

Your turn: implement `first_word` and `shout` - repo below.

Let me know how you go, share your learnings in the comments, and stay tuned for tomorrow, where we'll dive into structs & methods. 😍 📈 
