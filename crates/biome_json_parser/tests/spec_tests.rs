mod spec_test;

mod ok {
    //! Tests that must pass according to the JSON specification
    tests_macros::gen_tests! {"tests/json_test_suite/ok/*.json", crate::spec_test::run, "ok"}
}

mod err {
    //! Tests that must fail according to the JSON specification
    tests_macros::gen_tests! {"tests/json_test_suite/err/*.json", crate::spec_test::run, "error"}
}

mod undefined {
    //! parsers are free to accept or reject content
    tests_macros::gen_tests! {"tests/json_test_suite/undefined/*.json", crate::spec_test::run, "undefined"}
}

mod allow_comments {
    //! Tests should pass even with comments in json
    tests_macros::gen_tests! {"tests/json_test_suite/allow_comments/ok/*.json", crate::spec_test::run, "ok"}
}

mod allow_trainling_commas {
    //! Tests with trailing commas in json
    tests_macros::gen_tests! {"tests/json_test_suite/allow_trailing_commas/ok/*.json", crate::spec_test::run, "ok"}
    tests_macros::gen_tests! {"tests/json_test_suite/allow_trailing_commas/err/*.json", crate::spec_test::run, "error"}
}
