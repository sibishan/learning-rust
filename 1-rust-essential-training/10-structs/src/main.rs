#[derive(Debug)]
#[derive(Clone)]

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 10,
            propellant: 0.0
        }
    }
}

struct Color(u8, u8, u8); // RBG
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn old_main() {
    let mut vehicle = Shuttle::new("Endeavour");

    let mut vehicle2 = Shuttle {
        ..vehicle.clone()
    };

    let vehicle_name = vehicle.get_name();

    println!("{}", vehicle_name);
    vehicle.crew_size = 6;

    vehicle.add_fuel(1000.1);
    println!("{:?}", vehicle);
    println!("{:?}", vehicle2);

    let red = Color(255, 0, 0);
    println!("{}, {}, {}", red.0, red.1, red.2);

    let y = get_y(Point(1,1,1));
    println!("y is {}", y);

}

struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Test Passed!");
}
