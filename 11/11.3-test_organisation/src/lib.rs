// There are two types of tests in Rust, Unit Tests and Integration Tests.
// Unit tests test individual parts of a project, single functions or modules.
// They can use private members of objects defined locally.
// Integration tests run from the perspective of an outside user to any libraries.
// They can only use the public interfaces that users would have access to.

// Unit tests are defined in the same file as the code they are testing.
// This makes sense for Rust where it wouldn't for a language like Python because
// Rust code is compiled or is used from within a crate so an end user will never
// see the tests. Python code, however, is shipped as is and can be viewed as
// text. Having tests present in the source files would bulk up a release
// and make its source code more confusing.

// By convention, Unit tests are organised into a module called test within each
// file. Each of these modules has the #[cfg(test)] attribute which tells the
// compiler not to include it in a build of the final binary.

// An interesting thing about Rust is that the privacy rules don't stop you from
// testing private methods. See below for an example on this.
// This works because tests are just Rust code and you can always bring the code
// of the parent into scope within the child.

pub fn add_two(a: i32) -> i32 {
    private_add_two(a)
}

fn private_add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod private_tests_demo {

    use super::*;

    #[test]
    fn test_add_two() {
        assert!(add_two(2) == 4);
    }

    #[test]
    fn test_private_add_two() {
        assert!(private_add_two(2) == 4);
    }
}

// Please see ../tests/adder_test.rs for information about Integration tests.__rust_force_expr!
