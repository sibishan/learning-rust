fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    return new_fuel;
}

fn main() {
    let planet = "earth";
    if true {
        println!("planet: {}", planet);
        let mut planet = 4;
        println!("planet: {}", planet);
    }
    println!("planet: {}", planet);

    // string
    let mut message = String::from("earth");
    println!("message: {}", message);
    message.push_str(" is home.");
    println!("message: {}", message);

    // ownership
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet: {}", inner_planet);
    }
    println!("outer_planet: {}", outer_planet);

    // transfer ownership
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel: {}", rocket_fuel);
}
