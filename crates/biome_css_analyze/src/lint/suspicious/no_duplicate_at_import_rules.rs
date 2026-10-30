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
use biome_rowan::{AstNode, Text, TextRange};
use biome_rule_options::no_duplicate_at_import_rules::NoDuplicateAtImportRulesOptions;
use biome_string_case::StrOnlyExtension;

use crate::utils::DuplicateGroup;

declare_lint_rule! {
    /// Disallow duplicate `@import` rules.
    ///
    /// Different quote styles and the `url()` form are treated as the same URL. Imports of that URL
    /// are duplicates when either import is unconditional or when their media lists share a
    /// condition. Two imports of the same URL remain valid only when both are conditional and their
    /// media conditions do not overlap.
    /// SCSS load imports are ignored because they don't emit CSS `@import` rules.
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
    type State = DuplicateGroup<Text>;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateAtImportRulesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut imports = ImportGroups::default();
        for item in node {
            let AnyCssRootItem::AnyCssRule(AnyCssRule::CssAtRule(at_rule)) = item else {
                continue;
            };
            let Ok(rule) = at_rule.rule() else {
                continue;
            };
            match rule {
                AnyCssAtRule::CssImportAtRule(import_rule) => {
                    let Ok(url) = import_rule.url() else {
                        continue;
                    };
                    let Some(media) = normalized_media_queries(import_rule.media()) else {
                        continue;
                    };
                    imports.add(url.to_trimmed_text(), media, import_rule.range());
                }
                AnyCssAtRule::ScssImportAtRule(import_rule) => {
                    for item in import_rule.imports() {
                        let Ok(item) = item else {
                            continue;
                        };
                        let AnyScssImportItem::ScssPlainImport(import) = &item else {
                            continue;
                        };
                        let Ok(url) = import.url() else {
                            continue;
                        };
                        let Some(media) = normalized_media_queries(import.media()) else {
                            continue;
                        };
                        imports.add(url.to_trimmed_text(), media, item.range());
                    }
                }
                _ => {}
            }
        }
        imports.into_duplicates()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateGroup {
            name,
            first,
            duplicates,
        } = state;
        let name = name.text();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The stylesheet "<Emphasis>{name}</Emphasis>" is imported more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The same stylesheet is imported again here."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "Each "<Emphasis>"@import"</Emphasis>" should be unique unless differing by media queries."
                })
                .note(markup! {
                    "Importing the same stylesheet more than once is redundant and can cause an extra request."
                })
                .note(markup! {
                    "Consider removing the duplicated imports."
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

/// Imports that are not duplicates of an earlier import, with the duplicates found for each.
#[derive(Default)]
struct ImportGroups {
    /// Per normalized URL: the media conditions and group index of each non-duplicate import.
    originals_by_url: HashMap<String, Vec<(HashSet<String>, usize)>>,
    groups: Vec<DuplicateGroup<Text>>,
}

impl ImportGroups {
    /// Records an import. It duplicates an earlier import of the same URL when either import is
    /// unconditional or their media conditions overlap.
    fn add(&mut self, url: Text, media: Vec<String>, range: TextRange) {
        let media: HashSet<String> = media.into_iter().collect();
        let originals = self
            .originals_by_url
            .entry(normalize_import_url(&url))
            .or_default();
        let original = originals
            .iter()
            .find(|(previous, _)| {
                media.is_empty() || previous.is_empty() || !media.is_disjoint(previous)
            })
            .map(|&(_, group)| group);

        match original {
            Some(group) => self.groups[group].duplicates.push(range),
            None => {
                originals.push((media, self.groups.len()));
                self.groups.push(DuplicateGroup {
                    name: url,
                    first: range,
                    duplicates: Vec::new(),
                });
            }
        }
    }

    fn into_duplicates(self) -> Box<[DuplicateGroup<Text>]> {
        self.groups
            .into_iter()
            .filter(|group| !group.duplicates.is_empty())
            .collect()
    }
}
