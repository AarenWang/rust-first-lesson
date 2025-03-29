//extern crate cargo_01;
//use say_bye;  // way 1
use say_bye::cargo_01; // way 2

fn main() {
    println!("Hello, world!");
    //say_bye::cargo_01::say(); // way 1
    cargo_01::say(); // way 2
}
