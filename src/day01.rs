// Day 1: Hello, Rustacean + Variables and Mutability

fn greet() -> String {
    // your code here
    todo!()
}

fn double_counter() -> i32 {
    // your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, Rustacean!");
    }

    #[test]
    fn test_double_counter() {
        assert_eq!(double_counter(), 32);
    }
}
