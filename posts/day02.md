Bob's 10-Day Rust 🦀 Beginner Challenge - Day 2

Good morning Rustaceans!

Stricter language, better guarantees, and more performant code. Let's learn some more Rust today.
How did you do yesterday, comment below (link to post there as well if you want to catch up).

A big difference between Python and Rust is that the latter is statically typed, that is types are checked at compile time, not runtime.

Also you can be more precise how much memory you want to use, and how you want to use it. This is a big shift in mindset coming from Python, but it's also what allows Rust to be so performant and safe.

Rust can infer your types, for example `let pi = 3.14;`; here it infers that `pi` is a `f64` (floating point number). But you can also be explicit, for example `let pi: f32 = 3.14;`, this way you are telling the compiler that you want to use a 32-bit floating point number, which uses less memory but is also less precise.

Another important thing to learn soon is how to format strings. In Python we love f-strings. In Rust, `format!` uses `{}` placeholders, closest to Python's `str.format()`. For more complex types, `{:?}` gives you a debug representation, which is useful for more complex types like tuples, vecs, etc.

Enough talking, let's write some code. Implement `describe_types`, creating variables for each primitive type and return a formatted string. Tests verify the output contains `"int: 42"`, `"float: 3.14"`, and `"tuple: (7, "Rust")"`.

Yep, still pretty basic, but we have to run through some of these concepts before we can do the more fun stuff. Stick with it, because I plan on building bigger things here after this beginner challenge.
(If you already want to add some things to the challenge wish list, open an issue in the repo below.)

As Dennis Ritchie said:
> "The only way to learn a new programming language is by writing programs in it."

So let's write a bit of Rust code every day (ideally not using AI or minimally).

Are you excited at the thought of the compiler being your pair programmer, catching things at compile time? It's one of the things I love the most about Rust!

---

Two ways to proceed from here:

1. On our Rust Platform: make an account and go through intro exercise # 3. Zero setup, rated (gamified), done. 

2. Clone the dedicated GitHub repo where I will put each day's exercise into. I am leaving the solutions out to not spoiler anything.

Ok this concludes day 2.
Tomorrow we'll look at functions and two constructs that might surprise you as Python developer: `if` and `match` as expressions that return values. And a real win: the compiler enforcing exhaustive matching, no unhandled cases allowed.

---

Setting out to learn Rust? Tell me in the comments below, I'd be curious to hear what you'd use it for ...
