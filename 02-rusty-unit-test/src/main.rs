fn main() {
    println!("Hello, world!");
}

/// Adds two 32-bit signed integers.
///
/// # Arguments
///
/// * `a` - The first integer to add.
/// * `b` - The second integer to add.
///
/// # Returns
///
/// The sum of `a` and `b` as a 32-bit signed integer.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `add` function with basic positive integer values.
    ///
    /// Verifies that `add(2, 3)` correctly returns `5`.
    #[test]
    fn test_add_basic() {
        let result = add(2, 3);
        assert_eq!(result, 5, "2 + 3 should equal 5");
    }
}