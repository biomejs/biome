mod spec_test;

mod ok {
    //! Tests that are valid YAML
    tests_macros::gen_tests! {"tests/yaml_test_suite/ok/**/*.yaml", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail because they are not valid YAML
    tests_macros::gen_tests! {"tests/yaml_test_suite/err/**/*.yaml", crate::spec_test::run, "error"}
}
