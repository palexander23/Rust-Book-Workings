fn main() {
    // Consider a situation where we only care about one match pattern.
    // This can be expressed as seen below    
    let some_u8_value = Some(3u8); // Appending u8 to a number gives type.

    println!("Clunky, verbose single pattern match:");
    match some_u8_value {
        Some(3) => println!("Matched!"),
        _ => println!("Not matched!"),
    }

    // We can also use the if let phrase, which is syntactic sugar for 
    // the code written above.
    // As you might have guessed we can use an else to give the unmatched case. 
    let some_u8_value = Some(2u8);

    println!("\nIf Let syntax:");
    if let Some(3) = some_u8_value {
        println!("Matched");
    } else {
        println!("Not Matched!");
    }

}
