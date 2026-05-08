Bob's 10-Day Rust 🦀 Beginner Challenge - Day 8

Yesterday was busy, but I am back for day 8.
Today: error handling.
This one is a real mindset shift coming from Python.

In Python you wrap code that can raise an exception in `try/except`.
The caller needs to catch the exception, but if they forget, the error will bubble up which could cause the program to crash if it goes unhandled. 

Rust takes a different approach. Errors are values.
`Result<T, E>` is either `Ok(value)` or `Err(reason)`.
The function signature's return type tells you it can fail.
The compiler forces you to handle both cases before your code even runs. 📈

Two functions for you today where you will implement this pattern.
I will defer to the repo for the hints + signatures, but the gist is:
- Return Ok() when things go right, Err() when they don't.
- You'll also learn about `?` for propagating errors up the call stack without the extra `match` boilerplate.

Using a plain `String` for the error type is simplified.
In real projects you'd reach for a typed error enum so callers can pattern-match on the failure kind.
But that introduces more new concepts, today is about learn about `Result` + `?`, to be continued...

Let me know how you go and if you also find Rust's error handling enlightening?
I think it's a game-changer for writing more robust code.

Back tomorrow with closures and iterators. 🚀
