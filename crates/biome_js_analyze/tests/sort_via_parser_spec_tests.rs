mod sort_via_parser_spec_test;

mod ok {
    tests_macros::gen_tests! {
        "tests/sort_via_parser/ok/**/*.txt",
        crate::sort_via_parser_spec_test::run,
        "ok"
    }
}
