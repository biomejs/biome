mod quick_test;
mod spec_test;

mod formatter {

    mod html {
        tests_macros::gen_tests! {"tests/specs/html/**/*.{html,vue}", crate::spec_test::run, ""}
    }
}
