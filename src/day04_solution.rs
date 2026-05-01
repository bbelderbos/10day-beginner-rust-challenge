// Returns a borrowed slice (&str) of the input, no allocation, no copy.
// `split_whitespace` yields an iterator (Pythonic things in Rust 😍);
// `.next()` pulls the first item as Option<&str>.
// `unwrap_or("")` falls back to an empty slice if the input has no words
// (similar to dict.get(key, default) in Python).
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Returns an owned `String` because `to_uppercase` creates a new allocation.
// Using `format!` is more idiomatic than the `+` operator for building strings.
// While `+` moves ownership of the left-hand `String` and appends the right-hand `&str`,
// `format!` is more readable, handles multiple parts/types easily, and avoids
// the "clunky" requirement of ensuring the first element is an owned String.
fn shout(s: &str) -> String {
    format!("{}!", s.to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("one"), "one");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_shout() {
        assert_eq!(shout("hello"), "HELLO!");
        assert_eq!(shout("rust"), "RUST!");
    }
}
