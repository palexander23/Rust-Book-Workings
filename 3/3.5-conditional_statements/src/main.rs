fn main() {
    println!("Hello, world!");

    if_else_if_demo(63);
    conditional_let_statement(3, 7);
    loop_return_value_demo();
    while_loop_demo();
    for_loop_demo();
}

fn if_else_if_demo(x: i32) {
    
    if x % 4 == 0 {
        println!("x is divisible by 4");
    } else if x % 3 == 0 {
        println!("x is divisible by 3");
    } else if x % 2 == 0 {
        println!("x is divisible by 2");
    } else {
        println!("Number not divisible!");
    }
}

fn conditional_let_statement(x: i32, y: i32) {
    let condition = true;
    let number = if condition { x } else { y };

    println!("number is {}", number);
}

fn loop_return_value_demo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Return value, placed into result
        }
    };

    println!("The result is {}", result);
}

fn while_loop_demo() {
    let cycle_limit = 3;
    let mut cycles = 0;

    while cycles < cycle_limit {
        cycles += 1;
    }
    
    println!("Cycles: {}", cycles);
}

fn for_loop_demo() {
    let a = (1..=5).rev();

    for element in a {
        print!("{} ", element);
    }
    println!("");
}