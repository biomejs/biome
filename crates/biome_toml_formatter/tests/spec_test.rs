use biome_formatter::{Expand, IndentStyle, LineWidth};
use biome_toml_formatter::{TomlFormatOptions, format_node};
use biome_toml_parser::parse_toml;
use std::fmt::Write;
use std::path::Path;

/// Formats a spec file and snapshots the output.
///
/// The output must be stable when formatted again, and a document without
/// errors must still parse without errors after formatting. Files whose name
/// contains `expand_always`, `expand_never`, `tabs`, or `narrow` use the
/// matching option.
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let path = Path::new(spec_input_file);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let mut options = TomlFormatOptions::default();
    if file_name.contains("expand_always") {
        options = options.with_expand(Expand::Always);
    }
    if file_name.contains("expand_never") {
        options = options.with_expand(Expand::Never);
    }
    if file_name.contains("tabs") {
        options = options.with_indent_style(IndentStyle::Tab);
    }
    if file_name.contains("narrow") {
        options = options.with_line_width(LineWidth::try_from(40).unwrap());
    }

    let parse = parse_toml(&input);
    let output = format(&input, options.clone());

    let reformatted = format(&output, options.clone());
    similar_assert(&output, &reformatted, "formatting isn't idempotent");

    if !parse.has_errors() {
        let reparsed = parse_toml(&output);
        assert!(
            !reparsed.has_errors(),
            "the formatted document has errors: {:#?}\n{output}",
            reparsed.diagnostics()
        );
    }

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

fn format(source: &str, options: TomlFormatOptions) -> String {
    let parse = parse_toml(source);
    format_node(options, &parse.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code()
}

fn similar_assert(expected: &str, actual: &str, message: &str) {
    assert!(
        expected == actual,
        "{message}\n--- first pass ---\n{expected}\n--- second pass ---\n{actual}"
    );
}
