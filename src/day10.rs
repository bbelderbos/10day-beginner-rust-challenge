use std::collections::HashMap;

/// Count occurrences of each word in `text`.
///
/// - Split by whitespace.
/// - Strip non-alphabetic characters from each word.
/// - Lowercase before counting.
/// - Skip empty results (e.g. a token that was pure punctuation).
///
/// Hint: `HashMap::entry(key).or_insert(0)` is the get-or-default pattern.
fn word_count(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Return the top `n` `(word, count)` pairs from `map`.
/// Sort by count descending; break ties alphabetically.
///
/// Hint: collect into a `Vec`, then `.sort_by()` with a custom comparator.
/// `.then()` on `Ordering` is your tiebreak.
fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    todo!()
}

/// Return the top `n` words from `text` formatted as `"word (count)"`.
/// Wires `word_count` and `top_n` together into one tool.
fn summarize(text: &str, n: usize) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let counts = word_count("the quick brown fox jumps over the lazy dog the fox");
        assert_eq!(counts["the"], 3);
        assert_eq!(counts["fox"], 2);
        assert_eq!(counts["quick"], 1);
    }

    #[test]
    fn test_punctuation_stripped() {
        let counts = word_count("hello, world! hello.");
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 1);
    }

    #[test]
    fn test_case_insensitive() {
        let counts = word_count("Rust rust RUST");
        assert_eq!(counts["rust"], 3);
    }

    #[test]
    fn test_top_n() {
        let counts = word_count("the fox the fox the");
        let top = top_n(&counts, 1);
        assert_eq!(top[0], ("the", 3));
    }

    #[test]
    fn test_top_n_tiebreak() {
        let counts = word_count("apple banana apple banana cherry");
        let top = top_n(&counts, 2);
        assert_eq!(top[0].1, 2);
        assert_eq!(top[1].1, 2);
        assert!(top[0].0 < top[1].0);
    }

    #[test]
    fn test_summarize() {
        let out = summarize("the fox the fox the dog", 2);
        assert_eq!(out, vec!["the (3)", "fox (2)"]);
    }

    #[test]
    fn test_summarize_strips_punctuation() {
        let out = summarize("Hello, hello! World.", 2);
        assert_eq!(out, vec!["hello (2)", "world (1)"]);
    }
}
