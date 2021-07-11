// Example struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Initialise a struct based off the definition we created above.
    // Note the fields don't have to be specified in the same order.
    let user1 = User {
        email: String::from("bop.bing@bibble.com"),
        username: String::from("bob_bing"),
        sign_in_count: 0,
        active: true,
    };

    // We can access data in that struct using dot notation.
    println!("user1 email: {}, sign_in_count: {}, username: {}, active: {}", 
        user1.email, 
        user1.sign_in_count, 
        user1.username, 
        user1.active
    );

    // Define a second user that is mutable. 
    let mut user2 = User {
        username: String::from("hedgehog"),
        email: String::from("hedge@hog.com"),
        active: true,
        sign_in_count: 4,
    };

    println!("user2 Before Changes:");
    println!("user2 email: {}, sign_in_count: {}, username: {}, active: {}", 
        user2.email, 
        user2.sign_in_count, 
        user2.username, 
        user2.active,
    );

    // Change some things about the second user:
    user2.email = String::from("hoglle@heady.commers");
    user2.sign_in_count += 1;

    println!("user2 After Changes:");
    println!("user2 email: {}, sign_in_count: {}, username: {}, active: {}", 
        user2.email, 
        user2.sign_in_count, 
        user2.username, 
        user2.active,
    );

    // Define new users using a builder function
    // Use both string literals and String objects
    let user3 = build_user("bob@building.com", "bob");
    let user4 = build_user(&user1.email, &user1.username);

    // We can define new structs using the details from another using specific syntax
    let user5 = User {
        email: String::from("hello@world.com"), 
        username: String::from("hello"),
        ..user1
    };

    println!("user3:");
    print_user(&user3);

    println!("user4:");
    print_user(&user4);

    println!("user5:");
    print_user(&user5);

}

// Define a function to create a User from given username and email
// Notice how &str type has been used in arguments so they can be defined by 
// string literals or String objects
fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}

// Other syntax could be used here to do this in a more concise way
// But, to be honest I don't like the tendency Rust has for taking away 
// information about what a function is doing (e.g. not needing a return keyword)
// As it seems to make things more difficult to read for C users without
// adding any new functionality. 
// It also takes away the ability to use &str to support both  String and &str argumennts.

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// Define a function for printing a User
// I know we're not meant to know how to do this until the next few sections of the book but
// I'm going to do a bit of guessing on how this is all meant to work.
// Notice how I had to use the & reference specifier to make sure that the main function
// doesn't give up ownership of the User objects to the print function. 
fn print_user(user: &User) {
    println!("email: {}, sign_in_count: {}, username: {}, active: {}", 
        user.email, 
        user.sign_in_count, 
        user.username, 
        user.active,
    );
}
