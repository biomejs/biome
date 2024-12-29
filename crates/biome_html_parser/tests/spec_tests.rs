mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/html_specs/ok/**/*.html", crate::spec_test::run, "ok"}
}

mod error {
    tests_macros::gen_tests! {"tests/html_specs/error/**/*.html", crate::spec_test::run, "error"}
}
