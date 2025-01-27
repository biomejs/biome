use ascii_table::{Align, AsciiTable};
use std::collections::{HashMap, HashSet};

use crate::{Outcome, TestResult, TestResults};

pub fn emit_compare(
    base_results: &TestResults,
    new_results: &TestResults,
    test_suite: &str,
    markdown: bool,
) {
    let base_total = base_results.summary.tests_ran as isize;
    let new_total = new_results.summary.tests_ran as isize;
    let total_diff = new_total - base_total;

    let base_passed = base_results.summary.passed as isize;
    let new_passed = new_results.summary.passed as isize;
    let passed_diff = new_passed - base_passed;

    let base_failed = base_results.summary.failed as isize;
    let new_failed = new_results.summary.failed as isize;
    let failed_diff = new_failed - base_failed;

    let base_panics = base_results.summary.panics as isize;
    let new_panics = new_results.summary.panics as isize;
    let panics_diff = new_panics - base_panics;

    let base_coverage = base_results.summary.coverage;
    let new_coverage = new_results.summary.coverage;
    let coverage_diff = new_coverage - base_coverage;

    let report_diff = compare_diffs(base_results, new_results);

    if markdown {
        /// Generates a proper diff format, with some bold text if things change.
        fn diff_format(diff: isize, i_am_passed_results: bool, show_increase: bool) -> String {
            let good = "✅ ";
            let bad = "❌ ";
            let up = "⏫ ";
            let down = "⏬ ";

            let emoji = if show_increase {
                match diff.cmp(&0) {
                    std::cmp::Ordering::Less => {
                        if i_am_passed_results {
                            format!("{bad}{down}")
                        } else {
                            format!("{good}{down}")
                        }
                    }
                    std::cmp::Ordering::Equal => String::new(),
                    std::cmp::Ordering::Greater => {
                        if i_am_passed_results {
                            format!("{good}{up}")
                        } else {
                            format!("{bad}{up}")
                        }
                    }
                }
            } else {
                String::new()
            };

            format!(
                "{}{}{}{}{}",
                emoji,
                if diff != 0 { "**" } else { "" },
                if diff > 0 { "+" } else { "" },
                diff,
                if diff != 0 { "**" } else { "" }
            )
        }

        println!("\n### {test_suite}\n");

        println!("| Test result | `main` count | This PR count | Difference |");
        println!("| :---------: | :----------: | :-----------: | :--------: |");

        println!(
            "| Total | {} | {} | {} |",
            base_total,
            new_total,
            diff_format(total_diff, false, false)
        );

        println!(
            "| Passed | {} | {} | {} |",
            base_passed,
            new_passed,
            diff_format(passed_diff, true, true)
        );

        println!(
            "| Failed | {} | {} | {} |",
            base_failed,
            new_failed,
            diff_format(failed_diff, false, true)
        );

        println!(
            "| Panics | {} | {} | {} |",
            base_panics,
            new_panics,
            diff_format(panics_diff, false, true)
        );

        println!(
            "| Coverage | {:.2}% | {:.2}% | {} |",
            base_coverage,
            new_coverage,
            format_args!(
                "{}{}{:.2}%{}",
                if coverage_diff.abs() > f64::EPSILON {
                    "**"
                } else {
                    ""
                },
                if coverage_diff > 0_f64 { "+" } else { "" },
                coverage_diff,
                if coverage_diff.abs() > f64::EPSILON {
                    "**"
                } else {
                    ""
                },
            ),
        );

        fn summary(title: &str, tests: &[&TestResult]) {
            if !tests.is_empty() {
                println!();
                println!(
                    "<details><summary><b>{} ({}):</b></summary>",
                    title,
                    tests.len()
                );
                println!("\n```");
                let mut test_cases = tests.iter().map(|test| &test.test_case).collect::<Vec<_>>();
                test_cases.sort_unstable();
                for test_case in test_cases {
                    println!("{test_case}");
                }
                println!("```");
                println!("</details>");
            }
        }

        summary(":fire: Regression", &report_diff.regression);
        summary(":tada: Fixed", &report_diff.fixed);
        summary(":boom: Failed to Panic", &report_diff.failed_to_panic);
        summary(
            ":interrobang: Panic To Failed",
            &report_diff.panic_to_failed,
        );
        summary(":heavy_plus_sign: Added Tests", &report_diff.added_tests);
        summary(
            ":heavy_minus_sign: Removed Tests",
            &report_diff.removed_tests,
        );
    } else {
        let mut table = AsciiTable::default();

        println!("{test_suite} conformance changes:");

        table
            .column(0)
            .set_header("Tests result")
            .set_align(Align::Left);
        table
            .column(1)
            .set_header("main branch")
            .set_align(Align::Right);
        table.column(2).set_header("PR").set_align(Align::Right);
        table
            .column(3)
            .set_header("Difference")
            .set_align(Align::Right);

        let passed_diff = base_passed - new_passed;
        let failed_diff = base_failed - new_failed;
        let panics_diff = base_panics - new_panics;
        let passed_row: Vec<&dyn std::fmt::Display> =
            vec![&"Passed", &base_passed, &new_passed, &passed_diff];
        let failed_row: Vec<&dyn std::fmt::Display> =
            vec![&"Failed", &base_failed, &new_failed, &failed_diff];
        let panics_row: Vec<&dyn std::fmt::Display> =
            vec![&"Panics", &base_panics, &new_panics, &panics_diff];
        table.print(vec![passed_row, failed_row, panics_row]);
    }
}

struct ReportDiff<'a> {
    pub regression: Vec<&'a TestResult>,
    pub fixed: Vec<&'a TestResult>,
    pub failed_to_panic: Vec<&'a TestResult>,
    pub panic_to_failed: Vec<&'a TestResult>,
    pub added_tests: Vec<&'a TestResult>,
    pub removed_tests: Vec<&'a TestResult>,
}

impl ReportDiff<'_> {
    pub fn new() -> Self {
        Self {
            regression: vec![],
            fixed: vec![],
            failed_to_panic: vec![],
            panic_to_failed: vec![],
            added_tests: vec![],
            removed_tests: vec![],
        }
    }
}

fn compare_diffs<'a>(
    base_results: &'a TestResults,
    new_results: &'a TestResults,
) -> ReportDiff<'a> {
    let mut report_diff = ReportDiff::new();

    let mut all_test_cases: HashSet<&str> = HashSet::new();
    let mut base_by_test_case: HashMap<&str, &TestResult> = HashMap::new();
    for detail in base_results.details.iter() {
        all_test_cases.insert(&detail.test_case);
        base_by_test_case.insert(&detail.test_case, detail);
    }

    let mut new_by_test_case: HashMap<&str, &TestResult> = HashMap::new();
    for detail in new_results.details.iter() {
        all_test_cases.insert(&detail.test_case);
        new_by_test_case.insert(&detail.test_case, detail);
    }

    for path in all_test_cases {
        let base_result = base_by_test_case.get(path);
        let new_result = new_by_test_case.get(path);

        match (base_result, new_result) {
            (None, Some(new)) => {
                report_diff.added_tests.push(new);
            }
            (Some(base), None) => {
                report_diff.removed_tests.push(base);
            }
            (Some(base), Some(new)) => {
                match (&base.outcome, &new.outcome) {
                    (Outcome::Passed, Outcome::Failed | Outcome::Panicked) => {
                        report_diff.regression.push(new)
                    }
                    (Outcome::Failed | Outcome::Panicked, Outcome::Passed) => {
                        report_diff.fixed.push(new)
                    }
                    (Outcome::Failed, Outcome::Panicked) => report_diff.failed_to_panic.push(new),
                    (Outcome::Panicked, Outcome::Failed) => report_diff.panic_to_failed.push(new),
                    // we want to ignore cases where both results yield the same enum
                    // this means that their status hasn't changed, not worth tracking
                    _ => {}
                }
            }
            _ => unreachable!(),
        }
    }

    report_diff
}
