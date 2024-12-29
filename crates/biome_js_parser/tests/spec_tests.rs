mod spec_test;

mod ok {
    tests_macros::gen_tests! {"tests/js_test_suite/ok/**/*.{js,cjs,mjs,jsx,ts,tsx,d.ts}", crate::spec_test::run, "ok"}
}
mod err {
    tests_macros::gen_tests! {"tests/js_test_suite/error/**/*.{js,cjs,mjs,jsx,ts,tsx,d.ts}", crate::spec_test::run, "error"}
}
