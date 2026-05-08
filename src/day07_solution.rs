// `&[i32]` is a borrowed slice — like passing a list view in Python without copying.
// `.iter().min()` returns `Option<&i32>` because the slice might be empty.
// The `?` operator on Option short-circuits to `None` if the slice is empty,
// so we never have to call `.unwrap()` and risk a panic.
// `.sum()` needs a type annotation because Rust can't infer the result type.
fn score_summary(scores: &[i32]) -> Option<(i32, i32, f64)> {
    let min = *scores.iter().min()?;
    let max = *scores.iter().max()?;
    let sum: i32 = scores.iter().sum();
    let avg = sum as f64 / scores.len() as f64;
    Some((min, max, avg))
}

// Manual loop with `Vec::new()` and `.push()` — the imperative style.
// `&score` in the for-binding pattern destructures the &i32 down to i32.
// Idiomatic alternative: `scores.iter().filter(|&&s| s >= threshold).copied().collect()`.
fn passing_scores(scores: &[i32], threshold: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for &score in scores {
        if score >= threshold {
            result.push(score);
        }
    }
    result
}

// `.to_vec()` makes an owned copy so we can sort without mutating the caller's slice.
// `b.cmp(a)` (note the order) gives descending sort; `.cmp()` lives on `Ord`.
// `.into_iter().take(n).collect()` is lazy: it stops after `n` items.
fn top_n_scores(scores: &[i32], n: usize) -> Vec<i32> {
    let mut sorted = scores.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted.into_iter().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_summary() {
        let scores = vec![60, 75, 90, 45, 80];
        let (min, max, avg) = score_summary(&scores).unwrap();
        assert_eq!(min, 45);
        assert_eq!(max, 90);
        assert!((avg - 70.0).abs() < 0.01);
    }

    #[test]
    fn test_score_summary_empty() {
        let scores: Vec<i32> = vec![];
        assert_eq!(score_summary(&scores), None);
    }

    #[test]
    fn test_passing_scores() {
        let scores = vec![60, 75, 90, 45, 80];
        assert_eq!(passing_scores(&scores, 70), vec![75, 90, 80]);
    }

    #[test]
    fn test_passing_scores_none() {
        let scores = vec![40, 50, 60];
        assert_eq!(passing_scores(&scores, 70), Vec::<i32>::new());
    }

    #[test]
    fn test_top_n_scores() {
        let scores = vec![60, 75, 90, 45, 80];
        assert_eq!(top_n_scores(&scores, 3), vec![90, 80, 75]);
    }

    #[test]
    fn test_top_n_scores_more_than_len() {
        let scores = vec![60, 75];
        assert_eq!(top_n_scores(&scores, 5), vec![75, 60]);
    }

    #[test]
    fn test_top_n_scores_zero() {
        let scores = vec![60, 75, 90];
        assert_eq!(top_n_scores(&scores, 0), Vec::<i32>::new());
    }
}
