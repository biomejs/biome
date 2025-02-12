use std::collections::{HashMap, HashSet};

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssAtRule, AnyCssRule, CssImportAtRule, CssRuleList};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Disallow duplicate `@import` rules.
    ///
    /// This rule checks if the file urls of the @import rules are duplicates.
    ///
    /// This rule also checks the imported media queries and alerts of duplicates.
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
        sources: &[RuleSource::Stylelint("no-duplicate-at-import-rules")],
    }
}

impl Rule for NoDuplicateAtImportRules {
    type Query = Ast<CssRuleList>;
    type State = CssImportAtRule;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut import_url_map: HashMap<String, HashSet<String>> = HashMap::new();
        for rule in node {
            match rule {
                AnyCssRule::CssAtRule(item) => match item.rule().ok()? {
                    AnyCssAtRule::CssImportAtRule(import_rule) => {
                        let import_url = import_rule
                            .url()
                            .ok()?
                            .to_trimmed_string()
                            .to_lowercase_cow()
                            .replace("url(", "")
                            .replace(')', "")
                            .replace('"', "'");
                        if let Some(media_query_set) = import_url_map.get_mut(&import_url) {
                            // if the current import_rule has no media queries or there are no queries saved in the
                            // media_query_set, this is always a duplicate
                            if import_rule.media().to_trimmed_string().is_empty()
                                || media_query_set.is_empty()
                            {
                                return Some(import_rule);
                            }

                            for media in import_rule.media() {
                                match media {
                                    Ok(media) => {
                                        if !media_query_set.insert(
                                            media.to_trimmed_string().to_lowercase_cow().into(),
                                        ) {
                                            return Some(import_rule);
                                        }
                                    }
                                    _ => return None,
                                }
                            }
                        } else {
                            let mut media_set: HashSet<String> = HashSet::new();
                            for media in import_rule.media() {
                                match media {
                                    Ok(media) => {
                                        media_set.insert(
                                            media.to_trimmed_string().to_lowercase_cow().into(),
                                        );
                                    }
                                    _ => return None,
                                }
                            }
                            import_url_map.insert(import_url, media_set);
                        }
                    }
                    _ => return None,
                },
                _ => return None,
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
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
