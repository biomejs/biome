use super::*;
use biome_analyze::{AnalyzerOptions, Never, RuleCategoriesBuilder, RuleFilter};
use biome_diagnostics::category;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Severity, print_diagnostic_to_string};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_package::{Dependencies, PackageJson};
use std::slice;

#[ignore]
#[test]
fn quick_test() {
    const SOURCE: &str = r#"f({ prop: () => {} })"#;

    let parsed = parse(SOURCE, JsFileSource::tsx(), JsParserOptions::default());

    let mut error_ranges: Vec<TextRange> = Vec::new();
    let options = AnalyzerOptions::default();
    let rule_filter = RuleFilter::Rule("nursery", "useExplicitType");

    let dependencies = Dependencies(Box::new([("buffer".into(), "latest".into())]));

    let services = crate::JsAnalyzerServices::from((
        Default::default(),
        project_layout_with_top_level_dependencies(dependencies),
        JsFileSource::tsx(),
    ));

    crate::analyze(
        &parsed.tree(),
        AnalysisFilter {
            enabled_rules: Some(slice::from_ref(&rule_filter)),
            ..AnalysisFilter::default()
        },
        &options,
        &[],
        services,
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                error_ranges.push(diag.location().span.unwrap());
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
            }

            for action in signal.actions() {
                let new_code = action.mutation.commit();
                eprintln!("new code!!!");
                eprintln!("{new_code}");
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    // assert_eq!(error_ranges.as_slice(), &[]);
}

#[test]
fn quick_test_suppression() {
    const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                // biome-ignore lint/suspicious:whole group
                p == f;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                j == k;
            }
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        AnalysisFilter::default(),
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path("example.js")
                    .with_file_source_code(SOURCE);

                let code = error.category().unwrap();
                if code == category!("lint/suspicious/noDoubleEquals") {
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    panic!("unexpected diagnostic");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn suppression() {
    const SOURCE: &str = "
            function checkSuppressions1(a, b) {
                a == b;
                // biome-ignore lint/suspicious:whole group
                p == f;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                j == k;
                /* biome-ignore lint/complexity/useWhile: multiple block comments */ /* biome-ignore lint/suspicious/noDoubleEquals: multiple block comments */
                o == m;
                // biome-ignore lint/complexity/useWhile: multiple line comments
                // biome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                d == x;
                z == v;
            }

            // biome-ignore lint/suspicious/noDoubleEquals: do not suppress warning for the whole function
            function checkSuppressions2(a, b) {
                a == b;
            }

            function checkSuppressions3(a, b) {
                a == b;
                // biome-ignore lint/suspicious: whole group
                a == b;
                // biome-ignore lint/suspicious/noDoubleEquals: single rule
                a == b;
                /* biome-ignore lint/complexity/useWhile: multiple block comments */ /* biome-ignore lint(suspicious/noDoubleEquals): multiple block comments */
                a == b;
                // biome-ignore lint/complexity/useWhile: multiple line comments
                // biome-ignore lint/suspicious/noDoubleEquals: multiple line comments
                a == b;
                a == b;
            }

            // biome-ignore lint(suspicious/noDoubleEquals): do not suppress warning for the whole function
            function checkSuppressions4(a, b) {
                a == b;
            }

            function checkSuppressions5() {
                // biome-ignore format explanation
                // biome-ignore format(:
                // biome-ignore (value): explanation
                // biome-ignore unknown: explanation
            }
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let mut lint_ranges: Vec<TextRange> = Vec::new();
    let mut parse_ranges: Vec<TextRange> = Vec::new();

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        AnalysisFilter::default(),
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let span = diag.get_span();
                let error = diag
                    .with_severity(Severity::Warning)
                    .with_file_path("example.js")
                    .with_file_source_code(SOURCE);

                let code = error.category().unwrap();
                if code == category!("lint/suspicious/noDoubleEquals") {
                    let text = print_diagnostic_to_string(&error);
                    eprintln!("{text}");
                    lint_ranges.push(span.unwrap());
                }

                if code == category!("suppressions/parse") {
                    parse_ranges.push(span.unwrap());
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
    assert_eq!(
        lint_ranges.as_slice(),
        &[
            TextRange::new(TextSize::from(67), TextSize::from(69)),
            TextRange::new(TextSize::from(651), TextSize::from(653)),
            TextRange::new(TextSize::from(845), TextSize::from(847)),
            TextRange::new(TextSize::from(932), TextSize::from(934)),
            TextRange::new(TextSize::from(1518), TextSize::from(1520)),
            TextRange::new(TextSize::from(1713), TextSize::from(1715)),
        ]
    );

    assert_eq!(
        parse_ranges.as_slice(),
        &[
            TextRange::new(TextSize::from(1817), TextSize::from(1828)),
            TextRange::new(TextSize::from(1868), TextSize::from(1869)),
            TextRange::new(TextSize::from(1902), TextSize::from(1903)),
            TextRange::new(TextSize::from(1955), TextSize::from(1962)),
        ]
    );
}

#[test]
fn suppression_syntax() {
    const SOURCE: &str = "
            // biome-ignore lint/suspicious/noDoubleEquals: single rule
            a == b;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_syntax().build(),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code != category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_simple() {
    const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
let bar = 33;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_multiple() {
    const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/

/**
* biome-ignore-all lint/suspicious/noDebugger: reason2
*/


let foo = 2;
let bar = 33;
debugger;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_all() {
    const SOURCE: &str = "
/**
* biome-ignore-all lint: reason
*/

let foo = 2;
let bar = 33;
debugger;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_multiple2() {
    const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
* biome-ignore-all lint/suspicious/noDebugger: reason2
*/


let foo = 2;
let bar = 33;
debugger;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_with_unused() {
    const SOURCE: &str = "
/**
* biome-ignore-all lint/style/useConst: reason
*/


let foo = 2;
/**
* biome-ignore lint/style/useConst: reason
*/
let bar = 33;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code != category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_with_block_comment() {
    const SOURCE: &str = "
/*
* Top level comment here. It could be a banner or a license comment
* MIT
*/
/**
* biome-ignore-all lint/style/useConst: reason
*/

let foo = 2;
let bar = 33;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_with_shebang() {
    const SOURCE: &str = "#!/usr/bin/env bun
/**
* biome-ignore-all lint/style/useConst: reason
*/

let foo = 2;
let bar = 33;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn top_level_suppression_with_shebang_and_comment() {
    const SOURCE: &str = "#!/usr/bin/env bun

/* Arbitrary comment here
 */
/**
* biome-ignore-all lint/style/useConst: reason
*/

let foo = 2;
let bar = 33;
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("style", "useConst")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn suppression_range_should_report_after_end() {
    const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/style/useConst: single rule
a == b;
let c;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a == b;
let c;
// biome-ignore-end lint/style/useConst: single rule
        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[
            RuleFilter::Rule("suspicious", "noDoubleEquals"),
            RuleFilter::Rule("style", "useConst"),
        ]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code != category!("lint/suspicious/noDoubleEquals") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn suppression_range_should_report_after_end_v2() {
    const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/suspicious/noDebugger: single rule
a == b;
debugger;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a === b;
debugger;
// biome-ignore-end lint/suspicious/noDebugger: single rule
debugger;

        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[
            RuleFilter::Rule("suspicious", "noDoubleEquals"),
            RuleFilter::Rule("suspicious", "noDebugger"),
        ]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    let mut has_diagnostics = false;
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                has_diagnostics = true;
                let code = diag.category().unwrap();
                if code != category!("lint/suspicious/noDebugger") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
    assert!(has_diagnostics, "must have diagnostics");
}

#[test]
fn suppression_range_should_not_report_after_end() {
    const SOURCE: &str = "
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
// biome-ignore-start lint/style/useConst: single rule
a == b;
let c;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule
a === b;
let f;
// biome-ignore-end lint/style/useConst: single rule
let d;

        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[
            RuleFilter::Rule("suspicious", "noDoubleEquals"),
            RuleFilter::Rule("style", "useConst"),
        ]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let error = diag
                    .with_file_path("dummyFile")
                    .with_file_source_code(SOURCE);
                let text = print_diagnostic_to_string(&error);
                eprintln!("{text}");
                panic!("Unexpected diagnostic");
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn suppression_range_should_report_when_contains_inner_comment() {
    const SOURCE: &str = "
// biome-ignore lint/complexity/useArrowFunction: single rule
const foo0 = function (bar: string) {
  // biome-ignore lint/style/noParameterAssign: single rule
  bar = 'baz';
};";

    let parsed = parse(SOURCE, JsFileSource::ts(), JsParserOptions::default());

    let enabled_rules = vec![
        RuleFilter::Rule("complexity", "useArrowFunction"),
        RuleFilter::Rule("style", "noParameterAssign"),
    ];

    let filter = AnalysisFilter {
        enabled_rules: Some(enabled_rules.as_slice()),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default();
    let root = parsed.tree();

    let services = crate::JsAnalyzerServices::from((
        Default::default(),
        Default::default(),
        JsFileSource::ts(),
    ));

    crate::analyze(&root, filter, &options, &[], services, |signal| {
        if let Some(diag) = signal.diagnostic() {
            let error = diag
                .with_file_path("dummyFile")
                .with_file_source_code(SOURCE);
            let text = print_diagnostic_to_string(&error);
            eprintln!("{text}");
            panic!("Unexpected diagnostic");
        }

        ControlFlow::<Never>::Continue(())
    });
}

#[test]
fn unused_range_suppression() {
    const SOURCE: &str = "
// biome-ignore-all lint/suspicious/noDoubleEquals: single rule
a == b;
// biome-ignore-start lint/suspicious/noDoubleEquals: single rule
a == b;
a == b;
// biome-ignore-end lint/suspicious/noDoubleEquals: single rule

        ";

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default().with_lint().build(),
        enabled_rules: Some(&[RuleFilter::Rule("suspicious", "noDoubleEquals")]),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    let mut has_diagnostics = false;
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                has_diagnostics = true;
                let code = diag.category().unwrap();
                if code != category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
    assert!(has_diagnostics, "must have diagnostics");
}

#[test]
fn suppression_action() {
    const SOURCE: &str = r#"// biome-ignore lint: tst
var foo = {
	zod: "lorem", // the text range starts at this line, which is missed by the suppression comment
	ipsum: "ipsum",
}
        "#;

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default()
            .with_assist()
            .with_lint()
            .with_syntax()
            .build(),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    crate::analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code == category!("suppressions/unused") {
                    panic!("unexpected diagnostic {code:?}");
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );
}

#[test]
fn suppression_issue_5562() {
    const SOURCE: &str = r#"console.log("should error");

// biome-ignore lint/suspicious/noConsole: foo
console.log("should be suppressed");"#;

    let parsed = parse(
        SOURCE,
        JsFileSource::js_module(),
        JsParserOptions::default(),
    );

    let filter = AnalysisFilter {
        categories: RuleCategoriesBuilder::default()
            .with_assist()
            .with_lint()
            .with_syntax()
            .build(),
        ..AnalysisFilter::default()
    };

    let options = AnalyzerOptions::default();
    let mut diagnostic_found = false;
    analyze(
        &parsed.tree(),
        filter,
        &options,
        &[],
        Default::default(),
        |signal| {
            if let Some(diag) = signal.diagnostic() {
                let code = diag.category().unwrap();
                if code == category!("lint/suspicious/noConsole") {
                    diagnostic_found = true
                }
            }

            ControlFlow::<Never>::Continue(())
        },
    );

    assert!(diagnostic_found, "must have diagnostics");
}

fn project_layout_with_top_level_dependencies(dependencies: Dependencies) -> Arc<ProjectLayout> {
    let manifest = PackageJson::default().with_dependencies(dependencies);

    let project_layout = ProjectLayout::default();
    project_layout.insert_node_manifest("/".into(), manifest);

    Arc::new(project_layout)
}
