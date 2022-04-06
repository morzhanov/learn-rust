pub fn references() {
    // MAIN RULES
    // - At any given time, you can have either one mutable reference or any number of immutable references.
    // - References must always be valid.

    // A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type.

    let s1 = String::from("hello");
    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *

    // We call the action of creating a reference borrowing.

    // mutable reference
    // First, we change s to be mut. Then we create a mutable reference with &mut s where we call the change function,
    // and update the function signature to accept a mutable reference with some_string: &mut String.
    // This makes it very clear that the change function will mutate the value it borrows.
    let mut s = String::from("hello");
    change(&mut s);
    let reference_to_nothing = no_dangle();
}

// we take &String rather than String.
// These ampersands represent references, and they allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// So what happens if we try to modify something we’re borrowing?
fn change(some_string: &String) {
    // cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    some_string.push_str(", world");
}

// Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.
// This code that attempts to create two mutable references to s will fail:
fn two_mutable_references_will_fail() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", r1, r2);
}

// As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
fn use_two_mutable_references_with_braces() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
fn mutable_reference_error() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, and {}", r1, r2, r3);

    // In languages with pointers, it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory
    // that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
    // In Rust, by contrast, the compiler guarantees that references will never be dangling references:
    // if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

    // let reference_to_nothing = dangle(); // error
}

// We also cannot have a mutable reference while we have an immutable one to the same value.
// Users of an immutable reference don’t expect the value to suddenly change out from under them!
// However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
//
// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
// For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:
fn three_mutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String
//
//     let s = String::from("hello"); // s is a new String
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.
// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
