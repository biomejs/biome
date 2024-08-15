use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_diagnostics::{Diagnostic, Location, Severity};
use biome_js_factory::make;
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::{
    binding_ext::AnyJsIdentifierBinding, AnyJsIdentifierUsage, JsIdentifierAssignment,
    JsIdentifierBinding, JsLanguage, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, TextRange,
    TsIdentifierBinding, T,
};
use biome_rowan::{AstNode, BatchMutation, SyntaxNodeCast, TriviaPieceKind};
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
                    "encountered an error while renaming the symbol \"{original_name}\" to \"{new_name}\""
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
        node: &impl RenamableNode,
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
        node: &impl RenamableNode,
        candidates: I,
    ) -> bool
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        for candidate in candidates {
            if self.rename_node_declaration(model, node, candidate.as_ref()) {
                return true;
            }
        }

        false
    }

    /// Rename the binding and all its references to "new_name".
    fn rename_any_renamable_node(
        &mut self,
        model: &SemanticModel,
        node: &AnyJsRenamableDeclaration,
        new_name: &str,
    ) -> bool {
        self.rename_node_declaration(model, node, new_name)
    }
}

impl RenameSymbolExtensions for BatchMutation<JsLanguage> {
    /// Rename the binding and all its references to "new_name".
    /// If we can´t rename the binding, the [BatchMutation] is never changes and it is left
    /// intact.
    fn rename_node_declaration(
        &mut self,
        model: &SemanticModel,
        node: &impl RenamableNode,
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

        let Ok(prev_name_token) = prev_binding.name_token() else {
            return false;
        };

        // We can rename references, if there is no conflicts in any scope
        // until the root.

        let all_references: Vec<_> = prev_binding.all_references(model).collect();
        let mut token_changes = Vec::with_capacity(all_references.len());
        let mut node_changes = vec![];

        for reference in all_references {
            // We can rename a reference if there is no binding named `new_name`
            // in the current scope or a parent scope.
            if reference
                .scope()
                .ancestors()
                .any(|scope| scope.get_binding(new_name).is_some())
            {
                return false;
            }

            let reference_syntax = reference.syntax();
            let Some(id_usage) = AnyJsIdentifierUsage::cast_ref(reference_syntax) else {
                continue;
            };
            let Ok(prev_ref_token) = id_usage.value_token() else {
                continue;
            };

            let new_name = make::ident(new_name);
            if let Some(reference_parent) = reference_syntax.parent() {
                if reference_parent.kind() == JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER {
                    // Handle renaming of shorthand properties.
                    // For example renaming `color` into `colorNew` in
                    // `let color = ...; const c = { color }` must result in
                    // `let colorNew = ...; const c = { color: colorNew }`
                    let trailing_trivia = prev_ref_token.trailing_trivia().pieces();
                    let new_property = make::js_property_object_member(
                        make::js_literal_member_name(prev_ref_token.with_trailing_trivia([]))
                            .into(),
                        make::token(T![:])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        make::js_identifier_expression(make::js_reference_identifier(
                            new_name.append_trivia_pieces(trailing_trivia),
                        ))
                        .into(),
                    );
                    node_changes.push((reference_parent, new_property.into_syntax()));
                    continue;
                }
            }
            token_changes.push((prev_ref_token, new_name));
        }

        // Now it is safe to push changes to the batch mutation
        // Rename binding
        self.replace_token(prev_name_token, make::ident(new_name));

        // Rename all references
        for (prev_token, next_token) in token_changes {
            self.replace_token(prev_token, next_token);
        }
        for (prev_node, next_node) in node_changes {
            self.replace_element(prev_node.into(), next_node.into());
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::rename::RenameError;
    use crate::{assert_rename_nok, assert_rename_ok};
    use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};
    use biome_js_syntax::TextRange;

    assert_rename_ok! {
        ok_rename_declaration,
            "let a;",
            "let b;",
            false,
        ok_rename_declaration_with_multiple_declarators,
            "let a1, a2;",
            "let b1, b2;",
            false,
        ok_rename_declaration_inner_scope,
            "let b; if (true) { let a; }",
            "let b; if (true) { let b; }",
            false,
        ok_rename_read_reference,
            "let a; a + 1;",
            "let b; b + 1;",
            false,
        ok_rename_namespace_reference,
            "namespace a { type c = number }; let x: a.c = 1;",
            "namespace b { type c = number }; let x: b.c = 1;",
            true,
        ok_rename_read_before_initit,
            "function f() { console.log(a); let a; }",
            "function f() { console.log(b); let b; }",
            false,
        ok_rename_write_reference,
            "let a; a = 1;",
            "let b; b = 1;",
            false,
        ok_rename_write_before_init,
            "function f() { a = 1; let a; }",
            "function f() { b = 1; let b; }",
            false,
        ok_rename_trivia_is_kept,
            "let /*1*/a/*2*/; /*3*/a/*4*/ = 1; /*5*/a/*6*/ + 1",
            "let /*1*/b/*2*/; /*3*/b/*4*/ = 1; /*5*/b/*6*/ + 1",
            false,
        ok_rename_function_same_name,
            "function a() { function b() {console.log(2)}; console.log(1); b(); } a();",
            "function b() { function b() {console.log(2)}; console.log(1); b(); } b();",
            false,
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

    fn snap_diagnostic(test_name: &str, diagnostic: &Error) {
        let content = print_diagnostic_to_string(diagnostic);

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
            &RenameError::CannotFindDeclaration("async".to_string()).with_file_path("example.js"),
        )
    }

    #[test]
    fn cannot_be_renamed() {
        let source_code = "async function f() {}";
        snap_diagnostic(
            "cannot_be_renamed",
            &RenameError::CannotBeRenamed {
                original_name: "async".to_string(),
                original_range: TextRange::new(0.into(), 5.into()),
                new_name: "await".to_string(),
            }
            .with_file_path("example.js")
            .with_file_source_code(source_code),
        )
    }
}
