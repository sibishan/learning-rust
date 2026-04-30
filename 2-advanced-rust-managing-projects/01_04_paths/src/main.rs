fn main() {
    crate::hello::english(); // hello
    crate::hello::spanish(); // hola
    hello::spanish();
    crate::hello::casual::english(); // hey
}

mod greeting {
    fn english() {
        println!("hello");
        spanish();
        casual::english();
    }

    fn spanish() {
        println!("hola");
    }

    mod casual {
        fn english() {
            println!("hey");
            crate::hello::spanish();
            super::spanish();
        }
    }
}