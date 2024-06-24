// // Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 5);
//     }

//     assert_eq!(x, 12);

//     let x = 42;
//     println!("{}", x); // Prints "42".
// }

pub fn shadow() -> i32 {
    let x: i32 = 5;
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    x
}

#[cfg(test)]
mod tests {
    use super::shadow;

    #[test]
    fn test_shadow() {
        assert_eq!(shadow(), 42);
    }
}
