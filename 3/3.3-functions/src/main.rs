
// Example functions
fn main() {
    println!("Hello, world!");
    another_function();
    argument_demo(4, 90);

    println!("{} + 1 == {}", 5, weird_rust_return_demo(5));
    println!("{} - 1 == {}", 5, normal_return_demo(5));
}

fn another_function() {
    println!("Another function");
}

fn argument_demo(x: i32, y: i64) {
    println!("x is {}, y is {}", x, y);
}

fn weird_rust_return_demo(x: i32) -> i32 {
    x + 1   // Note: no semicolon here, this turns a statement into an expression that 
            // outputs a value that can latch onto a variable
}

fn normal_return_demo(x: i32) -> i32 {
    return x - 1;
}