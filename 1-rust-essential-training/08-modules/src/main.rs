use std::io;
use rand::prelude::*;

fn old_main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);

    let mut rng = rand::rng();

    let random_num: f64 = rng.random();
    println!("number is {}", random_num);

    let random_num = rng.random_range(1..11);
    println!("number is {}", random_num);

}

fn main() {
    let mut rng = rand::rng();

    let random_num = rng.random_range(1..101);

    let mut buffer = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut buffer);
    let mut num: i32 = buffer.trim().parse().unwrap();
    
    while true {
        if num == random_num {
            println!("You guessed it correct! The random number was {}", random_num);
            break;
        }
        if num < random_num {
            println!("Your guess is lower than random number!");
        } else {
            println!("Your guess is higher than random number!");
        }

        buffer.clear();
        println!("Enter a number:");
        io::stdin().read_line(&mut buffer);
        num = buffer.trim().parse().unwrap();
    }
}
