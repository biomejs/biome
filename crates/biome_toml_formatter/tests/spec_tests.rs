mod spec_test;

mod formatter {
    mod toml {
        tests_macros::gen_tests! {"tests/specs/toml/**/*.toml", crate::spec_test::run, ""}
    }
}
