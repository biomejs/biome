use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsNamedImportSpecifier, JsImportNamedClause, TriviaPieceKind, T};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

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
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// import { type A } from "mod";
    /// ```
    ///
    /// ### Valid
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
        source: RuleSource::EslintTypeScript("no-import-type-side-effects"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseGroupedTypeImport {
    type Query = Ast<JsImportNamedClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import_clause = ctx.query();
        if import_clause.type_token().is_some() {
            return None;
        }
        let specifiers = import_clause.named_specifiers().ok()?.specifiers();
        if specifiers.is_empty() {
            // import {} from ...
            return None;
        }
        specifiers
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
        let import_clause = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                import_clause.named_specifiers().ok()?.range(),
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
        let import_clause = ctx.query();
        let named_import_specifiers = import_clause.named_specifiers().ok()?;
        let named_import_specifiers_list = named_import_specifiers.specifiers();
        let new_named_import_specifiers_list = make::js_named_import_specifier_list(
            named_import_specifiers_list
                .iter()
                .filter_map(|specifier| specifier.ok())
                .map(|specifier| specifier.with_type_token(None))
                .collect::<Vec<_>>(),
            named_import_specifiers_list
                .separators()
                .filter_map(|separator| separator.ok())
                .collect::<Vec<_>>(),
        );
        let new_node = import_clause
            .clone()
            .with_type_token(Some(
                make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            ))
            .with_named_specifiers(
                named_import_specifiers
                    .clone()
                    .with_specifiers(new_named_import_specifiers_list),
            );
        let mut mutation = ctx.root().begin();
        mutation.replace_node(import_clause.clone(), new_node);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use "<Emphasis>"import type"</Emphasis>" instead." }.to_owned(),
            mutation,
        })
    }
}
