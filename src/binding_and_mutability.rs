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

// // Fill the blanks in the code to make it compile
// fn main() {
//     let __ __ = 1;
//     __ += 2; 
    
//     assert_eq!(x, 3);
//     println!("Success!");
// }

// Fill the blanks in the code to make it compile
pub fn mutable() -> i32 {
    let  mut x = 1;
    x += 2;
    x
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_initialize() {
        assert_eq!(super::initialize(), 5);
    }
    
    #[test]
    fn test_mutable() {
        assert_eq!(super::mutable(), 3);
    }
}

