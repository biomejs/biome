use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsDeclarationClause, AnyJsExportClause, AnyJsExpression, JsExport, JsReferenceIdentifier,
    JsSyntaxToken, TextRange, binding_ext::AnyJsBindingDeclaration,
    declaration_ext::is_in_ambient_context,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_mutable_exports::NoMutableExportsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow exporting variables declared with `let` or `var`.
    ///
    /// A variable declared with `let` or `var` can be reassigned. When a module exports such
    /// a variable, every module that imports it sees the new value after a reassignment. Values
    /// that change from far away make code harder to follow and debug.
    ///
    /// Exporting a `const` variable, a function, or a class is allowed. If a value has to
    /// change over time, export functions that read and update it instead.
    ///
    /// Type declaration files (`.d.ts`) and ambient `declare` code only describe code that
    /// lives elsewhere, so they are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export let count = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export var count = 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let count = 1;
    /// export { count };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let count = 1;
    /// export default count;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const count = 1;
    /// export function getCount() {}
    /// export class Counter {}
    ///
    /// const total = 1;
    /// export { total as default };
    /// ```
    ///
    pub NoMutableExports {
        version: "next",
        name: "noMutableExports",
        language: "js",
        sources: &[RuleSource::EslintImport("no-mutable-exports").same()],
        recommended: false,
    }
}

pub enum RuleState {
    /// `export let name;` or `export var name;`.
    Declaration,
    /// `export { name };` or `export default name;`, where `name` is declared with `let` or
    /// `var`. `kind` is the `let` or `var` token of that declaration.
    Reference {
        name: JsReferenceIdentifier,
        kind: JsSyntaxToken,
    },
}

impl Rule for NoMutableExports {
    type Query = Semantic<JsExport>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoMutableExportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        // In a Svelte component's instance script, `export let` declares a prop rather than a
        // module export; `noSvelteExportLet` covers that case.
        let node = ctx.query();
        if source_type.language().is_definition_file()
            || source_type.as_embedding_kind().is_svelte_instance_script()
            || is_in_ambient_context(node.syntax())
        {
            return Box::default();
        }

        let model = ctx.model();
        match node.export_clause() {
            Ok(AnyJsExportClause::AnyJsDeclarationClause(
                AnyJsDeclarationClause::JsVariableDeclarationClause(clause),
            )) => clause
                .declaration()
                .ok()
                .filter(|declaration| declaration.is_let() || declaration.is_var())
                .map(|_| RuleState::Declaration)
                .into_iter()
                .collect(),
            Ok(AnyJsExportClause::JsExportNamedClause(clause)) => clause
                .specifiers()
                .iter()
                .flatten()
                .filter(|specifier| !specifier.exports_only_types())
                .filter_map(|specifier| reference_state(model, specifier.local_name().ok()?))
                .collect(),
            Ok(AnyJsExportClause::JsExportDefaultExpressionClause(clause)) => clause
                .expression()
                .ok()
                .and_then(|expression| match expression.omit_parentheses() {
                    AnyJsExpression::JsIdentifierExpression(identifier) => {
                        reference_state(model, identifier.name().ok()?)
                    }
                    _ => None,
                })
                .into_iter()
                .collect(),
            _ => Box::default(),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            RuleState::Declaration => {
                let export_token = ctx.query().export_token().ok()?;
                let Ok(AnyJsExportClause::AnyJsDeclarationClause(
                    AnyJsDeclarationClause::JsVariableDeclarationClause(clause),
                )) = ctx.query().export_clause()
                else {
                    return None;
                };
                let kind = clause.declaration().ok()?.kind().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    TextRange::new(
                        export_token.text_trimmed_range().start(),
                        kind.text_trimmed_range().end(),
                    ),
                    markup! {
                        "This export can be reassigned because it is declared with "<Emphasis>{kind.text_trimmed()}</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Modules that import this variable see its value change whenever it is reassigned, which makes the code harder to follow."
                })
            }
            RuleState::Reference { name, kind } => {
                let name_token = name.value_token().ok()?;
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    name.range(),
                    markup! {
                        "This export refers to "<Emphasis>{name_token.text_trimmed()}</Emphasis>", which can be reassigned because it is declared with "<Emphasis>{kind.text_trimmed()}</Emphasis>"."
                    },
                )
                .detail(kind.text_trimmed_range(), markup! {
                    "The variable is declared here."
                });
                if matches!(
                    ctx.query().export_clause(),
                    Ok(AnyJsExportClause::JsExportDefaultExpressionClause(_))
                ) {
                    diagnostic.note(markup! {
                        "A default export keeps the value the variable had when the module ran, so importers don't see later reassignments. This makes it easy to read an outdated value."
                    })
                } else {
                    diagnostic.note(markup! {
                        "Modules that import this variable see its value change whenever it is reassigned, which makes the code harder to follow."
                    })
                }
            }
        };
        Some(diagnostic.note(markup! {
            "Declare the variable with "<Emphasis>"const"</Emphasis>" instead. If the value needs to change, export functions that read and update it."
        }))
    }
}

/// Returns the state for an exported reference if it resolves to a variable declared with
/// `let` or `var`.
fn reference_state(model: &SemanticModel, name: JsReferenceIdentifier) -> Option<RuleState> {
    let declaration = model.binding(&name)?.tree().declaration()?;
    let declaration = declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration);
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };
    if is_in_ambient_context(declarator.syntax()) {
        return None;
    }
    let declaration = declarator.declaration()?;
    if !(declaration.is_let() || declaration.is_var()) {
        return None;
    }
    Some(RuleState::Reference {
        name,
        kind: declaration.kind_token().ok()?,
    })
}
