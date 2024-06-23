// // Fix the error below with least amount of modification
// fn main() {
//     let x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//     }
//     println!("The value of x is {} and value of y is {}", x, y); 
// }

// Fix the error below with least amount of modification
pub fn scope() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// // Fix the error with the use of define_x
// fn main() {
//     println!("{}, world", x); 
// }

// fn define_x() {
//     let x = "hello";
// }

// Fix the error with the use of define_x
pub fn define() {
    define_x();
}

fn define_x() -> &'static str {
    let x: &str = "hello";
    x

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_scope() {
        assert_eq!(super::scope(), ());
    }

    #[test]
    fn test_define() {
        assert_eq!(super::define(), ());
    }

    #[test]
    fn test_define_x() {
        assert_eq!(super::define_x(), "hello");
    }
    
}