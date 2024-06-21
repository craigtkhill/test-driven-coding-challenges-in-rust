// Fix the error below with least amount of modification to the code
// fn initialize() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

pub fn initialize() -> i32 {
    let x: i32 = 5; // Uninitialized with the value of 5.
    let _y: i32; // Add an underscore to suppress the warning.

    x
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_initialize() {
        assert_eq!(super::initialize(), 5);
    }
}
