Write rust tests for the following

{C++ test code}

here's a template of what I want

pub fn return_true() -> bool {
    // TODO: Implement this function to make the test pass
    todo!() // This will cause the test to fail until you implement it
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert!(return_true(), "Should return true");
    }
}
