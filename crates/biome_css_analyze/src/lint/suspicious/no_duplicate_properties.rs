use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssDeclarationOrRuleList, CssKeyframesAtRule};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_properties::NoDuplicatePropertiesOptions;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow duplicate properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate properties. It ignores custom properties.
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
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, Box<str>));
    type Signals = Option<Self::State>;
    type Options = NoDuplicatePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range())?;

        let mut seen: FxHashMap<Box<str>, TextRange> = FxHashMap::default();

        for declaration in rule.declarations() {
            let prop = declaration.property();
            let prop_name = prop.to_trimmed_text();
            let prop_range = prop.range();

            let is_custom_property = prop_name.starts_with("--");

            if is_custom_property {
                continue;
            }

            // We skip this declaration if it's inside a keyframes block.
            if prop
                .syntax()
                .ancestors()
                .any(|node| CssKeyframesAtRule::can_cast(node.kind()))
            {
                continue;
            }

            match seen.entry(prop_name.clone().into()) {
                Entry::Occupied(entry) => {
                    return Some((*entry.get(), (prop_range, prop_name.into())));
                }
                Entry::Vacant(entry) => {
                    entry.insert(prop_range);
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
                <Emphasis>{duplicate_property_name}</Emphasis> " is already defined here."
            })
            .note(markup! {
                "Remove or rename the duplicate property to ensure consistent styling."
            }),
        )
    }
}
