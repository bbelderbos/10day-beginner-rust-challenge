// `filter_map` combines filter and map: returning `None` drops the item, `Some(x)` keeps it.
// `?` inside the closure exits early with `None` if the record is malformed —
// missing colon or non-numeric score — so bad records are silently skipped.
// We collect into `(String, u32)` first so we can sort, then format afterwards.
fn top_scorers(records: &[&str], threshold: u32) -> Vec<String> {
    let mut pairs: Vec<(String, u32)> = records
        .iter()
        .filter_map(|record| {
            let (name, score_str) = record.split_once(':')?;
            let score: u32 = score_str.trim().parse().ok()?;
            if score >= threshold {
                Some((name.trim().to_string(), score))
            } else {
                None
            }
        })
        .collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs
        .into_iter()
        .map(|(name, score)| format!("{} ({})", name, score))
        .collect()
}

// `impl Fn(u32) -> u32` is a generic bound. Any callable that takes a u32
// and returns a u32 fits — named functions, closures, even closures that capture
// variables from the surrounding scope. Closer to Python's "functions are values"
// than to a pattern like `Callable[[int], int]` from the typing module.
fn apply_bonus(scores: &[u32], bonus: impl Fn(u32) -> u32) -> Vec<u32> {
    scores.iter().map(|&s| bonus(s)).collect()
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
