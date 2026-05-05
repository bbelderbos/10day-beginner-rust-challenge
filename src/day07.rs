/// Return `Some((min, max, average))` over a slice of scores.
/// Return `None` if the slice is empty.
fn score_summary(scores: &[i32]) -> Option<(i32, i32, f64)> {
    todo!()
}

/// Return all scores at or above `threshold`, in their original order.
fn passing_scores(scores: &[i32], threshold: i32) -> Vec<i32> {
    todo!()
}

/// Return the top `n` scores, sorted from highest to lowest.
/// If `n` is greater than the input length, return all scores sorted.
fn top_n_scores(scores: &[i32], n: usize) -> Vec<i32> {
    todo!()
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
