mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/tailwind_specs/ok/**/*.txt", crate::spec_test::run, "ok"}
}

mod error {
    tests_macros::gen_tests! {"tests/tailwind_specs/error/**/*.txt", crate::spec_test::run, "error"}
}
