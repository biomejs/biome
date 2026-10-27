mod spec_test;

mod formatter {
    mod toml {
        tests_macros::gen_tests! {"tests/specs/toml/**/*.toml", crate::spec_test::run, ""}
    }
}

/// Formats the TOML 1.1.0 documents of
/// [toml-test](https://github.com/toml-lang/toml-test), vendored by
/// `biome_toml_parser`.
mod toml_test {
    use crate::spec_test::{assert_formatting_preserves, format};
    use biome_formatter::{Expand, IndentStyle, LineWidth};
    use biome_toml_formatter::{TomlFormatOptions, format_node};
    use biome_toml_parser::parse_toml;
    use std::path::Path;

    const SUITE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../biome_toml_parser/tests/toml-test"
    );

    fn files(prefix: &str) -> Vec<String> {
        std::fs::read_to_string(Path::new(SUITE).join("files-toml-1.1.0"))
            .unwrap()
            .lines()
            .filter(|file| file.starts_with(prefix) && file.ends_with(".toml"))
            .map(str::to_string)
            .collect()
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

        let files = files("valid/");
        assert!(files.len() > 200, "the suite is incomplete");

        for file in files {
            let source = std::fs::read_to_string(Path::new(SUITE).join(&file)).unwrap();
            for options in &option_sets {
                let output = format(&source, options);
                assert_formatting_preserves(&source, &output, options);
            }
        }
    }

    /// Formats documents with errors: the invalid documents and every prefix of
    /// the valid ones. Formatting may refuse a document with errors, but must
    /// not panic, and must only change whitespace and commas when it succeeds.
    #[test]
    fn formatting_documents_with_errors_doesnt_panic() {
        let options = TomlFormatOptions::default();

        for file in files("") {
            let Ok(source) =
                String::from_utf8(std::fs::read(Path::new(SUITE).join(&file)).unwrap())
            else {
                continue;
            };

            let mut inputs = vec![source.as_str()];
            if file.starts_with("valid/") {
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
                    "{file}: formatting changed more than whitespace:\n{input}\n---\n{printed}"
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
