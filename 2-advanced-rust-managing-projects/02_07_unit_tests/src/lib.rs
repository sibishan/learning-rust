/*****************************
 * Library of Math Functions *
 *****************************/
pub mod add_ops; // code in add_ops.rs
pub mod mul_ops; // code in mul_ops.rs

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ut_add_and_mul_ops() { // test involving both modules
		let x = add_ops::add_two(3.14159, 2.71828); // 5.85987
		let y = mul_ops::mul_three(3.0, 4.0, 5.0); // 30.0
		let result = add_ops::add_three(x, y, 123.4); // 159.25987
		assert_eq!(result, 189.25987);
	}
}