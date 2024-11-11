pub fn array_of_multiples(num: i32, length: usize) -> Vec<i32> {
    let len = length as i32;
    let mut vec = Vec::new();
    for i in 1..=len {
        let multiple = i * num;
        vec.push(multiple);
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_numbers() {
        assert_eq!(array_of_multiples(7, 5), vec![7, 14, 21, 28, 35]);
        assert_eq!(
            array_of_multiples(12, 10),
            vec![12, 24, 36, 48, 60, 72, 84, 96, 108, 120]
        );
        assert_eq!(
            array_of_multiples(17, 7),
            vec![17, 34, 51, 68, 85, 102, 119]
        );
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(
            array_of_multiples(630, 14),
            vec![630, 1260, 1890, 2520, 3150, 3780, 4410, 5040, 5670, 6300, 6930, 7560, 8190, 8820]
        );
    }

    #[test]
    fn test_short_sequences() {
        assert_eq!(array_of_multiples(140, 3), vec![140, 280, 420]);
    }

    #[test]
    fn test_medium_sequences() {
        assert_eq!(
            array_of_multiples(7, 8),
            vec![7, 14, 21, 28, 35, 42, 49, 56]
        );
    }

    #[test]
    fn test_long_sequences() {
        assert_eq!(
            array_of_multiples(11, 21),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 110, 121, 132, 143, 154, 165, 176, 187, 198,
                209, 220, 231
            ]
        );
    }
}
