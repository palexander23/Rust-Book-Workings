// Define an enum to express an IP address type.
// Variants are given in the comma separated list.
enum IpAddressKind {
    V4,
    V6,
}

// Now define a struct that makes use of this enum.
// We can use the enum as a type.
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

// We can express the above functionality in a more concise way.  
// An enum can be defined with data encoded within eachenum variant.
#[derive(Debug)]
enum IpAddrConcise {
    V4(String),
    V6(String),
}

// Each variant can store different types and amounts of associated data.
// This is useful for our IP address example. 
#[allow(dead_code)]
#[derive(Debug)]
enum IpAddrv4u8 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Like for structs we can use an impl block to define member functions
// for these new types.
// I'm skipping ahead here and using a match statement based off the one 
// seen in chapter 2 of the book. 
impl IpAddrv4u8 {
    fn print(&self) {
        match self {
            IpAddrv4u8::V4(n0, n1, n2, n3) => println!("{}.{}.{}.{}", n0, n1, n2, n3),
            IpAddrv4u8::V6(address) => println!("{}", address),
        }
    }
}


#[allow(dead_code)]
fn main() {
    // Lets create instances of the custom variable.
    // The type is specified with the {Type}::{variant} syntax.
    let _ip4 = IpAddressKind::V4;
    let _ip6 = IpAddressKind::V6;

    // Now lets create instances of the struct 
    let _home = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    // Now lets create some instances of the concise version.
    // The variant is selected and the data is placed in the brackets.
    let _home = IpAddrConcise::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrConcise::V6(String::from("::1"));

    // Now lets create an instance of the version with differing types.
    let _home = IpAddrv4u8::V4(127, 0, 0, 1);
    let _loopback = IpAddrv4u8::V6(String::from("::1"));
    
    // Debug prints
    println!("Debug printouts for IP addresses:");
    println!("home: {:?}, loopback: {:?}\n", _home, _loopback);

    // Use print method
    println!("Method printouts for IP addresses:");
    
    print!("Home: ");
    _home.print();

    print!("Loopback: ");
    _loopback.print();


}
