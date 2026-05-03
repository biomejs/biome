// CommonMark spec compliance tests (https://spec.commonmark.org/)
//
// Spec provenance:
//   Version: 0.31.2
//   URL: https://spec.commonmark.org/0.31.2/spec.json
//   Downloaded: 2026-01-24
//   Examples: 652
//
// To update the spec, run:
//   just update-commonmark-spec <version>
//
// After updating, verify with:
//   just test-markdown-conformance

use crate::runner::{TestCase, TestRunOutcome, TestSuite};
use biome_markdown_parser::{document_to_html, parse_markdown};
use biome_markdown_syntax::MdDocument;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{AstNode, TextRange};
use serde::Deserialize;
use std::io;
use std::path::Path;

const SPEC_JSON: &str = include_str!("spec.json");

#[derive(Debug, Deserialize)]
struct SpecTest {
    markdown: String,
    html: String,
    example: u32,
    section: String,
}

struct CommonMarkTestCase {
    name: String,
    markdown: String,
    expected_html: String,
}

impl TestCase for CommonMarkTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let parsed = parse_markdown(&self.markdown);

        let Some(document) = MdDocument::cast(parsed.syntax()) else {
            return TestRunOutcome::IncorrectlyErrored {
                errors: vec![ParseDiagnostic::new(
                    format!("Bogus node: {:?}", parsed.syntax().kind()),
                    TextRange::empty(0.into()),
                )],
                files: Default::default(),
            };
        };

        let actual = document_to_html(
            &document,
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );

        let expected = normalize_html(&self.expected_html);
        let actual_normalized = normalize_html(&actual);

        if expected == actual_normalized {
            TestRunOutcome::Passed(Default::default())
        } else {
            TestRunOutcome::IncorrectlyErrored {
                errors: vec![ParseDiagnostic::new(
                    format!(
                        "HTML mismatch\nExpected:\n{}\nActual:\n{}",
                        self.expected_html, actual
                    ),
                    TextRange::empty(0.into()),
                )],
                files: Default::default(),
            }
        }
    }
}

// Normalize HTML for comparison, preserving whitespace inside <pre> blocks.
fn normalize_html(html: &str) -> String {
    let mut result = Vec::new();
    let mut in_pre = false;

    for line in html.lines() {
        if line.contains("<pre") {
            in_pre = true;
        }

        if in_pre {
            result.push(line.to_string());
        } else {
            result.push(line.trim_end().to_string());
        }

        if line.contains("</pre>") {
            in_pre = false;
        }
    }

    result.join("\n").trim().to_string() + "\n"
}

pub(crate) struct CommonMarkTestSuite;

impl TestSuite for CommonMarkTestSuite {
    fn name(&self) -> &str {
        "markdown/commonmark"
    }

    fn base_path(&self) -> &str {
        "xtask/coverage/src/markdown"
    }

    fn checkout(&self) -> io::Result<()> {
        Ok(())
    }

    fn is_test(&self, _path: &Path) -> bool {
        false
    }

    fn load_test(&self, _path: &Path) -> Option<Box<dyn TestCase>> {
        None
    }

    fn load_all(&self) -> Option<Vec<Box<dyn TestCase>>> {
        let tests: Vec<SpecTest> =
            serde_json::from_str(SPEC_JSON).expect("Failed to parse spec.json");

        let cases = tests
            .into_iter()
            .map(|spec| {
                Box::new(CommonMarkTestCase {
                    name: format!("example_{:03}_{}", spec.example, spec.section),
                    markdown: spec.markdown,
                    expected_html: spec.html,
                }) as Box<dyn TestCase>
            })
            .collect();

        Some(cases)
    }
}
