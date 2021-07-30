// Define an enum for coin types.
// A match statement works like a coin sorter.
// Each possible coin type is known and accounted for.

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter,
    StateQuarter(UsState),
}

// I have placed this function in an impl block just for fun
// All the standard coins are represented as well as the Quarters
// minted with images of a US State.
// A special case has been made for those which prints out the state.
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            Coin::StateQuarter(state) => {
                println!("Quarter is from {:?}!", state);
                25
            }
        }
    }
}

// We can use an Option enum to encode a result that may or may not have a value.
// Rust then forces us to handle both cases.
// Define a function which takes an Option<i32/None> and returns 
// i32 + 1 or None depending whether the Option is i32 or None.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // Same as Option::None => Option::None as Option is imported automatically.
        None => None,
        Some(i) => Some(i + 1),
    }
}

// We can use _ as a placeholder for "all other values" at the end of a match.
// This can be very useful if we don't want to list all possible options
// Define a function which does thing thing if a given number is a set of 
// particular values.
fn print_odd_below_10(x: u8) {
    match x {
        1 => println!("one!"),
        3 => println!("three!"),
        5 => println!("five!"),
        7 => println!("seven!"),
        9 => println!("nine!"),
        _ => (),
    }
}

fn main() {
    // Define a set of coins and print their values
    let dime = Coin::Dime;
    let quart = Coin::Quarter;
    let sp_quart = Coin::StateQuarter(UsState::Alabama);

    println!("Coin: {:?} Value: {}", dime, dime.value_in_cents());
    println!("Coin: {:?} Value: {}", quart, quart.value_in_cents());
    println!("Coin: {:?} Value: {}", sp_quart, sp_quart.value_in_cents());

    // Test the plus_one function
    let some = Option::Some(34);
    let none = None::<i32>; // This is how you set the type of a None!

    println!("some: {:?}", plus_one(some));
    println!("none: {:?}", plus_one(none));

    // Test print odd below 10 function
    print_odd_below_10(1);
    print_odd_below_10(2);
    print_odd_below_10(5);
    print_odd_below_10(11);
}
