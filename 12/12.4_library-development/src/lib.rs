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

fn search<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut lines_found: Vec<&str> = vec![];

    let file_lines = file_content.lines(); 

    for line in file_lines {
        if line.contains(query) {
            lines_found.push(line);
        }
    }

    lines_found
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

        let test_config = Config::new(&test_arg_vector).unwrap();

        assert_eq!(test_config.query, test_arg_vector[1]);
        assert_eq!(test_config.filepath, test_arg_vector[2]);
    }

    #[test]
    #[should_panic]
    fn test_error_on_too_few_args() {
        let test_arg_vector = vec!["arg0".to_string(), "arg1".to_string()];
        let _config_result = Config::new(&test_arg_vector).unwrap();
    }
}
