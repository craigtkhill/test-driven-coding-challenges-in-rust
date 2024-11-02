pub fn name_string(name: &str) -> String {
    name.to_owned() + "Edabit"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mubashir() {
        assert_eq!(name_string("Mubashir"), "MubashirEdabit");
    }

    #[test]
    fn test_matt() {
        assert_eq!(name_string("Matt"), "MattEdabit");
    }

    #[test]
    fn test_cpp() {
        assert_eq!(name_string("C++"), "C++Edabit");
    }

    #[test]
    fn test_airforce() {
        assert_eq!(name_string("Airforce"), "AirforceEdabit");
    }

    // Additional test to check empty string handling
    #[test]
    fn test_empty_string() {
        assert_eq!(name_string(""), "Edabit");
    }
}
