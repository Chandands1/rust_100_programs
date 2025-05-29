// In src/lib.rs or src/main.rs (though typically in lib.rs for reusable code)

// Function to be tested
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // This attribute ensures the 'tests' module is only compiled when running tests
mod tests {
    // Import everything from the outer scope (our `add` function)
    use super::*;

    #[test] // This attribute marks the function as a test function
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5); // Assert that 2 + 3 equals 5
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -5), -6); // Assert with negative numbers
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 7), 7); // Assert with zero
    }
}