use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
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
    }
}

impl Rule for NoDuplicateCustomProperties {
    type Query = Ast<CssDeclarationOrRuleList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut custom_properties: Vec<SyntaxToken<CssLanguage>> = vec![];
        for item in node {
            if let Some(css_declaration) = item.as_css_declaration_with_semicolon() {
                if let Ok(property) = css_declaration.declaration().ok()?.property() {
                    if let Some(property) = property.as_css_generic_property() {
                        if let Some(ident) = property.name().ok()?.as_css_identifier() {
                            let value_token = ident.value_token().ok()?;
                            custom_properties.push(value_token);
                        }
                    }
                }
            }
        }
        if let Some(duplicate) = check_duplicate_custom_properties(custom_properties) {
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
