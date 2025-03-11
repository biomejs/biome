use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::display::PrintDiagnostic;
use biome_diagnostics::termcolor;
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownSyntaxNode;
use biome_rowan::SyntaxKind;
use biome_test_utils::has_bogus_nodes_or_empty_slots;
use std::fmt::Write;
use std::fs;
use std::path::Path;

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

    let test_case_path = Path::new(test_case);

    let file_name = test_case_path
        .file_name()
        .expect("Expected test to have a file name")
        .to_str()
        .expect("File name to be valid UTF8");

    let content = fs::read_to_string(test_case_path)
        .expect("Expected test path to be a readable file in UTF8 encoding");

    let parsed = parse_markdown(&content);

    let formatted_ast = format!("{:#?}", parsed.syntax());
    let mut snapshot = String::new();
    writeln!(snapshot, "\n## Input\n\n```\n{content}\n```\n\n").unwrap();

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
            panic!(
                "Expected no errors to be present in a test case that is expected to pass but the following diagnostics are present:\n{formatted_diagnostics}"
            )
        }

        writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        snapshot.write_str(formatted_diagnostics).unwrap();
        writeln!(snapshot, "```").unwrap();
    }

    // During development, we'll be more lenient about tests
    match outcome {
        ExpectedOutcome::Pass => {
            let missing_required = formatted_ast.contains("missing (required)");
            let has_bogus_nodes = parsed
                .syntax()
                .descendants()
                .any(|node| node.kind().is_bogus());

            if has_bogus_nodes {
                // Print details about the bogus nodes to help debug
                let bogus_nodes: Vec<_> = parsed
                    .syntax()
                    .descendants()
                    .filter(|node| node.kind().is_bogus())
                    .collect();

                eprintln!("Found {} bogus nodes:", bogus_nodes.len());
                for (i, node) in bogus_nodes.iter().enumerate() {
                    eprintln!(
                        "Bogus node #{}: Parent: {:?}",
                        i + 1,
                        node.parent().map(|p| p.kind())
                    );
                }

                // Print the tree structure for debugging
                eprintln!("\nTree structure:");
                print_tree_structure(&parsed.syntax(), 0);
            }

            if missing_required || has_bogus_nodes {
                panic!(
                    "Parsed tree of a 'OK' test case should not contain any missing required children or bogus nodes: \n {formatted_ast:#?} \n\n {formatted_ast}"
                );
            }

            let syntax = parsed.syntax();
            if has_bogus_nodes_or_empty_slots(&syntax) {
                panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
            }
        }
        ExpectedOutcome::Fail => {
            // For err tests, we do want to verify diagnostics are emitted
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
        // Use `assert_debug_snapshot` for more stable output
        insta::assert_snapshot!(file_name, snapshot);
    });
}

/// Helper function to print the tree structure for debugging
fn print_tree_structure(node: &MarkdownSyntaxNode, depth: usize) {
    let indent = "  ".repeat(depth);
    eprintln!("{}{:?}", indent, node.kind());

    for child in node.children() {
        print_tree_structure(&child, depth + 1);
    }
}
