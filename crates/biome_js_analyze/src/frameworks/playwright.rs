use biome_js_syntax::{
    AnyJsExpression, JsCallArguments, JsCallExpression, JsObjectExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, TokenText};

/// Extracts the object name from an expression.
///
/// For identifier expressions (e.g., `page`), returns the identifier name.
/// For member expressions (e.g., `context.page`), returns the member name.
pub(crate) fn get_object_name(expr: &AnyJsExpression) -> Option<TokenText> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            Some(id.name().ok()?.value_token().ok()?.token_text_trimmed())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => Some(
            member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        _ => None,
    }
}

/// Checks if a name represents a Playwright page or frame.
///
/// Returns true if the name:
/// - Is exactly "page" or "frame"
/// - Ends with "Page" or "Frame" (e.g., "myPage", "childFrame")
pub(crate) fn is_page_or_frame_name(name: &str) -> bool {
    name == "page" || name == "frame" || name.ends_with("Page") || name.ends_with("Frame")
}

/// Gets the page/frame name from an expression if it represents a page or frame.
///
/// Combines `get_object_name` and `is_page_or_frame_name` for convenience.
pub(crate) fn get_page_or_frame_name(expr: &AnyJsExpression) -> Option<TokenText> {
    let name = get_object_name(expr)?;
    if is_page_or_frame_name(name.text()) {
        Some(name)
    } else {
        None
    }
}

/// Methods that return Locator objects in Playwright.
/// IMPORTANT: Keep this array sorted for binary search.
pub(crate) const LOCATOR_METHODS: &[&str] = &[
    "and",
    "first",
    "getByAltText",
    "getByLabel",
    "getByPlaceholder",
    "getByRole",
    "getByTestId",
    "getByText",
    "getByTitle",
    "last",
    "locator",
    "nth",
    "or",
];

/// Checks if a name is a Playwright describe mode (`parallel` or `serial`).
pub(crate) fn is_describe_mode(s: &str) -> bool {
    s == "parallel" || s == "serial"
}

/// Checks if a name is a Playwright describe modifier (`only`, `skip`, or `fixme`).
pub(crate) fn is_describe_modifier(s: &str) -> bool {
    s == "only" || s == "skip" || s == "fixme"
}

/// Checks if a method name is a Playwright locator method.
fn is_locator_method(name: &str) -> bool {
    LOCATOR_METHODS.binary_search(&name).is_ok()
}

/// Collects member names from an expression chain in "outside-in" order.
/// For example, `test.describe.skip` returns `["test", "describe", "skip"]`.
///
/// Uses `TokenText` to avoid string allocations where possible.
pub(crate) fn collect_member_names(expr: &AnyJsExpression) -> Option<Vec<TokenText>> {
    let mut names = Vec::new();
    collect_member_names_rec(expr, &mut names)?;
    Some(names)
}

fn collect_member_names_rec(expr: &AnyJsExpression, names: &mut Vec<TokenText>) -> Option<()> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            let name = id.name().ok()?;
            let token = name.value_token().ok()?;
            names.push(token.token_text_trimmed());
            Some(())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // First recurse on object to get outer names
            let object = member.object().ok()?;
            collect_member_names_rec(&object, names)?;
            // Then add this member name
            let m = member.member().ok()?;
            let n = m.as_js_name()?;
            let t = n.value_token().ok()?;
            names.push(t.token_text_trimmed());
            Some(())
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            // First recurse on object
            let object = member.object().ok()?;
            collect_member_names_rec(&object, names)?;
            // For computed members, extract string literal value using inner_string_text
            if let Ok(expr) = member.member()
                && let Some(literal) = expr.as_any_js_literal_expression()
                && let Some(string_lit) = literal.as_js_string_literal_expression()
                && let Ok(inner) = string_lit.inner_string_text()
            {
                names.push(inner);
                return Some(());
            }
            None
        }
        _ => None,
    }
}

/// Checks if the callee is a test() or it() call.
/// Matches patterns like: test(), it(), test.skip(), test.only(), etc.
pub(crate) fn is_test_call(callee: &AnyJsExpression) -> bool {
    match callee {
        AnyJsExpression::JsIdentifierExpression(id) => {
            if let Ok(name) = id.name()
                && let Ok(token) = name.value_token()
            {
                let text = token.text_trimmed();
                return text == "test" || text == "it";
            }
            false
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // Exclude describe blocks, hooks, and steps - these are not test calls
            if let Ok(member_name) = member.member()
                && let Some(name) = member_name.as_js_name()
                && let Ok(token) = name.value_token()
            {
                let text = token.text_trimmed();
                if matches!(
                    text,
                    "describe" | "step" | "beforeEach" | "afterEach" | "beforeAll" | "afterAll"
                ) {
                    return false;
                }
            }
            if let Ok(object) = member.object() {
                return is_test_call(&object);
            }
            false
        }
        _ => false,
    }
}

/// Gets the callback function from test arguments.
/// Returns the LAST function argument (not the first) to handle:
/// ```javascript
/// test("name", { retry: () => 2 }, async () => { /* ... */ })
/// ```
pub(crate) fn get_test_callback(args: &JsCallArguments) -> Option<AnyJsExpression> {
    let arg_list = args.args();
    let mut callback = None;

    for arg in arg_list.iter() {
        let Ok(arg) = arg else { continue };
        if let Some(expr) = arg.as_any_js_expression() {
            match expr {
                AnyJsExpression::JsArrowFunctionExpression(_)
                | AnyJsExpression::JsFunctionExpression(_) => {
                    callback = Some(expr.clone());
                }
                _ => {}
            }
        }
    }

    callback
}

/// Checks if a call expression is an expect() call or part of an expect chain.
/// Matches: expect(), expect.soft(), expect.poll(), expect(x).toBe(), expect(x).not.toBe(), etc.
pub(crate) fn is_expect_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };

    is_expect_expression(&callee)
}

/// Helper function to check if an expression is part of an expect chain.
/// This handles nested member expressions like expect(page).not.toHaveTitle
fn is_expect_expression(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            if let Ok(name) = id.name()
                && let Ok(token) = name.value_token()
            {
                let text = token.text_trimmed();
                return text == "expect"
                // support chai-style `assert` syntax from Vitest
                || text == "assert"
                // Include `expectTypeOf`/`assertType` for type assertions from `expect-type`
                || text == "expectTypeOf"
                || text == "assertType";
            }
            false
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // expect.soft(), expect.poll(), expect(...).method(), expect(...).not.method()
            if let Ok(object) = member.object() {
                // Recursively check the object - this handles chained member expressions
                // like expect(page).not where the object is itself a member expression
                // NB: This is overly permissive for certain Vitest constructs (ex: `expect.stringContaining()`)
                // that do not assert anything in and of themselves (see issue #9174)
                return is_expect_expression(&object);
            }
            false
        }
        AnyJsExpression::JsCallExpression(inner_call) => {
            // Handle chained expectations - check if the inner call is an expect call
            is_expect_call(inner_call)
        }
        _ => false,
    }
}

/// Checks if an expression (function body) contains an expect() call.
pub(crate) fn contains_expect_call(callback: &AnyJsExpression) -> bool {
    // Walk through all descendants looking for expect() calls
    for descendant in callback.syntax().descendants() {
        if descendant.kind() == JsSyntaxKind::JS_CALL_EXPRESSION
            && let Some(call) = JsCallExpression::cast(descendant)
            && is_expect_call(&call)
        {
            return true;
        }
    }
    false
}

/// Searches an expression chain for a static member name matching the predicate.
///
/// Walks through `JsStaticMemberExpression` and `JsCallExpression` chains,
/// testing each member name. Returns `true` if any member name satisfies `predicate`.
///
/// For example, given `expect.poll(() => x).toBe(1)`:
/// - `find_member_in_chain(expr, |n| n == "poll")` returns `true`
/// - `find_member_in_chain(expr, |n| n == "soft")` returns `false`
pub(crate) fn find_member_in_chain(
    expr: &AnyJsExpression,
    predicate: impl Fn(&str) -> bool + Copy,
) -> bool {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if let Ok(member_name) = member.member()
                && let Some(name) = member_name.as_js_name()
                && let Ok(token) = name.value_token()
                && predicate(token.text_trimmed())
            {
                return true;
            }
            if let Ok(object) = member.object() {
                return find_member_in_chain(&object, predicate);
            }
            false
        }
        AnyJsExpression::JsCallExpression(call) => {
            if let Ok(callee) = call.callee() {
                find_member_in_chain(&callee, predicate)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Checks if an expression is part of a Playwright call chain.
///
/// A Playwright call chain originates from a page/frame object or passes through
/// locator methods. This function recursively traverses member/call expression chains.
///
/// Returns `true` for patterns like:
/// - `page.click()` (direct page call)
/// - `page.locator('button').click()` (locator chain)
/// - `context.page.locator('x').click()` (member access to page)
/// - `myPage.locator('x').click()` (variable ending with "Page")
pub(crate) fn is_playwright_call_chain(expr: &AnyJsExpression) -> bool {
    match expr {
        // Base case: identifier like `page` or `frame`
        AnyJsExpression::JsIdentifierExpression(id) => {
            if let Ok(name) = id.name()
                && let Ok(token) = name.value_token()
            {
                return is_page_or_frame_name(token.text_trimmed());
            }
            false
        }

        // Member access: `context.page`, `page.locator`
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if let Ok(member_name) = member.member()
                && let Some(name) = member_name.as_js_name()
                && let Ok(token) = name.value_token()
            {
                // Check if member name is page/frame
                if is_page_or_frame_name(token.text_trimmed()) {
                    return true;
                }
            }
            // Recurse on object
            if let Ok(object) = member.object() {
                return is_playwright_call_chain(&object);
            }
            false
        }

        // Call expression: `page.locator()` result
        AnyJsExpression::JsCallExpression(call) => {
            if let Ok(callee) = call.callee()
                && let Some(member) =
                    biome_js_syntax::JsStaticMemberExpression::cast_ref(callee.syntax())
            {
                // Check if it's a locator method
                if let Ok(member_name) = member.member()
                    && let Some(name) = member_name.as_js_name()
                    && let Ok(token) = name.value_token()
                    && is_locator_method(token.text_trimmed())
                {
                    // Verify the chain continues to page/frame
                    if let Ok(object) = member.object() {
                        return is_playwright_call_chain(&object);
                    }
                }
                // Also recurse for any call on page/frame
                if let Ok(object) = member.object() {
                    return is_playwright_call_chain(&object);
                }
            }
            false
        }

        _ => false,
    }
}

/// Checks if an object expression has a boolean property with the given key and value.
///
/// For example, `has_bool_property(obj, "force", true)` returns `true` for `{ force: true }`.
pub(crate) fn has_bool_property(obj_expr: &JsObjectExpression, key: &str, value: bool) -> bool {
    let expected_text = if value { "true" } else { "false" };
    for member in obj_expr.members().into_iter().flatten() {
        if let Some(prop) = member.as_js_property_object_member()
            && let Ok(prop_name) = prop.name()
            && let Some(name_text) = prop_name.name()
            && name_text.text() == key
            && let Ok(prop_value) = prop.value()
            && let Some(literal) = prop_value.as_any_js_literal_expression()
            && let Some(bool_lit) = literal.as_js_boolean_literal_expression()
            && let Ok(value_token) = bool_lit.value_token()
            && value_token.text_trimmed() == expected_text
        {
            return true;
        }
    }
    false
}

/// Checks if an object expression has a string property with the given key and value.
///
/// For example, `has_string_property(obj, "waitUntil", "networkidle")` returns `true`
/// for `{ waitUntil: 'networkidle' }`.
pub(crate) fn has_string_property(obj_expr: &JsObjectExpression, key: &str, value: &str) -> bool {
    for member in obj_expr.members().into_iter().flatten() {
        if let Some(prop) = member.as_js_property_object_member()
            && let Ok(prop_name) = prop.name()
            && let Some(name_text) = prop_name.name()
            && name_text.text() == key
            && let Ok(prop_value) = prop.value()
            && let Some(literal) = prop_value.as_any_js_literal_expression()
            && let Some(string_lit) = literal.as_js_string_literal_expression()
            && let Ok(inner) = string_lit.inner_string_text()
            && inner == value
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::JsFileSource;

    fn get_callee(source: &str) -> AnyJsExpression {
        let parsed = parse(
            source,
            JsFileSource::js_module(),
            JsParserOptions::default(),
        );
        let call = parsed
            .tree()
            .syntax()
            .descendants()
            .find_map(JsCallExpression::cast)
            .expect("Expected a call expression");
        call.callee().expect("Expected a callee")
    }

    #[test]
    fn test_is_test_call_valid() {
        // Direct test/it calls
        assert!(is_test_call(&get_callee("test('name', () => {})")));
        assert!(is_test_call(&get_callee("it('name', () => {})")));

        // Test modifiers
        assert!(is_test_call(&get_callee("test.skip('name', () => {})")));
        assert!(is_test_call(&get_callee("test.only('name', () => {})")));
        assert!(is_test_call(&get_callee("it.skip('name', () => {})")));
        assert!(is_test_call(&get_callee("it.only('name', () => {})")));
    }

    #[test]
    fn test_is_test_call_excludes_non_test_patterns() {
        // Describe blocks are not test calls
        assert!(!is_test_call(&get_callee(
            "test.describe('suite', () => {})"
        )));
        assert!(!is_test_call(&get_callee(
            "test.describe.only('suite', () => {})"
        )));
        assert!(!is_test_call(&get_callee(
            "test.describe.skip('suite', () => {})"
        )));

        // Hooks are not test calls
        assert!(!is_test_call(&get_callee("test.beforeEach(() => {})")));
        assert!(!is_test_call(&get_callee("test.afterEach(() => {})")));
        assert!(!is_test_call(&get_callee("test.beforeAll(() => {})")));
        assert!(!is_test_call(&get_callee("test.afterAll(() => {})")));

        // Steps are not test calls
        assert!(!is_test_call(&get_callee("test.step('step', () => {})")));
    }

    #[test]
    fn test_is_page_or_frame_name_valid() {
        assert!(is_page_or_frame_name("page"));
        assert!(is_page_or_frame_name("frame"));
        assert!(is_page_or_frame_name("myPage"));
        assert!(is_page_or_frame_name("childFrame"));
        assert!(is_page_or_frame_name("newPage"));
        assert!(is_page_or_frame_name("mainFrame"));
    }

    #[test]
    fn test_is_page_or_frame_name_invalid() {
        assert!(!is_page_or_frame_name("locator"));
        assert!(!is_page_or_frame_name("browser"));
        assert!(!is_page_or_frame_name("context"));
        assert!(!is_page_or_frame_name("element"));
        assert!(!is_page_or_frame_name("pageBuilder")); // "Page" must be at the end
        assert!(!is_page_or_frame_name("frameWork")); // "Frame" must be at the end
    }

    #[test]
    fn locator_methods_sorted() {
        assert!(LOCATOR_METHODS.is_sorted());
    }
}
