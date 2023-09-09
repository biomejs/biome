use crate::Parse;
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::{termcolor::Buffer, PrintDiagnostic};
use rome_js_syntax::{JsLanguage, JsSyntaxNode};
use rome_rowan::{AstNode, SyntaxKind, SyntaxSlot};
use std::{fmt::Debug, path::Path};

/// This check is used in the parser test to ensure it doesn't emit
/// bogus nodes without diagnostics, and in the analyzer tests to
/// check the syntax trees resulting from code actions are correct
pub fn has_bogus_nodes_or_empty_slots(node: &JsSyntaxNode) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty));
        }

        false
    })
}

/// This function analyzes the parsing result of a file and panic with a
/// detailed message if it contains any error-level diagnostic, bogus nodes,
/// empty list slots or missing required children
pub fn assert_errors_are_absent<T>(program: &Parse<T>, path: &Path)
where
    T: AstNode<Language = JsLanguage> + Debug,
{
    let syntax = program.syntax();
    let debug_tree = format!("{:?}", program.tree());
    let has_missing_children = debug_tree.contains("missing (required)");

    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!(
            "modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {}",
            syntax
        )
    }

    if !program.has_errors() && !has_missing_children {
        return;
    }

    let mut buffer = Buffer::no_color();
    for diagnostic in program.diagnostics() {
        let error = diagnostic
            .clone()
            .with_file_path(path.to_str().unwrap())
            .with_file_source_code(syntax.to_string());
        Formatter::new(&mut Termcolor(&mut buffer))
            .write_markup(markup! {
                {PrintDiagnostic::verbose(&error)}
            })
            .unwrap();
    }

    panic!("There should be no errors in the file {:?} but the following errors where present:\n{}\n\nParsed tree:\n{:#?}",
        path.display(),
        std::str::from_utf8(buffer.as_slice()).unwrap(),
        &syntax
	);
}
