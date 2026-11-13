//! Maps the expressions that read or test a value to the subjects of an
//! execution root.
//!
//! Reads and runtime tests go through [`subject_of`], so a test and a read of
//! the same subject get the same id even when their syntax differs, as in
//! `(o)?.v`, `o["v"]`, and `o.v`.

use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsName, unescape_js_identifier};
use biome_rowan::Text;
use std::borrow::Cow;

/// The largest integer that a JavaScript number holds exactly, 2^53 - 1.
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

/// Returns the id of the subject that `expression` reads, adding the subject
/// and every prefix of its path to `subjects` on first use.
///
/// A subject is an identifier that refers to a declaration, or a member access
/// whose object is a subject: a static member access with a named property,
/// such as `o.v` or `a.b.c`, or an element access whose key [`literal_key`]
/// accepts, such as `o["v"]` or `t[0]`. Parentheses around the expression and
/// its objects are skipped, and an optional access reads the same subject as a
/// plain one, as in TypeScript: `(o)?.v` is `o.v`.
///
/// Returns `None` for any other expression, including `this.v`, `o[k]`,
/// `o!.v`, `f().v`, a private name such as `o.#v`, and a global, and when the
/// table has used every [`FlowSubjectId`].
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only identifiers and member accesses can be subjects."
)]
pub(super) fn subject_of(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    subjects: &mut FlowSubjectTable,
) -> Option<FlowSubjectId> {
    let mut keys = Vec::new();
    let mut expression = expression.clone();
    let reference = loop {
        match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                expression = parenthesized.expression().ok()?;
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                keys.push(property_key(&member.member().ok()?)?);
                expression = member.object().ok()?;
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                keys.push(literal_key(&member.member().ok()?)?);
                expression = member.object().ok()?;
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => break identifier.name().ok()?,
            _ => return None,
        }
    };
    let mut subject = subjects.reference(model, &reference)?;
    for key in keys.into_iter().rev() {
        subject = subjects.intern(FlowSubject::Member {
            object: subject,
            key,
        })?;
    }
    Some(subject)
}

/// Returns the key of the property that `name` names.
///
/// Returns `None` for a private name, and for a name whose escape is
/// malformed, because its decoded text is unknown.
pub(super) fn property_key(name: &AnyJsName) -> Option<PropertyKey> {
    let AnyJsName::JsName(name) = name else {
        return None;
    };
    let token = name.value_token().ok()?;
    let text = match unescape_js_identifier(token.text_trimmed()) {
        Cow::Borrowed(_) => Text::from(token.token_text_trimmed()),
        Cow::Owned(decoded) => Text::from(decoded),
    };
    (!text.contains('\\')).then_some(PropertyKey::Name(text))
}

/// Returns the key of the property that the element access key `key` names
/// when its spelling is the property's name: a string literal without escapes,
/// such as `"v"`, or a decimal integer literal from `0` to 2^53 - 1 without
/// leading zeros or separators, such as `0`.
///
/// JavaScript converts a number key to a string, and such an integer converts
/// to its own digits, so `t[0]` reads the same property as `t["0"]`, and
/// `o["v"]` the same as `o.v`.
///
/// Returns `None` for every other key. That includes a number literal that
/// converts to other text, such as `0x0`, `1e0`, `1.0`, `1_0`, and
/// `9007199254740993`, which converts to `"9007199254740992"`, and an
/// expression that is not a literal, such as `k`, `-0`, or a template literal.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only string and number literals can name a property by their spelling."
)]
pub(super) fn literal_key(key: &AnyJsExpression) -> Option<PropertyKey> {
    match key {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => {
            let text = string.inner_string_text().ok()?;
            (!text.contains('\\')).then(|| PropertyKey::Name(text.into()))
        }
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number),
        ) => {
            let token = number.value_token().ok()?;
            let text = token.text_trimmed();
            let is_decimal_integer = text.bytes().all(|byte| byte.is_ascii_digit())
                && (text == "0" || !text.starts_with('0'));
            (is_decimal_integer
                && text
                    .parse::<u64>()
                    .is_ok_and(|value| value <= MAX_SAFE_INTEGER))
            .then(|| PropertyKey::Name(Text::from(token.token_text_trimmed())))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsExpressionStatement;
    use biome_languages::JsFileSource;
    use biome_rowan::AstNode;

    /// Returns the subject of each expression statement in the body of
    /// `function f(o, a)`, written as a dotted path, or `None` for an
    /// expression that is not a subject.
    fn subjects(body: &str) -> Vec<Option<String>> {
        let source = format!("function f(o, a) {{ {body} }}");
        let parsed = parse(&source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let mut table = FlowSubjectTable::default();
        let ids: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
            .map(|statement| subject_of(&statement.expression().unwrap(), &model, &mut table))
            .collect();
        let names: Vec<String> = table
            .as_slice()
            .iter()
            .map(|subject| match subject {
                FlowSubject::Binding(range) => source[*range].to_owned(),
                FlowSubject::Member { .. } => String::new(),
            })
            .collect();
        ids.into_iter()
            .map(|id| Some(path(table.as_slice(), &names, id?)))
            .collect()
    }

    fn path(subjects: &[FlowSubject], names: &[String], id: FlowSubjectId) -> String {
        match &subjects[id as usize] {
            FlowSubject::Binding(_) => names[id as usize].clone(),
            FlowSubject::Member {
                object,
                key: PropertyKey::Name(name),
            } => format!("{}.{name}", path(subjects, names, *object)),
        }
    }

    #[test]
    fn variables_and_named_properties_of_subjects_are_subjects() {
        assert_eq!(
            subjects("o; o.v; a.b.c; o?.v; (o).v; ((o.v)); o.\\u0076; a?.b?.c;"),
            [
                Some("o".to_owned()),
                Some("o.v".to_owned()),
                Some("a.b.c".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("a.b.c".to_owned()),
            ]
        );
    }

    #[test]
    fn other_expressions_are_not_subjects() {
        assert_eq!(
            subjects("this.v; o[k]; o!.v; (o as any).v; f().v; o.f().v; missing.v; o[k].v;"),
            vec![None; 8]
        );
    }

    #[test]
    fn literal_keys_name_the_property_they_spell() {
        assert_eq!(
            subjects(
                r#"o["v"]; o['v']; o?.["v"]; (o)["v"].w; o.v["w"]; o[0]; o["0"]; o["01"]; o[""]; o[9007199254740991];"#
            ),
            [
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v".to_owned()),
                Some("o.v.w".to_owned()),
                Some("o.v.w".to_owned()),
                Some("o.0".to_owned()),
                Some("o.0".to_owned()),
                Some("o.01".to_owned()),
                Some("o.".to_owned()),
                Some("o.9007199254740991".to_owned()),
            ]
        );
    }

    #[test]
    fn keys_spelled_differently_from_their_property_are_not_subjects() {
        assert_eq!(
            subjects(
                r#"o[-0]; o[0x0]; o[0b0]; o[0o0]; o[1e0]; o[1.0]; o[.5]; o[1_0]; o[1n]; o[9007199254740992]; o[12345678901234567890]; o["\u0076"]; o['\x76']; o[`v`]; o[("v")]; o[true]; o[null];"#
            ),
            vec![None; 17]
        );
    }

    #[test]
    fn a_path_and_its_prefixes_share_ids() {
        let source = "function f(o) { o.v.w; o.v; o; }";
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let mut table = FlowSubjectTable::default();
        let ids: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(JsExpressionStatement::cast)
            .map(|statement| subject_of(&statement.expression().unwrap(), &model, &mut table))
            .collect();
        // `o.v.w` adds `o` and `o.v` first, so later reads find them.
        assert_eq!(ids, [Some(2), Some(1), Some(0)]);
        assert_eq!(table.as_slice().len(), 3);
    }
}
