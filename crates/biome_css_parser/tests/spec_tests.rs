mod spec_test;

tests_macros::gen_tests! {"tests/css_test_suite/ok/**/*.{css,scss,sass}", crate::spec_test::run, "ok"}
tests_macros::gen_tests! {"tests/css_test_suite/error/**/*.{css,scss,sass}", crate::spec_test::run, "error"}
