// Here we have separated our argument parsing logic from our main function.
// We are storing both useful arguments within a struct to keep them from
// getting separated.

pub struct Config {
    pub query: String,
    pub filepath: String,
}

// We can create a Config struct using a constructor as seen below.
// The user may not give the correct number of arguments but rather than
// panicking the program we can return a result for the main function to
// deal with later.

use std::fs;

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}

// To make the main function more manageable we will split off all logic that
// isn't to do with retrieving the arguments or error handling.
// Main can't be tested so its important to keep it nice and small so it can be
// verified by inspection.

use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read in the contents of the file
    let contents = fs::read_to_string(config.filepath)?;

    println!("File contents:\n{}", contents);

    Ok(())
}
