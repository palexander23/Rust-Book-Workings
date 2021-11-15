//! # 13.2 Iterators
//! Iterators are a classic functional programming pattern that allow you to perform
//! actions on collections of items in a sequence.
//! An iterator can be created from any list of objects.
//!
//! Once of the benefits of iterators is the not needing to re-implement a bunch of 
//! loop-style code for every new list of items you come across. 
//! Turning a list into an iterator allows you to use the methods available to iterators
//! on top of for-each style syntax.
//!
//! A useful thing about iterators: they are lazy.
//! Creating an iterator takes no CPU time and very little extra memory.
//! They only take up time when they are evaluated.

/// Demonstrate basic creation of iterators from vectors  
/// 
/// Create an iterator from a vector and print it using a for loop.
/// This is very basic stuff that you can do with vectors as well but there are
/// many things that you can turn into iterators that don't already have this
/// functionality.
/// This makes iterators a useful common language for repeated tasks that
/// removes fiddly work that could go wrong.
fn basic_iterator_creation() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }
}

// Under the surface, iterators are very simple.
// All iterators must implement the trait Iterator, defined in the standard lib.
// This includes an associated type and a single function called next()
// Next returns a Some() containing the next number in the list or a None() if the 
// list has been exhausted.
// We can demonstrate that using the function below.

/// Demonstrate that iterators return Option types from their next function
/// 
/// Note that we've made the iterator mutable here where we didn't in the 
/// previous function. This is necessary because calling next() deletes an 
/// item in the iterator, effectively changing the variable.
/// We didn't need it in the previous function because the for loop took 
/// ownership of the iterator and made it mutable behind the scenes.
/// 
/// Note that we can still use v1. 
/// This is because v1.iter() created an iterator of references to items
/// in that vector. The vector was not moved, an interface was created 
/// for it.
/// If we wanted to move the values within the vector to the list we 
/// could instead call v1.into_iter(). This would move the items in the 
/// vector into an iterator of those values.
fn iterator_under_the_hood() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    println!("{:?}", v1);
}

// All iterators have a set of methods associated with them.
// These fall into two types. Those that consume the iterator,
// and those that produce another iterator.
// An example of the former is shown below:

/// Demonstrate the iterator method sum() which consumes the iterator
/// and returns the sum of all member variables. 
/// 
/// We need to annotate the type of total because the sum method returns
/// the type Sum which must be converted to a specific type that matches
/// the type of the iterator.
fn consume_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter =  v1.into_iter(); 

    let total: i32 = v1_iter.sum();

    println!("v1 sum: {}", &total);

    assert_eq!(total, 6);
}

/// Demonstrate the map function which generates a new iterator by 
/// performing a mathematical operation on each individual element in 
/// the source iterator to generate each new element in the prodoced
/// iterator.
/// 
/// These methods are also sometimes known as iterator adaptors.
/// 
/// One important note here is that iterators are useless on their own.
/// They are lazy, so will not be evaluated unless we specifically ask
/// them to be.
/// After we transform the vector with a map call, we then collect the 
/// values back into a vector so we use them properly. 
/// This evaluates the iterator (including the map operation) and 
/// creates a new vector from the iterator data.
/// 
/// We have to specify the type for the collect output as it's quite
/// open ended. Here we are specifying we want a vector out
/// so that is what we are given.
/// 
/// One more note, here is a very good example of the real use of 
/// closures!
/// Closures are best used with other functional programming
/// construct such as these.
fn transform_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.into_iter();

    let v1_incr: Vec<_> = v1_iter.map(|x| { x + 1 }).collect();

    println!("v1_incr: {:?}", v1_incr);
}

/// Demonstrate the filter function as another example of an iterator adaptor
/// 
/// Another example of closures being used with iterator adaptors is the 
/// filter method.
/// This method produces a new iterator by selecting members of the source
/// iterator using a closure that returns a boolean. 
/// The use of a closure means that any operation can be used as the filter
/// criterion.
/// 
/// Closures capture the environment they are created within so any variable
/// accessible in the scope of the iterator can be used.
/// 
/// This demo uses shoe size as an example
fn filter_shoe_size_iterator() {
    #[derive(Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
        shoes.into_iter().filter(|s| s.size == shoe_size ).collect()
    }

    let shoe_vec = vec![
        Shoe{
            size: 5,
            style: "laces".to_string(),
        },
        Shoe{
            size: 3,
            style: "sandles".to_string(),
        },
        Shoe{
            size: 8,
            style: "high heels".to_string(),
        },
        Shoe{
            size: 5,
            style: "slippers".to_string(),
        },
        Shoe{
            size: 18,
            style: "clown shoes".to_string(),
        },
    ];

    println!("Shoes list: {:?}", shoe_vec);
    println!("Shoes of Size 5: {:?}", shoes_in_size(shoe_vec, 5));

} 

/// Demonstrate the implementation of the Iterator trait to create our own iterators
/// 
/// Adding an implementation of the Iterator trait to a novel data source can 
/// dramatically speed up the development of systems that generate and consume 
/// data.
/// The programmer need only specify an associated type and an implementation
/// of the next function to interface with the default implementations of all the 
/// iterator methods found in the standard library.
/// This saves the programmer rewriting huge swathes of data manipulation code.
/// 
/// This demo will create an iterator type which acts as a data source.
/// It will return the numbers 1 to 5 then None from successive calls to its next function. 
/// 
/// We define a Counter type made up of a struct containing a single u32.
/// The u32 holds the current value of the counter. 
/// We then implement the Iterator type for the counter. 
/// The associated type is u32 and the next function increments and returns
/// the count until it is greater than 5, then it returns 0.
fn iterator_implementation() {
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            if self.count < 5{
                self.count += 1;
                return Some(self.count);
            } else {
                return None
            }
        }
    }

    let c1 = Counter::new();
    let c1_out: Vec<u32> = c1.collect();

    assert_eq!(c1_out, vec![1, 2, 3, 4, 5]);

    println!("Counter output: {:?}", &c1_out);
}

fn main() {
    basic_iterator_creation();
    iterator_under_the_hood();
    consume_iterator();
    transform_iterator();
    filter_shoe_size_iterator();
    iterator_implementation();
}
