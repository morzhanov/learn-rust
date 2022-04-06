fn modules() {
    // Modules let us organize code within a crate into groups for readability and easy reuse.
    // Modules also control the privacy of items, which is whether an item can be used by outside code (public)
    // or is an internal implementation detail and not available for outside use (private).

    // in src/libs.rs we define module for restaurant

    // Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots.
    // The reason for their name is that the contents of either of these two files form a module named crate
    // at the root of the crate’s module structure, known as the module tree.

    /*
       crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
    */

    // This tree shows how some of the modules nest inside one another (for example, hosting nests inside front_of_house).
    // The tree also shows that some modules are siblings to each other, meaning they’re defined in the same module
    // (hosting and serving are defined within front_of_house).
    // To continue the family metaphor, if module A is contained inside module B,
    // we say that module A is the child of module B and that module B is the parent of module A.
    // Notice that the entire module tree is rooted under the implicit module named crate.
}
