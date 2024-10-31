pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32 as f64) * 5 as f64) / 9 as f64
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9 as f64 / 5 as f64 + 32 as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.01;

    fn assert_approx_eq(a: f64, b: f64, msg: &str) {
        assert!(
            (a - b).abs() < EPSILON,
            "{} - expected {}, got {}",
            msg,
            b,
            a
        );
    }

    #[test]
    fn test_freezing_point_conversion() {
        assert_approx_eq(
            fahrenheit_to_celsius(32.0),
            0.0,
            "32°F should convert to 0°C",
        );
    }

    #[test]
    fn test_boiling_point_conversion() {
        assert_approx_eq(
            fahrenheit_to_celsius(212.0),
            100.0,
            "212°F should convert to 100°C",
        );
    }

    #[test]
    fn test_body_temperature_f_to_c() {
        assert_approx_eq(
            fahrenheit_to_celsius(98.6),
            37.0,
            "98.6°F should convert to approximately 37°C",
        );
    }

    #[test]
    fn test_negative_temperature_f_to_c() {
        assert_approx_eq(
            fahrenheit_to_celsius(-40.0),
            -40.0,
            "-40°F should convert to -40°C",
        );
    }

    #[test]
    fn test_celsius_to_fahrenheit_freezing() {
        assert_approx_eq(
            celsius_to_fahrenheit(0.0),
            32.0,
            "0°C should convert to 32°F",
        );
    }

    #[test]
    fn test_celsius_to_fahrenheit_boiling() {
        assert_approx_eq(
            celsius_to_fahrenheit(100.0),
            212.0,
            "100°C should convert to 212°F",
        );
    }

    #[test]
    fn test_body_temperature_c_to_f() {
        assert_approx_eq(
            celsius_to_fahrenheit(37.0),
            98.6,
            "37°C should convert to approximately 98.6°F",
        );
    }

    #[test]
    fn test_negative_temperature_c_to_f() {
        assert_approx_eq(
            celsius_to_fahrenheit(-40.0),
            -40.0,
            "-40°C should convert to -40°F",
        );
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_temp = 25.0;
        let converted = fahrenheit_to_celsius(celsius_to_fahrenheit(original_temp));
        assert_approx_eq(
            converted,
            original_temp,
            "Converting from C->F->C should return the original temperature",
        );
    }
}
