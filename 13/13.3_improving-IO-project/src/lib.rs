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

/// NEW TEXT
/// Rather than iterating through a list of strings and finding its length
/// we can use the next method on the Args iterator to collect each argument
/// in the location that it should be.
/// If a None is returned from that location then we know no argument was
/// given so we can end the program.
/// 
/// To make Config testable I have made it use a generic input for args.
/// This generic type must implement the Iterator type with the 
/// associated type of String.
/// This matches the definition of the Args iterator but still allows
/// the developer to create their own Args-like object with wanting
/// to test Config.

impl Config {
    pub fn new<I>(mut args: I) -> Result<Config, &'static str>
    where I: Iterator<Item = String>
    {
        let _program_path = args.next();

        let query = match args.next() {
           Some(arg) => arg,
           None => return Err("Error, not enough arguments!"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Error, not enough arguments!"),
        };

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
    let file_contents = fs::read_to_string(config.filepath)?;
    
    // Print out results
    for line in search(&config.query, &file_contents) {
        println!("{}", line);
    }

    Ok(())
}


// We will now use test-driven development to add the search functionality to our code.
// We will start by writing the test then add the code which should satisfy it 
// afterwards.

/// NEW TEXT
/// We've replaced the search code with  a single iterator based statement.
/// The lines call is used as an iterator rather than a vector.
/// We then apply a filter to remove lines that don't contain the query string
/// Finally we collect the iterator to a vector so we don't have to change
/// the function interfaces.
fn search<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    file_content
        .lines()
        .filter(|s| s.contains(query))
        .collect()
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let file_content = "\
Rust:
safe, fast, productive.
pick three.";
    

    assert_eq!(vec!["safe, fast, productive."], search(query, file_content));
    }
}


#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_argument_capture() {
        let test_arg_vector=
            vec!["test".to_string(), "arg1".to_string(), "arg2".to_string()];

        let test_args = test_arg_vector.into_iter();
        let test_config = Config::new(test_args).unwrap();

        assert_eq!(test_config.query, "arg1");
        assert_eq!(test_config.filepath, "arg2");
    }

    #[test]
    #[should_panic]
    fn test_error_on_too_few_args() {
        let test_arg_vector = vec!["arg0".to_string(), "arg1".to_string()]; 
        let test_args = test_arg_vector.into_iter();
        
        let _config_result = Config::new(test_args).unwrap();
    }
}
