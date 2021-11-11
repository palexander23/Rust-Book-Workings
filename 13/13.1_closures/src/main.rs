// 13.1 Closures

// Some times we want to include function-like behavior in our code without defining
// a specific interface that is accessible to wider parts of the code.
// We may also want to have flexibility in a small piece of functionality within a larger
// system, for instance the comparison function used to sort a list of items.

// We can do both of these things using closures, anonymous functions that can be assigned
// to variables and passed to functions.
// Closures capture the state of the scope they are defined in.

// Closures are defined using the syntax seen below.
// Any variables used are defined between the vertical pipes and one or more statements
// (often in curly brackets) follow that.
// Closures can then be called like functions so long as they're within scope.

fn basic_closure_example() {
    let increment = |x| x + 1;
    println!("Increment closure example: {} -> {}", 1, increment(1));
}

// Closures capture the state of the scope they were created in so functions can be used
// to generate application-specific closures at run-time.
// These will often perform just as fast as an equivalent system using functions.
// 
// There are a few useful bits of syntax here.
// Adding impl to the return tells the compiler that you will be returning an object that 
// implements the behavior specified. Every instance of a closure has a distinct type in 
// Rust so we can't tell the compiler the exact type a closure creator will return, 
// only what that type will do. 
// 
// The move command on the actual return statement tells the compiler that the closure
// needs to take ownership of the variables m and c. 
// This ensures that m and c live at least as long as the closure.
fn get_straight_line_function(m: f32, c: f32) -> impl Fn(f32) -> f32 {
    return move |x| m * x + c;
}

fn main() {
    basic_closure_example();

    // Straight line example
    let line1 = get_straight_line_function(5.0, 2.0);
    let line2 = get_straight_line_function(1.0, -2.0);

    println!("line1(1): {}", line1(1.0));
    println!("line2(1): {}", line2(1.0));

}
