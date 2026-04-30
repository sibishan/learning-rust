fn is_even(n: i32) -> bool {
    println!("Evaluating if {} is even...", n);
    match n % 2 {
        0 => true, // even
        _ => false // odd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(is_even(0), true);
    }

    #[test]
    fn test_positive_numbers() {
        assert_eq!(is_even(1), true);
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
    }

    #[test]
    #[ignore]
    fn test_negative_numbers() {
        assert_eq!(is_even(-1), false);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }
}
