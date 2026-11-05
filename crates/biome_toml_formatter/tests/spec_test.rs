use biome_rowan::Direction;
use biome_toml_formatter::{TomlFormatOptions, format_node};
use biome_toml_parser::parse_toml;
use biome_toml_syntax::{TomlSyntaxKind, TomlSyntaxNode};
use std::fmt::Write;
use std::path::Path;

/// Formats a spec file, which must be valid TOML, and snapshots the output,
/// which must pass [assert_formatting_preserves].
///
/// The formatter options come from an `options.json` next to the spec file,
/// if there is one.
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let path = Path::new(spec_input_file);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let options = load_options(&path.with_file_name("options.json"));

    let parse = parse_toml(&input);
    assert!(
        !parse.has_errors(),
        "spec inputs must be valid TOML: {:#?}",
        parse.diagnostics()
    );

    let output = format(&input, &options);
    assert_formatting_preserves(&input, &output, &options);

    let mut snapshot = String::new();
    writeln!(snapshot, "# Input\n\n```toml\n{input}```\n").unwrap();
    writeln!(snapshot, "# Options\n\n```\n{options}```\n").unwrap();
    writeln!(snapshot, "# Output\n\n```toml\n{output}```").unwrap();

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

// TODO: load `options.json` as a Biome configuration through
// `biome_formatter_test`, like the other formatters, once the configuration has
// a `toml` section and `biome_service` can format TOML files. Until then, this
// reads the `toml.formatter` section that configuration will have directly.
/// Reads the formatter options from the `toml.formatter` section of the
/// `options.json` at `path`, or returns the default options if there is no
/// such file.
fn load_options(path: &Path) -> TomlFormatOptions {
    let mut options = TomlFormatOptions::default();
    let Ok(content) = std::fs::read_to_string(path) else {
        return options;
    };

    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    let Some(formatter) = json["toml"]["formatter"].as_object() else {
        panic!("{}: expected a `toml.formatter` object", path.display());
    };

    for (name, value) in formatter {
        let value = match value {
            serde_json::Value::String(value) => value.clone(),
            value => value.to_string(),
        };
        let invalid = |error: &dyn std::fmt::Debug| -> ! {
            panic!("{}: invalid `{name}`: {error:?}", path.display())
        };
        match name.as_str() {
            "indentStyle" => {
                options.set_indent_style(value.parse().unwrap_or_else(|e| invalid(&e)))
            }
            "indentWidth" => {
                options.set_indent_width(value.parse().unwrap_or_else(|e| invalid(&e)))
            }
            "lineEnding" => options.set_line_ending(value.parse().unwrap_or_else(|e| invalid(&e))),
            "lineWidth" => options.set_line_width(value.parse().unwrap_or_else(|e| invalid(&e))),
            "expand" => options.set_expand(value.parse().unwrap_or_else(|e| invalid(&e))),
            "trailingNewline" => {
                options.set_trailing_newline(value.parse().unwrap_or_else(|e| invalid(&e)))
            }
            _ => panic!("{}: unknown formatter option `{name}`", path.display()),
        }
    }

    options
}

pub fn format(source: &str, options: &TomlFormatOptions) -> String {
    let parse = parse_toml(source);
    format_node(options.clone(), &parse.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code()
}

/// Asserts that `output`, the formatted `input`, is stable and, if `input` has
/// no errors, means the same as `input`: it has no errors, the same tokens
/// except commas, and the same comments in the same places.
pub fn assert_formatting_preserves(input: &str, output: &str, options: &TomlFormatOptions) {
    assert_eq!(
        format(output, options),
        output,
        "formatting isn't idempotent:\n{input}"
    );

    let original = parse_toml(input);
    if original.has_errors() {
        return;
    }

    let formatted = parse_toml(output);
    assert!(
        !formatted.has_errors(),
        "the formatted document has errors: {:#?}\n{output}",
        formatted.diagnostics()
    );
    assert_eq!(
        tokens(&formatted.syntax()),
        tokens(&original.syntax()),
        "formatting changed the tokens:\n{input}\n---\n{output}"
    );
    assert_eq!(
        comments(&formatted.syntax()),
        comments(&original.syntax()),
        "formatting changed the comments:\n{input}\n---\n{output}"
    );
}

/// Returns the text of every token except commas, which the formatter adds
/// and removes. Line breaks in multi-line strings are compared as LF because
/// the formatter prints them with the configured line ending.
fn tokens(root: &TomlSyntaxNode) -> Vec<String> {
    root.descendants_tokens(Direction::Next)
        .filter(|token| token.kind() != TomlSyntaxKind::COMMA)
        .map(|token| token.text_trimmed().replace("\r\n", "\n"))
        .collect()
}

/// Returns every comment in document order with the token that precedes it, so
/// moving a comment past a token changes the result. Commas are ignored
/// because the formatter moves them before trailing comments.
fn comments(root: &TomlSyntaxNode) -> Vec<(String, String)> {
    let mut comments = Vec::new();
    let mut previous = String::new();
    for token in root.descendants_tokens(Direction::Next) {
        for piece in token.leading_trivia().pieces() {
            if piece.is_comments() {
                comments.push((previous.clone(), piece.text().to_string()));
            }
        }
        if token.kind() != TomlSyntaxKind::COMMA {
            previous = token.text_trimmed().to_string();
        }
        for piece in token.trailing_trivia().pieces() {
            if piece.is_comments() {
                comments.push((previous.clone(), piece.text().to_string()));
            }
        }
    }
    comments
}
