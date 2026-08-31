mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/toml_test_suite/ok/**/*.toml", crate::spec_test::run, "ok"}
}

mod error {
    tests_macros::gen_tests! {"tests/toml_test_suite/error/**/*.toml", crate::spec_test::run, "error"}
}
