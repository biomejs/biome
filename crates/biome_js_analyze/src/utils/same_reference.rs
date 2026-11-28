use biome_js_syntax::{
    AnyJsAssignment, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsName,
    AnyJsTemplateElement, JsNumberLiteralExpression,
    numbers::{canonicalize_js_bigint_literal, parse_js_number_with_single_rounding},
    unescape_js_string,
};
use biome_rowan::{AstNode, Text, TokenText};

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
                if !push_same_key(member_key(&left)?, member_key(&right)?, &mut expressions) {
                    return Some(false);
                }
                expressions.push((left.object().ok()?, right.object().ok()?));
            }
        }
    }

    Some(true)
}

/// Returns whether assigning to `target` writes the variable or property that `reference`
/// reads, so `a.b = 1` matches the reference `a["b"]`.
///
/// `reference` is compared as in [`is_same_reference`]. Parentheses and TypeScript syntax
/// around `target`, such as `(a as T)` or `a!`, are ignored. A reference that uses optional
/// chaining (`?.`) at its last step never matches, since an assignment target can't use it.
///
/// Returns `None` if the syntax tree is incomplete.
pub(crate) fn is_same_assignment_target(
    target: AnyJsAssignment,
    reference: AnyJsExpression,
) -> Option<bool> {
    let mut target = target;
    let target = loop {
        target = match target {
            AnyJsAssignment::JsParenthesizedAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsNonNullAssertionAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsAsAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsSatisfiesAssignment(target) => target.assignment().ok()?,
            AnyJsAssignment::TsTypeAssertionAssignment(target) => target.assignment().ok()?,
            target => break target,
        };
    };
    let reference = reference.omit_type_wrappers()?;

    let (target_object, target_key) = match target {
        AnyJsAssignment::JsIdentifierAssignment(target) => {
            let Some(reference) = reference.as_js_identifier_expression() else {
                return Some(false);
            };
            return Some(
                target.name_token().ok()?.text_trimmed()
                    == reference.name().ok()?.value_token().ok()?.text_trimmed(),
            );
        }
        AnyJsAssignment::JsStaticMemberAssignment(target) => {
            (target.object().ok()?, name_key(target.member().ok()?)?)
        }
        AnyJsAssignment::JsComputedMemberAssignment(target) => {
            (target.object().ok()?, computed_key(target.member().ok()?)?)
        }
        _ => return None,
    };

    let Some(reference) = AnyJsMemberExpression::cast(reference.into_syntax()) else {
        return Some(false);
    };
    if is_optional_member(&reference) {
        return Some(false);
    }
    let mut expressions = Vec::new();
    if !push_same_key(target_key, member_key(&reference)?, &mut expressions) {
        return Some(false);
    }
    expressions.push((target_object, reference.object().ok()?));
    for (left, right) in expressions {
        if !is_same_reference(left, right)? {
            return Some(false);
        }
    }
    Some(true)
}

/// Returns `false` if `left` and `right` are different properties. Otherwise returns `true`,
/// and if both are computed, pushes them to `expressions` to be compared next.
fn push_same_key(
    left: MemberKey,
    right: MemberKey,
    expressions: &mut Vec<(AnyJsExpression, AnyJsExpression)>,
) -> bool {
    match (left, right) {
        (MemberKey::Static(left), MemberKey::Static(right))
        | (MemberKey::Private(left), MemberKey::Private(right)) => left == right,
        (MemberKey::Computed(left), MemberKey::Computed(right)) => {
            expressions.push((left, right));
            true
        }
        _ => false,
    }
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
    match member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => name_key(member.member().ok()?),
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            computed_key(member.member().ok()?)
        }
    }
}

/// Returns the key of the property `name` in `a.name`.
///
/// Returns `None` if the syntax tree is incomplete.
fn name_key(name: AnyJsName) -> Option<MemberKey> {
    Some(match name {
        AnyJsName::JsName(name) => {
            MemberKey::Static(name.value_token().ok()?.token_text_trimmed().into())
        }
        AnyJsName::JsPrivateName(name) => {
            MemberKey::Private(name.value_token().ok()?.token_text_trimmed().into())
        }
        AnyJsName::JsMetavariable(_) => return None,
    })
}

/// Returns the key of the property `property` in `a[property]`.
///
/// Returns `None` if the syntax tree is incomplete.
fn computed_key(property: AnyJsExpression) -> Option<MemberKey> {
    let property = property.omit_type_wrappers()?;
    Some(match static_property_name(&property) {
        Some(name) => MemberKey::Static(name),
        None => MemberKey::Computed(property),
    })
}

/// Returns the property name that `expression` produces when used as a computed member,
/// or `None` if it can't be determined reliably without running the code.
fn static_property_name(expression: &AnyJsExpression) -> Option<Text> {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
        ) => string_value(literal.inner_string_text().ok()?),
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
                    string_value(chunk.template_chunk_token().ok()?.token_text_trimmed())
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
            match (string_value(left.clone()), string_value(right.clone())) {
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

/// Returns the value of the string whose source text, without quotes, is `text`.
///
/// Returns `None` if `text` contains an escape sequence that [`unescape_js_string`] can't
/// decode correctly: a legacy octal escape such as `\1` or `\01`, or an escape for half of a
/// surrogate pair such as `\uD83D`.
pub(crate) fn string_value(text: TokenText) -> Option<Text> {
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            continue;
        }
        match chars.next() {
            Some('1'..='7') => return None,
            Some('0') if chars.peek().is_some_and(char::is_ascii_digit) => return None,
            Some('u') => {
                let code_point = if chars.next_if_eq(&'{').is_some() {
                    let digits: String = chars.by_ref().take_while(|c| *c != '}').collect();
                    u32::from_str_radix(&digits, 16).ok()
                } else {
                    let digits: String = chars.by_ref().take(4).collect();
                    u32::from_str_radix(&digits, 16).ok()
                };
                if code_point.is_none_or(|code_point| (0xD800..=0xDFFF).contains(&code_point)) {
                    return None;
                }
            }
            _ => {}
        }
    }
    Some(unescape_js_string(text))
}
