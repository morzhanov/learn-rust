fn option() {
    // The Option type encodes the very common scenario in which a value could be something or it could be nothing
    enum Option<T> {
        None,
        Some(T),
    }

    // The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // In short, because Option<T> and T (where T can be any type) are different types,
    // the compiler won’t let us use an Option<T> value as if it were definitely a valid value.
    // For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; // cannot add `Option<i8>` to `i8`

    // you have to convert an Option<T> to a T before you can perform T operations with it.
    // The match expression is a control flow construct that does just this when used with enums:
    // it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
    let y = match y {
        None => None,
        Some(i) => Some(i),
    };
    let sum = x + y; // works now
}
