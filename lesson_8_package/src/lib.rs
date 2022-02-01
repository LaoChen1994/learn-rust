pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to wait list");
            self::seat_at_table();
        }
        fn seat_at_table() {
            println!("seat at table");
        }
    }

    mod serving {
        fn take_order() {
            println!("take order");
            super::hosting::add_to_waitlist();
        }
        fn serve_order() {
            println!("serve order");
        }
        fn take_payment() {
            println!("take payment");
        }
    }
}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                // 这个seasonal_fruit是私有变量
                seasonal_fruit: String::from("Peach"),
            }
        }

        pub fn tell_fruit_food(&self) {
            println!("fruit is -> {}", self.seasonal_fruit);
        }
    }
}

pub fn eat_at_restaurant() {
    use front_of_house::hosting;
    use back_of_house::Breakfast;
    use back_of_house::Appetizer;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let mut appetizer = Appetizer::Soup;

    println!("I'd like {} toast, please", meal.toast);
}