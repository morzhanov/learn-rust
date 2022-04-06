// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope,
// just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.
use self::front_of_house::hosting;

// Providing New Names with the as Keyword
use std::io::Result as IoResult;

// re-exprorting
// When we bring a name into scope with the use keyword, the name available in the new scope is private.
// To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
pub use crate::front_of_house::hosting;

// public crates could be installed in Cargo.toml:
// rand = "0.8.3"
// then we can use rand crate:
// use rand::Rng;

// standard packages are provided with rust installation and we do not need to list them in Cargo.toml
use std::collections::HashMap;

// we can find crates on https://crates.io/

// we can nest package names in one line:
use std::{cmp::Ordering, io};

// If we want to bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator:
use std::collections::*;

fn use_keyword() {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
