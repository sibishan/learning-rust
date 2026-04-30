use std::env;
use std::fs;
use std::io::prelude::*;

fn old_main() {
    // command-line args
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (idx, arg) in env::args().enumerate() {
        println!("argument {} is {}", idx, arg);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);

    // reading files
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);

    // writing files
    let mut speech = String::new();
    speech.push_str("We choose to got to the Moon in this decade\n");
    speech.push_str("We choose to got to the Moon in this decade\n");
    speech.push_str("We choose to got to the Moon in this decade\n");
    speech.push_str("We choose to got to the Moon in this decade\n");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("planets.txt").unwrap();
    file.write(b"\nPluto");

}

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires 2 arguments: <file path> <search name>");
        return;
    }

    let arg1 = env::args().nth(1).unwrap();
    let arg2 = env::args().nth(2).unwrap();

    let contents = fs::read_to_string(arg1).unwrap();
    for line in contents.lines() {
        if line == arg2 {
            println!("{} is on the file", arg2);
            return;
        }
    }
    println!("{} is not on the file", arg2);

}