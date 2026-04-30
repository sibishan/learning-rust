fn main() {
    println!("Hello, world!");
}

mod hello {
    fn english() {
        println!("hello");
    }

    fn spanish() {
        println!("hola");
    }

    mod casual {
        fn english() {
            println!("hey");
        }
    }
}