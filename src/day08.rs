/// Parse a score string into a u32 in the range 0–100.
///
/// - Trim leading/trailing whitespace before parsing.
/// - On parse failure, return Err with this exact format:
///     "'abc' is not a valid number"
/// - On out-of-range, return Err with this exact format:
///     "101 is out of range (0-100)"
///
/// Hint: use `.parse()` + `.map_err()` + the `?` operator.
fn parse_score(s: &str) -> Result<u32, String> {
    todo!()
}

/// Parse a slice of score strings into a `Vec<u32>`.
/// Stop and return the first error encountered.
///
/// Hint: a for loop with `parse_score(...)?` is the cleanest path.
/// The `?` bubbles the first Err out of the function for you.
fn parse_scores(records: &[&str]) -> Result<Vec<u32>, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(parse_score("85"), Ok(85));
        assert_eq!(parse_score("0"), Ok(0));
        assert_eq!(parse_score("100"), Ok(100));
    }

    #[test]
    fn test_whitespace() {
        assert!(parse_score(" 42 ").is_ok());
    }

    #[test]
    fn test_out_of_range() {
        assert_eq!(
            parse_score("101"),
            Err("101 is out of range (0-100)".to_string())
        );
    }

    #[test]
    fn test_not_a_number() {
        assert_eq!(
            parse_score("abc"),
            Err("'abc' is not a valid number".to_string())
        );
    }

    #[test]
    fn test_negative_string() {
        assert_eq!(
            parse_score("-5"),
            Err("'-5' is not a valid number".to_string())
        );
    }

    #[test]
    fn test_parse_scores_all_valid() {
        let records = ["10", "55", "99"];
        assert_eq!(parse_scores(&records), Ok(vec![10, 55, 99]));
    }

    #[test]
    fn test_parse_scores_first_error_wins() {
        let records = ["10", "abc", "55"];
        assert!(parse_scores(&records).is_err());
    }

    #[test]
    fn test_parse_scores_empty() {
        let records: [&str; 0] = [];
        assert_eq!(parse_scores(&records), Ok(vec![]));
    }
}
