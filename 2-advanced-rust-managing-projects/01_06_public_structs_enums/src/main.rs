fn main() {
    let rect = shape::Rectangle::new(1.2, 3.4);
    println!("rect area is {}", rect.get_area());
    println!("rect width is {}", rect.width);
}

mod shape {
    pub struct Rectangle {
        pub width: f64,
        height: f64
    }

    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height
            }
        }

        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }
}