use math_funcs::*;
use rand::prelude::*;
mod test_utils;

#[test]
fn it_add_rand() {
    let mut rng = thread_rng();
	for i in 0..1000 { // test with 1000 sets of random numbers
		let x = rng.gen_range(0..100);
		let y = rng.gen_range(0..100);
		let z = rng.gen_range(0..100);
		assert_eq!(add_ops::add_two(x, y), x + y);
		assert_eq!(add_ops::add_three(x, y, z), x + y + z);
		test_utils::print_test();
	}
}

#[test]
fn it_mul_rand() {
    let mut rng = thread_rng();
	for i in 0..1000 { // test with 1000 sets of random numbers
		let x = rng.gen_range(0.0..100.0);
		let y = rng.gen_range(0.0..100.0);
		let z = rng.gen_range(0.0..100.0);
		assert_eq!(mul_ops::mul_two(x, y), x * y);
		assert_eq!(mul_ops::mul_three(x, y, z), x * y * z);
		test_utils::print_test();
	}
}
