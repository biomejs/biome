use biome_analyze::{
    Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssGenericProperty;
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_logical_properties::{
    UseLogicalPropertiesDirection, UseLogicalPropertiesOptions,
};
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
    ///   inset-block-start: 0;
    ///   margin-inline-start: 1rem;
    ///   border-inline-start: 1px solid;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `direction`
    ///
    /// The text direction used to map physical inline properties. It can be either `"ltr"` or
    /// `"rtl"`. Defaults to `"ltr"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "direction": "rtl"
    ///   }
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
        let name_token = name.declaration().ok()?;
        let normalized_name = name_token.text_trimmed().to_ascii_lowercase_cow();
        let logical_property =
            physical_to_logical_property(normalized_name.as_ref(), ctx.options().direction())?;

        Some(UseLogicalPropertiesState {
            span: name.range(),
            logical_property,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let property = ctx.query();
        let name = property.name().ok()?;
        let name_token = name.declaration().ok()?;
        let normalized_name = name_token.text_trimmed().to_ascii_lowercase_cow();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Use logical CSS properties over physical ones."
                },
            )
            .note(markup! {
                "Logical properties adapt better to different writing modes and layout directions."
            })
            .note(markup! {
                "Replace "<Emphasis>{normalized_name.as_ref()}</Emphasis>" with "<Emphasis>{state.logical_property}</Emphasis>"."
            }),
        )
    }
}

pub struct UseLogicalPropertiesState {
    span: TextRange,
    logical_property: &'static str,
}

/// Maps a physical property name to its logical `(ltr, rtl)` replacement names.
static LOGICAL_PROPERTIES: phf::Map<&'static str, (&'static str, &'static str)> = phf::phf_map! {
    // Sizing properties
    "width" => ("inline-size", "inline-size"),
    "min-width" => ("min-inline-size", "min-inline-size"),
    "max-width" => ("max-inline-size", "max-inline-size"),
    "height" => ("block-size", "block-size"),
    "min-height" => ("min-block-size", "min-block-size"),
    "max-height" => ("max-block-size", "max-block-size"),
    // Positioning properties
    "top" => ("inset-block-start", "inset-block-start"),
    "right" => ("inset-inline-end", "inset-inline-start"),
    "bottom" => ("inset-block-end", "inset-block-end"),
    "left" => ("inset-inline-start", "inset-inline-end"),
    // Margin properties
    "margin-top" => ("margin-block-start", "margin-block-start"),
    "margin-right" => ("margin-inline-end", "margin-inline-start"),
    "margin-bottom" => ("margin-block-end", "margin-block-end"),
    "margin-left" => ("margin-inline-start", "margin-inline-end"),
    // Padding properties
    "padding-top" => ("padding-block-start", "padding-block-start"),
    "padding-right" => ("padding-inline-end", "padding-inline-start"),
    "padding-bottom" => ("padding-block-end", "padding-block-end"),
    "padding-left" => ("padding-inline-start", "padding-inline-end"),
    // Border top properties
    "border-top" => ("border-block-start", "border-block-start"),
    "border-top-color" => ("border-block-start-color", "border-block-start-color"),
    "border-top-style" => ("border-block-start-style", "border-block-start-style"),
    "border-top-width" => ("border-block-start-width", "border-block-start-width"),
    // Border bottom properties
    "border-bottom" => ("border-block-end", "border-block-end"),
    "border-bottom-color" => ("border-block-end-color", "border-block-end-color"),
    "border-bottom-style" => ("border-block-end-style", "border-block-end-style"),
    "border-bottom-width" => ("border-block-end-width", "border-block-end-width"),
    // Border left properties
    "border-left" => ("border-inline-start", "border-inline-end"),
    "border-left-color" => ("border-inline-start-color", "border-inline-end-color"),
    "border-left-style" => ("border-inline-start-style", "border-inline-end-style"),
    "border-left-width" => ("border-inline-start-width", "border-inline-end-width"),
    // Border right properties
    "border-right" => ("border-inline-end", "border-inline-start"),
    "border-right-color" => ("border-inline-end-color", "border-inline-start-color"),
    "border-right-style" => ("border-inline-end-style", "border-inline-start-style"),
    "border-right-width" => ("border-inline-end-width", "border-inline-start-width"),
    // Border radius properties
    "border-top-left-radius" => ("border-start-start-radius", "border-start-end-radius"),
    "border-top-right-radius" => ("border-start-end-radius", "border-start-start-radius"),
    "border-bottom-left-radius" => ("border-end-start-radius", "border-end-end-radius"),
    "border-bottom-right-radius" => ("border-end-end-radius", "border-end-start-radius"),
};

fn physical_to_logical_property(
    property: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    let (ltr, rtl) = LOGICAL_PROPERTIES.get(property)?;
    Some(match direction {
        UseLogicalPropertiesDirection::Ltr => ltr,
        UseLogicalPropertiesDirection::Rtl => rtl,
    })
}
