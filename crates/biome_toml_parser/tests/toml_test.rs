//! Runs the TOML 1.1.0 cases of [toml-test](https://github.com/toml-lang/toml-test).
//!
//! Valid documents must parse without diagnostics. Invalid documents must
//! produce a diagnostic.

use biome_toml_parser::parse_toml;
use std::fmt::Write;
use std::path::Path;

const SUITE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/toml-test");

#[test]
fn toml_test_suite() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();
    let mut failures = String::new();
    let mut valid = 0;
    let mut invalid = 0;

    for file in files.lines().filter(|file| file.ends_with(".toml")) {
        let bytes = std::fs::read(suite.join(file)).unwrap();

        if file.starts_with("valid/") {
            valid += 1;
            let source = match String::from_utf8(bytes) {
                Ok(source) => source,
                Err(_) => {
                    writeln!(failures, "{file}: valid document isn't UTF-8").unwrap();
                    continue;
                }
            };

            let parse = parse_toml(&source);
            assert_eq!(
                parse.syntax().to_string(),
                source,
                "{file}: tree isn't lossless"
            );

            if parse.has_errors() {
                let messages: Vec<_> = parse
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| format!("{:?}", diagnostic))
                    .collect();
                writeln!(failures, "{file}: rejected valid document: {messages:#?}").unwrap();
            }
        } else {
            invalid += 1;
            // Invalid UTF-8 can't be represented as `&str` and is rejected
            // before parsing.
            let Ok(source) = String::from_utf8(bytes) else {
                continue;
            };

            let parse = parse_toml(&source);
            assert_eq!(
                parse.syntax().to_string(),
                source,
                "{file}: tree isn't lossless"
            );

            if !parse.has_errors() {
                writeln!(failures, "{file}: accepted invalid document").unwrap();
            }
        }
    }

    assert!(valid > 200 && invalid > 400, "the suite is incomplete");
    assert!(failures.is_empty(), "toml-test failures:\n{failures}");
}

/// Parses every prefix of every valid document. Truncation exercises the error
/// recovery of every construct; parsing must not panic and must stay lossless.
#[test]
fn truncated_documents_parse_losslessly() {
    let suite = Path::new(SUITE);
    let files = std::fs::read_to_string(suite.join("files-toml-1.1.0")).unwrap();

    for file in files
        .lines()
        .filter(|file| file.starts_with("valid/") && file.ends_with(".toml"))
    {
        let source = std::fs::read_to_string(suite.join(file)).unwrap();
        for (end, _) in source.char_indices().skip(1) {
            let prefix = &source[..end];
            let parse = parse_toml(prefix);
            assert_eq!(
                parse.syntax().to_string(),
                prefix,
                "{file}: prefix of {end} bytes"
            );
        }
    }
}
