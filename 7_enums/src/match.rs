fn match_example() {
    // match allows you to compare a value against a series of patterns and then execute code based on which pattern matches
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // we can use default option to handle default item
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: _,
    // which is a special pattern that matches any value and does not bind to that value.
    // This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {}
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // arm has two parts: a pattern and some code.
        // The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run.
        // The code in this case is just the value 1. Each arm is separated from the next with a comma.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
// This is how we can extract values out of enum variants.
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Let’s imagine that a friend is trying to collect all 50 state quarters.
// While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so if it’s one our friend doesn’t have,
// they can add it to their collection.
//
// In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter.
// When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state. Then we can use state in the code for that arm, like so:
fn value_in_cents_with_quarter(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// There’s one other aspect of match we need to discuss.
// Consider this version of our plus_one function that has a bug and won’t compile:
fn plus_one_without_catch(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // We didn’t handle the None case, so this code will cause a bug.
        // Luckily, it’s a bug Rust knows how to catch. If we try to compile this code, we’ll get this error:
        //      non-exhaustive patterns: `None` not covered
    }
}
