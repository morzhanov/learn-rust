use self::references;
use self::slices;

fn main() {
    // ownership rules
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // scope is the range within a program for which an item is valid
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward
        println(_s); // This will print `hello, world!`
                     // println(_s); // This will throw an error as we moved ownership of _s to println
    }

    // String type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time
    // you can create a String from a string literal using the from function, like so
    let _s = String::from("hello");
    println!("{}", _s); // This will print `hello, world!`
    println!("{}", _s); // This will print `hello, world!`

    // double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from
    // this kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
    // This is why string literals are fast and efficient.
    // But these properties only come from the string literal’s immutability.
    // Unfortunately, we can’t put a blob of memory into the binary for each piece of text
    // whose size is unknown at compile time and whose size might change while running the program.

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    //
    // The memory must be requested from the memory allocator at runtime.
    // We need a way of returning this memory to the allocator when we’re done with our String.
    // That first part is done by us: when we call String::from, its implementation requests the memory it needs.

    // in Rust the memory is automatically returned once the variable that owns it goes out of scope
    {
        let _s = String::from("hello"); // s is valid from this point forward
                                        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put the code to return the memory.
    // Rust calls drop automatically at the closing curly bracket.

    // String consists of fields:
    // - len - length of string
    // - ptr - pointer to the heap with string contents
    // - capacity - string capacity

    // move

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    // We do not copy the data on the heap that the pointer refers to.

    // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid.
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // Check out what happens when you try to use s1 after s2 is created; it won’t work:
    let _s1 = String::from("hello");
    let _s2 = _s1;
    // println!("{}, world!", _s1); // error: borrow of moved value: `s1`

    // clone
    // clone copies heap contents of s1 to s2, so s1 is still valid to use
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    // There’s another wrinkle we haven’t talked about yet. This code using integers – part of which was shown in Listing 4-2 – works and is valid:
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    //
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    // In other words, there’s no difference between deep and shallow copying here, so calling clone
    // wouldn’t do anything different from the usual shallow copying and we can leave it out.

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack like integers are.
    // If a type implements the Copy trait, a variable is still valid after assignment to another variable.
    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
    // If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
    // To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.
    //
    // So what types implement the Copy trait? You can check the documentation for the given type to be sure,
    // but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    // Here are some of the types that implement Copy:
    //
    // - All the integer types, such as u32.
    // - The Boolean type, bool, with values true and false.
    // - All the floating point types, such as f64.
    // - The character type, char.
    // - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // ownership and functions
    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
                   // Here, x goes out of scope, then s. But because s's value was moved, nothing
                   // special happens.

    //
    // returning values can also transfer ownership
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let _s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(_s2); // s2 is moved into
                                         // takes_and_gives_back, which also
                                         // moves its return value into s3

    // rust does let us return multiple values using a tuple
    let _s1 = String::from("hello");
    let (_s2, _len) = calculate_length(s1);
    println!("The length of '{}' is {}.", _s2, _len);

    references::references();
    slices::slices();
}

fn println(str: &str) {
    println!("{}", str)
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
