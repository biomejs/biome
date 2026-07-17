mod quick_test;
mod spec_test;

mod formatter {
    mod yaml_module {
        tests_macros::gen_tests! {"tests/specs/yaml/**/*.{yaml,yml}", crate::spec_test::run, ""}
    }
}
