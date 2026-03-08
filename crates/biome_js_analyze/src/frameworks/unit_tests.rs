use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsName, AnyJsStatement, JsCallExpression,
};

/// The four lifecycle hooks recognised by Jest/Vitest/similar frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum LifecycleHook {
    BeforeEach,
    BeforeAll,
    AfterEach,
    AfterAll,
}

impl LifecycleHook {
    /// Returns the canonical source-level name, e.g. `"beforeEach"`.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::BeforeEach => "beforeEach",
            Self::BeforeAll => "beforeAll",
            Self::AfterEach => "afterEach",
            Self::AfterAll => "afterAll",
        }
    }

    /// Extracts the `LifecycleHook` from a call expression's callee, returning
    /// `None` if the call is not a recognised hook (bare or member form).
    ///
    /// Bare form: `beforeEach(...)`, `afterAll(...)`, etc.
    /// Member form: `test.beforeEach(...)`, `describe.afterAll(...)`, etc.,
    /// where the object must be a known test root (`test`, `it`, `describe`).
    pub(crate) fn from_call_expression(call: &JsCallExpression) -> Option<Self> {
        let callee = call.callee().ok()?.omit_parentheses();

        match callee {
            // beforeEach(...) / afterAll(...) etc.
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let token = ident.name().ok()?.value_token().ok()?;
                Self::from_name(token.text_trimmed())
            }

            // test.beforeEach(...) / describe.afterAll(...) etc.
            AnyJsExpression::JsStaticMemberExpression(member) => {
                // The right-hand member must be a known hook name.
                let AnyJsName::JsName(n) = member.member().ok()? else {
                    return None;
                };
                let hook = Self::from_name(n.value_token().ok()?.text_trimmed())?;

                // The left-hand object must be a known test root identifier.
                let object_is_test_root = member
                    .object()
                    .ok()
                    .map(|o| o.omit_parentheses())
                    .and_then(|o| {
                        if let AnyJsExpression::JsIdentifierExpression(i) = o {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .and_then(|i| i.name().and_then(|r| r.value_token()).ok())
                    .is_some_and(|tok| matches!(tok.text_trimmed(), "test" | "it" | "describe"));

                object_is_test_root.then_some(hook)
            }

            _ => None,
        }
    }

    fn from_name(s: &str) -> Option<Self> {
        match s {
            "beforeEach" => Some(Self::BeforeEach),
            "beforeAll" => Some(Self::BeforeAll),
            "afterEach" => Some(Self::AfterEach),
            "afterAll" => Some(Self::AfterAll),
            _ => None,
        }
    }
}

/// Returns `true` if the call expression is a test case call:
/// `it(...)`, `test(...)`, and their variants with modifiers
/// (`.only`, `.skip`, `.each`, `.todo`, `.failing`, etc.).
/// Also covers aliases: `xit`, `xtest`, `fit`, `ftest`.
///
/// Describe blocks are excluded — only leaf test cases are matched.
pub(crate) fn is_unit_test(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    if !callee.contains_a_test_pattern() {
        return false;
    }
    // Exclude describe blocks — we want only leaf test cases
    !is_describe_call(call)
}

/// Returns `true` if the call expression is a describe block:
/// - Bare call: `describe(...)`, `fdescribe(...)`, `xdescribe(...)`
/// - Member call: `test.describe(...)`, `it.describe(...)`, `describe.each(...)`
///   where the object is a known test root (`test`, `it`, or `describe`).
///
/// Only `JsStaticMemberExpression` callees are considered — computed member
/// expressions like `obj["describe"]()` are not matched.
pub(crate) fn is_describe_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    let callee = callee.omit_parentheses();

    match callee {
        // describe(...) / fdescribe(...) / xdescribe(...)
        AnyJsExpression::JsIdentifierExpression(ident) => ident
            .name()
            .and_then(|r| r.value_token())
            .is_ok_and(|tok| matches!(tok.text_trimmed(), "describe" | "fdescribe" | "xdescribe")),

        // test.describe(...) / it.describe(...) / describe.each(...) etc.
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // The right-hand member must be "describe".
            let member_is_describe = member
                .member()
                .ok()
                .and_then(|m| {
                    if let AnyJsName::JsName(n) = m {
                        Some(n)
                    } else {
                        None
                    }
                })
                .and_then(|n| n.value_token().ok())
                .is_some_and(|tok| tok.text_trimmed() == "describe");

            if !member_is_describe {
                return false;
            }

            // The left-hand object must be a known test root identifier.
            member
                .object()
                .ok()
                .map(|o| o.omit_parentheses())
                .and_then(|o| {
                    if let AnyJsExpression::JsIdentifierExpression(i) = o {
                        Some(i)
                    } else {
                        None
                    }
                })
                .and_then(|i| i.name().and_then(|r| r.value_token()).ok())
                .is_some_and(|tok| matches!(tok.text_trimmed(), "test" | "it" | "describe"))
        }

        _ => false,
    }
}

/// Returns the list of direct-child statements from the callback passed to a
/// `describe(...)` call, or `None` if the callback is not a recognisable
/// function literal with a block body.
pub(crate) fn describe_body_statements(call: &JsCallExpression) -> Option<Vec<AnyJsStatement>> {
    let args = call.arguments().ok()?;
    let [_, callback_arg] = args.get_arguments_by_index([0, 1]);
    let callback_arg = callback_arg?;
    let expr = callback_arg.as_any_js_expression()?;

    let body = match expr {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok()?,
        AnyJsExpression::JsFunctionExpression(func) => {
            AnyJsFunctionBody::JsFunctionBody(func.body().ok()?)
        }
        _ => return None,
    };

    let AnyJsFunctionBody::JsFunctionBody(block) = body else {
        return None;
    };

    Some(block.statements().into_iter().collect())
}

#[cfg(test)]
mod tests {
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{JsCallExpression, JsFileSource};
    use biome_rowan::AstNode;

    use super::{LifecycleHook, describe_body_statements, is_describe_call, is_unit_test};

    fn first_call(src: &str) -> JsCallExpression {
        let parse =
            biome_js_parser::parse(src, JsFileSource::js_module(), JsParserOptions::default());
        parse
            .syntax()
            .descendants()
            .find_map(JsCallExpression::cast)
            .expect("no call expression found in snippet")
    }

    // ── is_describe_call ────────────────────────────────────────────────────

    #[test]
    fn describe_bare() {
        assert!(is_describe_call(&first_call("describe('suite', () => {})")));
    }

    #[test]
    fn fdescribe_bare() {
        assert!(is_describe_call(&first_call(
            "fdescribe('suite', () => {})"
        )));
    }

    #[test]
    fn xdescribe_bare() {
        assert!(is_describe_call(&first_call(
            "xdescribe('suite', () => {})"
        )));
    }

    #[test]
    fn test_describe_member() {
        assert!(is_describe_call(&first_call(
            "test.describe('suite', () => {})"
        )));
    }

    #[test]
    fn it_describe_member() {
        assert!(is_describe_call(&first_call(
            "it.describe('suite', () => {})"
        )));
    }

    #[test]
    fn describe_describe_member() {
        assert!(is_describe_call(&first_call(
            "describe.describe('suite', () => {})"
        )));
    }

    #[test]
    fn it_call_is_not_describe() {
        assert!(!is_describe_call(&first_call("it('test', () => {})")));
    }

    #[test]
    fn test_call_is_not_describe() {
        assert!(!is_describe_call(&first_call("test('test', () => {})")));
    }

    #[test]
    fn unknown_dot_describe_is_not_describe() {
        // "foo" is not a known test root, so foo.describe() should not match.
        assert!(!is_describe_call(&first_call(
            "foo.describe('suite', () => {})"
        )));
    }

    #[test]
    fn computed_member_describe_is_not_describe() {
        // obj["describe"]() uses a computed member — should not match.
        assert!(!is_describe_call(&first_call(
            r#"obj["describe"]('suite', () => {})"#
        )));
    }

    // ── is_unit_test ────────────────────────────────────────────────────────

    #[test]
    fn it_is_unit_test() {
        assert!(is_unit_test(&first_call("it('does something', () => {})")));
    }

    #[test]
    fn test_is_unit_test() {
        assert!(is_unit_test(&first_call(
            "test('does something', () => {})"
        )));
    }

    #[test]
    fn xit_is_unit_test() {
        assert!(is_unit_test(&first_call("xit('does something', () => {})")));
    }

    #[test]
    fn xtest_is_unit_test() {
        assert!(is_unit_test(&first_call(
            "xtest('does something', () => {})"
        )));
    }

    #[test]
    fn fit_is_unit_test() {
        assert!(is_unit_test(&first_call("fit('does something', () => {})")));
    }

    #[test]
    fn it_only_is_unit_test() {
        assert!(is_unit_test(&first_call(
            "it.only('does something', () => {})"
        )));
    }

    #[test]
    fn test_skip_is_unit_test() {
        assert!(is_unit_test(&first_call(
            "test.skip('does something', () => {})"
        )));
    }

    #[test]
    fn describe_is_not_unit_test() {
        assert!(!is_unit_test(&first_call("describe('suite', () => {})")));
    }

    #[test]
    fn test_describe_is_not_unit_test() {
        assert!(!is_unit_test(&first_call(
            "test.describe('suite', () => {})"
        )));
    }

    #[test]
    fn unrelated_call_is_not_unit_test() {
        assert!(!is_unit_test(&first_call("console.log('hello')")));
    }

    // ── LifecycleHook::from_call_expression ─────────────────────────────────

    #[test]
    fn before_each_bare() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("beforeEach(() => {})")),
            Some(LifecycleHook::BeforeEach)
        );
    }

    #[test]
    fn before_all_bare() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("beforeAll(() => {})")),
            Some(LifecycleHook::BeforeAll)
        );
    }

    #[test]
    fn after_each_bare() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("afterEach(() => {})")),
            Some(LifecycleHook::AfterEach)
        );
    }

    #[test]
    fn after_all_bare() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("afterAll(() => {})")),
            Some(LifecycleHook::AfterAll)
        );
    }

    #[test]
    fn test_before_each_member() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("test.beforeEach(() => {})")),
            Some(LifecycleHook::BeforeEach)
        );
    }

    #[test]
    fn describe_after_all_member() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("describe.afterAll(() => {})")),
            Some(LifecycleHook::AfterAll)
        );
    }

    #[test]
    fn it_before_all_member() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("it.beforeAll(() => {})")),
            Some(LifecycleHook::BeforeAll)
        );
    }

    #[test]
    fn before_bare_is_not_hook() {
        // Mocha's `before` shorthand is intentionally excluded.
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("before(() => {})")),
            None
        );
    }

    #[test]
    fn after_bare_is_not_hook() {
        // Mocha's `after` shorthand is intentionally excluded.
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("after(() => {})")),
            None
        );
    }

    #[test]
    fn unknown_root_before_each_is_not_hook() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("foo.beforeEach(() => {})")),
            None
        );
    }

    #[test]
    fn computed_member_before_each_is_not_hook() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call(r#"obj["beforeEach"]()"#)),
            None
        );
    }

    #[test]
    fn it_call_is_not_hook() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("it('test', () => {})")),
            None
        );
    }

    #[test]
    fn describe_call_is_not_hook() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("describe('suite', () => {})")),
            None
        );
    }

    // ── describe_body_statements ────────────────────────────────────────────

    fn first_describe_call(src: &str) -> JsCallExpression {
        let parse =
            biome_js_parser::parse(src, JsFileSource::js_module(), JsParserOptions::default());
        parse
            .syntax()
            .descendants()
            .find_map(JsCallExpression::cast)
            .expect("no call expression found in snippet")
    }

    #[test]
    fn describe_body_with_arrow_callback() {
        let call = first_describe_call("describe('suite', () => { it('a', () => {}); })");
        let stmts = describe_body_statements(&call).expect("should return statements");
        assert_eq!(stmts.len(), 1);
    }

    #[test]
    fn describe_body_with_function_callback() {
        let call = first_describe_call(
            "describe('suite', function() { it('a', () => {}); it('b', () => {}); })",
        );
        let stmts = describe_body_statements(&call).expect("should return statements");
        assert_eq!(stmts.len(), 2);
    }

    #[test]
    fn describe_body_with_empty_callback() {
        let call = first_describe_call("describe('suite', () => {})");
        let stmts = describe_body_statements(&call).expect("should return statements");
        assert_eq!(stmts.len(), 0);
    }

    #[test]
    fn describe_body_with_no_callback_returns_none() {
        // Only one argument — no callback, so returns None.
        let call = first_describe_call("describe('suite')");
        assert!(describe_body_statements(&call).is_none());
    }

    #[test]
    fn describe_body_with_expression_body_arrow_returns_none() {
        // Arrow with expression body, not a block — returns None.
        let call = first_describe_call("describe('suite', () => null)");
        assert!(describe_body_statements(&call).is_none());
    }

    // ── as_str ───────────────────────────────────────────────────────────────

    #[test]
    fn as_str_before_each() {
        assert_eq!(LifecycleHook::BeforeEach.as_str(), "beforeEach");
    }

    #[test]
    fn as_str_before_all() {
        assert_eq!(LifecycleHook::BeforeAll.as_str(), "beforeAll");
    }

    #[test]
    fn as_str_after_each() {
        assert_eq!(LifecycleHook::AfterEach.as_str(), "afterEach");
    }

    #[test]
    fn as_str_after_all() {
        assert_eq!(LifecycleHook::AfterAll.as_str(), "afterAll");
    }

    #[test]
    fn hook_name_member_form() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("test.beforeEach(() => {})")),
            Some(LifecycleHook::BeforeEach)
        );
    }

    #[test]
    fn hook_non_hook_returns_none() {
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("it('a', () => {})")),
            None
        );
    }

    #[test]
    fn hook_before_shorthand_returns_none() {
        // Mocha's `before` is not in the hook list.
        assert_eq!(
            LifecycleHook::from_call_expression(&first_call("before(() => {})")),
            None
        );
    }
}
