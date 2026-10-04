#![expect(
    clippy::disallowed_methods,
    reason = "This rule compares import media queries that can span multiple tokens."
)]

use std::collections::{HashMap, HashSet};

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssRootItem, AnyCssRule, AnyScssImportItem, CssMediaQueryList, CssRootItemList,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_at_import_rules::NoDuplicateAtImportRulesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Disallow duplicate `@import` rules.
    ///
    /// Different quote styles and the `url()` form are treated as the same URL. Imports of that URL
    /// are duplicates when either import is unconditional or when their media lists share a
    /// condition. Two imports of the same URL remain valid only when both are conditional and their
    /// media conditions do not overlap.
    /// Sass load imports are ignored because they don't emit CSS `@import` rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @import 'a.css';
    /// @import 'a.css';
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @import "a.css";
    /// @import 'a.css';
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @import url('a.css');
    /// @import url('a.css');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @import 'a.css';
    /// @import 'b.css';
    /// ```
    ///
    /// ```css
    /// @import url('a.css') tv;
    /// @import url('a.css') projection;
    /// ```
    ///
    pub NoDuplicateAtImportRules {
        version: "1.8.0",
        name: "noDuplicateAtImportRules",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("no-duplicate-at-import-rules").same(), RuleSource::EslintCss("no-duplicate-imports").inspired()],
    }
}

impl Rule for NoDuplicateAtImportRules {
    type Query = Ast<CssRootItemList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoDuplicateAtImportRulesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut imports_by_url: HashMap<String, HashSet<String>> = HashMap::new();
        for item in node {
            let AnyCssRootItem::AnyCssRule(AnyCssRule::CssAtRule(at_rule)) = item else {
                continue;
            };
            match at_rule.rule().ok()? {
                AnyCssAtRule::CssImportAtRule(import_rule) => {
                    let url = import_rule.url().ok()?.to_trimmed_text();
                    let media = normalized_media_queries(import_rule.media())?;
                    if is_duplicate_import(
                        &mut imports_by_url,
                        normalize_import_url(url.text()),
                        media,
                    ) {
                        return Some(import_rule.range());
                    }
                }
                AnyCssAtRule::ScssImportAtRule(import_rule) => {
                    for item in import_rule.imports() {
                        let item = item.ok()?;
                        let AnyScssImportItem::ScssPlainImport(import) = &item else {
                            continue;
                        };
                        let url = import.url().ok()?.to_trimmed_text();
                        let url = url.text().to_string();
                        let media = normalized_media_queries(import.media())?;
                        if is_duplicate_import(
                            &mut imports_by_url,
                            normalize_import_url(&url),
                            media,
                        ) {
                            return Some(item.range());
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, span: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Each "<Emphasis>"@import"</Emphasis>" should be unique unless differing by media queries."
                },
            )
            .note(markup! {
                    "Consider removing one of the duplicated imports."
            }),
        )
    }
}

fn normalized_media_queries(media: CssMediaQueryList) -> Option<Vec<String>> {
    media
        .into_iter()
        .map(|query| {
            Some(
                query
                    .ok()?
                    .to_trimmed_text()
                    .to_lowercase_cow()
                    .into_owned(),
            )
        })
        .collect()
}

fn normalize_import_url(url: &str) -> String {
    url.to_lowercase_cow()
        .replace("url(", "")
        .replace(')', "")
        .replace('"', "'")
}

fn is_duplicate_import(
    imports_by_url: &mut HashMap<String, HashSet<String>>,
    url: String,
    media: Vec<String>,
) -> bool {
    let Some(previous_media) = imports_by_url.get_mut(&url) else {
        imports_by_url.insert(url, media.into_iter().collect());
        return false;
    };

    if media.is_empty() || previous_media.is_empty() {
        return true;
    }

    media.into_iter().any(|media| !previous_media.insert(media))
}
