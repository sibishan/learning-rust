fn main() {
    hello::english();
    hello::casual::english();
}

mod hello {
    pub fn english() {
        println!("hello");
        spanish(); // calling a private function in the same module
    }

    fn spanish() {
        println!("hola");
    }

    pub mod casual {
        pub fn english() {
            println!("hey");
            super::spanish(); // calling a private function in the parent module
        }
    }
}