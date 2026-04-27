fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("the sum is {}", sum);
}

fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn old_main() {
    say_hello();
    say_hello();
    let x = 9;
    let y = 4;
    say_the_sum(x, y);
    say_a_number(x as i32);

    let result = square(9);
    println!("the result is {:?}", result);
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    return (1.8 * c) + 32.0;
}

fn main() {
    let c_temp = 23.0;
    let f_temp = celsius_to_fahrenheit(c_temp);
    assert_eq!(f_temp, 73.4);
    println!("Test Passed!");
}
