fn main() {
    // Lets define a string and then take slices of it!
    let s = String::from("Hello, World");
    
    // Take slices of the words.
    // The starting index is included, the ending index is not
    let hello = &s[0..5];       
    let world = &s[7..12];
    println!("{} {}", hello, world);

    // If the strings include the first of last bytes, they can be omitted.
    let hello = &s[..5];
    let world = &s[7..];
    println!("{} {}", hello, world);

    println!("{}", first_word_demo(&s));
    
    // We can also do this with arrays as follows:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    
    assert_eq!(slice, &[2, 3]);
}

// Takes a string and returns a slice containing its first word.
// The reason this works is that the return value is really just a funny sort of 
// reference to the string still owned in the scope above passed as s.
// We might think that having s as a String would be better but by making it a &str
// We can use it for both string literals (defined as &str) and Strings
// (passed as slices of the whole string).
fn first_word_demo(s: &str) -> &str {
    
    // Convert string to an array of bytes
    let bytes = s.as_bytes();

    // Loop through those bytes looking for the first space.
    for (i, &item) in bytes.iter().enumerate() {
        
        // If a space is found the slice from 0 to the index of the space is returned
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found the whole word is returned.
    // Default return value has to be present otherwise the function is upset
    return &s[..]
} 
