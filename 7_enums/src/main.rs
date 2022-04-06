fn main() {
    // Enums allow you to define a type by enumerating its possible variants.
    enum IpAddrKind {
        V4,
        V6,
    }

    // We can create instances of each of the two variants of IpAddrKind like this:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // However, representing the same concept using just an enum is more concise: rather than an enum inside a struct,
    // we can put data directly into each enum variant.
    // This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
    enum IpAddrUpdated {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
    // Version four type IP addresses will always have four numeric components that will have values between 0 and 255.
    // If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct.
    // Enums handle this case with ease:
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
    // You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddrWithStructs {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // This enum has four variants with different types:
    //
    // - Quit has no data associated with it at all.
    // - Move has named fields like a struct does.
    // - Write includes a single String.
    // - ChangeColor includes three i32 values.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Defining an enum with variants is similar to defining different kinds of struct definitions,
    // except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
    // The following structs could hold the same data that the preceding enum variants hold:
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
                                              // But if we used the different structs, which each have their own type, we couldn’t as easily define
                                              // a function to take any of these kinds of messages as we could with the Message enum

    // There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl,
    // we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

// We can then, for instance, define a function that takes any IpAddrKind:
fn route(ip_kind: IpAddrKind) {}

// using enums with structs
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
