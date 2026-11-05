mod spec_test;

mod ok {
    //! Tests that are valid TOML
    tests_macros::gen_tests! {"tests/toml_test_suite/ok/**/*.toml", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail because they are not valid TOML
    tests_macros::gen_tests! {"tests/toml_test_suite/err/**/*.toml", crate::spec_test::run, "error"}
}
