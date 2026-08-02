use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_semantic::model::CustomProperty;
use biome_css_syntax::{
    AnyCssDeclarationName, AnyCssDimension, AnyCssFunction, AnyCssGenericComponentValue,
    AnyCssGenericPropertyValueOrExpression, AnyCssValue, CssGenericComponentValueList,
    CssPropertyAtRule,
    keywords::{
        ANGLE_UNITS, BASIC_KEYWORDS, COLOR_FUNCTIONS, COLOR_KEYWORDS, IMAGE_FUNCTIONS,
        LENGTH_UNITS, MATH_FUNCTIONS, RESOLUTION_UNITS, TIME_UNITS, TRANSFORM_FUNCTIONS,
    },
};
use biome_property_codec::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxMultiplier,
    PropertySyntaxResult, PropertySyntaxType, decode_css_identifier,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};
use biome_rule_options::no_inconsistent_property_init_value::NoInconsistentPropertyInitValueOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Checks that the `initial-value` of an `@property` rule follows the value format declared by its `syntax`.
    ///
    /// Browsers do not register a custom property when its `initial-value` does not follow this
    /// format.
    ///
    /// For function values, this rule checks the function name but does not check its arguments.
    /// It leaves the browser to validate:
    ///
    /// - math functions such as `calc()`, `min()`, and `max()` used with `<angle>`, `<integer>`,
    ///   `<length>`, `<length-percentage>`, `<number>`, `<percentage>`, `<resolution>`, or `<time>`;
    /// - color functions such as `rgb()` and `color-mix()` used with `<color>`;
    /// - image functions such as `linear-gradient()` and `image-set()` used with `<image>`;
    /// - transform functions such as `rotate()` and `translateX()` used with
    ///   `<transform-function>` or `<transform-list>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// `red` is a color, not a length, so the browser does not register `--size`.
    ///
    /// ```css,expect_diagnostic
    /// @property --size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: red;
    /// }
    /// ```
    ///
    /// `#fff` is a color, not an image, so the browser does not register `--background-image`.
    ///
    /// ```css,expect_diagnostic
    /// @property --background-image {
    ///   syntax: "<image>";
    ///   inherits: false;
    ///   initial-value: #fff;
    /// }
    /// ```
    ///
    /// `<color>#` requires one or more colors separated by commas. The browser does not register
    /// `--palette` because `red blue` has no comma.
    ///
    /// ```css,expect_diagnostic
    /// @property --palette {
    ///   syntax: "<color>#";
    ///   inherits: false;
    ///   initial-value: red blue;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Both `1rem` and `calc(1px + 2px)` use length values, so they follow their declared formats.
    ///
    /// ```css
    /// @property --size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: 1rem;
    /// }
    ///
    /// @property --calculated-size {
    ///   syntax: "<length>";
    ///   inherits: false;
    ///   initial-value: calc(1px + 2px);
    /// }
    /// ```
    ///
    pub NoInconsistentPropertyInitValue {
        version: "next",
        name: "noInconsistentPropertyInitValue",
        language: "css",
        recommended: true,
    }
}

impl Rule for NoInconsistentPropertyInitValue {
    type Query = Semantic<CssPropertyAtRule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoInconsistentPropertyInitValueOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let property = find_property(ctx, node)?;
        let PropertySyntaxResult::Value(syntax) = property.syntax() else {
            return None;
        };
        if syntax.is_universal() {
            return None;
        }

        let initial_value = find_initial_value(node)?;
        if matches_syntax(syntax, &initial_value) {
            None
        } else {
            Some(initial_value.range())
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! { "The "<Emphasis>"initial-value"</Emphasis>" does not match the registered property syntax." },
            )
            .note(markup! {
                "A mismatched initial value prevents the custom property from being registered."
            })
            .note(markup! {
                "Use an initial value accepted by the "<Emphasis>"syntax"</Emphasis>" descriptor."
            }),
        )
    }
}

fn find_property(
    ctx: &RuleContext<NoInconsistentPropertyInitValue>,
    node: &CssPropertyAtRule,
) -> Option<CustomProperty> {
    ctx.model()
        .global_custom_variables()
        .at_property_by_range(node.range())
}

fn find_initial_value(node: &CssPropertyAtRule) -> Option<CssGenericComponentValueList> {
    let block = node.block().ok()?.as_css_declaration_block()?.clone();
    let mut initial_value = None;

    for declaration in block.declarations() {
        let Some(declaration_with_semicolon) = declaration.as_css_declaration_with_semicolon()
        else {
            continue;
        };
        let Ok(declaration) = declaration_with_semicolon.declaration() else {
            continue;
        };
        let Ok(property) = declaration.property() else {
            continue;
        };
        let Some(property) = property.as_css_generic_property() else {
            continue;
        };
        let Ok(name) = property.name() else {
            continue;
        };
        let Some(name) = declaration_name_text(&name) else {
            continue;
        };
        if !decode_css_identifier(name.text()).eq_ignore_ascii_case("initial-value") {
            continue;
        }

        initial_value = property.value().ok().and_then(|value| match value {
            AnyCssGenericPropertyValueOrExpression::CssGenericComponentValueList(value) => {
                Some(value)
            }
            _ => None,
        });
    }

    initial_value
}

fn matches_syntax(syntax: &PropertySyntax, values: &CssGenericComponentValueList) -> bool {
    let PropertySyntax::Components(components) = syntax else {
        return true;
    };

    components
        .iter()
        .any(|component| matches_component(component, values))
}

fn matches_component(
    component: &PropertySyntaxComponent,
    values: &CssGenericComponentValueList,
) -> bool {
    let mut values = values.iter();
    match component.multiplier {
        PropertySyntaxMultiplier::None => {
            if matches!(
                component.name,
                PropertySyntaxComponentName::Type(PropertySyntaxType::TransformList)
            ) {
                let mut count = 0;
                for value in values {
                    if !matches_value(&component.name, &value) {
                        return false;
                    }
                    count += 1;
                }
                return count > 0;
            }
            let Some(value) = values.next() else {
                return false;
            };
            values.next().is_none() && matches_value(&component.name, &value)
        }
        PropertySyntaxMultiplier::SpaceSeparated => {
            let mut count = 0;
            for value in values {
                if !matches_value(&component.name, &value) {
                    return false;
                }
                count += 1;
            }
            count > 0
        }
        PropertySyntaxMultiplier::CommaSeparated => {
            let mut expect_value = true;
            let mut count = 0;
            for value in values {
                if expect_value {
                    if !matches_value(&component.name, &value) {
                        return false;
                    }
                    count += 1;
                } else {
                    let AnyCssGenericComponentValue::CssGenericDelimiter(delimiter) = value else {
                        return false;
                    };
                    if !delimiter
                        .value()
                        .is_ok_and(|token| token.text_trimmed() == ",")
                    {
                        return false;
                    }
                }
                expect_value = !expect_value;
            }
            count > 0 && !expect_value
        }
    }
}

fn matches_value(
    component: &PropertySyntaxComponentName,
    value: &AnyCssGenericComponentValue,
) -> bool {
    let Some(value) = value.as_any_css_value() else {
        return false;
    };

    match component {
        PropertySyntaxComponentName::Type(syntax_type) => matches_type(*syntax_type, value),
        PropertySyntaxComponentName::CustomIdentifier(identifier) => identifier_text(value)
            .is_some_and(|value| decode_css_identifier(value.text()) == identifier.as_ref()),
    }
}

fn matches_type(syntax_type: PropertySyntaxType, value: &AnyCssValue) -> bool {
    if matches_function(value, MATH_FUNCTIONS) {
        return matches!(
            syntax_type,
            PropertySyntaxType::Angle
                | PropertySyntaxType::Integer
                | PropertySyntaxType::Length
                | PropertySyntaxType::LengthPercentage
                | PropertySyntaxType::Number
                | PropertySyntaxType::Percentage
                | PropertySyntaxType::Resolution
                | PropertySyntaxType::Time
        );
    }

    match syntax_type {
        PropertySyntaxType::Angle => matches_dimension(value, ANGLE_UNITS),
        PropertySyntaxType::Color => {
            matches!(value, AnyCssValue::CssColor(_))
                || identifier_text(value).is_some_and(|identifier| {
                    let identifier = decode_css_identifier(identifier.text());
                    COLOR_KEYWORDS
                        .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                        .is_ok()
                })
                || matches_function(value, COLOR_FUNCTIONS)
        }
        PropertySyntaxType::CustomIdent => identifier_text(value).is_some_and(|identifier| {
            let identifier = decode_css_identifier(identifier.text());
            !identifier.eq_ignore_ascii_case("default")
                && BASIC_KEYWORDS
                    .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                    .is_err()
        }),
        PropertySyntaxType::Image => matches_url(value) || matches_function(value, IMAGE_FUNCTIONS),
        PropertySyntaxType::Integer => value.as_css_number().is_some_and(|number| {
            number
                .value_token()
                .is_ok_and(|token| is_integer(token.text_trimmed()))
        }),
        PropertySyntaxType::Length => matches_length(value),
        PropertySyntaxType::LengthPercentage => matches_length(value) || matches_percentage(value),
        PropertySyntaxType::Number => matches!(value, AnyCssValue::CssNumber(_)),
        PropertySyntaxType::Percentage => matches_percentage(value),
        PropertySyntaxType::Resolution => matches_resolution(value),
        PropertySyntaxType::String => matches!(value, AnyCssValue::CssString(_)),
        PropertySyntaxType::Time => matches_dimension(value, TIME_UNITS),
        PropertySyntaxType::TransformFunction => matches_function(value, TRANSFORM_FUNCTIONS),
        PropertySyntaxType::TransformList => matches_function(value, TRANSFORM_FUNCTIONS),
        PropertySyntaxType::Url => matches_url(value),
    }
}

fn is_integer(value: &str) -> bool {
    let value = value
        .strip_prefix('+')
        .or_else(|| value.strip_prefix('-'))
        .filter(|value| !value.is_empty())
        .unwrap_or(value);
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn matches_resolution(value: &AnyCssValue) -> bool {
    let AnyCssValue::AnyCssDimension(AnyCssDimension::CssRegularDimension(dimension)) = value
    else {
        return false;
    };
    matches_dimension(value, RESOLUTION_UNITS)
        && dimension.value_token().is_ok_and(|token| {
            token
                .text_trimmed()
                .parse::<f64>()
                .is_ok_and(|value| value > 0.0)
        })
}

fn function_name_is_in(function: &biome_css_syntax::CssFunction, names: &[&str]) -> bool {
    function
        .name()
        .ok()
        .and_then(|name| name.as_css_identifier().cloned())
        .and_then(|identifier| identifier.value_token().ok())
        .is_some_and(|token| {
            let name = decode_css_identifier(token.text_trimmed());
            names
                .binary_search(&name.to_ascii_lowercase_cow().as_ref())
                .is_ok()
        })
}

fn identifier_text(value: &AnyCssValue) -> Option<TokenText> {
    match value {
        AnyCssValue::CssIdentifier(identifier) => {
            Some(identifier.value_token().ok()?.token_text_trimmed())
        }
        AnyCssValue::CssCustomIdentifier(identifier) => {
            Some(identifier.value_token().ok()?.token_text_trimmed())
        }
        AnyCssValue::AnyCssDashedIdentifier(identifier) => Some(
            identifier
                .as_css_dashed_identifier()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        _ => None,
    }
}

fn declaration_name_text(name: &AnyCssDeclarationName) -> Option<TokenText> {
    match name {
        AnyCssDeclarationName::CssIdentifier(identifier) => {
            Some(identifier.value_token().ok()?.token_text_trimmed())
        }
        AnyCssDeclarationName::AnyCssDashedIdentifier(identifier) => Some(
            identifier
                .as_css_dashed_identifier()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        _ => None,
    }
}

fn matches_dimension(value: &AnyCssValue, units: &[&str]) -> bool {
    let AnyCssValue::AnyCssDimension(AnyCssDimension::CssRegularDimension(dimension)) = value
    else {
        return false;
    };
    dimension.unit_token().is_ok_and(|token| {
        let unit = decode_css_identifier(token.text_trimmed());
        units
            .binary_search(&unit.to_ascii_lowercase_cow().as_ref())
            .is_ok()
    })
}

fn matches_length(value: &AnyCssValue) -> bool {
    matches_dimension(value, LENGTH_UNITS)
        || value.as_css_number().is_some_and(|number| {
            number.value_token().is_ok_and(|token| {
                token
                    .text_trimmed()
                    .parse::<f64>()
                    .is_ok_and(|value| value == 0.0)
            })
        })
}

fn matches_percentage(value: &AnyCssValue) -> bool {
    matches!(
        value,
        AnyCssValue::AnyCssDimension(AnyCssDimension::CssPercentage(_))
    )
}

fn matches_function(value: &AnyCssValue, names: &[&str]) -> bool {
    let AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(function)) = value else {
        return false;
    };
    function_name_is_in(function, names)
}

fn matches_url(value: &AnyCssValue) -> bool {
    matches!(
        value,
        AnyCssValue::AnyCssFunction(AnyCssFunction::CssUrlFunction(_))
    ) || matches_function(value, &["src"])
}
