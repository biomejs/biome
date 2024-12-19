use std::collections::HashSet;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssKeyframesItem, AnyCssKeyframesSelector, CssKeyframesBlock};
use biome_diagnostics::Severity;
use biome_rowan::AstNode;
use biome_string_case::StrLikeExtension;

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
        sources:&[RuleSource::Stylelint("keyframe-block-no-duplicate-selectors")],
    }
}

impl Rule for NoDuplicateSelectorsKeyframeBlock {
    type Query = Ast<CssKeyframesBlock>;
    type State = AnyCssKeyframesSelector;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut selector_list: HashSet<String> = HashSet::new();
        for keyframe_item in node.items() {
            match keyframe_item {
                AnyCssKeyframesItem::CssKeyframesItem(item) => {
                    let keyframe_selector = item.selectors().into_iter().next()?.ok()?;
                    if !selector_list.insert(
                        keyframe_selector
                            .to_trimmed_string()
                            .to_ascii_lowercase_cow()
                            .to_string(),
                    ) {
                        return Some(keyframe_selector);
                    }
                }
                _ => return None,
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                   "The duplicate keyframe selector is overwritten by later one."
                },
            )
            .note(markup! {
                    "Consider using a different percentage value or keyword to avoid duplication"
            }),
        )
    }
}
