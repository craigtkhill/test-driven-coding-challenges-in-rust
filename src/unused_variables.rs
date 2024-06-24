// fn main() {
//     let x = 1;
// }

// // Warning: unused variable: `x`

pub fn unused_variable() -> i32 {
    let x: i32 = 1;
    return x;
}

// Warning: unused variable: `x`

#[cfg(test)]
mod tests {
    use super::unused_variable;

    #[test]
    fn test_unused_variable() {
        assert_eq!(unused_variable(), 1);
    }
}
