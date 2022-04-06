fn paths() {
    // To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.
    // If we want to call a function, we need to know its path.
    //
    // A path can take two forms:
    //
    // - An absolute path starts from a crate root by using a crate name or a literal crate.
    // - A relative path starts from the current module and uses self, super, or an identifier in the current module.
    // Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

    // How do we call the add_to_waitlist function?
    // This is the same as asking, what’s the path of the add_to_waitlist function?
    // we simplified our code a bit by removing some of the modules and functions.
    // We’ll show two ways to call the add_to_waitlist function from a new function eat_at_restaurant defined in the crate root.
    // The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword
    // you can see the code in the src/libs.rs

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }

    // if we compile the code we will get an error: module `hosting` is private
    // The error messages say that module hosting is private.
    // In other words, we have the correct paths for the hosting module and the add_to_waitlist function,
    // but Rust won’t let us use them because it doesn’t have access to the private sections.

    // we should expose fn with pub keyword (see in src/libs.rs)

    // We can also construct relative paths that begin in the parent module by using super at the start of the path.
    // This is like starting a filesystem path with the .. syntax. Why would we want to do this?
    // Consider the code that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer.
    // The function fix_incorrect_order calls the function serve_order by specifying the path to serve_order starting with super:
    fn serve_order() {}

    mod back_of_house {
        // The fix_incorrect_order function is in the back_of_house module,
        // so we can use super to go to the parent module of back_of_house, which in this case is crate, the root.
        // From there, we look for serve_order and find it. Success!
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        fn cook_order() {}
    }

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
