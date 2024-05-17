use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{CssDeclarationOrRuleList, CssLanguage};
use biome_rowan::{SyntaxToken, TextRange};

declare_rule! {
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
        version: "next",
        name: "noDuplicateCustomProperties",
        recommended: false,
        sources: &[RuleSource::Stylelint("declaration-block-no-duplicate-properties")],
    }
}

impl Rule for NoDuplicateCustomProperties {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let custom_properties = node.into_iter().filter_map(|item| {
            item.as_css_declaration_with_semicolon()
                .and_then(|css_declaration| {
                    css_declaration
                        .declaration()
                        .ok()?
                        .property()
                        .ok()?
                        .as_css_generic_property()
                        .and_then(|css_generic_property| {
                            css_generic_property
                                .name()
                                .ok()?
                                .as_css_identifier()
                                .and_then(|css_identifier| css_identifier.value_token().ok())
                        })
                })
        });
        if let Some(duplicate) = check_duplicate_custom_properties(custom_properties.collect()) {
            return Some(duplicate.text_trimmed_range());
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Duplicate custom properties are not allowed."
                },
            )
            .note(markup! {
                    "Consider removing the duplicate custom property."
            }),
        )
    }
}

fn check_duplicate_custom_properties(
    custom_properties: Vec<SyntaxToken<CssLanguage>>,
) -> Option<SyntaxToken<CssLanguage>> {
    let mut seen = std::collections::HashSet::<&str>::new();
    for value in custom_properties.iter() {
        let trimmed_text = value.text_trimmed();
        if !seen.insert(trimmed_text) {
            return Some(value.clone());
        }
    }
    None
}
