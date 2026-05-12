use std::collections::HashMap;

// `HashMap` is Rust's dict. The `entry().or_insert(0)` pattern is the idiomatic
// counter — equivalent to Python's `Counter`
// `*` dereferences the &mut value returned by entry() so we can `+= 1` it.
// Punctuation is stripped via `chars().filter(is_alphabetic)`.
// `flat_map(|c| c.to_lowercase())` does the lowercase in the same iterator pass
// instead of building a String and then calling `.to_lowercase()` on it (which
// would allocate twice per word). `char::to_lowercase` returns an iterator
// because one char can lowercase to multiple (e.g. Turkish 'İ' → 'i' + dot),
// hence `flat_map` rather than `map`.
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .flat_map(|c| c.to_lowercase())
            .collect();
        if !clean.is_empty() {
            *map.entry(clean).or_insert(0) += 1;
        }
    }
    map
}

// Sort by descending count, ties broken alphabetically. The comparator closure
// uses `b.cmp(a)` to flip the order and `.then(...)` to chain a secondary key.
// We borrow the keys as &str to avoid cloning Strings into the result.
// `sort_unstable_by` is faster and uses less memory than `sort_by`. The catch
// is that it doesn't preserve the relative order of equal elements — but our
// tie-breaker (alphabetical) already makes the result deterministic, so there
// are no "equal elements" left for stability to matter. Default to unstable
// whenever you have a total ordering.
fn top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut pairs: Vec<(&str, usize)> = map.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    pairs.into_iter().take(n).collect()
}

// The capstone wires `word_count` and `top_n` together into a single helper.
// `into_iter().map(...).collect()` is the workhorse iterator pattern: own the
// pairs, transform them into formatted strings, and gather them into a Vec.
// `format!("{word} ({count})")` uses captured identifiers (stable since Rust
// 1.58) — same as Python f-strings. Reads better than positional `{}, {}`
// because you don't have to mentally line up the placeholders with arguments.
fn summarize(text: &str, n: usize) -> Vec<String> {
    let counts = word_count(text);
    top_n(&counts, n)
        .into_iter()
        .map(|(word, count)| format!("{word} ({count})"))
        .collect()
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
