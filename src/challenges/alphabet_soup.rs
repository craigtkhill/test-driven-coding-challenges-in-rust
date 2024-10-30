pub fn alphabet_soup(s: &str) -> String {
    todo!() // Implementation will be added later
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sorting() {
        assert_eq!(alphabet_soup("hello"), "ehllo");
        assert_eq!(alphabet_soup("edabit"), "abdeit");
    }

    #[test]
    fn test_tech_words() {
        assert_eq!(alphabet_soup("hacker"), "acehkr");
        assert_eq!(alphabet_soup("geek"), "eegk");
        assert_eq!(alphabet_soup("javascript"), "aacijprstv");
    }

    #[test]
    fn test_longer_words() {
        assert_eq!(alphabet_soup("credibility"), "bcdeiiilrty");
        assert_eq!(alphabet_soup("apostrophe"), "aehoopprst");
        assert_eq!(alphabet_soup("determination"), "adeeiimnnortt");
    }

    #[test]
    fn test_short_and_complex() {
        assert_eq!(alphabet_soup("win"), "inw");
        assert_eq!(alphabet_soup("synthesis"), "ehinsssty");
    }

    // Additional edge cases
    #[test]
    fn test_empty_string() {
        assert_eq!(alphabet_soup(""), "");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(alphabet_soup("a"), "a");
    }

    #[test]
    fn test_repeated_characters() {
        assert_eq!(alphabet_soup("aaa"), "aaa");
    }
}
