// #[derive(Debug)]

use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        return (self.width + self.height) * 2
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    return b;
}

fn old_main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u8
    };

    // println!("rect is {:?}", rect);
    println!("width is {}", rect.get_perimeter());

    let biggest = get_biggest(1, 2);
    println!("biggest is {}", biggest);

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 5,
        propellant: 834950.1
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    println!("boxed_vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));

}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    return Box::new(*a + *b);
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);
    
    println!("Tests Passed!")
}
