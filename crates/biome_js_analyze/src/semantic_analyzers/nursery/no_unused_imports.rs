use crate::{semantic_services::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    binding_ext::AnyJsBindingDeclaration, AnyJsImportClause, JsIdentifierBinding, JsImport,
    JsImportNamedClause, JsLanguage, JsNamedImportSpecifierList, T,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, NodeOrToken, SyntaxResult,
};

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
    /// ## Valid
    ///
    /// ```ts
    /// import { A, type B } from 'mod';
    ///
    /// function f(arg: B): A {
    ///     return new A(arg);
    /// }
    /// ```
    pub(crate) NoUnusedImports {
        version: "next",
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
        model.all_references(binding).next().is_none().then_some(())
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
            AnyJsBindingDeclaration::JsImportDefaultClause(_)
            | AnyJsBindingDeclaration::JsImportNamespaceClause(_) => {
                let import = declaration.parent::<JsImport>()?;
                mutation.transfer_leading_trivia_to_sibling(import.syntax());
                mutation.remove_node(import);
            }
            AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsBogusNamedImportSpecifier(_) => {
                let specifier_list = declaration.parent::<JsNamedImportSpecifierList>()?;
                if specifier_list.iter().count() == 1 {
                    let import_clause =
                        JsImportNamedClause::cast(specifier_list.syntax().parent()?.parent()?)?;
                    remove_named_import_from_import_clause(&mut mutation, import_clause).ok()?;
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
            AnyJsBindingDeclaration::JsDefaultImportSpecifier(declaration) => {
                mutation.remove_node(declaration);
            }
            AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => {
                let import_clause = JsImportNamedClause::cast(declaration.syntax().parent()?)?;
                remove_named_import_from_import_clause(&mut mutation, import_clause).ok()?;
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

fn remove_named_import_from_import_clause(
    mutation: &mut BatchMutation<JsLanguage>,
    import_clause: JsImportNamedClause,
) -> SyntaxResult<()> {
    if let Some(default_specifier) = import_clause.default_specifier() {
        let default_clause = make::js_import_default_clause(
            default_specifier.local_name()?,
            make::token_decorated_with_space(T![from]),
            import_clause.source()?,
        )
        .build();
        mutation.replace_node(
            AnyJsImportClause::from(import_clause),
            default_clause.into(),
        );
    } else if let Some(import) = import_clause.syntax().parent() {
        mutation.transfer_leading_trivia_to_sibling(&import);
        mutation.remove_element(NodeOrToken::Node(import));
    }
    Ok(())
}

const fn is_import(declaration: &AnyJsBindingDeclaration) -> bool {
    matches!(
        declaration,
        AnyJsBindingDeclaration::JsDefaultImportSpecifier(_)
            | AnyJsBindingDeclaration::JsImportDefaultClause(_)
            | AnyJsBindingDeclaration::JsImportNamespaceClause(_)
            | AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
            | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
            | AnyJsBindingDeclaration::TsImportEqualsDeclaration(_)
    )
}
