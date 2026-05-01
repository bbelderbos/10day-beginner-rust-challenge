/// Return a grade message based on a numeric score.
///
/// Step 1 — use `if/else if/else` to determine the grade:
///   score >= 90 → "Excellent"
///   score >= 75 → "Good"
///   score >= 50 → "Pass"
///   otherwise   → "Fail"
///
/// Step 2 — use `match` on the first character of the grade to return:
///   'E' → "Top"
///   'G' → "Decent"
///   'P' → "Basic"
///   anything else → "None"
fn grade_message(score: i32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top() {
        assert_eq!(grade_message(95), "Top");
    }

    #[test]
    fn test_decent() {
        assert_eq!(grade_message(78), "Decent");
    }

    #[test]
    fn test_basic() {
        assert_eq!(grade_message(52), "Basic");
    }

    #[test]
    fn test_none() {
        assert_eq!(grade_message(10), "None");
    }
}
