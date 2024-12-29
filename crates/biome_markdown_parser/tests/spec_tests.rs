mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/md_test_suite/ok/**/*.md", crate::spec_test::run, "ok"}
    tests_macros::gen_tests! {"tests/md_test_suite/error/**/*.md", crate::spec_test::run, "error"}
}
