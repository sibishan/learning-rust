/*********************************
 * Functions to Multiply Numbers *
 *********************************/
use std::ops::Mul;
use std::fmt::Display;

/* multiply two numbers of the same type */
pub fn mul_two<T: Mul<Output = T> + Display>(a: T, b: T) -> T {
    print_mul(&[&a, &b]);
    a * b
}

/* multiply three numbers of the same type */
pub fn mul_three<T: Mul<Output = T> + Display>(a: T, b: T, c: T) -> T {
    print_mul(&[&a, &b, &c]);
    a * b * c
}

/* print a message displaying operands being multiplied */
fn print_mul<T: Display>(operands: &[T]) -> String {
    let mut message = String::from("Multiplying ");
    if let Some((last, elements)) = operands.split_last() {
        for n in elements {
            message.push_str(&format!("{} * ", n));
        }
        message.push_str(&format!("{}", last)); 
    }
    println!("{}", message);
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_mul_two() {
        assert_eq!(mul_two(1, 2), 2);
        assert_eq!(mul_two(1.2, 3.4), 4.08);
    }

    #[test]
    fn ut_mul_three() {
        assert_eq!(mul_three(1, 2, 3), 6);
        assert_eq!(mul_three(1.2, 3.4, 5.6), 22.848);
    }

    #[test]
    fn ut_print_mul() { // Rust allows testing of private functions
        let message = print_mul(&['x', 'y', 'z']);
        assert_eq!(message, String::from("Multiplying x * y * z"));
    }
}