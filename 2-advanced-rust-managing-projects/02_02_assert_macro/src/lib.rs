fn is_even(number: i32) -> bool {
    match number / 2 {
        0 => true, // number is even
        _ => false // number is odd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(42));
        assert!(!is_even(13));
    }
}
