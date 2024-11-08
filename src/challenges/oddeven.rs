pub fn oddeven(numbers: Vec<i32>) -> bool {
    let evens = numbers.iter().filter(|x| *x % 2 == 0);
    let odds = numbers.iter().filter(|x| *x % 2 == 1);
    evens.count() < odds.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(
            oddeven(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            "Should return true for sequence 1-9"
        );
    }

    #[test]
    fn test2() {
        assert!(
            oddeven(vec![1]),
            "Should return true for single element [1]"
        );
    }

    #[test]
    fn test3() {
        assert!(
            oddeven(vec![1, 2, 3, 4, 5, 6, 7, 9]),
            "Should return true for sequence without 8"
        );
    }

    #[test]
    fn test4() {
        assert!(
            !oddeven(vec![42, 1, 66]),
            "Should return false for [42, 1, 66]"
        );
    }

    #[test]
    fn test5() {
        assert!(
            !oddeven(vec![2, 3, 4, 5, 6, 7, 8]),
            "Should return false for sequence 2-8"
        );
    }
}
