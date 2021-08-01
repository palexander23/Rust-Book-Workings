// This is the only file that can see into the directory front_of_house.
// That means we must bring all the modules defined in there that we need
// into scope here so other files in this directory, such as lib.rs, can
// import them from here.

pub mod hosting;
