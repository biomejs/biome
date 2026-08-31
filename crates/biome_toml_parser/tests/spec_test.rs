use biome_console::{
    fmt::{Formatter, Termcolor},
    markup,
};
use biome_diagnostics::{DiagnosticExt, display::PrintDiagnostic, termcolor};
use biome_rowan::SyntaxKind;
use biome_test_utils::validate_eof_token;
use biome_toml_parser::parse_toml;
use std::{fmt::Write, fs, path::Path};

#[derive(Clone, Copy)]
enum ExpectedOutcome {
    Pass,
    Fail,
}

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, outcome: &str) {
    let outcome = match outcome {
        "ok" => ExpectedOutcome::Pass,
        "error" => ExpectedOutcome::Fail,
        _ => panic!("Invalid expected outcome {outcome}"),
    };
    let test_case_path = Path::new(test_case);
    let file_name = test_case_path
        .file_name()
        .expect("Expected test to have a file name")
        .to_str()
        .expect("Expected file name to be valid UTF-8");
    let content =
        fs::read_to_string(test_case_path).expect("Expected test path to contain readable UTF-8");
    let parsed = parse_toml(&content);

    validate_eof_token(parsed.syntax());
    let formatted_ast = format!("{:#?}", parsed.tree());
    let mut snapshot = String::new();
    writeln!(snapshot, "## Input\n```toml\n{content}\n```\n").unwrap();
    writeln!(
        snapshot,
        "## AST\n\n```\n{formatted_ast}\n```\n\n## CST\n\n```\n{:#?}\n```\n",
        parsed.syntax()
    )
    .unwrap();

    if !parsed.diagnostics().is_empty() {
        let mut diagnostics_buffer = termcolor::Buffer::no_color();
        let termcolor = &mut Termcolor(&mut diagnostics_buffer);
        let mut formatter = Formatter::new(termcolor);
        for diagnostic in parsed.diagnostics() {
            let diagnostic = diagnostic
                .clone()
                .with_file_path(file_name)
                .with_file_source_code(&content);
            formatter
                .write_markup(markup! {{ PrintDiagnostic::verbose(&diagnostic) }})
                .expect("Failed to render diagnostic");
        }
        let diagnostics = std::str::from_utf8(diagnostics_buffer.as_slice())
            .expect("Expected diagnostics to be valid UTF-8");
        if matches!(outcome, ExpectedOutcome::Pass) {
            panic!("Expected the fixture to parse without errors:\n{diagnostics}");
        }
        writeln!(snapshot, "## Diagnostics\n\n```\n{diagnostics}```\n").unwrap();
    }

    match outcome {
        ExpectedOutcome::Pass => {
            if formatted_ast.contains("missing (required)")
                || parsed
                    .syntax()
                    .descendants()
                    .any(|node| node.kind().is_bogus())
            {
                panic!("Valid TOML produced missing children or bogus nodes:\n{formatted_ast}");
            }
        }
        ExpectedOutcome::Fail if parsed.diagnostics().is_empty() => {
            panic!("Invalid TOML must produce diagnostics");
        }
        ExpectedOutcome::Fail => {}
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}
