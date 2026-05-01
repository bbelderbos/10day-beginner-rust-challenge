# Submitting your work

Create a fork and each day, push your solution on a branch there. If you want, open a pull request for review (I won't merge it to not overwrite the template files).

## One-time setup

1. **Fork** this repo on GitHub (top-right "Fork" button).
2. **Clone your fork** locally:
   ```bash
   git clone git@github.com:<your-username>/10day-beginner-rust-challenge.git
   cd 10day-beginner-rust-challenge
   ```
3. **Add this repo as the upstream remote** so you can pull new days as they're released:
   ```bash
   git remote add upstream git@github.com:bbelderbos/10day-beginner-rust-challenge.git
   ```

## Each day

1. **Pull the latest day** from upstream:
   ```bash
   git checkout main
   git pull upstream main
   ```
2. **Create a branch** for the day:
   ```bash
   git checkout -b day-03
   ```
3. **Solve `src/dayXX.rs`** — fill in the `todo!()` bodies until the tests pass:
   ```bash
   cargo test day03
   ```
4. **Commit and push** to your fork:
   ```bash
   git add src/day03.rs
   git commit -m "day 3: control flow"
   git push origin day-03
   ```
5. [Optional] **Open a pull request** from your `day-03` branch to `bbelderbos/10day-beginner-rust-challenge:main`. GitHub will prompt you with a "Compare & pull request" button after the push.
