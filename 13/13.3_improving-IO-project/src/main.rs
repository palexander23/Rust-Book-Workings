// In this chapter we will be creating a version of the grep command line utility.
// The folder for each section will contain the same program in different states.
// The final section will contain the completed program.

// A fundamental part of grep is its ability to read arguments from the cli.
// This section will focus on this functionality.
// There are arguments on cargo.io that could help with this but we shall
// write our own for this tutorial.
// Arguments are retrieved using the standard lib function
// std::env::args. This function returns an iterator containing all arguments.

use std::env;

// Another fundamental part of grep is reading files.
// This is completed using another part of the standard library called std::fs
// The function fs::read_to_string takes the name of a file and returns the
// contents of the file as a string.

use std::process;

use minigrep::{Config, run};

/// NEW TEXT
/// To make the program more Rustly, we can remove some of the clone calls we've had 
/// to use previously.
/// env::args returns an iterator! We can use this directly so we don't have to do 
/// lots of borrowing of string slices or anything silly like that. 
/// The only edit we've made to the main function is to remove the call to 
/// collect on args to leave it as an iterator make is a direct pass rather 
/// than a pass to a reference.

fn main() {
    // Get string query and filename
    let arg_vec= env::args();

    let config = Config::new(arg_vec).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Checking for {} in {}", config.query, config.filepath);

    // Here we use if let rather than unwrap_or_else to handle errors as
    // run doesn't return any values that we want to unwrap.
    // What this says is if the return of run(config) is in the format of
    // Err(e) run the following code with e as an argument.
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
