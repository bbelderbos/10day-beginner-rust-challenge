// Rust infers types where possible. `42` becomes i32, `3.14` becomes f64.
// `char` is a single Unicode scalar value (4 bytes)
// single quotes denote chars, double quotes denote string slices (&str).
// Tuples can mix types — here (i32, &str).
// `format!` uses `{}` for Display and `{:?}` for Debug, latter is needed for tuples.
fn describe_types() -> String {
    let int = 42;
    let float = 3.14;
    let flag = true;
    let letter = 'Z';
    let pair = (7, "Rust");

    format!(
        "int: {}, float: {}, bool: {}, char: {}, tuple: {:?}",
        int, float, flag, letter, pair
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_types() {
        let output = describe_types();
        assert!(output.contains("int: 42"));
        assert!(output.contains("float: 3.14"));
        assert!(output.contains("bool: true"));
        assert!(output.contains("char: Z"));
        assert!(output.contains("tuple: (7, \"Rust\")"));
    }
}
