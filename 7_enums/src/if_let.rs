fn if_let() {
    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
    // Consider the program that matches on an Option<u8> value in the config_max variable but only wants to execute code if the value is the Some variant.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Instead, we could write this in a shorter way using if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
    // In this case, the pattern is Some(max), and the max binds to the value inside the Some.
    // We can then use max in the body of the if let block in the same way as we used max in the corresponding match arm.
    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    //
    // In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
}
