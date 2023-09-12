mod quick_test;
mod spec_test;

mod formatter {

    mod js_module {
        tests_macros::gen_tests! {"tests/specs/js/module/**/*.js", crate::spec_test::run, "module"}
    }

    mod js_script {
        tests_macros::gen_tests! {"tests/specs/js/script/**/*.js", crate::spec_test::run, "script"}
    }

    mod ts_module {
        tests_macros::gen_tests! {"tests/specs/ts/**/*.ts", crate::spec_test::run, "module"}
    }
    mod jsx_module {
        tests_macros::gen_tests! {"tests/specs/jsx/**/*.jsx", crate::spec_test::run, "module"}
    }

    mod tsx_module {
        tests_macros::gen_tests! {"tests/specs/tsx/**/*.tsx", crate::spec_test::run, "module"}
    }
}
