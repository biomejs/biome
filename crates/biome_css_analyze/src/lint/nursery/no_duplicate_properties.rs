use std::{borrow::Cow, collections::hash_map::Entry};

use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_string_case::StrOnlyExtension;
use rustc_hash::FxHashMap;

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
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties")],
    }
}

impl Rule for NoDuplicateProperties {
    type Query = Semantic<CssDeclarationOrRuleList>;
    type State = (TextRange, (TextRange, String));
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let model = ctx.model();

        let rule = model.get_rule_by_range(node.range())?;

        let mut seen: FxHashMap<Cow<'_, str>, TextRange> = FxHashMap::default();

        for declaration in rule.declarations.iter() {
            let prop = &declaration.property;
            let prop_name = prop.name.to_lowercase_cow();
            let prop_range = prop.range;

            let is_custom_property = prop_name.starts_with("--");

            if is_custom_property {
                continue;
            }

            match seen.entry(prop_name.clone()) {
                Entry::Occupied(entry) => {
                    return Some((*entry.get(), (prop_range, prop_name.to_string())));
                }
                Entry::Vacant(_) => {
                    seen.insert(prop_name, prop_range);
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
