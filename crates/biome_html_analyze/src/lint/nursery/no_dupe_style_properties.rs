use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnySvelteBindingProperty, HtmlAttributeList};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_dupe_style_properties::NoDupeStylePropertiesOptions;

declare_lint_rule! {
    /// Disallow duplicate `style:` directives on the same Svelte element.
    ///
    /// Having two `style:` directives for the same CSS property on a single element is redundant
    /// and likely a mistake. Only one of them will take effect.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <div style:color="red" style:color="blue"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <div style:color="red" style:background="blue"></div>
    /// ```
    ///
    pub NoDupeStyleProperties {
        version: "next",
        name: "noDupeStyleProperties",
        language: "html",
        domains: &[RuleDomain::Svelte],
        recommended: true,
        sources: &[RuleSource::EslintSvelte("no-dupe-style-properties").same()],
    }
}

pub struct State {
    /// Range of the duplicate directive.
    duplicate_range: TextRange,
    /// The property name.
    name: String,
    /// Range of the first occurrence.
    original_range: TextRange,
}

impl Rule for NoDupeStyleProperties {
    type Query = Ast<HtmlAttributeList>;
    type State = State;
    type Signals = Box<[Self::State]>;
    type Options = NoDupeStylePropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut seen: Vec<(String, TextRange)> = Vec::new();
        let mut violations: Vec<State> = Vec::new();

        for attribute in node.iter() {
            let Some(directive) = attribute
                .as_any_svelte_directive()
                .and_then(|dir| dir.as_svelte_style_directive())
            else {
                continue;
            };

            let Ok(value) = directive.value() else {
                continue;
            };
            let Ok(property) = value.property() else {
                continue;
            };

            let name_text = property_name_text(&property);
            let Some(name_text) = name_text else {
                continue;
            };

            if let Some((_, original_range)) =
                seen.iter().find(|(prev_name, _)| prev_name == &name_text)
            {
                violations.push(State {
                    duplicate_range: directive.range(),
                    name: name_text.clone(),
                    original_range: *original_range,
                });
            } else {
                seen.push((name_text, directive.range()));
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name.as_str();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.duplicate_range,
                markup! {
                    "Duplicate "<Emphasis>"style:"</Emphasis>" directive for property "<Emphasis>{name}</Emphasis>"."
                },
            )
            .detail(
                state.original_range,
                "This is the first occurrence of the directive.",
            ),
        )
    }
}

/// Extract the property name text from an `AnySvelteBindingProperty`.
/// Handles both `SvelteName` (simple identifier) and `SvelteLiteral` (hyphenated name).
fn property_name_text(property: &AnySvelteBindingProperty) -> Option<String> {
    if let Some(svelte_name) = property.as_svelte_name() {
        let token = svelte_name.ident_token().ok()?;
        return Some(token.text_trimmed().to_string());
    }
    if let Some(svelte_literal) = property.as_svelte_literal() {
        let token = svelte_literal.value_token().ok()?;
        return Some(token.text_trimmed().to_string());
    }
    None
}
