// To understand modules, lets define an API for a restaurant.
// Not one used by a real restaurant, one that actually defines the 
// activities of a restaurant.
// These activities can be split into front of house and back of house.

// This file, lib.rs defines the contents of what is called the crate root.
// It is given this name as this file defines a module called crate.
// The files here could be accessed from another binary crate within
// package by using the absolute path crate::front_of_house::hosting.

// Lets write the front of house activities module.
// We can split this module down into two separate modules to further
// Categorise what's going on.  
#[allow(dead_code)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}
