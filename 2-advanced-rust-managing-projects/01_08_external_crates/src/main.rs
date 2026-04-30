use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let num = rng.gen::<u32>();
    println!("num is {}", num);
}