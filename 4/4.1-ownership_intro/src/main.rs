
#[allow(unused_variables)]
fn main() {

    // Create two integers, both equal 5
    // y is a new variable referenced to the value of 5, not a copy
    let x = 5;
    let y = x;

    // Now lets create two strings.
    // Rather than using a GC, Rust implements ownership of heap data.
    // When a variable goes out of scope, the heap memory is released.
    // If two variables could share the same pointer this would create double free errors.
    // To prevent this, when a new heap variable is defined as a "copy" of an old one the previous
    // variable dereferenced, effectively 'moving' the pointer. 
    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}", s1) This won't work!
    // But printing s2 will
    println!("{}", s2);

    // If we do want to create copies of the entire heap data (deep copy) then the clone method can be used.
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    // Now both variables are still in scope
    println!("{} {}", s1, s2);

    function_parameter_ownership_demo();
}

#[allow(unused_variables)]
fn function_parameter_ownership_demo() {
    // Bring some stack and heap variables into scope
    let s = String::from("string tring");
    let x = 5;

    take_ownership(s);
    
    // println!("{}", s); Doesn't compile!
    // Ownership of s has been passed to the take_ownership function.
    // When the function completes, the value is dereferenced.

    makes_copy(x);
    println!("{}", x);

    // Create a new variable and demonstrate movement by return.
    // The pointer to the heap assigned to s is transferred to s_new.
    // s is invalidated.
    let s = String::from("wowow");
    let s_new = takes_and_gives_back(s);

    // println!("{}", s); Doesn't compile!
    println!("{}", s_new);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(integer: i32) {
    println!("{}", integer);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string // Return some_string
}
