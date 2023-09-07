#![allow(non_snake_case)]

mod spec_test;

mod ok {
    //! Tests that must pass according to the CSS specification
    tests_macros::gen_tests! {"tests/css_test_suite/ok/*.css", crate::spec_test::run, "ok"}
}
