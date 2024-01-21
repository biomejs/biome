use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExportNamedSpecifier, JsExportNamedClause, JsFileSource, T};
use biome_rowan::{
    trim_leading_trivia_pieces, AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind,
};

declare_rule! {
    /// Promotes the use of `export type` for types.
    ///
    /// _TypeScript_ allows specifying a `type` marker on an `export` to indicate that the `export` doesn't exist at runtime.
    /// This allows transpilers to safely drop exports of types without looking for their definition.
    ///
    /// The rule ensures that types are exported using a type-only `export`.
    /// It also groups inline type exports into a grouped `export type`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface I {}
    /// export { I };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type T = number;
    /// export { T };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// import type { T } from "./mod.js";
    /// export { T };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// export { type X, type Y };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class C {}
    /// function f() {}
    /// export { C, f };
    /// ```
    ///
    /// This rules checks only the identifiers that are defined in a file.
    /// It doesn't warn against a type exported as a value in a re-export clause such as:
    ///
    /// ```ts,ignore
    /// export { TypeA } from "./mod.ts"
    /// ```
    pub(crate) UseExportType {
        version: "1.5.0",
        name: "useExportType",
        source: RuleSource::EslintTypeScript("consistent-type-exports"),
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseExportType {
    type Query = Semantic<JsExportNamedClause>;
    type State = ExportTypeFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.language().is_typescript() {
            return None;
        }
        let export_named_clause = ctx.query();
        if export_named_clause.type_token().is_some() {
            // `export type {}`
            return None;
        }
        let mut exports_only_types = true;
        let mut specifiers_requiring_type_marker = Vec::new();
        for specifier in export_named_clause.specifiers() {
            let Ok((ref_name, specifier)) =
                specifier.and_then(|specifier| Ok((specifier.local_name()?, specifier)))
            else {
                exports_only_types = false;
                continue;
            };
            if specifier.type_token().is_some() {
                // `export { type <specifier> }`
                continue;
            }
            let model = ctx.model();
            let binding = model.binding(&ref_name)?;
            let binding = binding.tree();
            if binding.is_type_only() {
                specifiers_requiring_type_marker.push(specifier);
            } else {
                exports_only_types = false;
            }
        }
        if exports_only_types {
            Some(ExportTypeFix::GroupTypeExports)
        } else if specifiers_requiring_type_marker.is_empty() {
            None
        } else {
            Some(ExportTypeFix::AddInlineType(
                specifiers_requiring_type_marker,
            ))
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        let (diagnostic_range, diagnostic_message) = match state {
            ExportTypeFix::GroupTypeExports => (
                range,
                markup! {
                    "All exports are only types and should thus use "<Emphasis>"export type"</Emphasis>"."
                },
            ),
            ExportTypeFix::AddInlineType(specifiers) => {
                let range = specifiers
                    .iter()
                    .map(|x| x.range())
                    .reduce(|acc, x| acc.cover(x))
                    .unwrap_or(range);
                (
                    range,
                    markup! {
                        "Several exports are only types and should thus use "<Emphasis>"export type"</Emphasis>"."
                    },
                )
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            diagnostic_range,
            diagnostic_message,
        ).note(markup! {
            "Using "<Emphasis>"export type"</Emphasis>" allows transpilers to safely drop exports of types without looking for their definition."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let export_named_clause = ctx.query();
        let mut mutation = ctx.root().begin();
        let type_token =
            Some(make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]));
        let diagnostic = match state {
            ExportTypeFix::GroupTypeExports => {
                let specifier_list = export_named_clause.specifiers();
                let mut new_specifiers = Vec::new();
                for specifier in specifier_list.iter().filter_map(|x| x.ok()) {
                    if let Some(type_token) = specifier.type_token() {
                        let mut new_specifier = specifier.with_type_token(None);
                        if type_token.has_trailing_comments() {
                            new_specifier = new_specifier.prepend_trivia_pieces(
                                trim_leading_trivia_pieces(type_token.trailing_trivia().pieces()),
                            )?;
                        }
                        new_specifiers.push(new_specifier);
                    } else {
                        new_specifiers.push(specifier)
                    }
                }
                let new_specifier_list = make::js_export_named_specifier_list(
                    new_specifiers,
                    specifier_list
                        .separators()
                        .filter_map(|sep| sep.ok())
                        .collect::<Vec<_>>(),
                );
                mutation.replace_node(
                    export_named_clause.clone(),
                    export_named_clause
                        .clone()
                        .with_type_token(type_token)
                        .with_specifiers(new_specifier_list),
                );
                JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Use a grouped "<Emphasis>"export type"</Emphasis>"." }
                        .to_owned(),
                    mutation,
                }
            }
            ExportTypeFix::AddInlineType(specifiers) => {
                for specifier in specifiers {
                    mutation.replace_node(
                        specifier.clone(),
                        specifier.clone().with_type_token(type_token.clone()),
                    );
                }
                JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Use inline "<Emphasis>"type"</Emphasis>" exports." }
                        .to_owned(),
                    mutation,
                }
            }
        };
        Some(diagnostic)
    }
}

#[derive(Debug)]
pub(crate) enum ExportTypeFix {
    /**
     * Group inline type exports such as `export { type A, type B }` into `export type { A, B }`.
     */
    GroupTypeExports,
    AddInlineType(Vec<AnyJsExportNamedSpecifier>),
}
