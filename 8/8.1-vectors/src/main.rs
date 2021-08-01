// In this chapter we study a few useful data structures defined called collections. 
// Collections contain multiple values stored on the heap, not the stack.
// This means these data structures can grow and shrink at run-time.

// In this section we'll look at the vector, a structure for storing multiple values
// of the same type next to each other.
fn main() {
    // Lets start by creating an empty vector.
    // Note we need to define the new object's type if we're just using the new method
    let _v: Vec<i32> = Vec::new(); 

    // If we're defining initial values, we can use a macro instead.
    // In both cases the type is obvious so we don't need to specify.
    let _v = vec![1, 2, 3, 4];
    let _v8 = vec![1u8, 2u8, 123u8];

    // We can add values to a vector using the push method.
    // In this case, the first push call tells Rust the type needed for the vector.
    let mut v = Vec::new();

    v.push(5); 
    v.push(6);
    v.push(7);

    // We can get and set the element of a vector with standard v[n] notation. 
    println!("Vector is {:?}", v);
    println!("1st element (out of 0) is {:?}", v[1]);
    v[1] = 8;
    println!("1st element (out of 0) is {:?}", v[1]);

    // We can also generate references to individual elements
    // Below are two ways of doing that
    let v = vec![1, 2, 3, 4, 5];

    // This option will crash your program if the element doesn't exist.
    let mut third: &i32 = &v[3];

    // This option lets you do decide what to do if the element doesn't exist.
    match v.get(3) {
        Some(_third) => third = _third,
        None => print!("Panic!"),
    }

    println!("Vector is {:?}", v);
    println!("3rd element (out of 0) is {:?}", third);

    
    // We can iterate through the vector as if it were a list
    for i in &v {
        println!("{}", i);
    }

    let mut v = v;
    for i in &mut v {
        *i += 50;
    }


    // If we need to we can have vectors with multiple types by defining an enum
    // with multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(232),
        SpreadsheetCell::Float(4.34),
        SpreadsheetCell::Text(String::from("Boop")),
    ];
}
