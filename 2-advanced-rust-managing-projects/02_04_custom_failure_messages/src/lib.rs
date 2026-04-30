fn add_two(a: i32, b: i32) -> i32 {
    a + 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_assert() {
        let result = add_two(2, 2);
        assert!(result == 4, "Expected 4; result was {}", result);
    }

    #[test]
    fn test_with_assert_eq() {
        let result = add_two(2, 2);
        assert_eq!(result, 4, "Expected 4; result was {}", result);
    }

    #[test]
    fn test_with_assert_ne() {
        assert_ne!(add_two(2, 2), 3);
    }
}
