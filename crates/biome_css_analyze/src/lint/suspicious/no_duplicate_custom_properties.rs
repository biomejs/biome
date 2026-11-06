use crate::services::semantic::Semantic;
use crate::utils::{DuplicateGroup, collect_duplicates};
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::CssDeclarationOrRuleList;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_duplicate_custom_properties::NoDuplicateCustomPropertiesOptions;

declare_lint_rule! {
    /// Disallow duplicate custom properties within declaration blocks.
    ///
    /// This rule checks the declaration blocks for duplicate custom properties.
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
    type State = DuplicateGroup<TokenText>;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateCustomPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let Some(rule) = model.get_rule_by_range(node.range()) else {
            return Box::default();
        };

        collect_duplicates(
            rule.declarations()
                .iter()
                .filter(|declaration| {
                    declaration
                        .declaration()
                        .syntax()
                        .parent()
                        .and_then(|parent| parent.parent())
                        .as_ref()
                        == Some(node.syntax())
                })
                .filter_map(|declaration| {
                    let prop = declaration.property();
                    let prop_name = prop.value().ok()?;

                    if !prop_name.text().starts_with("--") {
                        return None;
                    }

                    Some((prop_name.clone(), prop_name, prop.range()))
                }),
        )
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
                "The custom property "<Emphasis>{name}</Emphasis>" is declared more than once in the same block."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    <Emphasis>{name}</Emphasis>" is declared again here."
                },
            );
        }

        Some(
            diagnostic
                .note(markup! {
                    "Duplicate custom properties can lead to unexpected behavior and may override previous declarations unintentionally."
                })
                .note(markup! {
                    "Remove or rename the duplicate custom property to ensure consistent styling."
                }),
        )
    }
}
