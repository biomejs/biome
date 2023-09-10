use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_diagnostics::{Diagnostic, Location, Severity};
use biome_rowan::{AstNode, BatchMutation, SyntaxNodeCast, TriviaPiece};
use rome_js_semantic::{ReferencesExtensions, SemanticModel};
use rome_js_syntax::{
    binding_ext::AnyJsIdentifierBinding, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage,
    JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TextRange,
    TsIdentifierBinding,
};
use serde::{Deserialize, Serialize};
use std::fmt;

pub trait RenamableNode {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode>;
}

impl RenamableNode for JsIdentifierBinding {
    fn binding(&self, _: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(self.syntax().clone())
    }
}

impl RenamableNode for TsIdentifierBinding {
    fn binding(&self, _: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(self.syntax().clone())
    }
}

impl RenamableNode for JsReferenceIdentifier {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(model.binding(self)?.syntax().clone())
    }
}

impl RenamableNode for JsIdentifierAssignment {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(model.binding(self)?.syntax().clone())
    }
}

impl RenamableNode for AnyJsIdentifierBinding {
    fn binding(&self, _: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(self.syntax().clone())
    }
}

pub enum AnyJsRenamableDeclaration {
    JsIdentifierBinding(JsIdentifierBinding),
    JsReferenceIdentifier(JsReferenceIdentifier),
    JsIdentifierAssignment(JsIdentifierAssignment),
    TsIdentifierBinding(TsIdentifierBinding),
}

impl RenamableNode for AnyJsRenamableDeclaration {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        match self {
            AnyJsRenamableDeclaration::JsIdentifierBinding(node) => {
                RenamableNode::binding(node, model)
            }
            AnyJsRenamableDeclaration::JsReferenceIdentifier(node) => {
                RenamableNode::binding(node, model)
            }
            AnyJsRenamableDeclaration::JsIdentifierAssignment(node) => {
                RenamableNode::binding(node, model)
            }
            AnyJsRenamableDeclaration::TsIdentifierBinding(node) => {
                RenamableNode::binding(node, model)
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum RenameError {
    CannotFindDeclaration(String),
    CannotBeRenamed {
        original_name: String,
        original_range: TextRange,
        new_name: String,
    },
}

impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenameError::CannotBeRenamed {
                original_name,
                new_name,
                ..
            } => {
                write!(
                    f,
                    "encountered an error while renaming the symbol \"{}\" to \"{}\"",
                    original_name, new_name
                )
            }
            RenameError::CannotFindDeclaration(_) => {
                write!(
                    f,
                    "encountered an error finding a declaration at the specified position"
                )
            }
        }
    }
}

impl Diagnostic for RenameError {
    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        match self {
            RenameError::CannotFindDeclaration(node) => {
                fmt.write_markup(
                    markup! { "Can't find the declaration. Found node "{{node}} }
                )
            }
            RenameError::CannotBeRenamed { original_name, new_name, .. } => {
                fmt.write_markup(
                    markup! { "Can't rename from "<Emphasis>{{original_name}}</Emphasis>" to "<Emphasis>{{new_name}}</Emphasis>"" }
                )
            }
        }
    }

    fn location(&self) -> Location<'_> {
        let location = Location::builder();
        if let RenameError::CannotBeRenamed { original_range, .. } = self {
            location.span(original_range).build()
        } else {
            location.build()
        }
    }
}

impl TryFrom<JsSyntaxNode> for AnyJsRenamableDeclaration {
    type Error = RenameError;

    fn try_from(node: JsSyntaxNode) -> Result<Self, Self::Error> {
        let node_name = node.text_trimmed().to_string();
        match node.kind() {
            JsSyntaxKind::JS_IDENTIFIER_BINDING => node
                .cast::<JsIdentifierBinding>()
                .map(AnyJsRenamableDeclaration::JsIdentifierBinding)
                .ok_or(Self::Error::CannotFindDeclaration(node_name)),
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => node
                .cast::<JsReferenceIdentifier>()
                .map(AnyJsRenamableDeclaration::JsReferenceIdentifier)
                .ok_or(Self::Error::CannotFindDeclaration(node_name)),
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => node
                .cast::<JsIdentifierAssignment>()
                .map(AnyJsRenamableDeclaration::JsIdentifierAssignment)
                .ok_or(Self::Error::CannotFindDeclaration(node_name)),
            JsSyntaxKind::TS_IDENTIFIER_BINDING => node
                .cast::<TsIdentifierBinding>()
                .map(AnyJsRenamableDeclaration::TsIdentifierBinding)
                .ok_or(Self::Error::CannotFindDeclaration(node_name)),
            _ => Err(Self::Error::CannotFindDeclaration(node_name)),
        }
    }
}

pub trait RenameSymbolExtensions {
    /// Rename the binding and all its references to "new_name".
    fn rename_node_declaration(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode,
        new_name: &str,
    ) -> bool;

    /// Rename a symbol using the new name from the candidates iterator
    /// until the first success.
    ///
    /// A usual use case is to append a suffix to a variable name.
    ///
    /// ```ignore
    /// let new_name = "new_name";
    /// let candidates = (2..).map(|i| format!("{}{}", new_name, i).into());
    /// let candidates = once(Cow::from(new_name)).chain(candidates);
    /// batch.try_rename_node_declaration_until_success(model, node, candidates);
    /// ```
    fn rename_node_declaration_with_retry<S, I>(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode + Clone,
        candidates: I,
    ) -> bool
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        for candidate in candidates {
            if self.rename_node_declaration(model, node.clone(), candidate.as_ref()) {
                return true;
            }
        }

        false
    }

    /// Rename the binding and all its references to "new_name".
    fn rename_any_renamable_node(
        &mut self,
        model: &SemanticModel,
        node: AnyJsRenamableDeclaration,
        new_name: &str,
    ) -> bool {
        self.rename_node_declaration(model, node, new_name)
    }
}

fn token_with_new_text(token: &JsSyntaxToken, new_text: &str) -> JsSyntaxToken {
    let new_text = format!(
        "{}{}{}",
        token.leading_trivia().text(),
        new_text,
        token.trailing_trivia().text()
    );

    let leading = token
        .leading_trivia()
        .pieces()
        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
        .collect::<Vec<_>>();
    let trailing = token
        .trailing_trivia()
        .pieces()
        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
        .collect::<Vec<_>>();

    JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, new_text.as_str(), leading, trailing)
}

impl RenameSymbolExtensions for BatchMutation<JsLanguage> {
    /// Rename the binding and all its references to "new_name".
    /// If we can´t rename the binding, the [BatchMutation] is never changes and it is left
    /// intact.
    fn rename_node_declaration(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode,
        new_name: &str,
    ) -> bool {
        let prev_binding = match node.binding(model).and_then(AnyJsIdentifierBinding::cast) {
            Some(prev_binding) => prev_binding,
            None => return false,
        };

        // We can rename a binding if there is no conflicts in the current scope.
        // We can shadow parent scopes, so we don´t check them.

        let syntax = prev_binding.syntax();
        let scope = model
            .scope_hoisted_to(syntax)
            .unwrap_or_else(|| model.scope(syntax));
        if scope.get_binding(new_name).is_some() {
            return false;
        }

        let name_token = match prev_binding.name_token() {
            Ok(name_token) => name_token,
            Err(_) => {
                return false;
            }
        };

        // We can rename references, if there is no conflicts in any scope
        // until the root.

        let all_references: Vec<_> = prev_binding.all_references(model).collect();
        let mut changes = Vec::with_capacity(all_references.len());

        for reference in all_references {
            let scope = reference.scope();
            if scope
                .ancestors()
                .find_map(|scope| scope.get_binding(new_name))
                .is_some()
            {
                return false;
            }

            let prev_token = match reference.syntax().kind() {
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => reference
                    .syntax()
                    .clone()
                    .cast::<JsReferenceIdentifier>()
                    .and_then(|node| node.value_token().ok()),
                JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => reference
                    .syntax()
                    .clone()
                    .cast::<JsIdentifierAssignment>()
                    .and_then(|node| node.name_token().ok()),
                _ => None,
            };

            if let Some(prev_token) = prev_token {
                let next_token = token_with_new_text(&prev_token, new_name);
                changes.push((prev_token, next_token));
            }
        }

        // Now it is safe to push changes to the batch mutation
        // Rename binding
        let Ok(prev_name_token) = prev_binding.name_token() else {
            return false;
        };

        let next_name_token = token_with_new_text(&name_token, new_name);
        self.replace_token(prev_name_token, next_name_token);

        // Rename all references
        for (prev_token, next_token) in changes {
            self.replace_token(prev_token, next_token);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::rename::RenameError;
    use crate::{assert_rename_nok, assert_rename_ok};
    use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};
    use rome_js_syntax::TextRange;

    assert_rename_ok! {
        ok_rename_declaration,
            "let a;",
            "let b;",
        ok_rename_declaration_with_multiple_declarators,
            "let a1, a2;",
            "let b1, b2;",
        ok_rename_declaration_inner_scope,
            "let b; if (true) { let a; }",
            "let b; if (true) { let b; }",
        ok_rename_read_reference,
            "let a; a + 1;",
            "let b; b + 1;",
        ok_rename_read_before_initit,
            "function f() { console.log(a); let a; }",
            "function f() { console.log(b); let b; }",
        ok_rename_write_reference,
            "let a; a = 1;",
            "let b; b = 1;",
        ok_rename_write_before_init,
            "function f() { a = 1; let a; }",
            "function f() { b = 1; let b; }",
        ok_rename_trivia_is_kept,
            "let /*1*/a/*2*/; /*3*/a/*4*/ = 1; /*5*/a/*6*/ + 1",
            "let /*1*/b/*2*/; /*3*/b/*4*/ = 1; /*5*/b/*6*/ + 1",
        ok_rename_function_same_name,
            "function a() { function b() {console.log(2)}; console.log(1); b(); } a();",
            "function b() { function b() {console.log(2)}; console.log(1); b(); } b();",
    }

    assert_rename_nok! {
        nok_rename_declaration_conflict_before, "let b; let a;",
        nok_rename_declaration_conflict_after, "let a; let b;",
        nok_rename_read_reference, "let a; if (true) { let b; a + 1 }",
        nok_rename_read_reference_conflict_hoisting_same_scope, "let a; if (true) { a + 1; var b; }",
        nok_rename_read_reference_conflict_hoisting_outer_scope, "let a; if (true) { a + 1; } var b;",
        nok_rename_write_reference, "let a; if (true) { let b; a = 1 }",
        nok_rename_read_reference_parent_scope_conflict, "function f() { let b; if(true) { console.log(a); } } var a;",
        nok_rename_function_conflict, "function a() {} function b() {}",
    }

    fn snap_diagnostic(test_name: &str, diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);

        });
    }

    #[test]
    fn cannot_find_declaration() {
        snap_diagnostic(
            "cannot_find_declaration",
            RenameError::CannotFindDeclaration("async".to_string()).with_file_path("example.js"),
        )
    }

    #[test]
    fn cannot_be_renamed() {
        let source_code = "async function f() {}";
        snap_diagnostic(
            "cannot_be_renamed",
            RenameError::CannotBeRenamed {
                original_name: "async".to_string(),
                original_range: TextRange::new(0.into(), 5.into()),
                new_name: "await".to_string(),
            }
            .with_file_path("example.js")
            .with_file_source_code(source_code),
        )
    }
}
