Bob's 10-Day Rust 🦀 Beginner Challenge - Day 5

Good morning Rustaceans!

Halfway there, let's keep building. How did Day 4 go? Ownership and borrowing are critical to understanding Rust, and it takes a while (it did for me!). The good news: it will click, and then it will make a lot of sense.

I will link a learning path on our platform to do some extra exercises if you have time.

Meta note: somebody asked for the solutions, and I completely forgot! I just pushed a solutions branch with my code including explanatory comments. I also added a CONTRIBUTING.md with instructions for how to best work and submit your own solutions.

With that out of the way, let's learn some new Rust concepts...

Today we look at `struct` + `impl`, in the context of temperature (day 2).

In Python you might reach for `@dataclass` or a (typed) named tuple. Rust splits the same idea in two: `struct` holds the data, `impl` the behavior. The split seems a bit redundant at first, but Rust wants to make data vs behavior explicit, and I think that's a good thing. An extension is traits (think interfaces or ABCs / protocols), something for a future project.

In today's mini challenge, you'll define a `Temperature` struct, and then implement some methods on it. The syntax is a bit different from Python, but the concepts are the same: you have a type that holds data, and you can define methods on it.

More concepts map to Python: when I read `Temperature::new(celsius)`, I think of a `@classmethod` constructor. Struct fields are private to outside callers by default (we did not open them up with `pub`), so a constructor like this gives us a clean public API. I like this enforced encapsulation, safer by default.

`&self` borrows in read-only mode by default. If we want to mutate, we need `&mut self`. We don't use it here yet, but from building a JSON parser (our cohort), I can tell you how nice it is to see at the signature level where data gets read vs mutated.

Notice the progression: Day 2 you wrote `celsius * 9.0 / 5.0 + 32.0` as a raw expression. Day 5 it lives on a type. Primitives → expressions → types. A nice arc that gets us ready for building more complex things.

Code in the repo below. What do you like or dislike about `struct` + `impl`, and how does it map back to Python for you? Comment below ...

On Monday, day 6, we look at two other important concepts in Rust: enums and `Option<T>`. Hope you enjoy this challenge and it gets you to write some Rust. 🚀

These are still 'mini challenges', but for the next series I am thinking of building a small project together = more fun. Stay tuned and like + follow to help grow this initiative 🙏
