pub fn convert(minutes: i32) -> i32 {
    minutes * 60
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(convert(6), 360, "Should convert 6 minutes to seconds");
    }

    #[test]
    fn test2() {
        assert_eq!(convert(4), 240, "Should convert 4 minutes to seconds");
    }

    #[test]
    fn test3() {
        assert_eq!(convert(8), 480, "Should convert 8 minutes to seconds");
    }

    #[test]
    fn test4() {
        assert_eq!(convert(60), 3600, "Should convert 60 minutes to seconds");
    }
}
