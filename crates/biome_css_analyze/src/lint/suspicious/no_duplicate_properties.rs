use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDeclarationOrRule, CssDeclarationOrRuleList, CssKeyframesAtRule, TwUtilityAtRule,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_duplicate_properties::NoDuplicatePropertiesOptions;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

use crate::utils::AnyPropertyStatement;

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties and declarations inside `@keyframes` blocks.
    ///
    /// ## SCSS limitations
    ///
    /// Declarations are compared within each authored declaration block. The rule does not expand
    /// mixins or includes, so duplicates introduced only after SCSS expansion are not reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   color: pink;
    ///   color: orange;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   color: pink;
    ///   background: orange;
    /// }
    /// ```
    ///
    pub NoDuplicateProperties {
        version: "1.9.4",
        name: "noDuplicateProperties",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties").same()],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, TokenText));
    type Signals = Option<Self::State>;
    type Options = NoDuplicatePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        // Tailwind resolves each `--value()` in a `@utility` and keeps the declarations that resolve,
        // so repeating a property there is the documented way to write fallbacks.
        if node.syntax().ancestors().any(|ancestor| {
            CssKeyframesAtRule::can_cast(ancestor.kind())
                || TwUtilityAtRule::can_cast(ancestor.kind())
        }) {
            return None;
        }

        let mut seen: FxHashMap<TokenText, TextRange> = FxHashMap::default();

        for item in node {
            let AnyCssDeclarationOrRule::CssDeclarationWithSemicolon(declaration) = item else {
                continue;
            };
            let Some(name) = AnyPropertyStatement::from(declaration).name() else {
                continue;
            };
            let prop_name = name.token_text_trimmed();

            if prop_name.text().starts_with("--") {
                continue;
            }

            match seen.entry(prop_name) {
                Entry::Occupied(entry) => {
                    return Some((
                        *entry.get(),
                        (name.text_trimmed_range(), entry.key().clone()),
                    ));
                }
                Entry::Vacant(entry) => {
                    entry.insert(name.text_trimmed_range());
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_occurrence_range, (duplicate_range, duplicate_property_name)) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                duplicate_range,
                markup! {
                    "Duplicate properties can lead to unexpected behavior and may override previous declarations unintentionally."
                },
            )
            .detail(first_occurrence_range, markup! {
                <Emphasis>{duplicate_property_name.text()}</Emphasis> " is already defined here."
            })
            .note(markup! {
                "Remove or rename the duplicate property to ensure consistent styling."
            }),
        )
    }
}
