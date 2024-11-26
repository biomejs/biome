use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDimension, CssFunction, CssGenericProperty, CssQueryFeaturePlain, CssSyntaxKind,
};
use biome_diagnostics::Severity;
use biome_rowan::{SyntaxNodeCast, TextRange};
use biome_string_case::StrLikeExtension;

const RESOLUTION_MEDIA_FEATURE_NAMES: [&str; 3] =
    ["resolution", "min-resolution", "max-resolution"];

declare_lint_rule! {
    /// Disallow unknown CSS units.
    ///
    /// For details on known CSS units, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#lengths).
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: 10pixels;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a {
    ///   width: calc(10px + 10pixels);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a {
    ///   width: 10px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10Px;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: 10pX;
    /// }
    /// ```
    ///
    /// ```css
    /// a {
    ///   width: calc(10px + 10px);
    /// }
    /// ```
    ///
    pub NoUnknownUnit {
        version: "1.8.0",
        name: "noUnknownUnit",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("unit-no-unknown")],
    }
}

pub struct NoUnknownUnitState {
    unit: String,
    span: TextRange,
}

impl Rule for NoUnknownUnit {
    type Query = Ast<AnyCssDimension>;
    type State = NoUnknownUnitState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        match node {
            AnyCssDimension::CssUnknownDimension(dimension) => {
                let unit_token = dimension.unit_token().ok()?;
                let unit = unit_token.text_trimmed().to_string();

                Some(NoUnknownUnitState {
                    unit,
                    span: unit_token.text_trimmed_range(),
                })
            }
            AnyCssDimension::CssRegularDimension(dimension) => {
                let unit_token = dimension.unit_token().ok()?;
                let unit = unit_token.text_trimmed().to_string();

                // The `x` unit is parsed as `CssRegularDimension`, but it is used for describing resolutions.
                // This check is to disallow the use of the `x` unit outside this specific context.
                if unit == "x" {
                    let mut allow_x = false;

                    for ancestor in dimension.unit_token().ok()?.ancestors() {
                        match ancestor.kind() {
                            CssSyntaxKind::CSS_FUNCTION => {
                                let function_name_token = ancestor
                                    .cast::<CssFunction>()?
                                    .name()
                                    .ok()?
                                    .value_token()
                                    .ok()?;
                                let function_name =
                                    function_name_token.text_trimmed().to_ascii_lowercase_cow();

                                if function_name.ends_with("image-set") {
                                    allow_x = true;
                                    break;
                                }
                            }
                            CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                                let property_name_token = ancestor
                                    .cast::<CssGenericProperty>()?
                                    .name()
                                    .ok()?
                                    .as_css_identifier()?
                                    .value_token()
                                    .ok()?;
                                let property_name =
                                    property_name_token.text_trimmed().to_ascii_lowercase_cow();

                                if property_name == "image-resolution" {
                                    allow_x = true;
                                    break;
                                }
                            }
                            CssSyntaxKind::CSS_QUERY_FEATURE_PLAIN => {
                                let feature_name_token = ancestor
                                    .cast::<CssQueryFeaturePlain>()?
                                    .name()
                                    .ok()?
                                    .value_token()
                                    .ok()?;
                                let feature_name =
                                    feature_name_token.text_trimmed().to_ascii_lowercase_cow();

                                if RESOLUTION_MEDIA_FEATURE_NAMES.contains(&feature_name.as_ref()) {
                                    allow_x = true;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if !allow_x {
                        return Some(NoUnknownUnitState {
                            unit,
                            span: unit_token.text_trimmed_range(),
                        });
                    }
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Unexpected unknown unit: "<Emphasis>{ state.unit }</Emphasis>
                },
            )
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#lengths">"MDN web docs"</Hyperlink>" for more details."
            })
            .footer_list(
                markup! {
                    "Use a known unit instead, such as:"
                },
                ["px", "em", "rem", "etc."],
            ),

        )
    }
}
