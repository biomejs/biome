mod quick_test;
mod spec_test;

mod formatter {
    mod graphql_module {
        tests_macros::gen_tests! {"tests/specs/graphql/**/*.graphql", crate::spec_test::run, ""}
    }
}
