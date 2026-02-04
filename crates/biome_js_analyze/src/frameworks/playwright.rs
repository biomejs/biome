use biome_js_syntax::AnyJsExpression;
use biome_rowan::{AstNode, TokenText};

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

/// Checks if a method name is a Playwright locator method.
fn is_locator_method(name: &str) -> bool {
    LOCATOR_METHODS.binary_search(&name).is_ok()
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

#[cfg(test)]
mod tests {
    use super::*;

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
