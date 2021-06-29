use std::io::{self, Write};

fn main() {
    println!("Welcome to the Fahrenheit Converter!");

    loop {
    
        // String to store input value
        let mut input = String::new();
    
        // Read input from cli
        // Flush output buffer to output before new line
        print!("Please enter the number you'd like to convert or enter q to quit: ");
        io::stdout()
            .flush()
            .expect("Could not flush the output buffer!");
        
        // Read in number
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line from terminal!");

        if input.trim() == "q" || input.trim() == "Q" {
            break;
        }

        let output: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Could not parse number!");
                continue;
            }
        };

        // Perform conversion
        let output = (output - 32.0) * (5.0 / 9.0);

        // Output result
        println!("{}°F == {:.2}°C", input.trim(), output);

    }
}
