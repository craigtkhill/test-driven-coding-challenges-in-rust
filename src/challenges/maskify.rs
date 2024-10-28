pub fn maskify(input: &str) -> String {
    // TODO: Implement this function to make the tests pass
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(maskify("4556364607935616"), "############5616");
    }

    #[test]
    fn test2() {
        assert_eq!(maskify("64607935616"), "#######5616");
    }

    #[test]
    fn test3() {
        assert_eq!(maskify("1"), "1");
    }

    #[test]
    fn test4() {
        assert_eq!(maskify(""), "");
    }

    #[test]
    fn test5() {
        assert_eq!(maskify("tBy>L/cMe+?<j:6n;C~H"), "################;C~H");
    }

    #[test]
    fn test6() {
        assert_eq!(maskify("12"), "12");
    }

    #[test]
    fn test7() {
        assert_eq!(
            maskify("8Ikhlf6yoxPOwi5cB014eWbRumj7vJ"),
            "##########################j7vJ"
        );
    }

    #[test]
    fn test8() {
        assert_eq!(maskify("123"), "123");
    }

    #[test]
    fn test9() {
        assert_eq!(maskify(")E$aCU=e\"_"), "######=e\"_");
    }

    #[test]
    fn test10() {
        assert_eq!(maskify("2673951408"), "######1408");
    }

    #[test]
    fn test11() {
        assert_eq!(maskify("1234"), "1234");
    }
}
