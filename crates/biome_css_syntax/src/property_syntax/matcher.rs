use super::{
    PropertySyntax, PropertySyntaxComponent, PropertySyntaxComponentName, PropertySyntaxMultiplier,
    PropertySyntaxType,
};
use crate::{
    AnyCssFunction, AnyCssGenericComponentValue, AnyCssValue, CssGenericComponentValueList,
    decode_css_identifier,
    keywords::{
        ANGLE_MATH_FUNCTIONS, ANGLE_UNITS, BASIC_KEYWORDS, COLOR_FUNCTIONS, COLOR_KEYWORDS,
        IMAGE_FUNCTIONS, INTEGER_MATH_FUNCTIONS, LENGTH_ENVIRONMENT_VARIABLES,
        NUMBER_ENVIRONMENT_VARIABLES, NUMBER_MATH_FUNCTIONS, TIME_UNITS, TRANSFORM_FUNCTIONS,
        TYPED_MATH_FUNCTIONS,
    },
};
use biome_rowan::{AstNode, AstNodeList};
use biome_string_case::StrLikeExtension;

const ABSOLUTE_LENGTH_UNITS: &[&str] = &["cm", "in", "mm", "pc", "pt", "px", "q"];

impl PropertySyntax {
    /// Returns whether `values` have a shape accepted by this registered-property syntax.
    ///
    /// Supported functions are classified by their names. Their arguments and computational
    /// independence remain subject to browser validation.
    pub fn matches_value(&self, values: &CssGenericComponentValueList) -> bool {
        let Self::Components(components) = self else {
            return true;
        };

        // A CSS-wide keyword matches any registered syntax only when it is the entire value.
        if values.len() == 1
            && values
                .first()
                .and_then(|value| value.as_any_css_value()?.identifier_text())
                .is_some_and(|identifier| {
                    let identifier = decode_css_identifier(identifier.text());
                    BASIC_KEYWORDS
                        .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                        .is_ok()
                })
        {
            return true;
        }

        components
            .iter()
            .any(|component| matches_component(component, values))
    }

    /// Returns whether `values` match this syntax and can be computed without external context.
    pub fn matches_initial_value(&self, values: &CssGenericComponentValueList) -> bool {
        self.matches_value(values) && is_computationally_independent(values)
    }
}

/// Returns whether an initial value can be computed without external style or environment data.
///
/// The check traverses nested values and rejects `attr()`, `env()`, `if()`, and `var()`,
/// context-dependent keywords such as `currentColor` and CSS-wide keywords, and relative length
/// units. Absolute length units and values whose computed form depends only on their own tokens are
/// accepted.
fn is_computationally_independent(values: &CssGenericComponentValueList) -> bool {
    values
        .syntax()
        .descendants()
        .filter_map(AnyCssValue::cast)
        .all(|value| {
            if matches!(
                value,
                AnyCssValue::AnyCssFunction(
                    AnyCssFunction::CssAttrFunction(_) | AnyCssFunction::CssIfFunction(_)
                )
            ) || value.matches_function(&["env", "var"])
            {
                return false;
            }
            if value.identifier_text().is_some_and(|identifier| {
                let identifier = decode_css_identifier(identifier.text());
                identifier.eq_ignore_ascii_case("currentcolor")
                    || BASIC_KEYWORDS
                        .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                        .is_ok()
            }) {
                return false;
            }
            !value.as_css_regular_dimension().is_some_and(|dimension| {
                dimension.matches_unit(crate::keywords::LENGTH_UNITS)
                    && !dimension.matches_unit(ABSOLUTE_LENGTH_UNITS)
            })
        })
}

/// Returns whether the complete value list satisfies a syntax component and its multiplier.
///
/// An unmultiplied component accepts exactly one value, except `<transform-list>`, which accepts
/// one or more transform functions. The `+` multiplier accepts one or more space-separated values,
/// while `#` accepts one or more comma-separated values.
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
                // Exhausting the loop validates every function; the count also rejects an empty
                // transform list.
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
            // Exhausting the loop validates every value; the count enforces the multiplier's
            // one-or-more requirement.
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
            // The count rejects an empty list, while `expect_value` rejects a trailing comma.
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
        PropertySyntaxComponentName::CustomIdentifier(identifier) => value
            .identifier_text()
            .is_some_and(|value| decode_css_identifier(value.text()) == identifier.as_ref()),
    }
}

fn matches_type(syntax_type: PropertySyntaxType, value: &AnyCssValue) -> bool {
    if let Some(matches) = matches_environment_function(syntax_type, value) {
        return matches;
    }
    if value.matches_function(TYPED_MATH_FUNCTIONS) {
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
    if value.matches_function(ANGLE_MATH_FUNCTIONS) {
        return syntax_type == PropertySyntaxType::Angle;
    }
    if value.matches_function(INTEGER_MATH_FUNCTIONS) {
        return matches!(
            syntax_type,
            PropertySyntaxType::Integer | PropertySyntaxType::Number
        );
    }
    if value.matches_function(NUMBER_MATH_FUNCTIONS) {
        return syntax_type == PropertySyntaxType::Number;
    }

    match syntax_type {
        PropertySyntaxType::Angle => value
            .as_css_regular_dimension()
            .is_some_and(|dimension| dimension.matches_unit(ANGLE_UNITS)),
        PropertySyntaxType::Color => {
            matches!(value, AnyCssValue::CssColor(_))
                || value.identifier_text().is_some_and(|identifier| {
                    let identifier = decode_css_identifier(identifier.text());
                    COLOR_KEYWORDS
                        .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                        .is_ok()
                })
                || value.matches_function(COLOR_FUNCTIONS)
        }
        PropertySyntaxType::CustomIdent => value.identifier_text().is_some_and(|identifier| {
            let identifier = decode_css_identifier(identifier.text());
            !identifier.eq_ignore_ascii_case("default")
                && BASIC_KEYWORDS
                    .binary_search(&identifier.to_ascii_lowercase_cow().as_ref())
                    .is_err()
        }),
        PropertySyntaxType::Image => value.matches_url() || value.matches_function(IMAGE_FUNCTIONS),
        PropertySyntaxType::Integer => value
            .as_css_number()
            .is_some_and(|number| number.is_integer()),
        PropertySyntaxType::Length => value.matches_length(),
        PropertySyntaxType::LengthPercentage => {
            value.matches_length() || value.matches_percentage()
        }
        PropertySyntaxType::Number => matches!(value, AnyCssValue::CssNumber(_)),
        PropertySyntaxType::Percentage => value.matches_percentage(),
        PropertySyntaxType::Resolution => value.matches_resolution(),
        PropertySyntaxType::String => matches!(value, AnyCssValue::CssString(_)),
        PropertySyntaxType::Time => value
            .as_css_regular_dimension()
            .is_some_and(|dimension| dimension.matches_unit(TIME_UNITS)),
        PropertySyntaxType::TransformFunction => value.matches_function(TRANSFORM_FUNCTIONS),
        PropertySyntaxType::TransformList => value.matches_function(TRANSFORM_FUNCTIONS),
        PropertySyntaxType::Url => value.matches_url(),
    }
}

fn matches_environment_function(
    syntax_type: PropertySyntaxType,
    value: &AnyCssValue,
) -> Option<bool> {
    if !value.matches_function(&["env"]) {
        return None;
    }
    let AnyCssValue::AnyCssFunction(AnyCssFunction::CssFunction(function)) = value else {
        return None;
    };

    let name_token = function.items().syntax().first_token()?;
    if name_token
        .next_token()
        .is_some_and(|token| !matches!(token.text_trimmed(), "," | ")"))
    {
        return Some(true);
    }

    let name = decode_css_identifier(name_token.text_trimmed());
    let name = name.to_ascii_lowercase_cow();
    if LENGTH_ENVIRONMENT_VARIABLES
        .binary_search(&name.as_ref())
        .is_ok()
    {
        return Some(matches!(
            syntax_type,
            PropertySyntaxType::Length | PropertySyntaxType::LengthPercentage
        ));
    }
    if NUMBER_ENVIRONMENT_VARIABLES
        .binary_search(&name.as_ref())
        .is_ok()
    {
        return Some(syntax_type == PropertySyntaxType::Number);
    }

    Some(true)
}
