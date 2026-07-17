//! Differential fuzzer: compares Biome's markdown HTML output against
//! commonmark.js reference output from a pre-generated corpus.
//!
//! The checked-in seed corpus (`seed.jsonl`) contains only passing cases.
//! Any failure is either a regression or a newly discovered mismatch.
//!
//! Run with: cargo test -p biome_markdown_parser --test fuzz_differential -- --ignored --nocapture

mod test_utils;

use biome_markdown_parser::{document_to_html, parse_markdown};
use biome_markdown_syntax::MdDocument;
use biome_rowan::AstNode;
use std::fs;
use std::path::{Path, PathBuf};
use test_utils::normalize_html;

/// FNV-1a 64-bit hash — deterministic across Rust toolchain versions.
fn content_hash(s: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in s.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

#[derive(serde::Deserialize)]
struct SeedCase {
    markdown: String,
    html: String,
}

struct Failure {
    hash: String,
    markdown: String,
    expected: String,
    actual: String,
}

fn run_corpus(path: &Path) -> (Vec<Failure>, usize) {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read corpus {}: {e}", path.display()));

    let mut failures = vec![];
    let mut total = 0usize;

    for (i, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let entry: SeedCase = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("Malformed JSON at {}:{}: {e}", path.display(), i + 1));

        let markdown = &entry.markdown;
        let expected_html = &entry.html;
        total += 1;

        let parsed = parse_markdown(markdown);
        let Some(doc) = MdDocument::cast(parsed.syntax()) else {
            failures.push(Failure {
                hash: content_hash(markdown),
                markdown: markdown.clone(),
                expected: expected_html.clone(),
                actual: "<parse failed>".to_string(),
            });
            continue;
        };

        let actual = document_to_html(
            &doc,
            parsed.list_tightness(),
            parsed.list_item_indents(),
            parsed.quote_indents(),
        );

        let expected_normalized = normalize_html(expected_html);
        let actual_normalized = normalize_html(&actual);

        if expected_normalized != actual_normalized {
            failures.push(Failure {
                hash: content_hash(markdown),
                markdown: markdown.clone(),
                expected: expected_html.clone(),
                actual,
            });
        }
    }

    (failures, total)
}

#[test]
#[ignore]
fn differential_fuzz_against_commonmark_js() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let corpus_dir = manifest_dir.join("tests/fuzz_corpus");

    // Always run the checked-in seed corpus (passing cases only)
    let seed_path = corpus_dir.join("seed.jsonl");
    let (mut all_failures, mut total) = run_corpus(&seed_path);

    // Optionally run an extended corpus if FUZZ_CORPUS env var is set
    if let Ok(extra_path) = std::env::var("FUZZ_CORPUS") {
        let (extra_failures, extra_total) = run_corpus(Path::new(&extra_path));
        all_failures.extend(extra_failures);
        total += extra_total;
    }

    // Write failure artifacts if FUZZ_FAILURES_DIR is set
    if let Ok(failures_dir) = std::env::var("FUZZ_FAILURES_DIR") {
        let dir = PathBuf::from(&failures_dir);
        fs::create_dir_all(&dir).expect("Failed to create failures directory");

        for failure in &all_failures {
            let base = dir.join(&failure.hash);
            fs::write(base.with_extension("md"), &failure.markdown).ok();
            fs::write(base.with_extension("expected.html"), &failure.expected).ok();
            fs::write(base.with_extension("actual.html"), &failure.actual).ok();
        }
    }

    // Print summary
    let passed = total - all_failures.len();
    eprintln!(
        "\nDifferential fuzz: {total} cases, {passed} passed, {} failed",
        all_failures.len()
    );

    if !all_failures.is_empty() {
        eprintln!("\n=== {} differential failures ===\n", all_failures.len());
        for (i, f) in all_failures.iter().enumerate().take(10) {
            eprintln!("--- Failure {} [{}] ---", i + 1, f.hash);
            eprintln!("Input:\n{}", f.markdown);
            eprintln!("Expected:\n{}", f.expected);
            eprintln!("Actual:\n{}", f.actual);
            eprintln!();
        }
        if all_failures.len() > 10 {
            eprintln!("... and {} more", all_failures.len() - 10);
        }
        panic!("{} differential mismatches found", all_failures.len());
    }

    eprintln!("All cases passed.");
}
