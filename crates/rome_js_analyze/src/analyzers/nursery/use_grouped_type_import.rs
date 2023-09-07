use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    AnyJsNamedImport, AnyJsNamedImportSpecifier, JsImportNamedClause, TriviaPieceKind, T,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Enforce the use of `import type` when an `import` only has specifiers with `type` qualifier.
    ///
    /// The [`--verbatimModuleSyntax`](https://www.typescriptlang.org/tsconfig#verbatimModuleSyntax) _TypeScript_'s compiler option causes _TypeScript_ to do simple and predictable transpilation on `import` declarations.
    /// Namely, it completely removes `import type` and any imported names with the `type` qualifier.
    ///
    /// For instance, the following code:
    ///
    /// ```ts,expect_diagnostic
    /// import { type A, type B } from "mod-1";
    /// import type { C, D } from "mod-2";
    /// ```
    ///
    /// is transpiled to:
    ///
    /// ```ts
    /// import "mod-1";
    /// ```
    ///
    /// Note that, an `import` that includes only names qualified with `type` is transpiled to a [side-effect `import`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#import_a_module_for_its_side_effects_only).
    /// This can be a surprising behavior: most of developers could expect the deletion of the `import`.
    ///
    /// This behavior may still be desirable for applying the potential side-effects of the imported module.
    /// In most cases you will not want to leave behind an unnecessary side effect `import`.
    /// In teh remaining cases, it is often preferable to explicitly use a side-effect `import` to apply the side-effects of a module:
    ///
    /// ```ts
    /// import "mod"; // side-effect import
    /// import type { A, B } from "mod";
    /// ```
    ///
    /// Source: https://typescript-eslint.io/rules/no-import-type-side-effects/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// import { type A } from "mod";
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// import type { A, B } from "mod";
    /// ```
    ///
    /// ```ts
    /// import { A, type B } from "mod";
    /// ```
    pub(crate) UseGroupedTypeImport {
        version: "1.0.0",
        name: "useGroupedTypeImport",
        recommended: true,
    }
}

impl Rule for UseGroupedTypeImport {
    type Query = Ast<JsImportNamedClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.type_token().is_some() || node.default_specifier().is_some() {
            return None;
        }
        if node
            .named_import()
            .ok()?
            .as_js_named_import_specifiers()?
            .specifiers()
            .is_empty()
        {
            // import {} from ...
            return None;
        }
        node.named_import()
            .ok()?
            .as_js_named_import_specifiers()?
            .specifiers()
            .iter()
            .all(|specifier| {
                let Ok(specifier) = specifier else {
                    return false;
                };
                match specifier {
                    AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => false,
                    AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                        specifier.type_token().is_some()
                    }
                    AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                        specifier.type_token().is_some()
                    }
                }
            })
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.named_import().ok()?.range(),
                markup! {
                    "The "<Emphasis>"type"</Emphasis>" qualifier can be moved just after "<Emphasis>"import"</Emphasis>" to completely remove the "<Emphasis>"import"</Emphasis>" at compile time."
                },
            )
            .note(markup! {
                "Only "<Emphasis>"import type"</Emphasis>" are removed at compile time."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let named_import = node.named_import().ok()?;
        let named_import_specifiers = named_import.as_js_named_import_specifiers()?;
        let named_import_specifiers_list = named_import_specifiers.specifiers();
        let new_named_import_specifiers_list = make::js_named_import_specifier_list(
            named_import_specifiers_list
                .iter()
                .filter_map(|specifier| specifier.ok())
                .map(|specifier| match specifier {
                    AnyJsNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(
                            specifier.with_type_token(None),
                        )
                    }
                    AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                            specifier.with_type_token(None),
                        )
                    }
                    specifier => specifier,
                })
                .collect::<Vec<_>>(),
            named_import_specifiers_list
                .separators()
                .filter_map(|separator| separator.ok())
                .collect::<Vec<_>>(),
        );
        let new_node = node
            .clone()
            .with_type_token(Some(
                make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            ))
            .with_named_import(AnyJsNamedImport::JsNamedImportSpecifiers(
                named_import_specifiers
                    .clone()
                    .with_specifiers(new_named_import_specifiers_list),
            ));
        mutation.replace_node(node.clone(), new_node);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use "<Emphasis>"import type"</Emphasis>" instead." }.to_owned(),
            mutation,
        })
    }
}
