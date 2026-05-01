/// A temperature reading in Celsius.
///
/// Implement these methods on `Temperature`:
///
/// - `new(celsius: f64) -> Self` — construct a new Temperature.
/// - `to_fahrenheit(&self) -> f64` — convert to Fahrenheit (F = C * 9/5 + 32).
/// - `is_fever(&self) -> bool` — true if 38.0°C or higher.
/// - `describe(&self) -> String` — return one of:
///     < 0.0  → "Freezing"
///     < 15.0 → "Cold"
///     < 25.0 → "Comfortable"
///     else   → "Hot"
struct Temperature {
    celsius: f64,
}

impl Temperature {
    // your methods here
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
