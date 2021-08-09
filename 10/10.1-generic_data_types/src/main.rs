// When we notice two or more different chunks of code performing the same set of
// actions on data of the same type we extract those actions into a function.
// This makes the code more readable and makes it easier to edit.

// We can do the same process on chunks of code which operate on different types 
// of data using Generics. These are the items in angle brackets used to define 
// objects like Results and Options. 

// This can be done directly in the function definitions as seen below.
// Here we define three functions, the first and second return the largest
// value from lists of i32s and chars respectively.
// The final function works on both types.

// Here are the two concrete type versions.
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Here is the generic typed version.
// Note the declaration of the generic type T in the angle brackets after the function name.
// Note Rust knows that some operations, such as comparisons can only be performed on some 
// types but not others. 
// To tell Rust what kind of operations can be done on T we can use Traits.
// These are the two keywords after the colon in the generic definition.
// The Copy trait lets Rust know the type can be copied cheaply, such as ints and chars.
// The PartialOrd tells Rust that items can be compared using < and > operators.
// These get rid of a few different errors.
fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest: T = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// We can also define structs to use a generic type using the same <> syntax.
// This is very useful if we're defining a type which has numerical parameters that can be
// floats or ints. A common example of this is a cartesian point.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// If applicable, we can also define a Point with different types for each coordinate.
#[derive(Debug)]
struct VersatilePoint<U: Copy, V: Copy> {
    x: U,
    y: V,
}

// Note we need the generic type specified in front of the Impl block as well as the 
// object name. This lets Rust know that we are using generic types.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also implement functions that are specific to a particular concrete type of Point.
// We can't overwrite methods implemented in generic blocks though.
impl Point<f32> {
    fn x_float(&self) -> &f32 {
        println!("I am a float!");
        &self.x
    }
}


// The generic types defined in a module method aren't always the same as those defined in the 
// the module definition.
// In the below example, we have a function that swaps the y coordinate of two Points.
// Obviously we need to account for the types of the two points being different.
// Note that the two generic types T, W were defined to encode the type of the other point.
// The returned Point takes the x coord type from the impl block header, and the y coord,
// from the function definition types.
impl<U: Copy, V: Copy> VersatilePoint<U, V> {
    fn swap_y<T: Copy, W: Copy>(&self, other: &VersatilePoint<T, W>) -> VersatilePoint<U, W> {
        VersatilePoint{
            x: self.x,
            y: other.y,
        }
    } 
}

fn new_point_demo() {
    // These two examples work
    let _int_point = Point { x: 1, y: 3 };
    let _float_point = Point { x: 2.4, y: 56.23 };

    // This one does not as in our definition we specified x and y to have the same Type.
    // let combined_point = Point { x: 2.4, y: 1 };

    // We can use the extra definition of VersatilePoint to store these coordinates.
    let _combined_point_one = VersatilePoint{ x: 2.4, y: 1};
    let _combined_point_two = VersatilePoint{ x: 3, y: 32.43 };

    println!("\n_int_point:    {:?}", _int_point);
    println!("_float_pofloat:  {:?}", _float_point);
    println!("_combined_point_one: {:?}", _combined_point_one);
    println!("\n_int_point.x: {:?}", _int_point.x());

    // This works as the x_float function has been defined for floats
    println!("_float_point.x_float: {:?}", _float_point.x_float());

    // This will not work as it has not been defined for ints.
    // println!("_int_point.x_float: {:?}", _int_point.x_float())

    println!("\nP1: {:?}", _combined_point_one);
    println!("P2: {:?}", _combined_point_two);
    println!("P1.swap_y(P2): {:?}",_combined_point_one.swap_y(&_combined_point_two))
    
}


// We can also have generic types in method definitions as shown below.


fn main() {
    let char_vec = vec!['a', 'b', 'd', '!', ';'];
    let i32_vec = vec![1, 2, 45, 36, 253];

    println!("largest_char output:  {}", largest_char(&char_vec));
    println!("largest_i32 output:   {}", largest_i32(&i32_vec));
    println!("largest<char> output: {}", largest(&char_vec));
    println!("largest<i32> output:  {}", largest(&i32_vec));

    new_point_demo();
}