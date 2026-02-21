mod spec_test;

mod formatter {
    mod markdown_module {
        tests_macros::gen_tests! {"tests/specs/markdown/**/*.md", crate::spec_test::run, ""}
    }
}
