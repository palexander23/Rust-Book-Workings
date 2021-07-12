// Program to calculate the area of a rectangle

// Struct to store rectangle dimensions in pixels
// Add the annotation #[derive(Debug)] to automatically generate debugging 
// formatting for printing this struct.
#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32,
}

// Define methods for Rect struct
// Methods borrow the Rect object using the &self argument
impl Rect {

    // Define a constructor for a Rect object
    // Uses the struct init syntax in a useful way
    // Maybe I am coming around to the Rust function return syntax a bit
    fn build(width: i32, height: i32) -> Rect {
        Rect {
            width,
            height,
        }
    }

    // Define a function to calculate the area of a Rect object
    // Takes the rect as a reference
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    // Define a function to print the details of a Rect object
    // Includes call to area to print area
    fn print_details(&self) {
        println!(
            "Width: {}, Height: {}, Area: {}", 
            self.width, 
            self.height, 
            self.area(),
        );
    }

    // Define a function to determine whether the called Rect can
    // fit inside another given as a method argument.
    fn can_hold(&self, other_rect: &Rect) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}


fn main() {
    // Define a rectangle and print its details
    let rect1 = Rect::build(3, 5); 
    rect1.print_details();

    // Define a second rectangle, edit it, and print its details
    // Use println! with the debugging format specifier for this 
    // intermediary step.
    let mut rect2 = Rect::build(4, 7);
    println!("rect2: {:?}", rect2);

    rect2.height = 5;
    rect2.print_details();

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}