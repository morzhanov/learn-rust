fn main() {
    println!("Hello, world!");

    another_function();
    with_params(2);
    print_labeled_measurement(2, 'A');

    // `Statements` are instructions that perform some action and do not return a value
    //
    // `Expressions` evaluate to a resulting value

    // Statements do not return values.
    // Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    // let x = (let y = 6);     expected expression, found statement (`let`)

    // Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
    // Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
    // this expression is a block that, in this case, evaluates to 4:
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(x);
    println!("The value of x is: {}", x);
}

// rust code uses snake case as the conventional style for function and variable names
// in which all letters are lowercase and underscores separate words
// here’s a program that contains an example function definition
fn another_function() {
    println!("Another function.");
}

// params
fn with_params(x: i32) {
    println!("The value of x is: {}", x);
}

// multiple params
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// functions can return values to the code that calls them
// we don’t name return values, but we must declare their type after an arrow (->)
// in Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
// you can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}

// if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error
// fn plus_one_statement(x: i32) -> i32 {
//     x + 1; // error: mismatched types
// }
