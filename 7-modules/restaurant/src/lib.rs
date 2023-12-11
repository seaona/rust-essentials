// we can organize functions into nested modules
mod front_of_house {
    // the contents of the hosting module are still private.
    // Making the module public doesn't make its contents public
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // it's a different scope (mod) so the function won't compile
        //hosting::add_to_waitlist();

        // we can reference the parent module instead
        super::hosting::add_to_waitlist();
    }
}


// because this function is sibling to front_of_house, we can refer to it
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // use path
    hosting::add_to_waitlist();

    // structs - order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // modify meal
    meal.toast = String::from("Wheat");
    // we cannot modify seasonal_fruit because it's not public
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    // enums - public appetizer
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // the parent module of back_of_house is crate (root)
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn funtion1() -> Result {
    //
}

fn function2() -> IoResult<()> {
    //
}

// group nested paths
use std::cmp::Ordering;
use std::io;
// instead
use std::{cpm::Ordering, io};

// group nested paths 2
use std::io;
use std::io::Write;
// instead
use std::io{self, Write};

// the global operator
use std::collections::*;