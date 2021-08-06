// Sometimes we just need to end a program completely when something goes completely wrong.
// For this we have the panic! macro. 
// This macro ends execution, unwinds the stack (cleans up all data on it) and prints
// the string given as an argument to the screen.

// Below is an example of a simple crash.
// It will show the error message and the location in the code it was called.
#[allow(dead_code)]
fn simple_crash() {
    panic!("Crash and burn");
}

// panic! can also be called in code that isn't ours.
// Below is a function that halts the program using a call to panic! in the 
// Rust underlying code.
// We can get a full back trace by setting the terminal environment variable
// RUST_BACKTRACE=1 when we run the program as follows:
// RUST_BACKTRACE=1 cargo run
#[allow(dead_code)]
fn vector_nonsense() {
    let v = vec![1, 2, 3];

    v[99];
}

fn main() {
    vector_nonsense();
}
