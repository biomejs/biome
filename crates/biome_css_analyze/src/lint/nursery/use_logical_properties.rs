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
    /// Enforce logical sizing properties over physical sizing properties.
    ///
    /// Physical properties such as `width` and `height` are tied to writing direction.
    /// Logical properties such as `inline-size` and `block-size` adapt more consistently
    /// across different writing modes.
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
    /// ### Valid
    ///
    /// ```css
    /// p {
    ///   inline-size: 100%;
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
        "width" => Some("inline-size"),
        "min-width" => Some("min-inline-size"),
        "max-width" => Some("max-inline-size"),
        "height" => Some("block-size"),
        "min-height" => Some("min-block-size"),
        "max-height" => Some("max-block-size"),
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
