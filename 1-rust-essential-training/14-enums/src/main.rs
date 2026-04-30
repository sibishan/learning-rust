

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (w + h) * 2.0,
            Shape::Triangle(a, b, c) => a + b + c
        }
    }
}

fn old_main() {
    let my_shape = Shape::Rectangle(2.0, 1.0);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c)
    }

    let number = 4u8;
    let result = match number {
        0 => "zero",
        1 => "one",
        2 => "three",
        _ => {
            println!("{} dis not match", number);
            "something else"
        }
    };
    println!("result is {}", result);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);

    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);

    let number = Some(13);
    if let Some(13) = number {
        println!("thirteen");
    }
}

enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Location is Unknown"),
            Location::Anonymous => println!("Location is Anonymous"),
            Location::Known(x, y) => println!("Location: ({}, {})", x, y)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();

    let address = Location::Anonymous;
    address.display();

    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
