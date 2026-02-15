mod spec_test;

// tests/specs/markdown folder does not exist yet
// TODO (tidefield): Figure out how to initiate this suite of tests

mod formatter {
    mod markdown_module {
        tests_macros::gen_tests! {"tests/specs/markdown/**/*.md", crate::spec_test::run, ""}
    }
}
