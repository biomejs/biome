use biome_js_syntax::{AnyJsExpression, AnyJsName, JsCallExpression};

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

#[cfg(test)]
mod tests {
    use biome_js_parser::JsParserOptions;
    use biome_js_syntax::{JsCallExpression, JsFileSource};
    use biome_rowan::AstNode;

    use super::{is_describe_call, is_unit_test};

    fn first_call(src: &str) -> JsCallExpression {
        let parse =
            biome_js_parser::parse(src, JsFileSource::js_module(), JsParserOptions::default());
        parse
            .syntax()
            .descendants()
            .find_map(JsCallExpression::cast)
            .expect("no call expression found in snippet")
    }

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
}
