mod reference_and_borrowing;

fn main() {
    // if-else
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if the condition isn’t a bool, we’ll get an error
    // let number = 3;
    // if number {          // runtime error
    //     println!("number was three");
    // }

    // rust will not automatically try to convert non-Boolean types to a Boolean
    // you must be explicit and always provide if with a Boolean as its condition
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // when we try to compile this code, we’ll get an error
    // the if and else arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program
    // let condition = true;
    // let number = if condition { 5 } else { "six" }; // compilation error
    // println!("The value of number is: {}", number);

    // loops
    // rust has three kinds of loops: loop, while, and for

    // loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop
    loop {
        println!("again!");
        // we need to call break to stop the loop
        break;
    }

    // returning result from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // the value of a while and for loops is always (), the unit type
    // let res = while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // res is () and cannot be evaluated

    // for allows us to iterate over elements in array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // we can run for loop needed times with this expression:
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
