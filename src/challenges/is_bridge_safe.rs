pub fn is_safe_bridge(bridge: &str) -> bool {
    let safe = if bridge.is_empty() {
        false
    } else if bridge.split(" ").count() > 1 {
        false
    } else {
        true
    };
    return safe;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_bridge() {
        assert!(
            is_safe_bridge("####"),
            "A solid bridge of 4 planks should be safe"
        );
    }

    #[test]
    fn test_broken_long_bridge() {
        assert!(
            !is_safe_bridge("## ####"),
            "A bridge with a gap should be unsafe"
        );
    }

    #[test]
    fn test_single_plank_bridge() {
        assert!(is_safe_bridge("#"), "A single plank bridge should be safe");
    }

    #[test]
    fn test_broken_small_bridge() {
        assert!(
            !is_safe_bridge("# #"),
            "A bridge with any gap should be unsafe"
        );
    }

    // Additional test cases to ensure comprehensive coverage
    #[test]
    fn test_empty_bridge() {
        assert!(!is_safe_bridge(""), "An empty bridge should be unsafe");
    }

    #[test]
    fn test_space_only_bridge() {
        assert!(
            !is_safe_bridge(" "),
            "A bridge of only spaces should be unsafe"
        );
    }

    #[test]
    fn test_very_long_solid_bridge() {
        assert!(
            is_safe_bridge("##############"),
            "A long solid bridge should be safe"
        );
    }

    #[test]
    fn test_bridge_with_trailing_space() {
        assert!(
            !is_safe_bridge("#### "),
            "A bridge with a trailing space should be unsafe"
        );
    }

    #[test]
    fn test_bridge_with_leading_space() {
        assert!(
            !is_safe_bridge(" ####"),
            "A bridge with a leading space should be unsafe"
        );
    }
}
