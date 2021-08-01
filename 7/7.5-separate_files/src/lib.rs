// Storing all modules in a single file is not great for code readability.
// In this section we will rewrite the restaurant library across multiple files.

// We have split the front of house module down into a separate file then
// created a directory to house submodules.
// We can gain access to that code from the top level module as follows:

// This line, a mod command without a block, loads the contents of a 
// module represented by the contents of a file with the name given. 
// Basically the contents of front_of_house are imported here.
mod front_of_house;

// We then bring the path to hosting into this top level scope.
// We also make it public, because if this is going to be a big library,
// then our code organisation should not affect how the code is exposed.
// Note that this path would still work if the hosting module was in the 
// front of house file.
// This allows modules to be split into their own directories as they grow.
pub use crate::front_of_house::hosting;

// We can then access the functions in hosting as we have before.
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // This one isn't pub
    // hosting::incr_waitlist();
}
