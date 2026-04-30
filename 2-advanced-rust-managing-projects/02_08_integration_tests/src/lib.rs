/*****************************
 * Library of Math Functions *
 *****************************/
pub mod add_ops; // code in add_ops.rs
pub mod mul_ops; // code in mul_ops.rs

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ut_add_ops() {
		assert_eq!(add_ops::add_two(1, 2), 3);
		assert_eq!(add_ops::add_two(1.2, 3.4), 4.6);
		assert_eq!(add_ops::add_three(1, 2, 3), 6);
		assert_eq!(add_ops::add_three(1.2, 3.4, 5.6), 10.2);
	}
}