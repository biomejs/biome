mod quick_test;
mod spec_test;

mod formatter {
    mod css_module {
        tests_macros::gen_tests! {"tests/specs/css/**/*.css", crate::spec_test::run, ""}
    }
}
