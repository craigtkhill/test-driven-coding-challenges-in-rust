pub fn number_of_syllables(word: &str) -> u32 {
    word.split("-").count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_two_syllables() {
        assert_eq!(number_of_syllables("buf-fet"), 2);
    }

    #[test]
    fn test_three_syllables() {
        assert_eq!(number_of_syllables("beau-ti-ful"), 3);
    }

    #[test]
    fn test_four_syllables() {
        assert_eq!(number_of_syllables("mon-u-men-tal"), 4);
    }

    #[test]
    fn test_six_syllables() {
        assert_eq!(number_of_syllables("on-o-mat-o-poe-ia"), 6);
    }

    #[test]
    fn test_three_syllables_overly() {
        assert_eq!(number_of_syllables("o-ver-ly"), 3);
    }

    #[test]
    fn test_two_syllables_pastry() {
        assert_eq!(number_of_syllables("pas-try"), 2);
    }

    #[test]
    fn test_two_syllables_fluid() {
        assert_eq!(number_of_syllables("flu-id"), 2);
    }

    #[test]
    fn test_three_syllables_syllable() {
        assert_eq!(number_of_syllables("syl-la-ble"), 3);
    }
}
