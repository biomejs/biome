mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/css_test_suite/ok/**/*.css", crate::spec_test::run, "ok"}
    tests_macros::gen_tests! {"tests/css_test_suite/error/**/*.css", crate::spec_test::run, "error"}
}
