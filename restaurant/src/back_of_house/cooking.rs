pub struct Breakfast {      // Struct is public but its members aren't!
    pub toast: String,	// Customer can choose which toast he wants
    seasonal_fruit: String,	// Only chef can choose fruit that is served
}

pub enum Appetizer {	// Enum is public and its possible variants too!
    Soup,
    Salad,
}

fn do_nothing() {}

fn fix_incorrect_order() {
    cook_order();

    crate::serve_order();
}

fn cook_order() {}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
        }
    }
}
