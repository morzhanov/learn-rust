pub fn slices() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.

    // A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // slice is a struct with fields:
    //  ptr - pointer to the data in the heap
    //  len - slice length

    // With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods.
    // In other words, these are equal:
    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number.
    // That means these are equal:
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So these are equal:
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];

    // String Literals Are Slices
    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.
    let _s = "Hello, world!";

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    // Just as we might want to refer to a part of a string, we might want to refer to part of an array.
    // We’d do so like this:
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// old first_word function which only could return a first word's width
fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// With all this information in mind, let’s rewrite first_word to return a slice.
// The type that signifies “string slice” is written as &str:
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
