fn main() {
    // A struct, or structure, is a custom data type that lets you package together
    // and name multiple related values that make up a meaningful group.
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // creating struct instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // assigning a value to struct field
    user1.email = String::from("anotheremail@example.com");

    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    // As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
    let _user = build_user("anotheremail@example.com", "username");
    let user = build_user_shorthand("anotheremail@example.com", "username");

    // we can use .. syntax to define new struct instance with value from another instance
    let user2 = User {
        email: String::from("another@example.com"),
        ..user
    };

    // Rust also supports structs that look similar to tuples, called tuple structs.
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather,
    // they just have the types of the fields.
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples,
    // and when naming each field as in a regular struct would be verbose or redundant.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Note that the black and origin values are different types, because they’re instances of different tuple structs.
    // Each struct you define is its own type, even though the fields within the struct have the same types.
    // For example, a function that takes a parameter of type Color cannot take a Point as an argument,
    // even though both types are made up of three i32 values.
    // Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces,
    // you can use a . followed by the index to access an individual value, and so on.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to ()
    // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value,
// and they contain some code that’s run when the method is called from somewhere else.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
// We can define associated functions that don’t have self as their first parameter (and thus are not methods)
// because they don’t need an instance of the type to work with.
// We’ve already used one function like this: the String::from function that’s defined on the String type.
impl Rectangle {
    // The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
    // Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.
    //
    // We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership,
    // and we just want to read the data in the struct, not write to it.
    // If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Note that we can choose to give a method the same name as one of the struct’s fields.
    // For example, we can define a method on Rectangle also named width:
    fn width(&self) -> bool {
        self.width > 0
    }

    // Often, but not always, when we give methods with the same name as a field we want it to only return the value in the field and do nothing else.
    // Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.
    // Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API.
    fn height(&self) -> bool {
        self.height
    }
}

// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn using_rect() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.
    // This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.
    let res = Rectangle::square(4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// we can use a shorthand to define struct fields from params
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
