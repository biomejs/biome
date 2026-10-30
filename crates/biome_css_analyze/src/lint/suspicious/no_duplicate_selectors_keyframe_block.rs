#![expect(
    clippy::disallowed_methods,
    reason = "This rule compares selectors that can span multiple tokens."
)]

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssKeyframesItem, CssKeyframesBlock};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, Text};
use biome_rule_options::no_duplicate_selectors_keyframe_block::NoDuplicateSelectorsKeyframeBlockOptions;
use biome_string_case::StrLikeExtension;
use std::borrow::Cow;

use crate::utils::{DuplicateGroup, collect_duplicates};

declare_lint_rule! {
    /// Disallow duplicate selectors within keyframe blocks.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @keyframes foo { from {} from {} }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @keyframes foo { from {} FROM {} }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @keyframes foo { 0% {} 0% {} }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @keyframes foo { 0% {} 100% {} }
    /// ```
    ///
    /// ```css
    /// @keyframes foo { from {} to {} }
    /// ```
    ///
    pub NoDuplicateSelectorsKeyframeBlock {
        version: "1.8.0",
        name: "noDuplicateSelectorsKeyframeBlock",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources:&[RuleSource::Stylelint("keyframe-block-no-duplicate-selectors").same(), RuleSource::EslintCss("no-duplicate-keyframe-selectors").same()],
    }
}

impl Rule for NoDuplicateSelectorsKeyframeBlock {
    type Query = Ast<CssKeyframesBlock>;
    type State = DuplicateGroup<Text>;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateSelectorsKeyframeBlockOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut selectors = Vec::new();
        for keyframe_item in node.items() {
            match keyframe_item {
                AnyCssKeyframesItem::CssKeyframesItem(item) => {
                    let Some(Ok(keyframe_selector)) = item.selectors().into_iter().next() else {
                        break;
                    };
                    let text = keyframe_selector.to_trimmed_text();
                    // Selectors are compared without case; only allocate when the text has uppercase.
                    let key = match text.to_ascii_lowercase_cow() {
                        Cow::Borrowed(_) => text.clone(),
                        Cow::Owned(lowercase) => lowercase.into(),
                    };
                    selectors.push((key, text, keyframe_selector.range()));
                }
                AnyCssKeyframesItem::CssAtRule(_) => {}
                AnyCssKeyframesItem::CssEmptyDeclaration(_) => {}
                AnyCssKeyframesItem::ScssKeyframesVariableDeclaration(_) => {}
                AnyCssKeyframesItem::CssBogusKeyframesItem(_) => break,
            }
        }
        collect_duplicates(selectors)
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
                "The keyframe selector "<Emphasis>{name}</Emphasis>" is used more than once."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "This selector is repeated here and overwrites the earlier one."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Consider using a different percentage value or keyword to avoid duplication"
        }))
    }
}
