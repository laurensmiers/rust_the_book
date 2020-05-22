use std::collections::*;
use std::{self, cmp::Ordering, io};

mod front_of_house;

pub use crate::front_of_house::hosting;
// pub use front_of_house::hosting;

mod back_of_house;

use crate::back_of_house::cooking::Breakfast as boh_breakfast;
use back_of_house::cooking;

mod util;

pub fn eat_at_restaurant() {
    // // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    front_of_house::hosting::add_to_waitlist();

    // // Making use of the 'use' keyword
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = boh_breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = cooking::Appetizer::Soup;
    let order2 = cooking::Appetizer::Salad;

    util::get_cutlery();
}

fn serve_order() {}
