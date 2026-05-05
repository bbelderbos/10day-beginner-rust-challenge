Bob's 10-Day Rust 🦀 Beginner Challenge - Day 7

Welcome back Rustaceans!

How did Day 6 go? Enums and Option are short concepts but they make Rust code elegant.

I hope you're starting to see the language's beauty and robustness
(e.g. as you saw yesterday, with `match` you have to catch all cases!)

Let's keep going!
Today we look at vectors and iteration. `Vec<T>` is Rust's `list[T]` (where `T` stands for "type").
It's a growable array, and it's the most common collection type.
You can push, pop, index, and iterate over a Vec.

--
💡 Unlike a fixed-size array that sits on the Stack, a Vec stores its data on the Heap.
This is what allows it to grow and shrink at runtime.

Think of it like this + performance tradeoffs:

- Stack: your desk (organized, fast, but small); very fast because the CPU knows exactly where everything is. But rigid because it must be of fixed size.

- Heap: a warehouse (huge, but you need a pointer/address to find your stuff); slower because CPU has to search for a big enough "spot" to put your data in. It's flexible, because you can grow your collection, but it might need reallocation if it outgrows its current space = affects performance.
--

Iteration is a powerful concept in Rust, and it comes with a rich set of methods that let you transform and filter data.
Especially nice if you like a more functional style of programming. 😍

Today I have 3 small functions to get comfortable with `.iter()` and some other iteration methods:

1) `score_summary` returns `(min, max, avg)`, but wrapped in `Option`. Empty slice in? `None` out. Why? `.iter().min()` already returns `Option<&i32>` because the slice might be empty. Rust pushes you to handle that case in the type system. You can use the `?` operator here. We'll dig deeper into this on Day 8; for now just know it makes your Rust error handling code more concise.

2) `passing_scores` filters by threshold. A simple for loop with `.push()` is perfectly fine. On Day 9 you'll see the iterator chain version.

3) `top_n_scores` returns the `n` highest scores. You'll sort the vector descending with `.sort_by()`, then turn it into an iterator to use `.take(n)`. This mimics Python's `sorted(scores, reverse=True)[:n]` we know and love.

A few possible quirks for us Pythonistas:

- Functions take `&[i32]` (a slice), not `Vec<i32>`. Slices are borrowed views (remember Day 4?), and because you're not passing ownership, you can use them with both arrays and vectors. They're more flexible and efficient for read-only access.

- `sum as f64 / scores.len() as f64`. Rust won't "auto-magically" do math on different types. Since `sum` is an `i32` and the length of `scores` is a `usize`, you need to cast them to `f64` to get a floating-point average.

- `.iter().min()` returns `Option<&i32>` (a reference) because it's just looking at ("borrowing") the data, not taking it. To get an `Option<i32>` (the actual value), we can use `.copied()`. It's a clean way to tell Rust: "I know this is an integer; just give me a copy of the value instead of a pointer to it."

And here we see recurring patterns; it's Day 6's `Option` again. The concepts you learn here will start to compound.

--
Starter code in the repo below (solution code on branch later this week ...)

Comment how you go, I am excited to see more Pythonistas like myself jump into Rust. 💪

Stay tuned. Tomorrow another big one: error handling with `Result` and the `?` operator. 🚀
