fn old_main() {
    // if statements
    let x = 3;

    if x + 1 != 3 {
        println!("x is NOT 3!");
    }

    let y = 5;

    if x > y {
        println!("x is greater than y!");
    } else if x < y {
        println!("x is less than y!");
    } else {
        println!("x and y are equal!");
    }

    let make_z_odd = true;
    let z = if make_z_odd { 5 } else { 6 };
    // if make_z_odd {
    //     z = 5;
    // } else {
    //     z = 6;
    // }
    println!("z is {}", z);

    // loops
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };
    println!("Done with loop!");
    println!("result is {}", result);

    // while loops
    let mut count = 0;
    let letters = ['a', 'b', 'c'];
    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;

    }

    // for loops
    let message = ['h', 'e', 'l', 'l', 'o'];
    for (idx, &item) in message.iter().enumerate() {
        println!("item {} is {}", idx, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }

    // nested loops
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!()
    }
}

fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    let mut sum = 0;
    for &num in numbers.iter() {
        
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }

        sum += num;
        mean = sum as f64 / numbers.len() as f64;
    }

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests Passed!");

}
