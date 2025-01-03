mod spec_test;

mod ok {
    //! Tests that are valid GritQL
    tests_macros::gen_tests! {"tests/grit_test_suite/ok/*.grit", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail because they are not valid GritQL
    tests_macros::gen_tests! {"tests/grit_test_suite/err/*.grit", crate::spec_test::run, "error"}
}
