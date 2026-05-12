/// Each record is `"Name:Score"` (e.g. `"Alice:92"`).
/// Parse them, keep only `score >= threshold`, and return formatted strings
/// like `"Alice (92)"` sorted by score descending. Silently skip malformed records.
///
/// Hint: an iterator chain (`.iter()`, `.filter_map()`, `.collect()`), then sort.
/// `split_once(':')` gives you `Option<(&str, &str)>`.
fn top_scorers(records: &[&str], threshold: u32) -> Vec<String> {
    todo!()
}

/// Apply a `bonus` function to each score and return the new scores.
///
/// `bonus` is any function or closure with signature `fn(u32) -> u32`.
/// `impl Fn(u32) -> u32` accepts both named functions and `|s| ...` closures.
fn apply_bonus(scores: &[u32], bonus: impl Fn(u32) -> u32) -> Vec<u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_scorers_filters() {
        let records = vec!["Alice:92", "Bob:45", "Carol:88", "Dave:71"];
        assert_eq!(top_scorers(&records, 80), vec!["Alice (92)", "Carol (88)"]);
    }

    #[test]
    fn test_top_scorers_sorted_desc() {
        let records = vec!["Carol:88", "Alice:92", "Bob:85"];
        assert_eq!(
            top_scorers(&records, 80),
            vec!["Alice (92)", "Carol (88)", "Bob (85)"]
        );
    }

    #[test]
    fn test_none_qualify() {
        let records = vec!["Alice:92", "Bob:45"];
        assert_eq!(top_scorers(&records, 100), Vec::<String>::new());
    }

    #[test]
    fn test_malformed_skipped() {
        let records = vec!["Alice:92", "badrecord", "Carol:88"];
        assert_eq!(top_scorers(&records, 80), vec!["Alice (92)", "Carol (88)"]);
    }

    #[test]
    fn test_apply_bonus_closure() {
        let scores = vec![10, 20, 30];
        let with_bonus = apply_bonus(&scores, |s| s + 5);
        assert_eq!(with_bonus, vec![15, 25, 35]);
    }

    #[test]
    fn test_apply_bonus_capping() {
        let scores = vec![90, 95, 100];
        let with_bonus = apply_bonus(&scores, |s| (s + 10).min(100));
        assert_eq!(with_bonus, vec![100, 100, 100]);
    }

    #[test]
    fn test_apply_bonus_named_function() {
        fn double(s: u32) -> u32 {
            s * 2
        }
        let scores = vec![1, 2, 3];
        assert_eq!(apply_bonus(&scores, double), vec![2, 4, 6]);
    }
}
