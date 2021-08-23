// The tests directory, next to src, is the location for all integration tests.
// This is a location known by the Rust compiler so it will include all tests in
// all files found in this directory on a cargo test call.

// First we have to import the code we want to test.
// The use of the crate title brings the contents of the crate root into scope.
use test_organisation;

// Because this is already a separate module from the rest of the library we don't
// need to wrap any of these tests into a module explicitly and we don't have to
// add the #[cfg(test)] attribute anywhere.

#[test]
fn test_add_two() {
    assert_eq!(test_organisation::add_two(1), 3);
}
