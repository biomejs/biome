//! Parses every prefix of the valid spec fixtures. Truncation exercises the
//! error recovery of every construct; parsing must not panic and the tree must
//! stay lossless.

use biome_toml_parser::parse_toml;
use std::path::Path;

#[test]
fn truncated_documents_parse_losslessly() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/toml_test_suite/ok");
    let mut count = 0;

    for entry in std::fs::read_dir(fixtures).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|extension| extension != "toml") {
            continue;
        }
        count += 1;

        let source = std::fs::read_to_string(&path).unwrap();
        for (end, _) in source.char_indices().skip(1) {
            let prefix = &source[..end];
            let parse = parse_toml(prefix);
            assert_eq!(
                parse.syntax().to_string(),
                prefix,
                "{}: prefix of {end} bytes",
                path.display()
            );
        }
    }

    assert!(count > 0, "no fixtures found");
}
