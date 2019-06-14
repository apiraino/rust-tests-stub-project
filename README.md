This is a stub project to learn how to configure a project to have all integration tests in one library. The gist of it:

- have a nested crate inside `/tests` with all tests and their utilities
- make this crate depend on the main project crate
- configure the main project to run this crate when runnning tests

### Advantages

- One library for smaller size and faster building times
- No more [false positives](https://github.com/rust-lang/cargo/issues/6430) about unused code: every integration module is compiled and linted as a separate binary, therefore you get a lot of warnings when that module does not use all utilities and common functions you may have.

### File system layout example

``` bash
$ pwd
~/Projects/tests-stub-prj/

$ tree .
.
├── Cargo.lock            <--- Main project crate
├── Cargo.toml
├── src
│   └── main.rs
└── tests
    └── testsuite
        ├── cratetests      <--- Tests crate: everything is compiled into one library
        │   ├── Cargo.lock
        │   ├── Cargo.toml
        │   ├── common
        │   │   ├── mod.rs
        │   │   └── utils.rs <--- common tests utilities
        │   └── lib.rs
        ├── lib.rs
        ├── test_module_1.rs <--- test module
        ├── test_module_2.rs <--- test module

5 directories, 12 files
```
