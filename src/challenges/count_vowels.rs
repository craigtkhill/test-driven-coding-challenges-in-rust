pub fn count_vowels(s: &str) -> u32 {
    let mut count: u32 = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for c in s.chars() {
        let lowercase_c = c.to_ascii_lowercase();
        if vowels.contains(&lowercase_c) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celebration() {
        assert_eq!(count_vowels("Celebration"), 5);
    }

    #[test]
    fn test_palm() {
        assert_eq!(count_vowels("Palm"), 1);
    }

    #[test]
    fn test_prediction() {
        assert_eq!(count_vowels("Prediction"), 4);
    }

    #[test]
    fn test_suite() {
        assert_eq!(count_vowels("Suite"), 3);
    }

    #[test]
    fn test_quote() {
        assert_eq!(count_vowels("Quote"), 3);
    }

    #[test]
    fn test_portrait() {
        assert_eq!(count_vowels("Portrait"), 3);
    }

    #[test]
    fn test_steam() {
        assert_eq!(count_vowels("Steam"), 2);
    }

    #[test]
    fn test_tape() {
        assert_eq!(count_vowels("Tape"), 2);
    }

    #[test]
    fn test_nightmare() {
        assert_eq!(count_vowels("Nightmare"), 3);
    }

    #[test]
    fn test_convention() {
        assert_eq!(count_vowels("Convention"), 4);
    }
}
