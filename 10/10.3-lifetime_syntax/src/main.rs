// So far we haven't talked about how Rust works out that any given reference is still valid.
// We have been borrowing references to objects for a while now but we have only been
// examining them within the function, it would be useful to be able to return them.

// Consider the code below.
// Rust knows that x has gone out of scope by the time we try to reference it with x so it 
// rejects this program at compilation.
// It does this by comparing the lifetime of r and x.
// It concludes that r lives longer than x so at some point in the program r will reference 
// unallocated memory.

// fn fail_func() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("{}", r);
// }

// Most of the time Rust works out the lifetime of all objects automatically.
// However there are instances where that can't be done.
// Lets consider the situation below where we want a function to borrow two string slices and 
// return one of them.
fn string_slice_demo () {
    let string1 = String::from("abcd");
    let string2 = "hello";
    
    let longest_string = longest(&string1.as_str(), &string2);
    println!("The longest string slice is: {}", longest_string);
}

// A naive way to approach writing the longest function would be as seen below.
// However this results in the error "missing lifetime specifier" on the return value.
// Rust doesn't know which of x or y will be returned so it can't work out the lifetime
// of the return value. This is fair enough as we can't predict this either!
// We need to add generic lifetime parameters that define the relationship between 
// references so Rust can analyse it properly.

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// The syntax for lifetime parameters slots into the same place as other definitions of generic types.
// Lifetimes look a little bit like type annotations and are placed before the type of a value
// when defining it. 
// If we were defining a mutable reference to an i32 assuming the lifetime a was 
// defined it might look like: &'a mut i32
// The & has an apostrophe and a name after it, this name is always very short and usually just "a". 

// Lets define a correct version of the longest function.
// What we need to do is tell the compiler know that, as far as this function is concerned, the
// lifetime of the return value will always be the same as that of the two parameters.
// We will define a lifetime parameter and then apply that parameter to the two function 
// parameters and the function return value.
// Note the definition of lifetime 'a in the angle brackets then its use throughout the rest of 
// the definition.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

// What we're effectively doing here is telling Rust to enforce that both parameters and the return
// value have a lifetime of at least 'a. The lifetimes of x and y are known so Rust can use this 
// to calculate the lifetime of the return value.


// We can also demonstrate how Rust calculates the concrete lifetime of 'a from the lifetimes of 
// x and y.
// Below is a function that also uses the longest function to compare two string slices.
// However, one slice is defined in a higher scope, and therefore has a longer lifetime that the other.
fn different_scopes_lifetime() {
    let string1 = String::from("hello");
    
    {
        let string2 = String::from("world!");
        let longest_str = longest(string1.as_str(), string2.as_str());
        println!("Longest string: {}", longest_str)
    }
}

// In this example, the parameters have two different scopes but the lifetime 'a can only resolve to 
// one concrete lifetime at compilation.
// To resolve this 'a is given a concrete lifetime equal to the shortest lifetime of the two parameters.
// This is the lifetime that it can be guaranteed that return reference will be valid regardless of which
// reference is returned.

// This can be further demonstrated by moving the definition of the result to the outer-most scope.
// We have to use Strings for this example so that the data is removed from the heap as the scope exits.
// This function will fail as the lifetime of string2 is less than that of result.

// fn outer_most_scope_fail() {
//     let string1 = String::from("Hello");
//     let result;

//     {
//         let string2 = String::from("World!");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("Longest String: {}", result);
// }


// Generic lifetimes also enable the use of references as struct fields.
// By stating that a particular field has a lifetime Rust knows not to allow the struct to outlive the 
// value that field is referenced to.
// The syntax is similar to that of function definitions, with angle brackets after the name giving the 
// lifetime parameter definition.  
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_lifetime_demo() {
    // Define some variables
    let string = String::from("I am important. I am less important.");
    let excerpt;

    // Perform some stuff that creates a reference to string
    {
        let important = string.split(".").next().expect("Could not find a .");
        
        // Create the instance of the struct. 
        excerpt = ImportantExcerpt {
            part: important,
        };
    }
    
    // The struct is still valid here because string is still in scope.
    // If string was moved into the lower scope it would not work as string does not live long enough
    // for an excerpt of it to be printed.
    println!("Important Excerpt: {:?}", excerpt);
}


// There are a set of steps the compiler completes when working out the lifetimes of function parameters
// and return values called the Lifetime Elision Rules. Elision refers to the natural removal of words
// or sounds from words/phrases in a spoken language. 
// The steps are as follows:
// 
//  1. All parameters are given their own lifetime:
//      fn bib(x: &i32) -> &i32 {...}
//      fn bob(x: &i32, y: &i32) -> &i32 {...}
//    Becomes:
//      fn bib<'a>(x: &'a i32) -> &i32 {...}
//      fn bob<'a, 'b>(x: &'a, y: &'bi32) -> &i32 {...}
//
//  2. If there is exactly one input lifetime parameter, that lifetimeis assigned to all output lifetime 
//     parameters:
//      fn bib<'a>(x: &'a i32) -> &i32 {...}
//      fn bob<'a, 'b>(x: &'a, y: &'bi32) -> &i32 {...}
//    Becomes:
//      fn bib<'a>(x: &'a i32) -> &'a i32 {...}
//      fn bob<'a, 'b>(x: &'a, y: &'bi32) -> &i32 {...}
//
//  3. If there are multiple input lifetime parameters but one of them is for &self or &mut self then the 
//     the output lifetime parameter is assigned to all output lifetime parameters.
//
// That third one can be shown with a bigger example.


// We use the same syntax as generic typing for giving impl blocks lifetimes.
// This is demonstrated with the impl block below for the ImportantExcerpt type.
// Note we must give <'a> in front of both impl and the type name as the lifetime is part of the type.
impl<'a> ImportantExcerpt<'a> {

    // The first elision rule means we don't need to annotate the lifetime of &self.
    fn level(&self) -> i32 {
        3
    }

    // The first rule gives the two input types separate lifetimes.
    // The third rule gives the output the same lifetime as &self, meaning we don't have to manually 
    // specify lifetimes.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please: {}", announcement);
        self.part
    }
}


// Another thing worth mentioning is the 'static reference.
// Applying this to an object forces it to live for the entire duration of the program.
// String literals have this lifetime as they are baked into the program's binary.
// They should almost never be used, though, as we rarely want stuff to live in RAM forever.


// We also need to know how to combine lifetime parameters with generic types.
// Below is an example of this.
// It is the normal longest function with an extra announcement of indeterminate type.
// We have specified both a generic lifetime and a generic type and used both in the 
// function. 
use std::fmt::Display;

fn _longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

fn main() {
    string_slice_demo();
    different_scopes_lifetime();
    struct_lifetime_demo();
}
