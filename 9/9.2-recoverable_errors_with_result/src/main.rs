// Some errors should be handled by the program without crashing.
// For instance attempting to open a file that doesn't exist could
// be followed by creating that file.

// For this kind of decision making we can use the Result enum.
// This enum defines variance Ok(T) and Err(E) where T, e are 
// generic types that we can set to whatever we want.
// Many predefined functions in Rust return Result objects.
// We can use the match syntax to handle each case.

// Lets demo this functionality with a file operation
// This is a basic example which just panics if the file doesn't exist
use std::fs::File;

fn _basic_file_handling_demo() {
   let file_result = File::open("hello.txt");

   let _f = match file_result {
       Ok(file_handle) => file_handle,
       Err(error) => panic!("Problem opening the file: {:?}", error),
   };
}

// We can then make a more advanced version which changes behaviour 
// depending on what kind of error occurs.
// Here we've added an inner match expression which checks the 
// type of the error and does stuff accordingly.
// Notice how the file handle returned in the inner match ends up 
// in the top _file_handle variable as the result of the inner 
// match is then returned by the outer one. 
// Note that there are more concise ways of doing this without 
// match statements called closures that are discussed in Chapter 13.
use std::io::ErrorKind;

fn error_type_demo() {
    let file_result = File::open("hello.txt");

    let _file_handle = match file_result {
        Ok(file_handle) => file_handle,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file_handle) => file_handle,
                Err(error) => panic!("Problem creating file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        }
    };
}


// We can make our code more concise by using some of the helper functions 
// defined for Result. One such function is unwrap()
// This function "unwraps" the Result to get to the useful return value inside.
// If it isn't an Ok variant then the panic! macro is called.
// This will give us behaviour just like the match expression above.
// We can also use the expect method which does the same but allows
// the programmer to define the panic message as well.
fn unwrap_expect_demo() {
    // This version does not allow a specific error message 
    let  _f = File::open("hello.txt").unwrap();

    // This is more useful and aids debugging
    let _f = File::open("hello.txt").expect("Failed to open hi.txt");
}


// Sometimes we might not be able to work out what to do about an error
// without more context. We might need to return the error.
// This is known as error propagation. 
// We can do this by setting the return type of a function as a Result
// and returning any errors as an Err variant of the Result.
// Lets define a function like this that tries to read a username from
// a file.
// If we want to be able to return an Error type we must first bring 
// it into scope with a call to use.

// Note that the file handle must be defined as mutable if we're going to 
// read or write from it. I expect this is due to the fact that it needs
// to be edited to signify we are using it. 
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // Attempt to open the file
    let file_result = File::open("username.txt");

    // Attempt to open the file, return the Err given on a failure
    let mut file_handle = match file_result {
        Ok(file_handle) => file_handle,
        Err(error) => return Err(error),
    };

    // Define a new string to hold the username 
    let mut username = String::new();

    // Attempt to read the contents of the file into a string
    // On success, return the username string wrapped in a Result
    // On Fail, return the an Err containing the error given
    match file_handle.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }

}


// This practice of propagating errors is so common that Rust provides
// an operator to perform it. Appending ? to the end of a statement 
// returning a Result will return if the Result returned is an Err.
// We can reimplement the above username code in a more concise way.
fn concise_read_username_from_file() -> Result<String, io::Error> {
    let mut file_handle = File::open("username")?;
    let mut username = String::new();
    
    file_handle.read_to_string(&mut username)?;
    return Ok(username);
}


// We can take this conciseness even further by moving everything
// on to one line.
// We've created the string at the beginning of the function and are 
// still returning it at the end.
// However, by chaining method calls after ? operators we can make code
// very concise.
fn conciserer_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}


// Since reading a string from a file is a common operation Rust has a 
// built in function to do it, the fs::read_to_string function.
// This function returns a Result of the kind we've been writing above
// so we can make an even more concise version of the username function.
use std::fs;

fn concisest_read_username_from_file() -> Result<String, io::Error> {
    // The Result returned by the function is passed straight back to the 
    // calling function
    fs::read_to_string("username.txt") 
}


fn main() {
    error_type_demo();
    unwrap_expect_demo();

    let username_result = read_username_from_file();
    println!("Result returned from Username retrieval: {:?}", username_result);
    
    let username_result = concise_read_username_from_file();
    println!("Result returned from Username retrieval: {:?}", username_result);

    let username_result = conciserer_read_username_from_file();
    println!("Result returned from Username retrieval: {:?}", username_result);

    let username_result = concisest_read_username_from_file();
    println!("Result returned from Username retrieval: {:?}", username_result);
}
