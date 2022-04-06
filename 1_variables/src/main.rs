fn main() {
    // variables by default are immutable
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; Cannot assign twice to immutable variable [E0384]

    // create mutable variable
    let mut x = 5;
    println!("Changing x variable with mut. The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const variable cannot be redeclared and cannot be mutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The const value is: {}", THREE_HOURS_IN_SECONDS);

    // variable shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // x only shadowed in inner block, here we are using old value of x
    println!("The value of x is: {}", x);

    // we can shadow variable and change it's type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("there are {} spaces", spaces);

    // we cannot change variable type if it's declared with mut and no let provided at the start of
    // the second declaration
    // let mut spaces = "   ";
    // spaces = spaces.len();       //  error

    // this would work, but we will get runtime warning that spaces variable should not be declared with mut
    let mut spaces = "   ";
    let spaces = spaces.len(); //  error
    println!("there are {} spaces", spaces);
}
