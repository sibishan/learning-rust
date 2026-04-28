fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    
    return &s;
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    return new_fuel;
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("Processing {} fuel", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    return length;
}

fn old_main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("Rocket fuel is: {} and length is {}", rocket_fuel, length);

    let rocket_fuel = produce_fuel();
    println!("Rocket fuel is: {}", rocket_fuel);

    // slice
    let message = String::from("Greetings from earth!");
    println!("Message: {}", message);

    let last_word = &message[15..];
    println!("Last word: {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[0..4];
    println!("Inner planets: {:?}", inner_planets);

    let first_word = get_first_word(&message);
    println!("First word: {}", first_word);
}

fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    for (idx, char) in s.chars().enumerate() {
        if char != ' ' {
            start = idx;
            break;
        }
    }

    let mut end = 0;
    for (idx, char) in s.chars().rev().enumerate() {
        if char != ' ' {
            end = s.len() - idx;
            break;
        }
    }

    return &s[start..end];
}

fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = "       There is space in front.";
    assert_eq!(trim_spaces(&test2), "There is space in front.");

    let test3 = "There is space at the end.       ";
    assert_eq!(trim_spaces(&test3[..]), "There is space at the end.");

    let test4 = "       There is space at both ends.       ";
    assert_eq!(trim_spaces(test4), "There is space at both ends.");

    let test5 = "        ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests Passed!")
}
