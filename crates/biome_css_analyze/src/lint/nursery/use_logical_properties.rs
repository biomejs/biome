use biome_analyze::{
    Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssDashedIdentifier, AnyCssDeclarationName, CssGenericProperty};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_logical_properties::UseLogicalPropertiesOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Enforce logical properties over physical properties.
    ///
    /// Physical properties such as `width`, `height`, `top`, `left`, `margin-top`, `padding-left`,
    /// `border-top`, `border-left-color`, etc. are tied to writing direction. Logical properties such
    /// as `inline-size`, `block-size`, `inset-block-start`, `margin-block-start`,
    /// `padding-inline-end`, `border-block-start`, `border-inline-start-color`, etc. adapt more
    /// consistently across different writing modes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   width: 100%;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   top: 0;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   margin-left: 1rem;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   border-left: 1px solid;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   inline-size: 100%;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   inset-block-start: 0;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   margin-inline-start: 1rem;
    /// }
    /// ```
    ///
    /// ```css
    /// p {
    ///   border-inline-start: 1px solid;
    /// }
    /// ```
    ///
    pub UseLogicalProperties {
        version: "next",
        name: "useLogicalProperties",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for UseLogicalProperties {
    type Query = Ast<CssGenericProperty>;
    type State = UseLogicalPropertiesState;
    type Signals = Option<Self::State>;
    type Options = UseLogicalPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let property = ctx.query();
        let name = property.name().ok()?;
        let name_token = declaration_name_value_token(&name)?;
        let normalized_name = name_token.text_trimmed().to_ascii_lowercase_cow();
        let logical_property = physical_to_logical_property(normalized_name.as_ref())?;

        Some(UseLogicalPropertiesState {
            span: name.range(),
            physical_property: normalized_name.to_string(),
            logical_property,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Use logical CSS properties over physical ones."
                },
            )
            .note(markup! {
                "Replace "<Emphasis>{state.physical_property.as_str()}</Emphasis>" with "<Emphasis>{state.logical_property}</Emphasis>"."
            })
            .note(markup! {
                "Logical properties adapt better to different writing modes and layout directions."
            }),
        )
    }
}

pub struct UseLogicalPropertiesState {
    span: TextRange,
    physical_property: String,
    logical_property: &'static str,
}

fn physical_to_logical_property(property: &str) -> Option<&'static str> {
    match property {
        // Sizing properties
        "width" => Some("inline-size"),
        "min-width" => Some("min-inline-size"),
        "max-width" => Some("max-inline-size"),
        "height" => Some("block-size"),
        "min-height" => Some("min-block-size"),
        "max-height" => Some("max-block-size"),
        // Positioning properties
        "top" => Some("inset-block-start"),
        "right" => Some("inset-inline-end"),
        "bottom" => Some("inset-block-end"),
        "left" => Some("inset-inline-start"),
        // Margin properties
        "margin-top" => Some("margin-block-start"),
        "margin-right" => Some("margin-inline-end"),
        "margin-bottom" => Some("margin-block-end"),
        "margin-left" => Some("margin-inline-start"),
        // Padding properties
        "padding-top" => Some("padding-block-start"),
        "padding-right" => Some("padding-inline-end"),
        "padding-bottom" => Some("padding-block-end"),
        "padding-left" => Some("padding-inline-start"),
        // Border top properties
        "border-top" => Some("border-block-start"),
        "border-top-color" => Some("border-block-start-color"),
        "border-top-style" => Some("border-block-start-style"),
        "border-top-width" => Some("border-block-start-width"),
        // Border bottom properties
        "border-bottom" => Some("border-block-end"),
        "border-bottom-color" => Some("border-block-end-color"),
        "border-bottom-style" => Some("border-block-end-style"),
        "border-bottom-width" => Some("border-block-end-width"),
        // Border left properties
        "border-left" => Some("border-inline-start"),
        "border-left-color" => Some("border-inline-start-color"),
        "border-left-style" => Some("border-inline-start-style"),
        "border-left-width" => Some("border-inline-start-width"),
        // Border right properties
        "border-right" => Some("border-inline-end"),
        "border-right-color" => Some("border-inline-end-color"),
        "border-right-style" => Some("border-inline-end-style"),
        "border-right-width" => Some("border-inline-end-width"),
        // Border radius properties
        "border-top-left-radius" => Some("border-start-start-radius"),
        "border-top-right-radius" => Some("border-start-end-radius"),
        "border-bottom-left-radius" => Some("border-end-start-radius"),
        "border-bottom-right-radius" => Some("border-end-end-radius"),
        _ => None,
    }
}

fn declaration_name_value_token(
    name: &AnyCssDeclarationName,
) -> Option<biome_css_syntax::CssSyntaxToken> {
    match name {
        AnyCssDeclarationName::AnyCssDashedIdentifier(
            AnyCssDashedIdentifier::CssDashedIdentifier(name),
        ) => name.value_token().ok(),
        AnyCssDeclarationName::AnyCssDashedIdentifier(
            AnyCssDashedIdentifier::ScssInterpolatedDashedIdentifier(_),
        ) => None,
        AnyCssDeclarationName::CssIdentifier(name) => name.value_token().ok(),
        AnyCssDeclarationName::TwValueThemeReference(name) => {
            name.reference().ok()?.value_token().ok()
        }
        AnyCssDeclarationName::ScssInterpolatedIdentifier(_) => None,
    }
}
