// Convert the &str literal (borrowed, read-only) into an owned String.
// The last expression has no semicolon, so it's the return value, no explicit `return` needed.
fn greet() -> String {
    "Hello, Rustacean!".to_string()
}

// Variables are immutable by default in Rust; if you want to mutate you have to use `mut`.
// The `for _ in 0..5` loop runs 5 times — `0..5` is like a Python range(0, 5), upper bound is exclusive.
fn double_counter() -> i32 {
    let mut counter = 1;
    for _ in 0..5 {
        counter *= 2;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, Rustacean!");
    }

    #[test]
    fn test_double_counter() {
        assert_eq!(double_counter(), 32);
    }
}
