use crate::CssRuleAction;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssGenericPropertyValueOrExpression, AnyCssValue, CssFunction, CssGenericProperty,
    CssSyntaxToken, decode_css_identifier,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange};
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
    /// ```css,expect_diagnostic
    /// p {
    ///   float: left;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   text-align: right;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   justify-content: left;
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   width: anchor-size(width);
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// p {
    ///   top: anchor(bottom);
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
    ///   float: inline-start;
    ///   text-align: end;
    ///   justify-content: start;
    ///   inline-size: anchor-size(inline);
    ///   inset-block-start: anchor(end);
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
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// p {
    ///   margin-left: 1rem;
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// p {
    ///   margin-inline-end: 1rem;
    /// }
    /// ```
    ///
    pub UseLogicalProperties {
        version: "next",
        name: "useLogicalProperties",
        language: "css",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseLogicalProperties {
    type Query = Ast<CssGenericProperty>;
    type State = UseLogicalPropertiesState;
    type Signals = Box<[Self::State]>;
    type Options = UseLogicalPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let property = ctx.query();
        let Ok(name) = property.name() else {
            return Box::default();
        };
        let Ok(name_token) = name.declaration() else {
            return Box::default();
        };
        let normalized_name = name_token
            .text_trimmed()
            .to_ascii_lowercase_cow()
            .into_owned();
        let direction = ctx.options().direction();
        let mut states = Vec::new();

        if let Some(logical_property) =
            physical_to_logical_property(normalized_name.as_str(), direction)
        {
            states.push(UseLogicalPropertiesState {
                span: name.range(),
                token: name_token,
                violation: LogicalPropertiesViolation::PropertyName {
                    physical: normalized_name.clone(),
                    replacement: logical_property,
                },
            });
        }

        collect_value_violations(property, normalized_name.as_str(), direction, &mut states);

        states.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (physical, replacement) = state.replacement();

        Some(
            RuleDiagnostic::new(rule_category!(), state.span, state.message())
                .note(markup! {
                    "Logical properties adapt better to different writing modes and layout directions."
                })
                .note(markup! {
                    "Replace "<Emphasis>{physical}</Emphasis>" with "<Emphasis>{replacement}</Emphasis>"."
                }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<CssRuleAction> {
        let (_, replacement) = state.replacement();
        let mut mutation = ctx.root().begin();
        let new_token = CssSyntaxToken::new_detached(state.token.kind(), replacement, [], []);
        mutation.replace_token_transfer_trivia(state.token.clone(), new_token);

        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the logical CSS replacement." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseLogicalPropertiesState {
    span: TextRange,
    token: CssSyntaxToken,
    violation: LogicalPropertiesViolation,
}

enum LogicalPropertiesViolation {
    PropertyName {
        physical: String,
        replacement: &'static str,
    },
    PropertyValue {
        physical: String,
        replacement: &'static str,
    },
    AnchorSizeValue {
        physical: String,
        replacement: &'static str,
    },
    AnchorValue {
        physical: String,
        replacement: &'static str,
    },
}

impl UseLogicalPropertiesState {
    fn message(&self) -> &'static str {
        match self.violation {
            LogicalPropertiesViolation::PropertyName { .. } => {
                "Use logical CSS properties over physical ones."
            }
            LogicalPropertiesViolation::PropertyValue { .. } => {
                "Use logical CSS values over physical ones."
            }
            LogicalPropertiesViolation::AnchorSizeValue { .. } => {
                "Use a logical size in anchor-size()."
            }
            LogicalPropertiesViolation::AnchorValue { .. } => "Use a logical side in anchor().",
        }
    }

    fn replacement(&self) -> (&str, &'static str) {
        match &self.violation {
            LogicalPropertiesViolation::PropertyName {
                physical,
                replacement,
            }
            | LogicalPropertiesViolation::PropertyValue {
                physical,
                replacement,
            }
            | LogicalPropertiesViolation::AnchorSizeValue {
                physical,
                replacement,
            }
            | LogicalPropertiesViolation::AnchorValue {
                physical,
                replacement,
            } => (physical.as_str(), replacement),
        }
    }
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

fn collect_value_violations(
    property: &CssGenericProperty,
    property_name: &str,
    direction: UseLogicalPropertiesDirection,
    states: &mut Vec<UseLogicalPropertiesState>,
) {
    let Ok(AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(values)) =
        property.value()
    else {
        return;
    };

    for value in values
        .iter()
        .filter_map(|component| component.as_any_css_value().cloned())
    {
        if let Some((token, physical)) = value_identifier_token(&value)
            && let Some(replacement) =
                physical_to_logical_value(property_name, physical.as_str(), direction)
        {
            states.push(UseLogicalPropertiesState {
                span: token.text_trimmed_range(),
                token,
                violation: LogicalPropertiesViolation::PropertyValue {
                    physical,
                    replacement,
                },
            });
        }
    }

    for function in values.syntax().descendants().filter_map(CssFunction::cast) {
        collect_function_violations(&function, direction, states);
    }
}

fn collect_function_violations(
    function: &CssFunction,
    direction: UseLogicalPropertiesDirection,
    states: &mut Vec<UseLogicalPropertiesState>,
) {
    let Some(function_name) = function_name(function) else {
        return;
    };

    let is_anchor_size = function_name == "anchor-size";
    let is_anchor = function_name == "anchor";

    if !is_anchor_size && !is_anchor {
        return;
    }

    for value in function
        .items()
        .syntax()
        .descendants()
        .filter_map(AnyCssValue::cast)
    {
        let Some((token, physical)) = value_identifier_token(&value) else {
            continue;
        };

        let replacement = if is_anchor_size {
            physical_to_logical_anchor_size(physical.as_str())
        } else {
            physical_to_logical_anchor_side(physical.as_str(), direction)
        };

        if let Some(replacement) = replacement {
            states.push(UseLogicalPropertiesState {
                span: token.text_trimmed_range(),
                token,
                violation: if is_anchor_size {
                    LogicalPropertiesViolation::AnchorSizeValue {
                        physical,
                        replacement,
                    }
                } else {
                    LogicalPropertiesViolation::AnchorValue {
                        physical,
                        replacement,
                    }
                },
            });
        }
    }
}

fn function_name(function: &CssFunction) -> Option<String> {
    let token = function
        .name()
        .ok()?
        .as_css_identifier()?
        .value_token()
        .ok()?;
    Some(
        decode_css_identifier(token.text_trimmed())
            .to_ascii_lowercase_cow()
            .into_owned(),
    )
}

fn value_identifier_token(value: &AnyCssValue) -> Option<(CssSyntaxToken, String)> {
    let token = match value {
        AnyCssValue::CssIdentifier(identifier) => identifier.value_token().ok()?,
        AnyCssValue::CssCustomIdentifier(identifier) => identifier.value_token().ok()?,
        AnyCssValue::AnyCssDashedIdentifier(identifier) => {
            identifier.as_css_dashed_identifier()?.value_token().ok()?
        }
        _ => return None,
    };
    let physical = decode_css_identifier(token.text_trimmed())
        .to_ascii_lowercase_cow()
        .into_owned();

    Some((token, physical))
}

fn physical_to_logical_value(
    property: &str,
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match property {
        "float" | "clear" => physical_to_logical_inline_value(value, direction),
        "text-align" | "justify-content" | "justify-items" | "justify-self" => {
            physical_to_logical_start_end_value(value, direction)
        }
        _ => None,
    }
}

fn physical_to_logical_anchor_size(value: &str) -> Option<&'static str> {
    match value {
        "width" => Some("inline"),
        "height" => Some("block"),
        _ => None,
    }
}

fn physical_to_logical_anchor_side(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match value {
        "top" => Some("start"),
        "bottom" => Some("end"),
        _ => physical_to_logical_start_end_value(value, direction),
    }
}

fn physical_to_logical_inline_value(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match (value, direction) {
        ("left", UseLogicalPropertiesDirection::Ltr)
        | ("right", UseLogicalPropertiesDirection::Rtl) => Some("inline-start"),
        ("right", UseLogicalPropertiesDirection::Ltr)
        | ("left", UseLogicalPropertiesDirection::Rtl) => Some("inline-end"),
        _ => None,
    }
}

fn physical_to_logical_start_end_value(
    value: &str,
    direction: UseLogicalPropertiesDirection,
) -> Option<&'static str> {
    match (value, direction) {
        ("left", UseLogicalPropertiesDirection::Ltr)
        | ("right", UseLogicalPropertiesDirection::Rtl) => Some("start"),
        ("right", UseLogicalPropertiesDirection::Ltr)
        | ("left", UseLogicalPropertiesDirection::Rtl) => Some("end"),
        _ => None,
    }
}
