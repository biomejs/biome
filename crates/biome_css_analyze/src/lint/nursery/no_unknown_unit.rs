use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssDimension, CssFunction, CssGenericProperty, CssQueryFeaturePlain, CssSyntaxKind,
};
use biome_rowan::{SyntaxNodeCast, TextRange};

use crate::utils::strip_vendor_prefix;

const RESOLUTION_MEDIA_FEATURE_NAMES: [&str; 3] =
    ["resolution", "min-resolution", "max-resolution"];

fn is_css_hack_unit(value: &str) -> bool {
    value == "\\0"
}

declare_rule! {
    /// Disallow unknown units.
    ///
    /// This rule considers units defined in the CSS Specifications, up to and including Editor's Drafts, to be known.
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
        version: "next",
        name: "noUnknownUnit",
        recommended: false,
        sources: &[RuleSource::Stylelint("unit-no-unknown")],
    }
}

pub struct RuleState {
    value: String,
    span: TextRange,
}

impl Rule for NoUnknownUnit {
    type Query = Ast<AnyCssDimension>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        // dbg!(ctx.root());

        match node {
            AnyCssDimension::CssUnknownDimension(dimension) => {
                let unit_token = dimension.unit_token().ok()?;
                let unit = unit_token.text_trimmed().to_string();

                if is_css_hack_unit(&unit) {
                    return None;
                }

                Some(RuleState {
                    value: unit,
                    span: unit_token.text_trimmed_range(),
                })
            }
            AnyCssDimension::CssRegularDimension(dimension) => {
                let unit_token = dimension.unit_token().ok()?;
                let unit = unit_token.text_trimmed().to_string();

                if unit == "x" {
                    let mut allow_x = false;

                    for ancestor in dimension.unit_token().ok()?.ancestors() {
                        match ancestor.kind() {
                            CssSyntaxKind::CSS_FUNCTION => {
                                let function_name = ancestor
                                    .cast::<CssFunction>()?
                                    .name()
                                    .ok()?
                                    .value_token()
                                    .ok()?
                                    .text_trimmed()
                                    .to_lowercase();

                                if strip_vendor_prefix(function_name.as_str()) == "image-set" {
                                    allow_x = true;
                                    break;
                                }
                            }
                            CssSyntaxKind::CSS_QUERY_FEATURE_PLAIN => {
                                let feature_name = ancestor
                                    .cast::<CssQueryFeaturePlain>()?
                                    .name()
                                    .ok()?
                                    .value_token()
                                    .ok()?
                                    .text_trimmed()
                                    .to_lowercase();

                                if RESOLUTION_MEDIA_FEATURE_NAMES.contains(&feature_name.as_str()) {
                                    allow_x = true;
                                    break;
                                }
                            }
                            CssSyntaxKind::CSS_GENERIC_PROPERTY => {
                                let property_name = ancestor
                                    .cast::<CssGenericProperty>()?
                                    .name()
                                    .ok()?
                                    .as_css_identifier()?
                                    .value_token()
                                    .ok()?
                                    .text_trimmed()
                                    .to_lowercase();

                                if property_name == "image-resolution" {
                                    allow_x = true;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    if !allow_x {
                        return Some(RuleState {
                            value: unit,
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
        let span = state.span;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown unit: "<Emphasis>{ state.value }</Emphasis>
                },
            )
            .note(markup! {
                "Fix to a known unit."
            }),
        )
    }
}
