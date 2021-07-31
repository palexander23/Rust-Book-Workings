# 7.1 Packages and Crates
No code for this section, just a brief description of what this chapter was about.

## Crates and Packages
A Crate is a library or a binary (made up of one or more files) that provides a set of functionality.
A Package is a collection of crates that work together to perform a broader set of functionality.
A Package must have at least one crate.
A package can have 0-n binary packages and 0-1 library packages.
The main.rs and lib.rs files in the src directory are "crate roots" of the package.
They signify the presence of two crates, a binary and library respectively, which share the name of the package.

## Multiple Binary Crates
Multiple binary crates can be placed in the same package by placing them in the `src/bin` directory.
It's common convention to have the code shared by each of the binaries placed in a central library that each of the binaries feed off of.
It should be noted that `src/examples` directory can be created to showcase the library. 
the `src/bin` directory should only be used for executable programs.