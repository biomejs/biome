use crate::{
    JsRuleAction,
    lint::style::use_import_type::add_module_items,
    services::{embedded_value_references::EmbeddedValueReferences, semantic::Semantic},
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExportNamedSpecifier, JsExport, JsExportNamedClause, JsExportNamedFromClause,
    JsExportNamedFromSpecifier, JsExportNamedFromSpecifierList, JsExportNamedSpecifierList,
    JsFileSource, JsSyntaxToken, T,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind, chain_trivia_pieces,
    declare_node_union, trim_leading_trivia_pieces,
};
use biome_rule_options::use_export_type::{Style, UseExportTypeOptions};

declare_lint_rule! {
    /// Promotes the use of `export type` for types.
    ///
    /// _TypeScript_ allows specifying a `type` keyword on an `export` to indicate that the `export` doesn't exist at runtime.
    /// This allows compilers to safely drop exports of types without looking for their definition.
    ///
    /// The rule ensures that all exports used only as a type use a type-only `export`.
    /// It also groups inline type exports into a grouped `export type`.
    ///
    /// If you use the TypeScript Compiler (TSC) to compile your code into JavaScript,
    /// then you can disable this rule, as TSC can remove exports only used as types.
    /// However, for consistency and compatibility with other compilers, you may want to enable this rule.
    /// In that case we recommend to enable TSC's [`verbatimModuleSyntax`](https://www.typescriptlang.org/tsconfig/#verbatimModuleSyntax).
    /// This configuration ensures that TSC preserves exports not marked with the `type` keyword.
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
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// The `style` option allows enforcing a style for exporting types.
    /// The option supports three values:
    ///
    /// - `inlineType`: always use `export { type T }` instead of `export type { T }`
    /// - `separatedType`: always use `export type { T }` instead of `export { type T }`
    /// - `auto`: use `export type { T }` or `export { type T, V }` when values are exported alongside types (default)
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "style": "inlineType"
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options
    /// import type { A } from "./mod.ts";
    /// export { A };
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options
    /// import { A } from "./mod.ts";
    /// export type { A };
    /// ```
    ///
    /// ```jsonc,options
    /// {
    ///     "options": {
    ///         "style": "separatedType"
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options
    /// import type { A } from "./mod.ts";
    /// export { A };
    /// ```
    ///
    /// ```ts,expect_diagnostic,use_options
    /// import { A, B } from "./mod.ts";
    /// export { type A, B };
    /// ```
    pub UseExportType {
        version: "1.5.0",
        name: "useExportType",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("consistent-type-exports").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseExportType {
    type Query = Semantic<AnyJsExportNamedClause>;
    type State = ExportTypeFix;
    type Signals = Option<Self::State>;
    type Options = UseExportTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.language().is_typescript() || source_type.language().is_definition_file() {
            return None;
        }
        let export_named_clause = ctx.query();
        let style = ctx.options().style.unwrap_or_default();
        match export_named_clause {
            AnyJsExportNamedClause::JsExportNamedClause(clause) => {
                let type_token = clause.type_token();
                if style == Style::InlineType && type_token.is_some() {
                    // Inline `export type` into `export { type }`
                    let specifiers = clause
                        .specifiers()
                        .iter()
                        .collect::<Result<Vec<_>, _>>()
                        .ok()?;
                    return if specifiers.is_empty() {
                        None
                    } else {
                        Some(ExportTypeFix::AddInlineTypeQualifiers(specifiers))
                    };
                }
                let model = ctx.model();
                let references = ctx
                    .get_service::<EmbeddedValueReferences>()
                    .expect("embedded value references service");
                match export_named_fix(
                    model,
                    &clause.specifiers(),
                    type_token.is_some(),
                    references,
                )? {
                    ExportNamedFix::UseImportType(specifiers) => {
                        if style == Style::InlineType {
                            if specifiers.is_empty() {
                                None
                            } else {
                                Some(ExportTypeFix::AddInlineTypeQualifiers(specifiers))
                            }
                        } else {
                            Some(ExportTypeFix::UseExportType)
                        }
                    }
                    ExportNamedFix::AddInlineTypeQualifiers(specifiers) => {
                        if style == Style::SeparatedType {
                            Some(ExportTypeFix::SeparateTypes(specifiers))
                        } else {
                            Some(ExportTypeFix::AddInlineTypeQualifiers(specifiers))
                        }
                    }
                    ExportNamedFix::RemoveInlineTypeQualifiers(type_tokens) => {
                        Some(ExportTypeFix::RemoveInlineTypeQualifiers(type_tokens))
                    }
                    ExportNamedFix::CanSeparateType => {
                        if style == Style::SeparatedType {
                            Some(ExportTypeFix::SeparateTypes(Vec::new()))
                        } else {
                            None
                        }
                    }
                }
            }
            AnyJsExportNamedClause::JsExportNamedFromClause(clause) => {
                let specifiers = clause.specifiers();
                if specifiers.is_empty() {
                    // Don't report empty `export {} from` or `export type {} from`
                    None
                } else if clause.type_token().is_some() {
                    if style == Style::InlineType {
                        // Inline `export type { ... }` into `export { type ... }`
                        let missing_type_tokens: Vec<_> = specifiers
                            .iter()
                            .filter_map(|specifier| specifier.ok())
                            .filter(|specifier| specifier.type_token().is_none())
                            .collect();
                        if missing_type_tokens.is_empty() {
                            None
                        } else {
                            Some(ExportTypeFix::AddInlineTypeQualifiers2(missing_type_tokens))
                        }
                    } else {
                        // Factorize `export type { type ... }` into `export type { ... }`
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
                    }
                } else {
                    let specifiers_with_type_marker: Vec<_> = specifiers
                        .iter()
                        .filter_map(|specifier| specifier.ok())
                        .filter(|specifier| specifier.type_token().is_some())
                        .collect();
                    let exports_only_types = specifiers.len() == specifiers_with_type_marker.len();
                    if specifiers_with_type_marker.is_empty() {
                        None
                    } else if exports_only_types && style != Style::InlineType {
                        Some(ExportTypeFix::UseExportType)
                    } else if style == Style::SeparatedType {
                        Some(ExportTypeFix::SeparateTypes2(specifiers_with_type_marker))
                    } else {
                        None
                    }
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
            ExportTypeFix::AddInlineTypeQualifiers2(specifiers) => {
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
            ExportTypeFix::SeparateTypes(specifiers) => {
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    named_export_clause.range(),
                    "Separate type exports from other exports.",
                );
                for specifier in specifiers {
                    diagnostic = diagnostic.detail(specifier.range(), "This export is a type.")
                }
                diagnostic
            }
            ExportTypeFix::SeparateTypes2(specifiers) => {
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    named_export_clause.range(),
                    "Separate type exports from other exports.",
                );
                for specifier in specifiers {
                    diagnostic = diagnostic.detail(specifier.range(), "This export is a type.")
                }
                diagnostic
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
                if let Some(type_token) = export_named_clause.type_token() {
                    // Inline `export type` into `export { type }`
                    mutation.remove_token(type_token);
                }
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
            ExportTypeFix::AddInlineTypeQualifiers2(specifiers) => {
                if let Some(type_token) = export_named_clause.type_token() {
                    // Inline `export type` into `export { type }`
                    mutation.remove_token(type_token);
                }
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
            ExportTypeFix::SeparateTypes(specifiers_requiring_type_marker) => {
                let export = export_named_clause.parent::<JsExport>()?;
                let export_token = export.export_token().ok()?;
                match export_named_clause {
                    AnyJsExportNamedClause::JsExportNamedClause(clause) => {
                        let specifiers = clause.specifiers();
                        let (named_type, named_value) = split_export_named_specifiers(
                            &specifiers,
                            specifiers_requiring_type_marker,
                        )?;
                        let (export_named_type, export_named_value) =
                            new_named_exports(export_token, clause, named_type, named_value)?;
                        add_module_items(
                            &mut mutation,
                            export.syntax(),
                            [export_named_type.into(), export_named_value.into()],
                        );
                    }
                    AnyJsExportNamedClause::JsExportNamedFromClause(_clause) => {
                        return None;
                    }
                }
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Extract types into a new export." }.to_owned(),
                    mutation,
                )
            }
            ExportTypeFix::SeparateTypes2(specifiers_requiring_type_marker) => {
                let export = export_named_clause.parent::<JsExport>()?;
                let export_token = export.export_token().ok()?;
                match export_named_clause {
                    AnyJsExportNamedClause::JsExportNamedClause(_clause) => {
                        return None;
                    }
                    AnyJsExportNamedClause::JsExportNamedFromClause(clause) => {
                        let specifiers = clause.specifiers();
                        let (named_type, named_value) = split_export_named_from_specifiers(
                            &specifiers,
                            specifiers_requiring_type_marker,
                        )?;
                        let (export_named_type, export_named_value) =
                            new_named_from_exports(export_token, clause, named_type, named_value)?;
                        add_module_items(
                            &mut mutation,
                            export.syntax(),
                            [export_named_type.into(), export_named_value.into()],
                        );
                    }
                }
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Extract types into a new export." }.to_owned(),
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
    AddInlineTypeQualifiers2(Vec<JsExportNamedFromSpecifier>),
    RemoveInlineTypeQualifiers(Vec<JsSyntaxToken>),
    SeparateTypes(Vec<AnyJsExportNamedSpecifier>),
    SeparateTypes2(Vec<JsExportNamedFromSpecifier>),
}

#[derive(Debug)]
enum ExportNamedFix {
    UseImportType(Vec<AnyJsExportNamedSpecifier>),
    AddInlineTypeQualifiers(Vec<AnyJsExportNamedSpecifier>),
    RemoveInlineTypeQualifiers(Vec<JsSyntaxToken>),
    CanSeparateType,
}

fn export_named_fix(
    model: &SemanticModel,
    specifiers: &JsExportNamedSpecifierList,
    has_type_token: bool,
    references: &EmbeddedValueReferences,
) -> Option<ExportNamedFix> {
    if specifiers.is_empty() {
        return None;
    };
    if has_type_token {
        let useless_type_tokens: Vec<_> = specifiers
            .iter()
            .filter_map(|specifier| specifier.ok()?.type_token())
            .collect();
        if useless_type_tokens.is_empty() {
            None
        } else {
            Some(ExportNamedFix::RemoveInlineTypeQualifiers(
                useless_type_tokens,
            ))
        }
    } else {
        let mut imports_only_types = true;
        let mut has_inline_types = false;
        let mut specifiers_requiring_type_marker = Vec::with_capacity(specifiers.len());
        for specifier in specifiers.iter() {
            let Ok(specifier) = specifier else {
                imports_only_types = false;
                continue;
            };
            if specifier.type_token().is_none() {
                if let Ok(local_name) = specifier.local_name()
                    && let local_name = local_name.value_token().ok()?.text_trimmed()
                    && model
                        .global_scope()
                        .bindings()
                        .filter(|binding| binding.syntax().text_trimmed() == local_name)
                        .all(|binding| binding.tree().is_type_only())
                    && !references.is_used_as_value(local_name)
                {
                    specifiers_requiring_type_marker.push(specifier);
                } else {
                    imports_only_types = false;
                }
            } else {
                has_inline_types = true;
            }
        }
        if imports_only_types {
            Some(ExportNamedFix::UseImportType(
                specifiers_requiring_type_marker,
            ))
        } else if specifiers_requiring_type_marker.is_empty() {
            if has_inline_types {
                Some(ExportNamedFix::CanSeparateType)
            } else {
                None
            }
        } else {
            Some(ExportNamedFix::AddInlineTypeQualifiers(
                specifiers_requiring_type_marker,
            ))
        }
    }
}

fn split_export_named_specifiers(
    specifiers: &JsExportNamedSpecifierList,
    specifiers_requiring_type_keyword: &[AnyJsExportNamedSpecifier],
) -> Option<(JsExportNamedSpecifierList, JsExportNamedSpecifierList)> {
    // There is at least one expprt that is not a type.
    // Thus there is at most `len - 1` type-only exports.
    let mut type_specifiers = Vec::with_capacity(specifiers.len() - 1);
    let mut type_specifier_separators = Vec::with_capacity(specifiers.len() - 1);
    let mut value_specifiers =
        Vec::with_capacity(specifiers.len() - specifiers_requiring_type_keyword.len());
    let mut value_specifier_separators =
        Vec::with_capacity(specifiers.len() - specifiers_requiring_type_keyword.len());
    for specifier_element in specifiers.elements() {
        let specifier = specifier_element.node().ok()?.clone();
        let trailing_sep = specifier_element.into_trailing_separator().ok()?;
        if let Some(type_token) = specifier.type_token() {
            let new_specifier = specifier
                .with_type_token(None)
                .trim_leading_trivia()?
                .prepend_trivia_pieces(chain_trivia_pieces(
                    type_token.leading_trivia().pieces(),
                    trim_leading_trivia_pieces(type_token.trailing_trivia().pieces()),
                ))?;
            type_specifiers.push(new_specifier);
            if let Some(trailing_sep) = trailing_sep {
                type_specifier_separators.push(trailing_sep);
            }
        } else if specifiers_requiring_type_keyword
            .iter()
            .any(|x| x.range().start() == specifier.range().start())
        {
            type_specifiers.push(specifier);
            if let Some(trailing_sep) = trailing_sep {
                type_specifier_separators.push(trailing_sep);
            }
        } else {
            value_specifiers.push(specifier);
            if let Some(trailing_sep) = trailing_sep {
                value_specifier_separators.push(trailing_sep);
            }
        }
    }
    let named_type =
        make::js_export_named_specifier_list(type_specifiers, type_specifier_separators);
    let named_value =
        make::js_export_named_specifier_list(value_specifiers, value_specifier_separators);
    Some((named_type, named_value))
}

fn split_export_named_from_specifiers(
    specifiers: &JsExportNamedFromSpecifierList,
    specifiers_requiring_type_keyword: &[JsExportNamedFromSpecifier],
) -> Option<(
    JsExportNamedFromSpecifierList,
    JsExportNamedFromSpecifierList,
)> {
    // There is at least one expprt that is not a type.
    // Thus there is at most `len - 1` type-only exports.
    let mut type_specifiers = Vec::with_capacity(specifiers.len() - 1);
    let mut type_specifier_separators = Vec::with_capacity(specifiers.len() - 1);
    let mut value_specifiers =
        Vec::with_capacity(specifiers.len() - specifiers_requiring_type_keyword.len());
    let mut value_specifier_separators =
        Vec::with_capacity(specifiers.len() - specifiers_requiring_type_keyword.len());
    for specifier_element in specifiers.elements() {
        let specifier = specifier_element.node().ok()?.clone();
        let trailing_sep = specifier_element.into_trailing_separator().ok()?;
        if let Some(type_token) = specifier.type_token() {
            let new_specifier = specifier
                .with_type_token(None)
                .trim_leading_trivia()?
                .prepend_trivia_pieces(chain_trivia_pieces(
                    type_token.leading_trivia().pieces(),
                    trim_leading_trivia_pieces(type_token.trailing_trivia().pieces()),
                ))?;
            type_specifiers.push(new_specifier);
            if let Some(trailing_sep) = trailing_sep {
                type_specifier_separators.push(trailing_sep);
            }
        } else if specifiers_requiring_type_keyword
            .iter()
            .any(|x| x.range().start() == specifier.range().start())
        {
            type_specifiers.push(specifier);
            if let Some(trailing_sep) = trailing_sep {
                type_specifier_separators.push(trailing_sep);
            }
        } else {
            value_specifiers.push(specifier);
            if let Some(trailing_sep) = trailing_sep {
                value_specifier_separators.push(trailing_sep);
            }
        }
    }
    let named_type =
        make::js_export_named_from_specifier_list(type_specifiers, type_specifier_separators);
    let named_value =
        make::js_export_named_from_specifier_list(value_specifiers, value_specifier_separators);
    Some((named_type, named_value))
}

fn new_named_exports(
    export_token: JsSyntaxToken,
    export_clause: &JsExportNamedClause,
    named_type_specifiers: JsExportNamedSpecifierList,
    named_value_specifiers: JsExportNamedSpecifierList,
) -> Option<(JsExport, JsExport)> {
    let l_curly_token = export_clause.l_curly_token().ok()?;
    let r_curly_token = export_clause.r_curly_token().ok()?;
    let semicolon_token = export_clause.semicolon_token();
    let type_export_clause = make::js_export_named_clause(
        l_curly_token.clone(),
        named_type_specifiers,
        r_curly_token.clone(),
    )
    .with_type_token(
        make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .build()
    .with_semicolon_token(semicolon_token.clone());
    let value_export_clause =
        make::js_export_named_clause(l_curly_token, named_value_specifiers, r_curly_token)
            .build()
            .with_semicolon_token(semicolon_token);
    let export_type = make::js_export(
        make::js_decorator_list([]),
        export_token,
        type_export_clause.into(),
    );
    let new_export_token = make::token(T![export])
        .with_leading_trivia([(TriviaPieceKind::Newline, "\n")])
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    let export_value = make::js_export(
        make::js_decorator_list([]),
        new_export_token,
        value_export_clause.into(),
    );
    Some((export_type, export_value))
}

fn new_named_from_exports(
    export_token: JsSyntaxToken,
    export_clause: &JsExportNamedFromClause,
    named_type_specifiers: JsExportNamedFromSpecifierList,
    named_value_specifiers: JsExportNamedFromSpecifierList,
) -> Option<(JsExport, JsExport)> {
    let l_curly_token = export_clause.l_curly_token().ok()?;
    let r_curly_token = export_clause.r_curly_token().ok()?;
    let from_token = export_clause.from_token().ok()?;
    let source = export_clause.source().ok()?;
    let semicolon_token = export_clause.semicolon_token();
    let type_export_clause = make::js_export_named_from_clause(
        l_curly_token.clone(),
        named_type_specifiers,
        r_curly_token.clone(),
        from_token.clone(),
        source.clone(),
    )
    .with_type_token(
        make::token(T![type]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .build()
    .with_semicolon_token(semicolon_token.clone());
    let export_type = make::js_export(
        make::js_decorator_list([]),
        export_token,
        type_export_clause.into(),
    );
    let value_export_clause = make::js_export_named_from_clause(
        l_curly_token,
        named_value_specifiers,
        r_curly_token,
        from_token,
        source,
    )
    .build()
    .with_semicolon_token(semicolon_token);
    let new_export_token = make::token(T![export])
        .with_leading_trivia([(TriviaPieceKind::Newline, "\n")])
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
    let export_value = make::js_export(
        make::js_decorator_list([]),
        new_export_token,
        value_export_clause.into(),
    );
    Some((export_type, export_value))
}
