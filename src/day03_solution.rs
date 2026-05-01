// `if` is an expression in Rust: it returns a value, so we can bind its result to `grade`.
// `match` is also an expression and is exhaustive: every possible variant must be handled
// (or use `_` as a catch-all). The compiler enforces this at compile time.
fn grade_message(score: i32) -> String {
    let grade = if score >= 90 {
        "Excellent"
    } else if score >= 75 {
        "Good"
    } else if score >= 50 {
        "Pass"
    } else {
        "Fail"
    };

    match grade.chars().next() {
        Some('E') => "Top".to_string(),
        Some('G') => "Decent".to_string(),
        Some('P') => "Basic".to_string(),
        _ => "None".to_string(),
    }
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
