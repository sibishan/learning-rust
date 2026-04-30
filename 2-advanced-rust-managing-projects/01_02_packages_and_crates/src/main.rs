/***************************
 * my_project binary crate *
 ***************************/

mod some_module;
use my_project; // library crate for my_project package

fn main() {
    println!("Running the my_project executable.");
    some_module::mod_func();
    my_project::lib_func();
}
