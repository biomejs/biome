use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, JsExportNamedClause, JsExportNamedFromClause, JsFileSource,
    JsSyntaxToken, T,
};
use biome_rowan::{
    chain_trivia_pieces, declare_node_union, trim_leading_trivia_pieces, AstNode, AstSeparatedList,
    BatchMutationExt, TriviaPieceKind,
};

declare_lint_rule! {
    /// Promotes the use of `export type` for types.
    ///
    /// _TypeScript_ allows adding the `type` keyword on an `export` to indicate that the `export` doesn't exist at runtime.
    /// This allows compilers to safely drop exports of types without looking for their definition.
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
    pub UseExportType {
        version: "1.5.0",
        name: "useExportType",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("consistent-type-exports")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseExportType {
    type Query = Semantic<AnyJsExportNamedClause>;
    type State = ExportTypeFix;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.language().is_typescript() || source_type.language().is_definition_file() {
            return None;
        }
        let export_named_clause = ctx.query();
        match export_named_clause {
            AnyJsExportNamedClause::JsExportNamedClause(clause) => {
                let specifiers = clause.specifiers();
                if specifiers.is_empty() {
                    // Don't report `export {}`
                    None
                } else if clause.type_token().is_some() {
                    // `export type { ... }`
                    let useless_type_tokens: Vec<_> = specifiers
                        .iter()
                        .filter_map(|specifier| specifier.ok()?.type_token())
                        .collect();
                    if useless_type_tokens.is_empty() {
                        None
                    } else {
                        Some(ExportTypeFix::RemoveInlineTypeQualifiers(
                            useless_type_tokens,
                        ))
                    }
                } else {
                    let mut exports_only_types = true;
                    let mut specifiers_requiring_type_marker = Vec::new();
                    for specifier in specifiers {
                        let Ok((ref_name, specifier)) = specifier
                            .and_then(|specifier| Ok((specifier.local_name()?, specifier)))
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
                        Some(ExportTypeFix::UseExportType)
                    } else if specifiers_requiring_type_marker.is_empty() {
                        None
                    } else {
                        Some(ExportTypeFix::AddInlineTypeQualifiers(
                            specifiers_requiring_type_marker,
                        ))
                    }
                }
            }
            AnyJsExportNamedClause::JsExportNamedFromClause(clause) => {
                let specifiers = clause.specifiers();
                if specifiers.is_empty() {
                    None
                } else if clause.type_token().is_some() {
                    let useless_type_tokens: Vec<_> = specifiers
                        .iter()
                        .filter_map(|specifier| specifier.ok()?.type_token())
                        .collect();
                    if useless_type_tokens.is_empty() {
                        None
                    } else {
                        Some(ExportTypeFix::RemoveInlineTypeQualifiers(
                            useless_type_tokens,
                        ))
                    }
                } else if specifiers
                    .iter()
                    .all(|x| x.is_ok_and(|x| x.type_token().is_some()))
                {
                    Some(ExportTypeFix::UseExportType)
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let named_export_clause = ctx.query();
        let diagnostic = match state {
            ExportTypeFix::UseExportType => RuleDiagnostic::new(
                rule_category!(),
                named_export_clause.range(),
                "All exports are only types.",
            ),
            ExportTypeFix::AddInlineTypeQualifiers(specifiers) => {
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    named_export_clause.range(),
                    "Some exports are only types.",
                );
                for specifier in specifiers {
                    diagnostic = diagnostic.detail(specifier.range(), "This export is a type.")
                }
                diagnostic
            }
            ExportTypeFix::RemoveInlineTypeQualifiers(type_tokens) => {
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    named_export_clause.type_token()?.text_trimmed_range(),
                    markup! {
                        "This "<Emphasis>"type"</Emphasis>" keyword makes all inline "<Emphasis>"type"</Emphasis>" keywords useless."
                    },
                );
                for type_token in type_tokens {
                    diagnostic = diagnostic.detail(
                        type_token.text_trimmed_range(),
                        markup! {
                            "This inline "<Emphasis>"type"</Emphasis>" keyword is useless."
                        },
                    )
                }
                return Some(diagnostic);
            }
        };
        Some(diagnostic.note(markup! {
            "Using "<Emphasis>"export type"</Emphasis>" allows compilers to safely drop exports of types without looking for their definition."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let export_named_clause = ctx.query();
        let mut mutation = ctx.root().begin();
        let diagnostic = match state {
            ExportTypeFix::UseExportType => {
                match export_named_clause {
                    AnyJsExportNamedClause::JsExportNamedClause(clause) => {
                        let specifier_list = clause.specifiers();
                        let mut new_specifiers = Vec::new();
                        for specifier in specifier_list.iter().filter_map(|x| x.ok()) {
                            if let Some(type_token) = specifier.type_token() {
                                let new_specifier = specifier
                                    .with_type_token(None)
                                    .trim_leading_trivia()?
                                    .prepend_trivia_pieces(chain_trivia_pieces(
                                        type_token.leading_trivia().pieces(),
                                        trim_leading_trivia_pieces(
                                            type_token.trailing_trivia().pieces(),
                                        ),
                                    ))?;
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
                            clause.clone(),
                            clause
                                .clone()
                                .with_type_token(Some(
                                    make::token(T![type])
                                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                                ))
                                .with_specifiers(new_specifier_list),
                        );
                    }
                    AnyJsExportNamedClause::JsExportNamedFromClause(clause) => {
                        let specifier_list = clause.specifiers();
                        let mut new_specifiers = Vec::new();
                        for specifier in specifier_list.iter().filter_map(|x| x.ok()) {
                            if let Some(type_token) = specifier.type_token() {
                                let new_specifier = specifier
                                    .with_type_token(None)
                                    .trim_leading_trivia()?
                                    .prepend_trivia_pieces(chain_trivia_pieces(
                                        type_token.leading_trivia().pieces(),
                                        trim_leading_trivia_pieces(
                                            type_token.trailing_trivia().pieces(),
                                        ),
                                    ))?;
                                new_specifiers.push(new_specifier);
                            } else {
                                new_specifiers.push(specifier)
                            }
                        }
                        let new_specifier_list = make::js_export_named_from_specifier_list(
                            new_specifiers,
                            specifier_list
                                .separators()
                                .filter_map(|sep| sep.ok())
                                .collect::<Vec<_>>(),
                        );
                        mutation.replace_node(
                            clause.clone(),
                            clause
                                .clone()
                                .with_type_token(Some(
                                    make::token(T![type])
                                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                                ))
                                .with_specifiers(new_specifier_list),
                        );
                    }
                }
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use "<Emphasis>"export type"</Emphasis>"." }.to_owned(),
                    mutation,
                )
            }
            ExportTypeFix::AddInlineTypeQualifiers(specifiers) => {
                for specifier in specifiers {
                    mutation.replace_node(
                        specifier.clone(),
                        specifier
                            .clone()
                            .with_leading_trivia_pieces([])?
                            .with_type_token(Some(
                                make::token(T![type])
                                    .with_leading_trivia_pieces(
                                        specifier.syntax().first_leading_trivia()?.pieces(),
                                    )
                                    .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                            )),
                    );
                }
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Add inline "<Emphasis>"type"</Emphasis>" keywords." }.to_owned(),
                    mutation,
                )
            }
            ExportTypeFix::RemoveInlineTypeQualifiers(type_tokens) => {
                for type_token in type_tokens {
                    mutation.remove_token(type_token.clone());
                }
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove useless inline "<Emphasis>"type"</Emphasis>" keywords." }
                        .to_owned(),
                    mutation,
                )
            }
        };
        Some(diagnostic)
    }
}

declare_node_union! {
    pub AnyJsExportNamedClause = JsExportNamedClause | JsExportNamedFromClause
}

impl AnyJsExportNamedClause {
    fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsExportNamedClause(clause) => clause.type_token(),
            Self::JsExportNamedFromClause(clause) => clause.type_token(),
        }
    }
}

#[derive(Debug)]
pub enum ExportTypeFix {
    /**
     * Group inline type exports such as `export { type A, type B }` into `export type { A, B }`.
     */
    UseExportType,
    AddInlineTypeQualifiers(Vec<AnyJsExportNamedSpecifier>),
    RemoveInlineTypeQualifiers(Vec<JsSyntaxToken>),
}
