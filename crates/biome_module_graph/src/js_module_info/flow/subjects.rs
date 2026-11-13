//! Maps the expressions that read or test a value to the subjects of an
//! execution root.
//!
//! Reads and runtime tests go through [`subject_of`], so a test and a read of
//! the same subject get the same id even when their syntax differs, as in
//! `(o)?.v` and `o.v`.

use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, AnyJsName, unescape_js_identifier};
use biome_rowan::Text;
use std::borrow::Cow;

/// Returns the id of the subject that `expression` reads, adding the subject
/// and every prefix of its path to `subjects` on first use.
///
/// A subject is an identifier that refers to a declaration, or a static member
/// access with a named property whose object is a subject, such as `o.v` or
/// `a.b.c`. Parentheses around the expression and its objects are skipped, and
/// an optional access reads the same subject as a plain one, as in TypeScript:
/// `(o)?.v` is `o.v`.
///
/// Returns `None` for any other expression, including `this.v`, `o["v"]`,
/// `o!.v`, `f().v`, a private name such as `o.#v`, and a global, and when the
/// table has used every [`FlowSubjectId`].
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only identifiers and static member accesses can be subjects."
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
            subjects(
                "this.v; o['v']; o[0]; o!.v; (o as any).v; f().v; o.f().v; missing.v; o[k].v;"
            ),
            vec![None; 9]
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
