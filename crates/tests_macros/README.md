# Tests macros

A set of utilities that automatically generate unit tests from files.

## Usage

First argument: glob that will passed to https://github.com/gilnaa/globwalk. Crate's cargo.toml will be the base directory. To pattern format see here: https://git-scm.com/docs/gitignore#_pattern_format  
Second argument: method that will be called with full path to each file.

One suggestion to organize tests is to put the macro inside a module.

```rust
mod some_mod {
    tests_macros::gen_tests!{"tests/*.{js,json}", run_test}

    // input_file and expected_file are full paths
    fn run_test(input_file: &str, expected_file: &str) {
        println!("{:?} {:?}", input_file, expected_file); 
    }
}
```

Test name is the "snake case" version of the file name.
this will generate the following for each file:

```rust
#[test]
pub fn somefilename()
{
    let test_file = "<crate's cargo.toml full path>/tests/sometest.txt";
    let test_expected_file = "<crate's cargo.toml full path>/tests/sometest.expected.txt";
    run_test(test_file, test_expected_file);
}
```

## How to run

```
> cargo test                                            // all tests in all crates
> cargo test -p crate-name                              // all tests of one crate
> cargo test -p crate-name -- some_mod::                // all tests of one crate and one module
> cargo test -p crate-name -- some_mod::somefilename    // just one test
```
