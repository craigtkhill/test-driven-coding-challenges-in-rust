pub fn double_char(input: &str) -> String {
    todo!() // This will cause the tests to fail until you implement it
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_string() {
        assert_eq!(double_char("String"), "SSttrriinngg");
    }

    #[test]
    fn test_with_spaces() {
        assert_eq!(double_char("Hello World!"), "HHeelllloo  WWoorrlldd!!");
    }

    #[test]
    fn test_numbers_and_symbols() {
        assert_eq!(double_char("1234!_ "), "11223344!!__  ");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(
            double_char("##^&%%*&%%$#@@!"),
            "####^^&&%%%%**&&%%%%$$##@@@@!!"
        );
    }

    #[test]
    fn test_lowercase_word() {
        assert_eq!(double_char("scandal"), "ssccaannddaall");
    }

    #[test]
    fn test_another_lowercase_word() {
        assert_eq!(double_char("economics"), "eeccoonnoommiiccss");
    }

    #[test]
    fn test_single_space() {
        assert_eq!(double_char(" "), "  ");
    }

    #[test]
    fn test_repeated_character() {
        assert_eq!(double_char("_______"), "______________");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(
            double_char("equip gallon read"),
            "eeqquuiipp  ggaalllloonn  rreeaadd"
        );
    }

    #[test]
    fn test_two_words() {
        assert_eq!(double_char("baby increase"), "bbaabbyy  iinnccrreeaassee");
    }
}
