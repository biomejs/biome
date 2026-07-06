//! Cheap check for whether a file can contain anything React Compiler would
//! analyze outside of `all` mode.
//!
//! This mirrors `has_react_like_functions` from the upstream
//! `react_compiler_oxc` crate: a shallow walk over function *definitions*,
//! matching their explicit name — or the name inferred from an enclosing
//! variable declarator or assignment — against the compiler's own
//! [`is_react_like_name`] predicate (capitalized components and `use`-prefixed
//! hooks). Like upstream, the walk does not descend into function bodies or
//! class bodies, so a hook defined inside a non-React factory function is not
//! detected; such factories are an anti-pattern that React Compiler itself
//! does not analyze reliably either.
//!
//! The check must over-approximate what `infer` mode compiles: a false
//! positive costs one wasted compilation, while a false negative silently
//! drops diagnostics for the file.

use biome_js_syntax::{
    AnyJsRoot, JsAssignmentExpression, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TokenText, WalkEvent};
use react_compiler_hir::environment::is_react_like_name;

/// Returns `true` if the file defines at least one function whose name looks
/// like a React component or hook.
pub fn has_react_like_functions(root: &AnyJsRoot) -> bool {
    // Innermost enclosing variable declarator / assignment name, used as the
    // inferred name for anonymous function and arrow expressions.
    let mut name_stack: Vec<(JsSyntaxNode, Option<TokenText>)> = Vec::new();
    let current_name = |stack: &[(JsSyntaxNode, Option<TokenText>)]| -> Option<TokenText> {
        stack.last().and_then(|(_, name)| name.clone())
    };

    let mut preorder = root.syntax().preorder();
    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => match node.kind() {
                JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                    let name = JsVariableDeclarator::cast_ref(&node)
                        .and_then(|declarator| declarator.id().ok())
                        .and_then(|id| id.as_any_js_binding()?.as_js_identifier_binding().cloned())
                        .and_then(|binding| binding.name_token().ok())
                        .map(|token| token.token_text_trimmed());
                    name_stack.push((node, name));
                }
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    let name = JsAssignmentExpression::cast_ref(&node)
                        .and_then(|assignment| assignment.left().ok())
                        .and_then(|left| {
                            left.as_any_js_assignment()?
                                .as_js_identifier_assignment()
                                .cloned()
                        })
                        .and_then(|target| target.name_token().ok())
                        .map(|token| token.token_text_trimmed());
                    name_stack.push((node, name));
                }
                JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                    let name = JsFunctionDeclaration::cast_ref(&node)
                        .and_then(|function| function.id().ok())
                        .and_then(|id| id.as_js_identifier_binding()?.name_token().ok())
                        .map(|token| token.token_text_trimmed());
                    if name.is_some_and(|name| is_react_like_name(&name)) {
                        return true;
                    }
                    preorder.skip_subtree();
                }
                JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                    let name = JsFunctionExportDefaultDeclaration::cast_ref(&node)
                        .and_then(|function| function.id())
                        .and_then(|id| id.as_js_identifier_binding()?.name_token().ok())
                        .map(|token| token.token_text_trimmed());
                    if name.is_some_and(|name| is_react_like_name(&name)) {
                        return true;
                    }
                    preorder.skip_subtree();
                }
                JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                    // An explicit function expression name takes precedence
                    // over the inferred name, matching the upstream visitor.
                    let name = JsFunctionExpression::cast_ref(&node)
                        .and_then(|function| function.id())
                        .map_or_else(
                            || current_name(&name_stack),
                            |id| {
                                id.as_js_identifier_binding()
                                    .and_then(|binding| binding.name_token().ok())
                                    .map(|token| token.token_text_trimmed())
                            },
                        );
                    if name.is_some_and(|name| is_react_like_name(&name)) {
                        return true;
                    }
                    preorder.skip_subtree();
                }
                JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    if current_name(&name_stack).is_some_and(|name| is_react_like_name(&name)) {
                        return true;
                    }
                    preorder.skip_subtree();
                }
                JsSyntaxKind::JS_CLASS_DECLARATION
                | JsSyntaxKind::JS_CLASS_EXPRESSION
                | JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                    preorder.skip_subtree();
                }
                _ => {}
            },
            WalkEvent::Leave(node) => {
                if name_stack
                    .last()
                    .is_some_and(|(context, _)| context == &node)
                {
                    name_stack.pop();
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_syntax::JsFileSource;

    fn check(source: &str) -> bool {
        let parsed = parse(source, JsFileSource::jsx(), JsParserOptions::default());
        has_react_like_functions(&parsed.tree())
    }

    #[test]
    fn detects_react_like_definitions() {
        // Component and hook declarations.
        assert!(check("function Component() {}"));
        assert!(check("function useThing() {}"));
        assert!(check("export default function App() {}"));
        // Hook names may have a digit after `use`, per the compiler's predicate.
        assert!(check("function use3rdParty() {}"));
        // Inferred names from declarators and assignments.
        assert!(check("const Component = () => {};"));
        assert!(check("const useThing = function () {};"));
        assert!(check("Component = () => {};"));
        // The name survives intermediate wrappers, e.g. `memo(...)`.
        assert!(check("const Component = memo(() => {});"));
    }

    #[test]
    fn skips_non_react_files() {
        assert!(!check("function increment() {}"));
        assert!(!check("const helper = () => {};"));
        assert!(!check("export default () => {};"));
        // `use`-prefixed but not hook-cased.
        assert!(!check("function user() {} function useful() {}"));
        // Uppercase declarator without a function value.
        assert!(!check("const Config = { a: 1 };"));
        // Function bodies are not traversed, matching the upstream prefilter.
        assert!(!check("function factory() { function useThing() {} }"));
        // Class bodies are skipped: the compiler does not compile classes.
        assert!(!check("class Foo { useThing() {} }"));
    }
}
