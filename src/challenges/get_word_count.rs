pub fn count_words(s: &str) -> usize {
    (s.split(" ")).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            count_words("It's high noon"),
            3,
            "Should count 'It's high noon' as 3 words"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            count_words("Is this easy mode"),
            4,
            "Should count 'Is this easy mode' as 4 words"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            count_words("What an easy task, right"),
            5,
            "Should count 'What an easy task, right' as 5 words"
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            count_words("This is a test"),
            4,
            "Should count 'This is a test' as 4 words"
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            count_words("Just an example here move along"),
            6,
            "Should count 'Just an example here move along' as 6 words"
        );
    }

    #[test]
    fn test6() {
        assert_eq!(
            count_words("How are you today?"),
            4,
            "Should count 'How are you today?' as 4 words"
        );
    }
}
