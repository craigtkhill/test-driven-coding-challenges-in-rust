pub fn bool_to_string(value: bool) -> String {
    if value == true {
        String::from("true")
    } else {
        String::from("false")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_conversion() {
        assert_eq!(
            bool_to_string(true),
            "true",
            "Should convert true to \"true\""
        );
    }

    #[test]
    fn test_false_conversion() {
        assert_eq!(
            bool_to_string(false),
            "false",
            "Should convert false to \"false\""
        );
    }
}
