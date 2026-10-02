//! Shared AST utilities for DOM lint rules.

use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsCallExpression, JsClassExpression, JsFunctionExpression, JsLanguage, JsObjectExpression,
    JsStaticMemberExpression, JsTemplateExpression, static_value::StaticValue,
};
use biome_rowan::{AstNode, SyntaxKindSet};

const DEFINITELY_NOT_DOM_NODE_KINDS: SyntaxKindSet<JsLanguage> = AnyJsLiteralExpression::KIND_SET
    .union(JsArrayExpression::KIND_SET)
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET);

/// Legacy DOM query methods.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LegacyDomQueryMethod {
    /// `getElementById()`.
    ElementById,
    /// `getElementsByClassName()`.
    ElementsByClassName,
    /// `getElementsByTagName()`.
    ElementsByTagName,
    /// `getElementsByName()`.
    ElementsByName,
}

impl LegacyDomQueryMethod {
    /// Maps the legacy DOM query method name to the corresponding enum variant.
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "getElementById" => Self::ElementById,
            "getElementsByClassName" => Self::ElementsByClassName,
            "getElementsByTagName" => Self::ElementsByTagName,
            "getElementsByName" => Self::ElementsByName,
            _ => return None,
        })
    }

    /// Returns the preferred `querySelector*` replacement for this legacy method.
    pub(crate) fn preferred_name(self) -> &'static str {
        match self {
            Self::ElementById => "querySelector",
            Self::ElementsByClassName | Self::ElementsByTagName | Self::ElementsByName => {
                "querySelectorAll"
            }
        }
    }
}

/// A DOM lookup candidate with a static member and one non-spread argument.
/// Receiver types and argument values are not validated.
pub(crate) struct DomQueryCall {
    pub(crate) member: JsStaticMemberExpression,
    pub(crate) argument: AnyJsExpression,
}

/// Matches `getElementById`, `getElementsByClassName`, `getElementsByTagName`,
/// or `getElementsByName` calls with one non-spread argument.
/// Computed access and optional chains are excluded.
pub(crate) fn legacy_dom_query_call(
    call: &JsCallExpression,
) -> Option<(LegacyDomQueryMethod, DomQueryCall)> {
    if call.is_optional_chain() {
        return None;
    }
    let query = dom_query_call(call)?;
    if query.member.is_optional_chain() {
        return None;
    }
    let name = query
        .member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?;
    let method = LegacyDomQueryMethod::from_name(name.text_trimmed())?;
    Some((method, query))
}

/// Matches `.querySelector()` calls with one non-spread argument.
/// Computed access, optional calls, and optional member access are excluded.
/// Callers must check for optional chains in the receiver and surrounding syntax.
pub(crate) fn query_selector_call(call: &JsCallExpression) -> Option<DomQueryCall> {
    let query = dom_query_call(call)?;
    let name = query
        .member
        .member()
        .ok()?
        .as_js_name()?
        .value_token()
        .ok()?;
    (name.text_trimmed() == "querySelector").then_some(query)
}

fn dom_query_call(call: &JsCallExpression) -> Option<DomQueryCall> {
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?.clone();
    if member.is_optional() {
        return None;
    }
    Some(DomQueryCall {
        member,
        argument: first_and_only_argument(call)?,
    })
}

/// Returns the only argument passed to `call`.
///
/// Useful for selecting DOM lookup calls with a single ID, class name, tag name,
/// name, or CSS selector argument before rewriting or combining queries.
///
/// Returns `None` if the call does not have exactly one non-spread argument or
/// its arguments cannot be read.
fn first_and_only_argument(call: &JsCallExpression) -> Option<AnyJsExpression> {
    let mut args = call.arguments().ok()?.args().into_iter();
    let argument = args.next()?.ok()?.as_any_js_expression()?.clone();

    if args.next().is_none() {
        Some(argument)
    } else {
        None
    }
}

/// Returns `true` for expressions excluded from DOM receiver checks to avoid
/// obvious false positives:
///
/// - literals like `"text"`, `null`, and `1`
/// - array and object literals like `[]` and `{}`
/// - function and class expressions
/// - template literals like `` `text` ``
/// - the identifier `undefined`
///
/// Everything else is treated as potentially DOM-like, including identifiers,
/// member expressions, and other unknown expressions.
pub(crate) fn is_definitely_not_dom_node(expr: &AnyJsExpression) -> bool {
    let expr = expr.clone().omit_parentheses();

    DEFINITELY_NOT_DOM_NODE_KINDS.matches(expr.syntax().kind())
        || expr
            .as_static_value()
            .is_some_and(|value| matches!(value, StaticValue::Undefined(_)))
}

#[cfg(test)]
mod tests {
    use super::{
        LegacyDomQueryMethod, is_definitely_not_dom_node, legacy_dom_query_call,
        query_selector_call,
    };
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::{AnyJsExpression, JsExpressionStatement};
    use biome_languages::JsFileSource;
    use biome_rowan::AstNode;

    fn expression(source: &str) -> AnyJsExpression {
        let parsed = parse(
            &format!("({source})"),
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        assert!(!parsed.has_errors(), "{source}");
        parsed
            .syntax()
            .descendants()
            .find_map(JsExpressionStatement::cast)
            .expect("expression statement")
            .expression()
            .expect("expression")
            .omit_parentheses()
    }

    #[test]
    fn excludes_non_dom_receivers() {
        for source in [
            "null",
            "true",
            "false",
            "0",
            "1n",
            "'text'",
            "/regex/",
            "[]",
            "({})",
            "(() => {})",
            "(function () {})",
            "(class {})",
            "`text`",
            "`${value}`",
            "tag`text`",
            "undefined",
            "(((null)))",
        ] {
            assert!(is_definitely_not_dom_node(&expression(source)), "{source}");
        }
    }

    #[test]
    fn allows_potential_dom_receivers() {
        for source in [
            "document",
            "element",
            "window.document",
            "getElement()",
            "new Node()",
            "this",
            "(element)",
            "condition ? first : second",
            "first || second",
        ] {
            assert!(!is_definitely_not_dom_node(&expression(source)), "{source}");
        }
    }

    #[test]
    fn matches_dom_query_methods() {
        for (name, method) in [
            ("getElementById", Some(LegacyDomQueryMethod::ElementById)),
            (
                "getElementsByClassName",
                Some(LegacyDomQueryMethod::ElementsByClassName),
            ),
            (
                "getElementsByTagName",
                Some(LegacyDomQueryMethod::ElementsByTagName),
            ),
            (
                "getElementsByName",
                Some(LegacyDomQueryMethod::ElementsByName),
            ),
            ("querySelector", None),
        ] {
            for (arguments, expected) in [
                ("()", None),
                ("('a')", Some("'a'")),
                ("(('a'))", Some("('a')")),
                ("('a',)", Some("'a'")),
                ("(selector)", Some("selector")),
                ("('a', 'b')", None),
                ("(...selectors)", None),
                ("('a', ...selectors)", None),
            ] {
                for callee in [format!("node.{name}"), format!("(node.{name})")] {
                    let source = format!("{callee}{arguments}");
                    let expression = expression(&source);
                    let call = expression.as_js_call_expression().expect("call expression");
                    let legacy = legacy_dom_query_call(call);
                    let selector = query_selector_call(call);
                    let query = if let Some(method) = method {
                        assert!(selector.is_none(), "{source}");
                        assert_eq!(
                            legacy.as_ref().map(|(kind, _)| *kind),
                            expected.map(|_| method),
                            "{source}"
                        );
                        legacy.map(|(_, query)| query)
                    } else {
                        assert!(legacy.is_none(), "{source}");
                        selector
                    };
                    let argument =
                        query.map(|query| query.argument.syntax().text_trimmed().to_string());
                    assert_eq!(argument.as_deref(), expected, "{source}");
                }
            }
        }
    }

    #[test]
    fn rejects_unsupported_dom_query_calls() {
        for name in [
            "getElementById",
            "getElementsByClassName",
            "getElementsByTagName",
            "getElementsByName",
            "querySelector",
        ] {
            for source in [
                format!("{name}('a')"),
                format!("node['{name}']('a')"),
                format!("node?.{name}('a')"),
                format!("node.{name}?.('a')"),
                "node.querySelectorAll('a')".to_string(),
                "node.unrelated('a')".to_string(),
            ] {
                let expression = expression(&source);
                let call = expression.as_js_call_expression().expect("call expression");
                assert!(legacy_dom_query_call(call).is_none(), "{source}");
                assert!(query_selector_call(call).is_none(), "{source}");
            }
        }
    }

    #[test]
    fn preserves_receiver_optional_chain_checks() {
        for source in [
            "node?.child.getElementById('a')",
            "(node?.child.getElementById)('a')",
        ] {
            let expression = expression(source);
            let call = expression.as_js_call_expression().expect("call expression");
            assert!(legacy_dom_query_call(call).is_none(), "{source}");
        }
        let expression = expression("node?.child.querySelector('a')");
        let call = expression.as_js_call_expression().expect("call expression");
        assert!(query_selector_call(call).is_some());
    }
}
