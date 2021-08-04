// Another common collection is the hash map.
// This is a more versatile version of Python's Dictionaries.
// Hash Maps are defined with the type HashMap<K, V>. 
// The maps stores a list of key value pairs with keys of type K and values of type V.

// As always we need to start by brining the HashMap object into scope.
use std::collections::HashMap;

// Lets do a quick hashmap creation demo
// We'll create and populate a hashmap to store the score of different teams in a game
fn hash_map_creation() {

    // Create an empty, mutable hashmap
    let mut scores = HashMap::new();

    // Fill it with data.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    println!("Scores Map: \n{:?}\n", scores);
}

// We can also create hash maps by zipping and collecting two vectors
fn hash_map_from_vectors() {
    
    // Define two vectors representing team name and score data
    let teams  = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];

    // Create the hash map.
    // Note we have got the type inferred with the underscores.
    // We then convert teams into iter and zip with the other array (also converted).
    // This gives a list of tuples, like in Python.
    // The collect function then turns the list of tuples into a hash map.
    let scores: HashMap<_, _> =
        teams.into_iter().zip(scores.into_iter()).collect();
 
    println!("Map from vectors: \n{:?}\n", scores);
}

// We will now demonstrate ownership of variables passed to a hashmap.
// Types with the Copy trait, such as i32, are copied into the hash map.
// For bigger things like Strings the ownership of the objects are passed to 
// the hash map and the old locations are removed from scope.
// We can also put references in hash maps but that's another can of worms.
fn hash_map_ownership_demo() {
    let teams = String::from("Blue");
    let scores = 10;

    let mut map = HashMap::new();
    map.insert(teams, scores);

    // We can still interact with the int list but not the String list. 
    println!("Map from moved items: \n{:?}\n", map);
    println!("Int that wasn't copied: {:?}", scores);
    // println!("String that was {:?}", teams); - Error
    println!("String is no longer accessible!");
    // If this was a vector they'd both go out of scope.
}

// We can retrieve elements out of the hash map using the get method.
fn accessing_values_in_hash_map() {
    // Create a map as we have done before 
    let teams  = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 50];

    let map: HashMap<_, _> = 
        teams.iter().zip(scores.iter()).collect();

    println!("\nMap to get values from: {:?}", map);

    let team_name = String::from("Blue");
    
    match map.get(&team_name) {
        Some(v) => println!("Blue Score: {}", v),
        None => println!("No team named Blue!"),
    }

    // We can also iterate over the values in the map in arbitrary order.
    println!("\nList iteration:");
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
}

// We will often want to update a hash map entry with a new value.
// We can either, replace whatever value was there,
// only change it if the key doesn't already have a value,
// or update it based on the value.
fn update_hash_map_values() {

    // Overwrite a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("\nHash Map after Overwrite:\n{:?}", scores);

    // Only insert a value if the key has no pre-existing value
    // The entry method gets the entry for the corresponding key for 
    // in-place editing. 
    // Can be read or modified directly.
    scores.entry(String::from("Blue")).or_insert(40);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("\nHash Map after or_insert:\n{:?}", scores);

    // Updating a value based on an old value
    let mut word_map = HashMap::new();

    // The or_insert function actually returns a reference to the corresponding value.
    // If the value for that key does not exist it is created and the reference returned.
    // If it does already exist then the reference is returned.
    // Either way, that reference can be used to access the data using the deference operator *.
    for word in "hello world wonderful world".split_whitespace() {
        let entry = word_map.entry(word).or_insert(0);
        *entry += 1;
    }

    println!("\nWord map:\n{:?}", word_map);
}

fn main() {
    hash_map_creation();
    hash_map_from_vectors();
    hash_map_ownership_demo();
    accessing_values_in_hash_map();
    update_hash_map_values();
}

