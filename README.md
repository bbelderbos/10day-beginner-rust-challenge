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

## Want a guided experience?

Do this challenge on the platform at [rustplatform.com/intro](https://rustplatform.com/intro/) for built-in feedback, hints, and progress tracking.

## Days planning (Monday 27th of April 2026 - Friday 8th of May 2026)

| Day | Topic |
|-----|-------|
| 1 | Hello, Rustacean + Variables and Mutability |
| 2 | Primitive Types |
| 3 | Functions and Return Values |
| 4 | Control Flow |
| 5 | Strings and Slices |
| 6 | Vectors |
| 7 | Enums and Match |
| 8 | Result Handling |
| 9 | Ownership and Borrowing |
| 10 | Pattern Matching |

---

Have fun and follow me on [LinkedIn](https://www.linkedin.com/in/bbelderbos/) for updates and more Python / Rust / AI content. 🚀 
