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

// // Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x;
//     x += 3;

//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!";

//     println!("Success!");
// }

// Remove a line in the code to make it compile
pub fn rebind() -> (i32, &'static str) {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;

    let _y = 4;
    // Shadowing
    let y = "I can also be bound to text!";
    return (x, y);
}

#[cfg(test)]
mod tests {
    use super::rebind;
    use super::shadow;

    #[test]
    fn test_shadow() {
        assert_eq!(shadow(), 42);
    }

    #[test]
    fn test_rebind() {
        assert_eq!(rebind(), (7, "I can also be bound to text!"));
    }
}
