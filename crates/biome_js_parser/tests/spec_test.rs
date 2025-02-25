use biome_configuration::Configuration;
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_deserialize::json::deserialize_from_str;
use biome_diagnostics::display::PrintDiagnostic;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::{print_diagnostic_to_string, termcolor};
use biome_fs::BiomePath;
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_rowan::SyntaxKind;
use biome_service::settings::Settings;
use biome_test_utils::has_bogus_nodes_or_empty_slots;
use camino::Utf8Path;
use std::fmt::Write;
use std::fs;

#[derive(Copy, Clone)]
pub enum ExpectedOutcome {
    Pass,
    Fail,
    Undefined,
}

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, outcome_str: &str) {
    let outcome = match outcome_str {
        "ok" => ExpectedOutcome::Pass,
        "error" => ExpectedOutcome::Fail,
        "undefined" => ExpectedOutcome::Undefined,
        _ => panic!("Invalid expected outcome {outcome_str}"),
    };

    let test_case_path = Utf8Path::new(test_case);

    let file_name = test_case_path
        .file_name()
        .expect("File name to be valid UTF8");

    let content = fs::read_to_string(test_case_path)
        .expect("Expected test path to be a readable file in UTF8 encoding");

    let mut options = JsParserOptions::default();

    let options_path = Utf8Path::new(test_case_path).with_extension("options.json");

    if options_path.exists() {
        let mut options_path = BiomePath::new(&options_path);

        let mut settings = Settings::default();
        // SAFETY: we checked its existence already, we assume we have rights to read it
        let (test_options, diagnostics) =
            deserialize_from_str::<Configuration>(options_path.get_buffer_from_file().as_str())
                .consume();

        settings
            .merge_with_configuration(test_options.unwrap_or_default(), None, None, &[])
            .unwrap();

        let settings = settings.languages.javascript.parser;

        if settings
            .parse_class_parameter_decorators
            .unwrap_or_default()
            .into()
        {
            options = options.with_parse_class_parameter_decorators();
        }

        if settings.grit_metavariables.unwrap_or_default().into() {
            options = options.with_metavariables();
        }

        if !diagnostics.is_empty() {
            for diagnostic in diagnostics {
                println!("{:?}", print_diagnostic_to_string(&diagnostic));
            }

            panic!("Configuration is invalid");
        }
    }

    let file_source = JsFileSource::try_from(test_case_path).unwrap_or_default();

    let extension = file_source.file_extension();
    let parsed = parse(&content, file_source, options);
    let formatted_ast = format!("{:#?}", parsed.tree());

    let mut snapshot = String::new();
    writeln!(snapshot, "\n## Input\n\n```{extension}\n{content}\n```\n\n").unwrap();

    writeln!(
        snapshot,
        r#"## AST

```
{formatted_ast}
```

## CST

```
{:#?}
```
"#,
        parsed.syntax()
    )
    .unwrap();

    let diagnostics = parsed.diagnostics();
    if !diagnostics.is_empty() {
        let mut diagnostics_buffer = termcolor::Buffer::no_color();

        let termcolor = &mut Termcolor(&mut diagnostics_buffer);
        let mut formatter = Formatter::new(termcolor);

        for diagnostic in diagnostics {
            let error = diagnostic
                .clone()
                .with_file_path(file_name)
                .with_file_source_code(&content);

            formatter
                .write_markup(markup! {
                    {PrintDiagnostic::verbose(&error)}
                })
                .expect("failed to emit diagnostic");
        }

        let formatted_diagnostics =
            std::str::from_utf8(diagnostics_buffer.as_slice()).expect("non utf8 in error buffer");

        if matches!(outcome, ExpectedOutcome::Pass) {
            panic!("Expected no errors to be present in a test case that is expected to pass but the following diagnostics are present:\n{formatted_diagnostics}")
        }

        writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        snapshot.write_str(formatted_diagnostics).unwrap();

        writeln!(snapshot, "```\n").unwrap();
    }

    match outcome {
        ExpectedOutcome::Pass => {
            let missing_required = formatted_ast.contains("missing (required)");
            if missing_required
                || parsed
                    .syntax()
                    .descendants()
                    .any(|node| node.kind().is_bogus())
            {
                panic!("Parsed tree of a 'OK' test case should not contain any missing required children or bogus nodes: \n {formatted_ast:#?} \n\n {formatted_ast}");
            }

            let syntax = parsed.syntax();
            if has_bogus_nodes_or_empty_slots(&syntax) {
                panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
            }
        }
        ExpectedOutcome::Fail => {
            if parsed.diagnostics().is_empty() {
                panic!("Failing test must have diagnostics");
            }
        }
        _ => {}
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => &test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"
type T = import;
    "#;

    let root = parse(code, JsFileSource::ts(), JsParserOptions::default());
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
