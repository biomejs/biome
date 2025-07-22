use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_custom_properties::NoDuplicateCustomPropertiesOptions;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

declare_lint_rule! {
    /// Disallow duplicate custom properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate custom properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { --custom-property: pink; --custom-property: orange;  }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { --custom-property: pink; background: orange; --custom-property: orange }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { --custom-property: pink; }
    /// ```
    ///
    /// ```css
    /// a { --custom-property: pink; --cUstOm-prOpErtY: orange; }
    /// ```
    ///
    pub NoDuplicateCustomProperties {
        version: "1.9.0",
        name: "noDuplicateCustomProperties",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-custom-properties").same()],
    }
}

impl Rule for NoDuplicateCustomProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, String));
    type Signals = Option<Self::State>;
    type Options = NoDuplicateCustomPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range())?;

        let mut seen: FxHashMap<Box<str>, TextRange> = FxHashMap::default();

        for declaration in rule.declarations() {
            let prop = declaration.property();
            let prop_name = prop.value().ok()?;
            let prop_range = prop.range();

            let is_custom_property = prop_name.starts_with("--");

            if !is_custom_property {
                continue;
            }

            match seen.entry(prop_name.text().into()) {
                Entry::Occupied(entry) => {
                    return Some((*entry.get(), (prop_range, prop_name.to_string())));
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
                    "Duplicate custom properties can lead to unexpected behavior and may override previous declarations unintentionally."
                },
            )
            .detail(first_occurrence_range, markup! {
                <Emphasis>{duplicate_property_name}</Emphasis> " is already defined here."
            })
            .note(markup! {
                "Remove or rename the duplicate custom property to ensure consistent styling."
            }),
        )
    }
}
