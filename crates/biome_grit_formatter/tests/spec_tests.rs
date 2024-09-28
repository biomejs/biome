mod spec_test;

mod formatter {

    mod grit_module {
        tests_macros::gen_tests! {"tests/specs/grit/*.grit", crate::spec_test::run, ""}
    }

    mod grit_patterns_module {
        tests_macros::gen_tests! {"tests/specs/grit/patterns/*.grit", crate::spec_test::run, "patterns"}
    }
}
