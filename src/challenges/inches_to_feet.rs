pub fn inches_to_feet(inches: i32) -> i32 {
    todo!() // Implementation needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12_inches_equals_1_foot() {
        assert_eq!(inches_to_feet(12), 1);
    }

    #[test]
    fn test_360_inches_equals_30_feet() {
        assert_eq!(inches_to_feet(360), 30);
    }

    #[test]
    fn test_3612_inches_equals_301_feet() {
        assert_eq!(inches_to_feet(3612), 301);
    }

    #[test]
    fn test_324_inches_equals_27_feet() {
        assert_eq!(inches_to_feet(324), 27);
    }

    #[test]
    fn test_3012_inches_equals_251_feet() {
        assert_eq!(inches_to_feet(3012), 251);
    }

    #[test]
    fn test_11_inches_equals_0_feet() {
        assert_eq!(inches_to_feet(11), 0);
    }
}
