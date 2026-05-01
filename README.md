# 10-Day Beginner Rust Challenge

A hands-on Rust challenge for absolute beginners. One exercise file per weekday — clone once, pull each morning to get the next day's exercises.

## Install Rust

If you don't have Rust installed yet, run this in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts (the defaults are fine). Then restart your terminal, or run:

```bash
source $HOME/.cargo/env
```

Verify it worked:

```bash
rustc --version
cargo --version
```

You should see version numbers for both. That's it — no other tools needed to start.

## How it works

Each weekday adds a new file (`src/day01.rs`, `src/day02.rs`, …). Open today's file, read the comments, and fill in the functions until the tests pass.

```bash
# Clone once
git clone git@github.com:bbelderbos/10day-beginner-rust-challenge.git
cd 100day-beginner-rust-challenge

# Each morning, pull the new day's file
git pull

# Run the tests for today's exercises
cargo test day01

# Run all tests across all days
cargo test
```

Your solutions live in `src/dayXX.rs`. Because each day is its own file, pulling new days should not cause merge conflicts with your existing solutions.

`cargo test` is your workflow — the exercises are exercised by the tests, not by `main.rs`. If you run `cargo build` or `cargo run` you'll see `dead_code` warnings on the day functions; that's expected and you can ignore them.

## Stuck? Peek at the solutions

The solutions live on a separate `solutions` branch so they don't spoil the main workflow. When you want to see a reference implementation:

```bash
git checkout solutions
cargo test solution      # run only the solution tests (all should pass)
git checkout main        # back to your work
```

On the `solutions` branch you'll find `src/dayXX_solution.rs` next to the templates — read, compare, then come back and finish your own.

## Submitting your work

See [CONTRIBUTING.md](CONTRIBUTING.md) for the fork → branch → PR flow.

## Days planning (Monday 27th of April 2026 - Friday 8th of May 2026)

| Day | Topic |
|-----|-------|
| 1 | Hello, Rustacean + Variables and Mutability |
| 2 | Primitive Types |
| 3 | Control Flow — `if` and `match` as expressions |
| 4 | Ownership & Borrowing — `&str` vs `String` |
| 5 | Structs & Methods |
| 6 | Enums & `Option` |
| 7 | Vectors & Iteration |
| 8 | Error Handling with `Result` and `?` |
| 9 | Closures & Iterators |
| 10 | Mini Project — Word Frequency Counter (`HashMap`) |

---

Have fun and follow me on [LinkedIn](https://www.linkedin.com/in/bbelderbos/) for updates and more Python / Rust / AI content. 🚀 
