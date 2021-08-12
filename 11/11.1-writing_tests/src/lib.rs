// Rust is very good an ensuring correctness in programs but it is not perfect.
// Therefore a comprehensive testing system has been written into cargo.
// The module "tests" found below is present by default in any newly created library. 
// By default it contains a single function called "it_works" which asserts 2+2 == 4.

// Tests can be run by calling 'cargo test' from the terminal within the project. 
// This will collect any function labelled with he #[test] attribute and compile them into a
// single binary, including checks on the results of each function.  

// We can add #[test] to as many functions as we want.
// ALl of them will be included in the metrics posted after a call to cargo test.

// Note the use of the #[should_panic] attribute to tell the tester that that particular 
// function should fail.
// Also note the use of the expected argument to tell the tester exactly what to look out 
// for in a panic message to ensure that the program has panicked for the right reason. 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "This test should fail!")]
    fn another() {
        panic!("This test should fail!");
    }
}


// We can check the results of boolean functions with the assert! macro.
// It takes a single bool, it panics if it is false and continues if true.
// Below we are reimplementing our Rectangle struct with the can_hold method to demo this.

#[allow(dead_code)]
struct Rectangle {
    width: u32,
    length: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

// We can then write a set of test functions for the Rectangle.
#[cfg(test)]
mod rectangle_tests {
    use super::*;

    fn get_test_rectangles() -> (Rectangle, Rectangle) {
        let larger = Rectangle{
            width: 50,
            length: 60,
        };
        
        let smaller = Rectangle {
            width: 40,
            length: 30,
        };

        return (larger, smaller)
    }

    #[test]
    fn larger_can_hold_smaller() {
        let (larger, smaller) = get_test_rectangles();   
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() { 
        let (larger, smaller) = get_test_rectangles();   
        assert!(!smaller.can_hold(&larger));
    }
}


// Another common way to test code is to assert that the return value of a function is
// equivalent to one we were expecting. 
// This can be achieved with the assert_eq! and assert_ne! macros.
// Consider the very simple example below with a test for a function which adds 2 to an int.

pub fn add_two(a: i32) -> i32 {
    a + 2 
}

#[cfg(test)]
mod tests_add_two {
    use super::add_two;

    #[test] 
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}


// We can also add custom text to an assert*! macro that is printed when the test fails.
// Lets demonstrate this by testing a function that takes a name and returns a personalised
// greeting containing that name.
// First we will test that a string with content is returned then we will check if the name
// is included.
pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests_greeting {
    use super::greeting;
    
    #[test]
    fn greeting_not_empty() {
        let result = greeting("Bill");
        assert_ne!(result, String::from(""), "Returned string was empty.");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Bill");
        assert!(result.contains("Bill"), "Greeting did not contain the name, greeting was: {}", result);
    }
}
