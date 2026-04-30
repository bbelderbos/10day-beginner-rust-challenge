/// Return the first whitespace-separated word of `s`.
/// If `s` is empty or has no words, return an empty string slice.
///
/// Notice the return type: `&str`. You're returning a slice borrowed from the input,
/// not a new allocation.
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// Return an uppercased copy of `s` with an exclamation mark appended.
/// e.g. "hello" → "HELLO!"
///
/// Notice the return type: `String`. You're creating new owned data.
pub fn shout(s: &str) -> String {
    todo!()
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
