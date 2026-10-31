use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsName,
    AnyJsTemplateElement, JsNumberLiteralExpression,
    numbers::{canonicalize_js_bigint_literal, parse_js_number_with_single_rounding},
    try_unescape_js_string,
};
use biome_rowan::{AstNode, Text};

/// Returns whether `left` and `right` read the same value: the same variable, property,
/// `this`, `super`, or literal.
///
/// Parentheses and the TypeScript syntax removed by [`AnyJsExpression::omit_type_wrappers`]
/// are ignored, so `f<T>` matches `f`. Properties are compared by name,
/// so `a.b`, `a["b"]`, and ``a[`b`]`` match, and so do `a[0]` and `a["0"]`. Optional
/// chaining (`?.`) must appear at the same places on both sides.
///
/// Calls never match, since each call can return a different value. Regular expression
/// literals never match, since each one creates a new object.
///
/// Returns `None` if the syntax tree is incomplete.
pub(crate) fn is_same_reference(left: AnyJsExpression, right: AnyJsExpression) -> Option<bool> {
    let mut expressions = vec![(left, right)];

    while let Some((left, right)) = expressions.pop() {
        let left = left.omit_type_wrappers()?;
        let right = right.omit_type_wrappers()?;

        match (&left, &right) {
            (
                AnyJsExpression::JsIdentifierExpression(left),
                AnyJsExpression::JsIdentifierExpression(right),
            ) => {
                if left.name().ok()?.value_token().ok()?.text_trimmed()
                    != right.name().ok()?.value_token().ok()?.text_trimmed()
                {
                    return Some(false);
                }
            }
            (AnyJsExpression::JsThisExpression(_), AnyJsExpression::JsThisExpression(_))
            | (AnyJsExpression::JsSuperExpression(_), AnyJsExpression::JsSuperExpression(_)) => {}
            (
                AnyJsExpression::AnyJsLiteralExpression(left),
                AnyJsExpression::AnyJsLiteralExpression(right),
            ) => {
                if !is_same_literal(left, right)? {
                    return Some(false);
                }
            }
            _ => {
                let (Some(left), Some(right)) = (
                    AnyJsMemberExpression::cast(left.into_syntax()),
                    AnyJsMemberExpression::cast(right.into_syntax()),
                ) else {
                    return Some(false);
                };
                if is_optional_member(&left) != is_optional_member(&right) {
                    return Some(false);
                }
                match (member_key(&left)?, member_key(&right)?) {
                    (MemberKey::Static(left_key), MemberKey::Static(right_key))
                    | (MemberKey::Private(left_key), MemberKey::Private(right_key)) => {
                        if left_key != right_key {
                            return Some(false);
                        }
                    }
                    (MemberKey::Computed(left_key), MemberKey::Computed(right_key)) => {
                        expressions.push((left_key, right_key));
                    }
                    _ => return Some(false),
                }
                expressions.push((left.object().ok()?, right.object().ok()?));
            }
        }
    }

    Some(true)
}

/// The property that a member expression reads.
enum MemberKey {
    /// A property whose name is known without running the code, such as `b` in `a.b`,
    /// `a["b"]`, and ``a[`b`]``.
    Static(Text),
    /// A private class member, such as `#b` in `a.#b`. It never matches a [`MemberKey::Static`]
    /// property, even `a["#b"]`.
    Private(Text),
    /// Any other computed property, such as `i` in `a[i]`.
    Computed(AnyJsExpression),
}

/// Returns `None` if the syntax tree is incomplete.
fn member_key(member: &AnyJsMemberExpression) -> Option<MemberKey> {
    Some(match member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => match member.member().ok()? {
            AnyJsName::JsName(name) => {
                MemberKey::Static(name.value_token().ok()?.token_text_trimmed().into())
            }
            AnyJsName::JsPrivateName(name) => {
                MemberKey::Private(name.value_token().ok()?.token_text_trimmed().into())
            }
            AnyJsName::JsMetavariable(_) => return None,
        },
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            let property = member.member().ok()?.omit_type_wrappers()?;
            match static_property_name(&property) {
                Some(name) => MemberKey::Static(name),
                None => MemberKey::Computed(property),
            }
        }
    })
}

/// Returns the property name that `expression` produces when used as a computed member,
/// or `None` if it can't be determined reliably without running the code.
fn static_property_name(expression: &AnyJsExpression) -> Option<Text> {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
        ) => try_unescape_js_string(literal.inner_string_text().ok()?),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
        ) => {
            let value = number_value(literal)?;
            // Rust prints these numbers the same way as JavaScript. JavaScript switches to
            // exponent notation outside this range, so `a[1e21]` reads the `"1e+21"` property.
            (value == 0.0 || (1e-6..1e21).contains(&value)).then(|| value.to_string().into())
        }
        AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
            let mut elements = template.elements().into_iter();
            match (elements.next(), elements.next()) {
                (None, _) => Some(Text::new_static("")),
                (Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)), None) => {
                    try_unescape_js_string(chunk.template_chunk_token().ok()?.token_text_trimmed())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn is_optional_member(member: &AnyJsMemberExpression) -> bool {
    match member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => member.is_optional(),
        AnyJsMemberExpression::JsComputedMemberExpression(member) => member.is_optional(),
    }
}

/// Returns `None` if the syntax tree is incomplete.
fn is_same_literal(left: &AnyJsLiteralExpression, right: &AnyJsLiteralExpression) -> Option<bool> {
    Some(match (left, right) {
        (
            AnyJsLiteralExpression::JsBigintLiteralExpression(left),
            AnyJsLiteralExpression::JsBigintLiteralExpression(right),
        ) => {
            canonicalize_js_bigint_literal(left.value_token().ok()?.text_trimmed())?
                == canonicalize_js_bigint_literal(right.value_token().ok()?.text_trimmed())?
        }
        (
            AnyJsLiteralExpression::JsBooleanLiteralExpression(left),
            AnyJsLiteralExpression::JsBooleanLiteralExpression(right),
        ) => left.value_token().ok()?.text_trimmed() == right.value_token().ok()?.text_trimmed(),
        (
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
            AnyJsLiteralExpression::JsNullLiteralExpression(_),
        ) => true,
        (
            AnyJsLiteralExpression::JsNumberLiteralExpression(left),
            AnyJsLiteralExpression::JsNumberLiteralExpression(right),
        ) => number_value(left)? == number_value(right)?,
        (
            AnyJsLiteralExpression::JsStringLiteralExpression(left),
            AnyJsLiteralExpression::JsStringLiteralExpression(right),
        ) => {
            let left = left.inner_string_text().ok()?;
            let right = right.inner_string_text().ok()?;
            match (
                try_unescape_js_string(left.clone()),
                try_unescape_js_string(right.clone()),
            ) {
                (Some(left), Some(right)) => left == right,
                // The same source text always has the same value.
                _ => left == right,
            }
        }
        _ => false,
    })
}

/// Returns `None` if the syntax tree is incomplete.
fn number_value(literal: &JsNumberLiteralExpression) -> Option<f64> {
    parse_js_number_with_single_rounding(literal.value_token().ok()?.text_trimmed())
}
