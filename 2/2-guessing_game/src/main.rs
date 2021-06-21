use std::io;                                    // This is how stdio is imported.
                                                // It is not needed for the println! macro
                                                // We have imported the io *thing* from std


use rand::Rng;                                  // Random number generator

use std::cmp::Ordering;                         // Enum: Less Greater, Equal
                                                // This is put out by the String.cmp() method
                                                // The result of the String.cmp() can be detected with a match statement                           

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    let mut guess = String::new();              // let creates variable, mut specifies that it is mutable
                                                // String is an extendable, UTF-8 encoded string
                                                // new is an associate function, working on the type, rather than an object (static function)

    loop {
    
        println!("Please input your guess below:");     

        guess.clear();                              // Empty string of current guess

        io::stdin()                                 // The stdin() function returns an instance of std::io::Stdin which is a handle for io
            .read_line(&mut guess)                  // Call the read line method, appends terminal line to given string. & is reference, mut is given to make the reference mutable
            .expect("Failed to read line");         // read_line() returns an io::Result object.
                                                    // If this object is contains an err object, crash the program and print the given strin
                                                   
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse number!");
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}