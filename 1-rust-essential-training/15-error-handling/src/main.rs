use std::fs;
use std::io;

fn main() {
    // panic!("AYOOOO");
    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println!("T-minus {}", count);
        // let x = 1 / count; // division by 0 error
    }

    // let contents = fs::read_to_string("the_ult_question.txt"); // file is not there
    let result = fs::read_to_string("the_ult_question.txt");
    let contents = match result {
        Ok(message)=> message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission Denied!"),
            _ => panic!("IDK!")
        }
    };
    println!("contents is {:?}", contents);
}
