use crate::{semantic_services::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsCombinedSpecifier, AnyJsImportClause,
    JsIdentifierBinding, JsImport, JsLanguage, JsNamedImportSpecifierList, JsSyntaxNode, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutation, BatchMutationExt};

declare_rule! {
    /// Disallow unused imports.
    ///
    /// Unused imports might be the result of an incomplete refactoring.
    /// The code fix can remove comments associated with an `import`.
    /// See the last invalid example.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import A from 'mod';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import * as A from 'mod';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// import { type A, B } from 'mod';
    ///
    /// export { B }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Header comment
    /// import /*inner comment */ A from 'mod'; // Associated comment
    ///
    /// // Another header comment
    /// import {
    ///     // A's header comment
    ///     type A, // A's comment
    ///     // B's header comment
    ///     B,
    /// } from 'mod';
    ///
    /// export { B }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// import { A, type B } from 'mod';
    ///
    /// function f(arg: B): A {
    ///     return new A(arg);
    /// }
    /// ```
    pub(crate) NoUnusedImports {
        version: "1.3.0",
        name: "noUnusedImports",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUnusedImports {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let declaration = binding.declaration()?;
        if !is_import(&declaration) {
            return None;
        }

        let model = ctx.model();
        binding.all_references(model).next().is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binding.range(),
                markup! {
                    "This "<Emphasis>"import"</Emphasis>" is unused."
                },
            )
            .note(markup! {
                "Unused imports might be the result of an incomplete refactoring."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let declaration = ctx.query().declaration()?;
        let mut mutation = ctx.root().begin();
        match declaration {
            AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_) => {
                let specifier_list = declaration.parent::<JsNamedImportSpecifierList>()?;
                if specifier_list.len() == 1 {
                    remove_import_specifier(&mut mutation, &specifier_list.syntax().parent()?)?;
                } else {
                    let following_separator = specifier_list
                        .iter()
                        .zip(specifier_list.separators().map(|separator| separator.ok()))
                        .find(|(specifier, _)| {
                            specifier
                                .as_ref()
                                .is_ok_and(|x| x.syntax() == declaration.syntax())
                        })
                        .and_then(|(_, separator)| separator);
                    if let Some(separator) = following_separator {
                        mutation.remove_token(separator);
                    }
                    mutation.remove_node(declaration);
                }
            }
            AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => {
                remove_import_specifier(&mut mutation, declaration.syntax())?;
            }
            AnyJsBindingDeclaration::TsImportEqualsDeclaration(_) => {
                mutation.remove_node(declaration);
            }
            _ => {
                return None;
            }
        }
        Some(JsRuleAction {
            mutation,
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove the unused import." }.to_owned(),
        })
    }
}

fn remove_import_specifier(
    mutation: &mut BatchMutation<JsLanguage>,
    specifier: &JsSyntaxNode,
) -> Option<()> {
    let clause = specifier.parent().and_then(AnyJsImportClause::cast)?;
    match &clause {
        AnyJsImportClause::JsImportCombinedClause(default_extra_clause) => {
            let default_specifier = default_extra_clause.default_specifier().ok()?;
            let from_token = default_extra_clause.from_token().ok()?;
            let source = default_extra_clause.source().ok()?;
            let assertion = default_extra_clause.assertion();
            if default_specifier.syntax() == specifier {
                let new_clause = match default_extra_clause.specifier().ok()? {
                    AnyJsCombinedSpecifier::JsNamedImportSpecifiers(named_specifier) => {
                        let named_clause =
                            make::js_import_named_clause(named_specifier, from_token, source);
                        let named_clause = if let Some(assertion) = assertion {
                            named_clause.with_assertion(assertion)
                        } else {
                            named_clause
                        };
                        AnyJsImportClause::JsImportNamedClause(named_clause.build())
                    }
                    AnyJsCombinedSpecifier::JsNamespaceImportSpecifier(namespace_specifier) => {
                        let namespace_clause = make::js_import_namespace_clause(
                            namespace_specifier,
                            from_token,
                            source,
                        );
                        let namespace_clause = if let Some(assertion) = assertion {
                            namespace_clause.with_assertion(assertion)
                        } else {
                            namespace_clause
                        };
                        AnyJsImportClause::JsImportNamespaceClause(namespace_clause.build())
                    }
                };
                mutation.replace_node(clause, new_clause);
            } else {
                let from_token = make::token_decorated_with_space(T![from])
                    .with_trailing_trivia_pieces(from_token.trailing_trivia().pieces());
                let default_clause =
                    make::js_import_default_clause(default_specifier, from_token, source);
                let default_clause = if let Some(assertion) = assertion {
                    default_clause.with_assertion(assertion)
                } else {
                    default_clause
                };
                mutation.replace_node(clause, default_clause.build().into());
            }
        }
        AnyJsImportClause::JsImportBareClause(_)
        | AnyJsImportClause::JsImportDefaultClause(_)
        | AnyJsImportClause::JsImportNamedClause(_)
        | AnyJsImportClause::JsImportNamespaceClause(_) => {
            // Remove the entire statement
            let import = clause.parent::<JsImport>()?;
            mutation.transfer_leading_trivia_to_sibling(import.syntax());
            mutation.remove_node(import);
        }
    }
    Some(())
}

const fn is_import(declaration: &AnyJsBindingDeclaration) -> bool {
    matches!(
        declaration,
        AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
    )
}
