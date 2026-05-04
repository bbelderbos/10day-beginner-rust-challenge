// A struct groups related data, think Python dataclass or namedtuple, no methods
struct Temperature {
    celsius: f64,
}

// `impl` blocks attach methods to a struct.
// `Self::new` is the constructor convention (no special `__init__` syntax).
// Methods that take `&self` borrow without consuming, like Python's `self`
// although Python can mutate where Rust is immutable by default
// here we only access read-only, if we were to mutate self, we'd have to use `&mut self`
impl Temperature {
    fn new(celsius: f64) -> Self {
        Self { celsius }
    }

    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    fn is_fever(&self) -> bool {
        self.celsius >= 38.0
    }

    fn describe(&self) -> String {
        if self.celsius < 0.0 {
            "Freezing".to_string()
        } else if self.celsius < 15.0 {
            "Cold".to_string()
        } else if self.celsius < 25.0 {
            "Comfortable".to_string()
        } else {
            "Hot".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(Temperature::new(0.0).to_fahrenheit(), 32.0);
        assert_eq!(Temperature::new(100.0).to_fahrenheit(), 212.0);
    }

    #[test]
    fn test_is_fever() {
        assert!(!Temperature::new(37.9).is_fever());
        assert!(Temperature::new(38.0).is_fever());
    }

    #[test]
    fn test_describe() {
        assert_eq!(Temperature::new(-5.0).describe(), "Freezing");
        assert_eq!(Temperature::new(10.0).describe(), "Cold");
        assert_eq!(Temperature::new(20.0).describe(), "Comfortable");
        assert_eq!(Temperature::new(30.0).describe(), "Hot");
    }
}
