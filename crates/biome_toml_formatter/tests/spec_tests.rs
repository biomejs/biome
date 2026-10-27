mod spec_test;

mod formatter {
    mod toml {
        tests_macros::gen_tests! {"tests/specs/toml/**/*.toml", crate::spec_test::run, ""}
    }
}

/// Formats the spec fixtures of `biome_toml_parser`: the valid documents in
/// `ok/` and the documents with errors in `err/`.
mod parser_fixtures {
    use crate::spec_test::{assert_formatting_preserves, format};
    use biome_formatter::{Expand, IndentStyle, LineWidth};
    use biome_toml_formatter::{TomlFormatOptions, format_node};
    use biome_toml_parser::parse_toml;
    use std::path::PathBuf;

    const FIXTURES: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../biome_toml_parser/tests/toml_test_suite"
    );

    fn fixtures(directory: &str) -> Vec<PathBuf> {
        let mut paths: Vec<_> = std::fs::read_dir(PathBuf::from(FIXTURES).join(directory))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "toml")
            })
            .collect();
        paths.sort();
        assert!(!paths.is_empty(), "no fixtures in {directory}");
        paths
    }

    /// Formats every valid document with several option sets.
    #[test]
    fn formatting_preserves_valid_documents() {
        let option_sets = [
            TomlFormatOptions::default(),
            TomlFormatOptions::default().with_expand(Expand::Always),
            TomlFormatOptions::default()
                .with_expand(Expand::Never)
                .with_indent_style(IndentStyle::Space)
                .with_line_width(LineWidth::try_from(20).unwrap()),
        ];

        for path in fixtures("ok") {
            let source = std::fs::read_to_string(&path).unwrap();
            for options in &option_sets {
                let output = format(&source, options);
                assert_formatting_preserves(&source, &output, options);
            }
        }
    }

    /// Formats documents with errors: the documents in `err/` and every prefix
    /// of the documents in `ok/`. Formatting may refuse a document with
    /// errors, but must not panic, and must only change whitespace and commas
    /// when it succeeds.
    #[test]
    fn formatting_documents_with_errors_doesnt_panic() {
        let options = TomlFormatOptions::default();
        let valid = fixtures("ok");

        for path in fixtures("err").iter().chain(&valid) {
            let source = std::fs::read_to_string(path).unwrap();

            let mut inputs = vec![source.as_str()];
            if valid.contains(path) {
                inputs.extend(source.char_indices().skip(1).map(|(end, _)| &source[..end]));
            }

            for input in inputs {
                let parse = parse_toml(input);
                let Ok(formatted) = format_node(options.clone(), &parse.syntax()) else {
                    continue;
                };
                let printed = formatted.print().unwrap().into_code();
                assert_eq!(
                    strip_whitespace(&printed),
                    strip_whitespace(input),
                    "{}: formatting changed more than whitespace:\n{input}\n---\n{printed}",
                    path.display()
                );
            }
        }
    }

    /// Removes whitespace and commas, which the formatter can add or remove.
    fn strip_whitespace(text: &str) -> String {
        text.chars()
            .filter(|char| !char.is_whitespace() && *char != ',')
            .collect()
    }
}
