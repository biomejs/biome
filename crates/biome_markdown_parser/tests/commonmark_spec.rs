//! CommonMark specification compliance test harness.
//!
//! This test runs all 652 CommonMark spec examples against Biome's markdown parser
//! and reports the compliance percentage.
//!
//! Run with: `cargo test -p biome_markdown_parser --test commonmark_spec -- --nocapture`

use biome_markdown_parser::{document_to_html, parse_markdown};
use biome_markdown_syntax::MdDocument;
use biome_rowan::AstNode;
use serde::Deserialize;

/// Embedded CommonMark spec test cases.
const SPEC_JSON: &str = include_str!("spec.json");

/// A single test case from the CommonMark spec.
#[derive(Debug, Deserialize)]
struct SpecTest {
    /// The markdown input
    markdown: String,
    /// The expected HTML output
    html: String,
    /// The example number in the spec
    example: u32,
    /// The section name
    section: String,
}

/// Information about a failed test.
#[derive(Debug)]
struct FailedTest {
    example: u32,
    section: String,
    markdown: String,
    expected: String,
    actual: String,
}

/// Normalize HTML for comparison.
///
/// CommonMark spec tests are strict about HTML output, but there are some
/// acceptable variations in whitespace that we handle here.
///
/// IMPORTANT: We preserve whitespace inside `<pre>` blocks since trailing
/// spaces are significant in code blocks per CommonMark spec.
fn normalize_html(html: &str) -> String {
    let mut result = Vec::new();
    let mut in_pre = false;

    for line in html.lines() {
        // Track <pre> block state
        // Note: CommonMark output has <pre><code> on same line, so check for <pre
        if line.contains("<pre") {
            in_pre = true;
        }

        // Only trim trailing whitespace outside of <pre> blocks
        if in_pre {
            result.push(line.to_string());
        } else {
            result.push(line.trim_end().to_string());
        }

        // Check for </pre> after processing the line
        if line.contains("</pre>") {
            in_pre = false;
        }
    }

    result.join("\n").trim().to_string() + "\n"
}

/// Show a unified diff between expected and actual HTML.
fn diff(expected: &str, actual: &str) -> String {
    let mut result = String::new();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();

    let max_lines = expected_lines.len().max(actual_lines.len());

    for i in 0..max_lines {
        let exp = expected_lines.get(i).unwrap_or(&"");
        let act = actual_lines.get(i).unwrap_or(&"");

        if exp != act {
            result.push_str(&format!("- {}\n", exp));
            result.push_str(&format!("+ {}\n", act));
        } else {
            result.push_str(&format!("  {}\n", exp));
        }
    }

    result
}

#[test]
fn commonmark_spec_compliance() {
    let tests: Vec<SpecTest> = serde_json::from_str(SPEC_JSON).expect("Failed to parse spec.json");
    let total = tests.len();

    let mut passed = 0;
    let mut failed: Vec<FailedTest> = Vec::new();
    let mut section_stats: std::collections::HashMap<String, (u32, u32)> =
        std::collections::HashMap::new();

    let log_progress = std::env::var("CMARK_PROGRESS").is_ok();
    for (index, test) in tests.iter().enumerate() {
        if log_progress {
            println!(
                "progress {}/{} example {} {}",
                index + 1,
                total,
                test.example,
                test.section
            );
        }
        let parsed = parse_markdown(&test.markdown);

        // Handle bogus nodes gracefully - count as failure instead of panicking
        let Some(document) = MdDocument::cast(parsed.syntax()) else {
            let section_entry = section_stats.entry(test.section.clone()).or_insert((0, 0));
            section_entry.1 += 1;
            failed.push(FailedTest {
                example: test.example,
                section: test.section.clone(),
                markdown: test.markdown.clone(),
                expected: test.html.clone(),
                actual: format!("<BOGUS NODE: {:?}>", parsed.syntax().kind()),
            });
            continue;
        };

        let actual = document_to_html(
            &document,
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );

        let expected_normalized = normalize_html(&test.html);
        let actual_normalized = normalize_html(&actual);

        let section_entry = section_stats.entry(test.section.clone()).or_insert((0, 0));
        section_entry.1 += 1; // total for section

        if expected_normalized == actual_normalized {
            passed += 1;
            section_entry.0 += 1; // passed for section
        } else {
            failed.push(FailedTest {
                example: test.example,
                section: test.section.clone(),
                markdown: test.markdown.clone(),
                expected: test.html.clone(),
                actual,
            });
        }
    }

    // Print summary
    println!("\n");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("                    CommonMark Spec Compliance Report");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    println!(
        "Overall: {}/{} ({:.1}%)",
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    );
    println!();

    // Print section breakdown
    println!("Section Breakdown:");
    println!("─────────────────────────────────────────────────────────────────────────────────");

    let mut sections: Vec<_> = section_stats.iter().collect();
    sections.sort_by_key(|(name, _)| *name);

    for (section, (section_passed, section_total)) in sections {
        let pct = (*section_passed as f64 / *section_total as f64) * 100.0;
        let status = if pct == 100.0 {
            "✓"
        } else if pct >= 80.0 {
            "○"
        } else {
            "✗"
        };
        println!(
            "  {} {:40} {:3}/{:3} ({:5.1}%)",
            status, section, section_passed, section_total, pct
        );
    }
    println!();

    // Print failures (limited to first 50)
    if !failed.is_empty() {
        println!("Failed Examples (showing first 50):");
        println!(
            "─────────────────────────────────────────────────────────────────────────────────"
        );

        for (i, failure) in failed.iter().take(50).enumerate() {
            println!();
            println!(
                "{}. Example {} [{}]",
                i + 1,
                failure.example,
                failure.section
            );
            println!("   Input:");
            for line in failure.markdown.lines() {
                println!("   │ {:?}", line);
            }
            println!("   Expected:");
            for line in failure.expected.lines() {
                println!("   │ {}", line);
            }
            println!("   Actual:");
            for line in failure.actual.lines() {
                println!("   │ {}", line);
            }
            println!("   Diff:");
            for line in diff(&failure.expected, &failure.actual).lines() {
                println!("   │ {}", line);
            }
        }

        if failed.len() > 50 {
            println!();
            println!("... and {} more failures", failed.len() - 50);
        }
    }

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");

    // For now, we don't fail the test - we're just measuring compliance
    // Once we reach high compliance, we can enable this assertion
    // assert!(passed == total, "Not all CommonMark spec tests pass");

    // Report the overall result
    let compliance_pct = (passed as f64 / total as f64) * 100.0;
    if compliance_pct < 50.0 {
        println!(
            "WARNING: Compliance is below 50% ({:.1}%). Parser may need significant work.",
            compliance_pct
        );
    }
}

/// Run a single example for debugging.
#[test]
#[ignore]
fn debug_single_example() {
    let tests: Vec<SpecTest> = serde_json::from_str(SPEC_JSON).expect("Failed to parse spec.json");

    // Change this to debug a specific example
    let example_num = 228;

    if let Some(test) = tests.iter().find(|t| t.example == example_num) {
        println!("Example {}: {}", test.example, test.section);
        println!("Markdown: {:?}", test.markdown);
        println!();

        let parsed = parse_markdown(&test.markdown);

        println!("CST (raw syntax):");
        println!("{:#?}", parsed.syntax());
        println!();

        println!("AST:");
        if parsed.syntax().kind() == biome_markdown_syntax::MarkdownSyntaxKind::MD_DOCUMENT {
            println!("{:#?}", parsed.tree());
        } else {
            println!(
                "Cannot cast to MdDocument - root is {:?}",
                parsed.syntax().kind()
            );
        }
        println!();
        println!();

        if parsed.has_errors() {
            println!("Parse errors:");
            for diag in parsed.diagnostics() {
                println!("  - {:?}", diag);
            }
            println!();
        }

        println!("List tightness: {:?}", parsed.list_tightness());
        println!();

        let actual = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );

        println!("Expected HTML:");
        println!("{}", test.html);
        println!();

        println!("Actual HTML:");
        println!("{}", actual);
        println!();

        let expected_normalized = normalize_html(&test.html);
        let actual_normalized = normalize_html(&actual);

        if expected_normalized == actual_normalized {
            println!("✓ PASS");
        } else {
            println!("✗ FAIL");
            println!("Diff:");
            println!("{}", diff(&test.html, &actual));
        }
    } else {
        println!("Example {} not found", example_num);
    }
}

/// Test specific sections for focused debugging.
#[test]
#[ignore]
fn debug_section() {
    let tests: Vec<SpecTest> = serde_json::from_str(SPEC_JSON).expect("Failed to parse spec.json");

    // Change this to debug a specific section
    let section = "List items";

    let section_tests: Vec<_> = tests.iter().filter(|t| t.section == section).collect();

    println!("Section: {} ({} tests)", section, section_tests.len());
    println!();

    let mut passed = 0;
    for test in &section_tests {
        let parsed = parse_markdown(&test.markdown);
        let actual = document_to_html(
            &parsed.tree(),
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );

        let expected_normalized = normalize_html(&test.html);
        let actual_normalized = normalize_html(&actual);

        if expected_normalized == actual_normalized {
            passed += 1;
            println!("  ✓ Example {}", test.example);
        } else {
            println!("  ✗ Example {}", test.example);
            println!("    Input: {:?}", test.markdown);
            println!("    Expected: {:?}", test.html);
            println!("    Actual: {:?}", actual);
        }
    }

    println!();
    println!(
        "Result: {}/{} ({:.1}%)",
        passed,
        section_tests.len(),
        (passed as f64 / section_tests.len() as f64) * 100.0
    );
}
