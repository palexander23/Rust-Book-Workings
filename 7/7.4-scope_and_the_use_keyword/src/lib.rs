// Nested modules can make paths to items in other modules excessively long.
// We can get around this using the use keyword to bring items into scope
// as if they were local objects.

// Lets demonstrate this with a simplified restaurant module as we've used
// before.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Rather than using the long absolute path down to the function we can 
// bring the hosting module up to the current scope.
// This is the Idiomatic path style, where the parent module is brought
// into scope so we know that the function used is not local.
use crate::front_of_house::hosting;

// We can also do this with a relative path.
// Commented to avoid multiple import error.
// use self::front_of_house::hosting;

pub fn load_n_diners(n: i32) {
    for _diner in 0..n {
        hosting::add_to_waitlist();     
    }
}


// However, when we want to import structs or enums, the idiomatic way
// is to use the full path as shown below.
// This is just the convention that has emerged, there's no real reason.
// I do think it separates the module path and the used functions well.
// Like with the idiomatic module path, there's a thing telling you 
// where the function is from, then the function. 
use std::collections::HashMap;

#[allow(dead_code)]
fn use_hash() {
    let mut hash_map = HashMap::new();
    hash_map.insert(2, 3);
}

// The exception to the above rule is, of course, when we bring two things
// with the same name but different origins as shown below with the result
// objects.
use std::fmt;
use std::io;

#[allow(dead_code)]
fn use_result() {
    let _fmt_result = fmt::Result::Ok;
    let _io_result = io::Result::Ok(2);
}


// We can solve the above problem a different way by using as after use.
// This is similar to the Python way of giving names ot things which 
// come from elsewhere to give them more context.
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

#[allow(dead_code)]
fn use_named_results() {
    let _fmt_result = FmtResult::Ok;
    let _io_result = IoResult::Ok(342340);
}


// As with any object without the pub keyword attached, the objects we
// bring into scope with the use keyword are private.
// However, we can attach the pub keyword to make them visible in the
// module above.
// This is called re-exporting. We can use it to make the objects 
// available for others to bring into their scope.

// This can be useful if we want to write our code in one structure but 
// expose it in a different one. 
// Our external interface code may be collected in a module but we don't
// want our users to see that.
#[allow(unused_imports)]
mod test_mod {
    use std::io::Result as NotAccessibleResult;
    pub use std::io::Result as AccessibleResult;
}

#[allow(dead_code)]
fn re_export_test() {
    // Succeeds
    let _result = test_mod::AccessibleResult::Ok(3);

    // Fails
    // let _result = test_mod::NotAccessibleResult::Ok(3);
}


// When importing things, we can use several tricks to shorten long use statements.
// The first is nested paths as seen below:
#[allow(unused_imports)]
use std::{cmp::Ordering, io::Chain};

// We can also include self in this list if we ant to import that first module.
// We can demonstrate this using rand which we have added to our Cargo.toml.
#[allow(unused_imports)]
use rand::{self, Rng};

// The second tidying method is to bring all items into scope using the glob operator:
#[allow(unused_imports)]
use std::collections::*;

