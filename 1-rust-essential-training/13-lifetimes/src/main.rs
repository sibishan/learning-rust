struct Shuttle<'a> {
    name: &'a str
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        return msg;
    }
}

fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y .len() {
        return x;
    }
    return x;
}

fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant);
    }
    // println!("propellant is {}", propellant); // will not work

    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 =String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    
    println!("result is {}", result);

    let vehicle = Shuttle {
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);

}
