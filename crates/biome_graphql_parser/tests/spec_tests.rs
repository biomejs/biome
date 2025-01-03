mod spec_test;

mod ok {
    //! Tests that are valid GraphQL
    tests_macros::gen_tests! {"tests/graphql_test_suite/ok/**/*.graphql", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail because they are not valid GraphQL
    tests_macros::gen_tests! {"tests/graphql_test_suite/err/**/*.graphql", crate::spec_test::run, "error"}
}
