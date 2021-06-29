#[allow(unused_variables)]
fn main() {
    // Define a string 
    let s1 = String::from("Hello, World");

    // Rather than passing it whole to a function and letting ownership be moved,
    // we can pass it by reference instead
    let s1_len = calculate_length(&s1);
    println!("String: \"{}\"\tLength: {}", s1, s1_len);

    // We can also define functions which take mutable references
    // However we have to define the variables as mutable 
    // change(&mut s1) doesn't work!
    let mut s2 = String::from("Hello, ");
    change(&mut s2);
    println!("s2: {}", s2);

    dangle_demo();
}

// s is a reference to a String.
// This function does not take ownership of s
// So when the function ends and s goes out of scope nothing happens.
fn calculate_length(s: &String) -> usize {
    // s.push_str(" world world"); // This doesn't work as the reference is not mutable
    s.len()                    
}

fn change(s: &mut String) {
    s.push_str(" world world world");
}

fn dangle_demo() {
    let s = no_dangle();
    println!("No dangle s: {}", s);
}

// The function below will not work as it will produce a dangling pointer
// Rust is nice and stops this becoming an issue at compile time.
// fn dangle() -> &String {
//     s = String::from("ehlle");
//     return &s;
// }

// This function does work and still effectively passes by reference as s is 
// moved to the given return variable and ownership is transferred, meaning just
// the pointer travels. 
fn no_dangle() -> String {
    let s = String::from("Helo");
    return s;
}


