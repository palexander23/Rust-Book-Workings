// Traits are used to tell a compiler that a new Type has a set of specific properties.
// This is used both in the declaration of functions/objects using generic types and in
// the definitions of new types.

// We will first consider defining a new Trait. 
// The definition of a Trait is actually just a set of definitions for methods that we 
// can call on objects with those types.
// Consider a text aggregate library that is being designed to display summaries of a
// variety of different media formats. 
// Consider also that we've created Types to store each of these formats. 
// To make processing each piece of media easier we can define a Trait that each of 
// these format types must share containing a method called "summarise".
// An example of this is shown below.

// Note, the name of the Trait conveys what the trait can do/what it can produce.
// Note that this starts off looking like a function definition from a impl block
// but after the function definition we have a semicolon rather than an implementation.
// Each type with this trait must have an impl block with an implementation of each of 
// the methods defined in the trait definitions or the program won't compile.
// This gives a developer freedom to implement the method differently for each type.
// This version is commented out to allow revisions further on.

// pub trait Summary {
//     fn summarise(&self) -> String;
// }


// One important restriction to how Traits work is that we can only implement a Trait for type 
// if one or both of the Trait and Type are defined in the code we're writing.
// We implement a Trait we have created on a standard library type (e.g. vector).
// We can also assign a trait defined in the standard library to a type we have locally defined.
// However we cannot implement a standard library trait on a standard library object.
// This could create conflicts where two libraries had implemented the the same trait for a
// library and Rust cannot determine which one to use.


// In a lot of situations it can be useful to have a default implementation.
// One such example is shown below with the reimplementation of the Summary Trait to include a 
// default summary string that is returned if called on a Type without specific implementation.
// The implementation is as shown below. The semicolon has been replaced with an implementation.

// pub trait Summary {
//     fn summarise(&self) -> String {
//         String::from("Read More...")
//     }
// }

// We can call unimplemented functions from default implementations as shown below
pub trait Summary {
    fn summarise_author(&self) -> String;

    fn summarise(&self) -> String {
        format!("Read More From {}...", self.summarise_author())
    }
}


// Lets now define types that use this trait.
// Lets define NewsArticle and Tweet types. 
// Both can be summarised but contain different data so their summaries will be
// constructed differently. 
// We've made the trait pub so it can be used by other developers when defining their
// own stuff.
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

// Note here that we have to specify which trait the functions below implement.
impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("{}", &self.author)
    }

    fn summarise(&self) -> String {
        format!("{} by {} ({})", &self.headline, &self.author, &self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", &self.username)
    }

    fn summarise(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

// We can then call the trait methods on the types as we would any other method.
fn get_test_types() -> (NewsArticle, Tweet) {
    let news_article = NewsArticle {
        headline: String::from("Big Fish"),
        author: String::from("Hungry Cat"),
        location: String::from("The Pond"),
        content: String::from("I saw a big fish today. Alas, it was too quick for me"),
    };

    let tweet = Tweet {
        username: String::from("BigFish"),
        content: String::from("Was nearly eaten by a big cat today 😢"),
        retweet: false,
        reply: false,
    };

    return(news_article, tweet);
}

fn summary_demo() {
    let (news_article, tweet) = get_test_types();

    println!("News Article Summary: {}", news_article.summarise());
    println!("Tweet Summary:        {}", tweet.summarise());
}


// We can also use Traits in place of parameter types in function definitions.
// We can define functions which work with an object of any type with a given Trait
// rather than a specific type. The syntax for this is as shown below.
// Rather than a concrete type we say impl <Trait>.
// Note that the & is not part of this syntax, it is just saying that the parameter
// is being borrowed rather than being copied.
pub fn notify(item: &impl Summary) {
    println!("Breaking News: {}", item.summarise())
}

fn notify_demo() {
    let (news_article, tweet) = get_test_types();
    println!("\nNotify Demo: ");
    notify(&news_article);
    notify(&tweet);
}

// Can also write this syntax in a different way as shown below.
// Here we are defining a generic type that implements Summary after the function name
// and using that type in the definition of the parameter item.
// This is functionally identical to the version of notify above. 
// This version can be less verbose in case where lots of items would have the T type.
fn _notify_verbose<T: Summary>(item: &T) {
    println!("Breaking News: {}", item.summarise())
}


// We can specify the requirement of having multiple Trait Bounds with the + syntax 
// as shown below.
use std::fmt::{Display, Debug};

// Note we can use the item directly in the print item as Rust knows any argument will 
// implement the Display trait.
fn _notify_disp(item: &(impl Summary + Display)) {
    println!("Breaking News: {}", item); 
}

fn _notify_disp_verbose<T: Summary + Display>(item: &T) {
    println!("Breaking News: {}", item); 
}


// Trait Bounds on generic types are very useful but using either syntax above can make 
// function definitions hard to read if there are many types with different bounds.
// Adding a where clause to a Rust function can make functions easier to read.
// Consider the following two function definitions.
// The first is quite an eyeful but the second compartmentalises the information.
fn _difficult_read<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}

fn _easy_read<T, U>(_t: &T, _u: &U)
    where T: Display + Clone,
          U: Clone + Debug,
    {}


// As you might imagine we can also specify return types with Trait Bounds as well.
// The syntax for this is shown below.
fn _return_summarisable() -> impl Summary {
    let (news_article, _tweet) = get_test_types();

    return news_article;
}

// However we must remember what is going on under the hood.
// When Rust compiles a project it goes through each function/object with generic types
// and creates versions for each concrete type that actually occurs in the program.
// knowing this it is obvious that we can't define a function that could return one type 
// or another based on unpredictable internal logic even if both implement Summary.


// We can use the impl syntax described here along with the generics implementation 
// syntax to conditionally compile methods depending on the Traits of the concrete
// types which replace the generic types at compile time.
// Consider the example below where we have defined a struct that holds a pair of 
// generic values of the same type.
// We know that the generic types could be replaced with any possible type.
// However we may want to have some type specific functionality enabled if certain types 
// are used.
// We have used the separate method block with the Trait Bounds to account for this.
#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y, }
    }
}

#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x < self.y {
            println!("x less than y")
        } else {
            println!("x greater than or equal to y");
        }
    }
}


// We can also implement a Trait for for any Types which implement another Trait using the
// following syntax. This is used extensively in the standard library.
// For instance, any object that can be printed can also be converted straight to a String
// using the same logic.
// Therefore the ToString Trait is implemented for any type with the Display trait using 
// the following syntax:
impl<T: Display> _ToString for T {
    fn
}

fn main() {
    summary_demo();
    notify_demo();
}
