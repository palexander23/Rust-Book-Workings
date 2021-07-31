// Rust uses paths to resolve where the item we want is within the module tree.
// These paths can be absolute, starting with the crate name, or relative,
// using a combination of self and super to reference items within itself or
// its parent respectively.

// Lets define a simplified restaurant function and show how we can access
// the functions within. 
// Simply put, adding pub to anything makes it visible from the parent module.
#[allow(dead_code)]
mod front_of_house {
    
    // The pub on both the module and the function make add_to_waitlist
    // visible from back_of_house.
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::waitlist_handling::incr_waitlist()
        }
    }

    // The pub on the function but not the mod makes the function only 
    // visible from inside the front_of_house module.
    mod waitlist_handling {
        pub fn incr_waitlist() {}
    }
    
    // The pub on this module makes it visible from outside this module.
    pub fn get_waitlist() {}
    
}

// This second module can access the items labelled public within
// front_of_house but not those labelled public any further down.
#[allow(dead_code)]
mod waiter {
    pub mod greeting {
        pub fn wait() {
            
            // Succeeds
            crate::front_of_house::get_waitlist();
            
            // Fails
            // crate::front_of_house::waitlist_handling::incr_waitlist()
        }
    }
}

// Lets say we want a public API that calls the private one defined above.
// We can give that the pub keyword to make it so.
// This function is technically part of the module crate, so crate can be 
// swapped for self with no effect on behaviour.
pub fn eat_at_restaurant() {
    // We can reference the add_to_waitlist function using either styles of
    // paths.

    // Absolute:
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative:
    self::front_of_house::hosting::add_to_waitlist();
}


// We can also make structs and enums public using the pub keyword but
// there are a few more tricky bits to think about.
// Lets define a back of house module which contains a struct called
// breakfast denoting the item people can order for breakfast.
#[allow(dead_code)]
mod back_of_house {

    // Here the struct is defined with only one of its items public.
    // We have to remember that pub gives access to the module above,
    // not just the indentation above. 
    // Inside back_of_house, both fields are accessible.
    // Outside, both the struct and the toast field are accessible,
    // but not the seasonal_fruit field, not even to read.
    // This means all future handling of Breakfast that includes the 
    // seasonal_fruit field must happen within the back_of_house module.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {

        // Note that Breakfast has a private field so it can't be 
        // constructed outside of the back_of_house class, 
        // hence the need for a constructor.
        pub fn generate_summer_breakfast(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
        
        // Send your meal back and swap the fruit for banana
        pub fn swap_fruit_for_banana(mut breakfast: Breakfast) {
            breakfast.seasonal_fruit = String::from("banana");
        } 
    }

    // Here we have a public enum
    // Note that if the enum is public then all of its variants are also 
    // public, this makes sense for enums but not for structs.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn order_breakfast() {
    // Order a meal with Rye toast but give yourself the option to change
    let mut meal = back_of_house::Breakfast::generate_summer_breakfast("Rhy");

    // Change your mind and pick up different toast from the buffet
    meal.toast = String::from("Wholewheat");

    // Ask the waiter to swap your fruit for the alternative option
    back_of_house::Breakfast::swap_fruit_for_banana(meal);
}
