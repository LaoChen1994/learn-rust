// main.rs
mod lib;

// use lib::front_of_house;
// use lib::eat_at_restaurant;

pub use lib::{ eat_at_restaurant, front_of_house, back_of_house };

// pub use lib::*;

fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
    let mut appetizer = back_of_house::Breakfast::summer("Brand");
    front_of_house::hosting::add_to_waitlist();
}
