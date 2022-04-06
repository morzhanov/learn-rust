// We define a module by starting with the mod keyword and then specify the name of the module
mod front_of_house {
    // this is private module we will make it public
    // mod hosting {
    //     fn add_to_waitlist() {}
    //
    //     fn seat_at_table() {}
    // }

    mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {} // this is private function
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// we can use structs and libs and make them public
// also we can choose which fields to expose
mod back_of_house {
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

    // In contrast, if we make an enum public, all of its variants are then public.
    // We only need the pub before the enum keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
