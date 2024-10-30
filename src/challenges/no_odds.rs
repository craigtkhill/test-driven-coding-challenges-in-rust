pub fn no_odds(nums: Vec<i32>) -> Vec<i32> {
    todo!("Remove odd numbers from the vector")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(no_odds(vec![1, 2, 3, 4, 5, 6, 7, 8]), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test2() {
        assert_eq!(no_odds(vec![43, 65, 23, 89, 53, 9, 6]), vec![6]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            no_odds(vec![718, 991, 449, 644, 380, 440]),
            vec![718, 644, 380, 440]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(no_odds(vec![148, 6, 16, 85]), vec![148, 6, 16]);
    }

    #[test]
    fn test5() {
        assert_eq!(no_odds(vec![9, 49, 23]), vec![]);
    }

    #[test]
    fn test6() {
        assert_eq!(no_odds(vec![34, 43, 32, 49, 40]), vec![34, 32, 40]);
    }

    #[test]
    fn test7() {
        assert_eq!(
            no_odds(vec![1232, 1990, 1284, 1391, 1958]),
            vec![1232, 1990, 1284, 1958]
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            no_odds(vec![2766, 2651, 2373, 2916, 2397, 2539]),
            vec![2766, 2916]
        );
    }

    #[test]
    fn test9() {
        assert_eq!(no_odds(vec![53, 65, 52, 62, 59]), vec![52, 62]);
    }

    #[test]
    fn test10() {
        assert_eq!(
            no_odds(vec![393, 156, 14, 166, 129, 246]),
            vec![156, 14, 166, 246]
        );
    }
}
