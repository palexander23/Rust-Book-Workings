fn main() {
    println!("Data type examples:"); 
    
    println!("Integer types:");
    let int32: i32 = -500_000;                  // 32-bit signed integer
    let uint32: i32 = 0xff;                       // 32-bit unsigned integer    

    let int128: i128 = 10000000000000000000;    // 128-bit signed integer
    let uint8: u8 = 0b0000_1000;                // 8-bit unsigned integer
    
    println!("{} {} {} {}", int32, uint32, int128, uint8);

    println!("Float types:");
    let float32: f32 = 200.234234234234;
    let float64: f64 = 200.234234234234;

    println!("{} {}", float32, float64);

    println!("Boolean Types:");
    let t = true;
    let f: bool = false;
    
    println!("{} {}", t, f);

    println!("Character Types:");
    let c = 'z';
    let z: char = '😻';                             // All chars are 4-byte unicode things

    println!("{} {}", c, z);

    println!("Tuples:");

    let tup: (i32, u8, char) = (34, 255, 'b');
    let (x,y,z) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("{} {} {}  {} {} {}", x, y, z, a, b, c);

    println!("Arrays");

    let x: [i32; 5] = [5, 4, 3, 2, 1];                  // Fixed length array of 5 type i32 elements
    for i in x {print!("{} ", i);}
    println!("");

    let x = [0; 5];                                     // Fixed length array initialized to all zeros
    for i in x {print!("{} ", i);}
    println!("");
}
