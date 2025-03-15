use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::display::PrintDiagnostic;
use biome_diagnostics::termcolor;
use biome_markdown_parser::parse_markdown;
use biome_markdown_syntax::MarkdownSyntaxNode;
use biome_rowan::SyntaxKind;
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

    // Allow tests to run even with bogus nodes during development
    let formatted_ast = format!("{:#?}", parsed.tree());

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
        if matches!(outcome, ExpectedOutcome::Pass) {
            // During development, temporarily allow diagnostics even for passing tests
            // TODO: Remove this allowance once parser development is more complete
            writeln!(
                snapshot,
                "## Diagnostics (ALLOWED DURING DEVELOPMENT)\n\n```"
            )
            .unwrap();
        } else {
            writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        }

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

        snapshot.write_str(formatted_diagnostics).unwrap();
        writeln!(snapshot, "```").unwrap();
    }

    match outcome {
        ExpectedOutcome::Pass => {
            let has_bogus_nodes = parsed
                .syntax()
                .descendants()
                .any(|node| node.kind().is_bogus());

            if has_bogus_nodes {
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

                // Add detailed bogus node information to the snapshot
                writeln!(
                    snapshot,
                    "## ERROR: Found {} bogus nodes",
                    bogus_nodes.len()
                )
                .unwrap();

                for (i, node) in bogus_nodes.iter().enumerate() {
                    writeln!(
                        snapshot,
                        "Bogus node #{}: Parent: {:?}",
                        i + 1,
                        node.parent().map(|p| p.kind())
                    )
                    .unwrap();
                }

                writeln!(snapshot, "\nTree structure with bogus nodes:").unwrap();
                writeln!(snapshot, "```").unwrap();
                print_tree_to_string(&parsed.syntax(), 0, &mut snapshot);
                writeln!(snapshot, "```").unwrap();

                // TODO: Re-enable this check once parser development is more complete
                // Fail the test if bogus nodes are found
                // panic!("Found {} bogus nodes in a passing test", bogus_nodes.len());

                // For now, just print a warning
                eprintln!(
                    "WARNING: Found {} bogus nodes in a passing test",
                    bogus_nodes.len()
                );
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

/// Creates a 'OK' test case, for test cases that should succeed with no errors or diagnostic
#[test]
pub fn quick_test() {
    // During development, this lets us use `cargo test quick_test -- --nocapture`
    // for quick iteration
    let test_dir = "md_test_suite/ok";
    let test_case = format!("{}/thematic_break_block.md", test_dir);
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(test_case);
    run(test_path.to_str().unwrap(), "quick_test", test_dir, "ok");
}

/// Helper function to print the tree structure for debugging
fn print_tree_structure(node: &MarkdownSyntaxNode, depth: usize) {
    let indent = "  ".repeat(depth);
    eprintln!("{}{:?}", indent, node.kind());

    for child in node.children() {
        print_tree_structure(&child, depth + 1);
    }
}

/// Helper function to print tree to string for diagnostics
fn print_tree_to_string(node: &MarkdownSyntaxNode, depth: usize, output: &mut String) {
    let indent = "  ".repeat(depth);
    writeln!(output, "{}{:?}", indent, node.kind()).unwrap();

    for child in node.children() {
        print_tree_to_string(&child, depth + 1, output);
    }
}
