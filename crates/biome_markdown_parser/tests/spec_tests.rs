mod spec_test;

mod ok {
    //! Tests that are valid Markdown
    tests_macros::gen_tests! {"tests/md_test_suite/ok/**/*.md", crate::spec_test::run, "ok"}
}
