Bob's 10-Day Rust 🦀 Beginner Challenge - Day 1

Ok let's get started! I have two little functions today for you to complete.

First of all why Rust? Learning Rust unlocks writing performant code and will make you write stricter Python code.

As with any new language, we start with hello world, and although you'll solve it quickly (single line return), we already hit some new things as Pythonistas:

- `def` = `fn`.
- Yep, unlike Python, we do curlies in Rust (they add extra meaning for scoping though, as you'll learn later).
- The biggest shift is to start thinking in different string types. `"Hello"` is a `&str` literal (borrowed, read-only). our function returns `String` (owned), and how to convert between the two. You'll learn more about this important distinction as we go.
- You'll see unit tests from the get-go embedded in each exercise, you can validate your code with `cargo test`.
- Type hints are not optional in Rust, every function's input args and returns need to contain types.

One thing that might trip you up coming from Python: in Rust you can omit `return`; the last expression in a function is the return value. However, if you add `;` it's turned into a statement and nothing returns.

Next, we mutate a variable in a second function. Here we learn about a Rust design decision I really like: variables are immutable by default. If you want to mutate something you have to say so explicitly with `mut`. 

This seems strict, but immutability by default actually means the compiler catches accidental mutations, a common source of bugs. It's also an important mindset shift coming from Python.

You're also asked to do an operation multiple times. For that you can use the classic for loop, which is similar to Python: `for _ in range(5): ...` -> `for _ in 0..5 { ... }`

---

Let's code (ideally without AI to really learn!) - are you excited to meet the compiler? It'll be a game changer!

You can proceed in two ways:

1. On our Rust Platform: make an account there and go through the first two intro Bites. Zero setup, rated (gamified), done. 

2. Clone the dedicated GitHub repo where I will put each day's exercise into. I am leaving the solutions out to not spoiler anything.

Ok this concludes day 1. Tomorrow we'll look at an important building block in Rust: primitive types.

What's your goal with Rust? I'd love to hear.
