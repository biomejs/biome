mod quick_test;
mod spec_test;

mod formatter {
    mod css_module {
        tests_macros::gen_tests! {"tests/specs/css/**/*.css", crate::spec_test::run, ""}
    }

    mod scss_module {
        tests_macros::gen_tests! {"tests/specs/scss/**/*.scss", crate::spec_test::run, ""}
    }
}

#[test]
#[ignore = "Deferred: hex-escape terminators gain a space before comments on reformat"]
fn deferred_escaped_identifier_comments_scss() {
    crate::spec_test::run(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/specs/deferred/scss/parity-function-adjacency-escape-comments.scss"
        ),
        "",
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/deferred/scss"),
        "",
    );
}
