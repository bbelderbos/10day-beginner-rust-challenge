// `Result<T, E>` models fallible operations — like Python try/except, but the error
// type is part of the function signature so callers can't ignore it.
// `?` propagates the error early: if `parse` fails, we return `Err(...)` immediately.
// `map_err` converts the parse error type into our own descriptive String.
fn parse_score(s: &str) -> Result<u32, String> {
    let n: u32 = s
        .trim()
        .parse()
        .map_err(|_| format!("'{}' is not a valid number", s.trim()))?;
    if n > 100 {
        return Err(format!("{} is out of range (0-100)", n));
    }
    Ok(n)
}

// `?` shines when chaining fallible calls. Inside the loop we delegate to `parse_score`,
// and `?` either unwraps the value into `n` or returns the first `Err` to the caller.
// The collection-with-?-loop is the bread-and-butter pattern for "all-or-nothing" parsing.
fn parse_scores(records: &[&str]) -> Result<Vec<u32>, String> {
    let mut out = Vec::with_capacity(records.len());
    for r in records {
        let n = parse_score(r)?;
        out.push(n);
    }
    Ok(out)
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
