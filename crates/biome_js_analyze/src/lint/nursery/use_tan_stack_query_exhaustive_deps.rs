use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, AnyJsObjectMember,
    JsArrayExpression, JsCallExpression, JsIdentifierBinding, JsImport, JsSyntaxKind, JsSyntaxNode,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_tan_stack_query_exhaustive_deps::UseTanStackQueryExhaustiveDepsOptions;
use rustc_hash::FxHashSet;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce that all dependencies used in TanStack Query's queryFn are included in the queryKey.
    ///
    /// This rule ensures that all variables and dependencies used inside the `queryFn` of TanStack Query
    /// hooks (like `useQuery` and `useInfiniteQuery`) are properly included in the `queryKey` array.
    /// This prevents stale closures and ensures proper cache invalidation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from '@tanstack/react-query';
    ///
    /// function MyComponent({ userId }) {
    ///   const query = useQuery({
    ///     queryKey: ['users'],
    ///     queryFn: () => fetchUser(userId)
    ///   });
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useQuery } from '@tanstack/react-query';
    ///
    /// function MyComponent({ userId }) {
    ///   const query = useQuery({
    ///     queryKey: ['users', userId],
    ///     queryFn: () => fetchUser(userId)
    ///   });
    /// }
    /// ```
    ///
    pub UseTanStackQueryExhaustiveDeps {
        version: "next",
        name: "useTanStackQueryExhaustiveDeps",
        language: "js",
        sources: &[RuleSource::EslintReactHooks("exhaustive-deps").same()],
        recommended: false,
    }
}

#[derive(Debug)]
pub struct QueryInfo {
    query_fn: AnyJsExpression,
    missing_deps: Vec<String>,
}

impl Rule for UseTanStackQueryExhaustiveDeps {
    type Query = Semantic<JsCallExpression>;
    type State = QueryInfo;
    type Signals = Option<Self::State>;
    type Options = UseTanStackQueryExhaustiveDepsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let options = ctx.options();

        // Check if this is a TanStack Query hook call
        if !is_tanstack_query_hook(call, model, options) {
            return None;
        }

        // Extract queryKey and queryFn from the call
        let Some((query_key, query_fn)) = extract_query_properties(call) else {
            return None;
        };

        // Use semantic model to find all captures in queryFn
        let query_fn_captures = get_semantic_captures(&query_fn, model, call);

        // Get existing dependencies from queryKey
        let existing_deps = get_query_key_dependencies(&query_key);

        // Find missing dependencies using semantic analysis
        let missing_deps = find_missing_dependencies(query_fn_captures, existing_deps, model);

        if !missing_deps.is_empty() {
            Some(QueryInfo {
                query_fn,
                missing_deps,
            })
        } else {
            None
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let deps_list = state.missing_deps.join(", ");
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.query_fn.range(),
                markup! {
                    "The following dependencies are missing in your queryKey: "<Emphasis>{deps_list}</Emphasis>
                },
            )
            .note(markup! {
                "Variables used in queryFn should be included in queryKey to ensure proper cache invalidation."
            }),
        )
    }
}

/// List of valid TanStack Query package names
const TANSTACK_QUERY_PACKAGES: &[&str] = &[
    "@tanstack/react-query",
    "@tanstack/react-query/experimental",
    "react-query", // Legacy package name
];

/// Checks if the current call expression is a TanStack Query hook.
/// This verifies both the function name and that it's imported from a valid TanStack Query package.
fn is_tanstack_query_hook(
    call: &JsCallExpression,
    model: &SemanticModel,
    options: &UseTanStackQueryExhaustiveDepsOptions,
) -> bool {
    let Some(callee) = call.callee().ok() else {
        return false;
    };

    let callee = callee.omit_parentheses();

    // Handle both direct calls (useQuery) and member calls (QueryClient.useQuery)
    let (hook_name, binding_to_check) =
        if let Some(member_expr) = AnyJsMemberExpression::cast_ref(callee.syntax()) {
            // Handle React.useQuery style calls (though less common with TanStack Query)
            let Some(member_name) = member_expr.member_name() else {
                return false;
            };
            let Some(object) = member_expr.object().ok() else {
                return false;
            };
            let Some(reference) = object.omit_parentheses().as_js_reference_identifier() else {
                return false;
            };
            (member_name.text().to_string(), model.binding(&reference))
        } else if let Some(identifier) = callee.as_js_reference_identifier() {
            // Handle direct calls like useQuery()
            let Some(name) = identifier.name().ok() else {
                return false;
            };
            (name.text().to_string(), model.binding(&identifier))
        } else {
            return false;
        };

    // Check if it's a hook we're interested in
    let is_target_hook = match hook_name.as_str() {
        "useQuery" => options.use_query,
        "useInfiniteQuery" => options.use_infinite_query,
        _ => false,
    };

    if !is_target_hook {
        return false;
    }

    // If there's a binding, verify it's imported from TanStack Query
    if let Some(binding) = binding_to_check {
        is_tanstack_query_import(&binding, &hook_name)
    } else {
        // If no binding found, it might be a global (less common for TanStack Query)
        // For now, we'll be permissive and allow it
        true
    }
}

/// Checks if a binding is imported from a TanStack Query package
fn is_tanstack_query_import(binding: &Binding, expected_name: &str) -> bool {
    // Check if it's a named import from TanStack Query
    if let Some(is_named) = is_named_tanstack_query_export(binding, expected_name) {
        return is_named;
    }

    // Check if it's a namespace import (import * as Query from '@tanstack/react-query')
    is_tanstack_query_export(binding)
}

/// Checks if the binding is exported from any TanStack Query package
fn is_tanstack_query_export(binding: &Binding) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| TANSTACK_QUERY_PACKAGES.contains(&source.text()))
}

/// Checks if the binding is a named export from TanStack Query with the expected name
fn is_named_tanstack_query_export(binding: &Binding, expected_name: &str) -> Option<bool> {
    let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
    let import_specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;

    let name_token = match &import_specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
            named_import.name().ok()?.value().ok()?
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => ident.name_token().ok()?,
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => {
            return Some(false);
        }
    };

    if name_token.text_trimmed() != expected_name {
        return Some(false);
    }

    let import = import_specifier.import_clause()?.parent::<JsImport>()?;
    import
        .source_text()
        .ok()
        .map(|import_name| TANSTACK_QUERY_PACKAGES.contains(&import_name.text()))
}

fn extract_query_properties(
    call: &JsCallExpression,
) -> Option<(JsArrayExpression, AnyJsExpression)> {
    let args = call.arguments().ok()?;
    let first_arg = args
        .args()
        .iter()
        .next()?
        .ok()?
        .as_any_js_expression()?
        .clone();

    let AnyJsExpression::JsObjectExpression(obj) = first_arg else {
        return None;
    };

    let mut query_key = None;
    let mut query_fn = None;

    for member in obj.members() {
        if let Ok(AnyJsObjectMember::JsPropertyObjectMember(prop)) = member {
            if let Ok(key) = prop.name() {
                if let Some(key_name) = key.as_js_literal_member_name() {
                    if let Ok(name) = key_name.value() {
                        let prop_name = name.text_trimmed();
                        match prop_name {
                            "queryKey" => {
                                if let Ok(value) = prop.value() {
                                    if let AnyJsExpression::JsArrayExpression(array) = value {
                                        query_key = Some(array);
                                    }
                                }
                            }
                            "queryFn" => {
                                if let Ok(value) = prop.value() {
                                    query_fn = Some(value);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Some((query_key?, query_fn?))
}

fn get_semantic_captures(
    query_fn: &AnyJsExpression,
    model: &SemanticModel,
    hook_call: &JsCallExpression,
) -> Vec<biome_js_semantic::Capture> {
    // Find the component function that contains this hook call
    let Some(component_function) = function_of_hook_call(hook_call) else {
        return Vec::new();
    };

    let component_range = component_function.text_range_with_trivia();

    // Handle different query function types
    match query_fn {
        AnyJsExpression::JsArrowFunctionExpression(arrow_fn) => {
            let closure = model.closure(arrow_fn);
            closure
                .all_captures()
                .filter(|capture| {
                    capture_needs_to_be_in_dependency_list(&capture, &component_range, model)
                })
                .collect()
        }
        AnyJsExpression::JsFunctionExpression(func_expr) => {
            let closure = model.closure(func_expr);
            closure
                .all_captures()
                .filter(|capture| {
                    capture_needs_to_be_in_dependency_list(&capture, &component_range, model)
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

// Adapted from useExhaustiveDependencies - simplified for TanStack Query
fn capture_needs_to_be_in_dependency_list(
    capture: &biome_js_semantic::Capture,
    component_function_range: &biome_js_syntax::TextRange,
    model: &SemanticModel,
) -> bool {
    let binding = capture.binding();

    // Ignore if imported
    if binding.is_imported() {
        return false;
    }

    let Some(decl) = binding.tree().declaration() else {
        return false;
    };

    // Use the same logic as useExhaustiveDependencies for determining stability
    use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
    match decl.parent_binding_pattern_declaration().unwrap_or(decl) {
        // These declarations are always stable
        AnyJsBindingDeclaration::JsClassDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_) => false,

        // Variable declarators are stable if they're const and outside component
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(declaration) = declarator
                .syntax()
                .ancestors()
                .find_map(biome_js_syntax::JsVariableDeclaration::cast)
            else {
                return true; // Default to requiring dependency if we can't analyze
            };

            let declaration_range = declaration.syntax().text_range_with_trivia();

            // If declared outside component function, it's stable
            if component_function_range
                .intersect(declaration_range)
                .is_none_or(biome_js_syntax::TextRange::is_empty)
            {
                return false;
            }

            // If it's const with constant initializer, it's stable
            if declaration.is_const() {
                if declarator
                    .initializer()
                    .and_then(|init| init.expression().ok())
                    .is_none_or(|expr| model.is_constant(&expr))
                {
                    return false;
                }
            }

            true
        }

        // All other cases need to be in dependency list
        _ => true,
    }
}

fn find_missing_dependencies(
    captures: Vec<biome_js_semantic::Capture>,
    existing_deps: FxHashSet<String>,
    _model: &SemanticModel,
) -> Vec<String> {
    let mut missing = Vec::new();

    for capture in captures {
        let capture_name = capture.node().text_trimmed().to_string();
        if !existing_deps.contains(&capture_name) {
            missing.push(capture_name);
        }
    }

    // Remove duplicates
    missing.sort();
    missing.dedup();
    missing
}

fn get_query_key_dependencies(query_key: &JsArrayExpression) -> FxHashSet<String> {
    let mut deps = FxHashSet::default();

    let elements = query_key.elements();
    for element in elements.iter() {
        if let Ok(element) = element {
            if let Some(expr) = element.as_any_js_expression() {
                extract_dependencies_from_expression(expr, &mut deps);
            } else if let Some(spread) = element.as_js_spread() {
                if let Ok(arg) = spread.argument() {
                    extract_dependencies_from_expression(&arg, &mut deps);
                }
            }
        }
    }

    deps
}

fn extract_dependencies_from_expression(expr: &AnyJsExpression, deps: &mut FxHashSet<String>) {
    match expr {
        // Simple identifier: userId
        AnyJsExpression::JsIdentifierExpression(ident) => {
            if let Ok(name) = ident.name() {
                if let Ok(token) = name.value_token() {
                    deps.insert(token.text_trimmed().to_string());
                }
            }
        }
        // Static member access: user.id, data.status
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // For member expressions, we want to track the root identifier
            // e.g., for "user.id", we want to track "user"
            let mut current = member.object().ok();
            while let Some(obj) = current {
                match obj {
                    AnyJsExpression::JsIdentifierExpression(ident) => {
                        if let Ok(name) = ident.name() {
                            if let Ok(token) = name.value_token() {
                                deps.insert(token.text_trimmed().to_string());
                            }
                        }
                        break;
                    }
                    AnyJsExpression::JsStaticMemberExpression(nested_member) => {
                        current = nested_member.object().ok();
                    }
                    _ => break,
                }
            }
        }
        // Computed member access: user[key], data["status"]
        AnyJsExpression::JsComputedMemberExpression(member) => {
            // For computed member expressions, track both object and property if it's an identifier
            let mut current = member.object().ok();
            while let Some(obj) = current {
                match obj {
                    AnyJsExpression::JsIdentifierExpression(ident) => {
                        if let Ok(name) = ident.name() {
                            if let Ok(token) = name.value_token() {
                                deps.insert(token.text_trimmed().to_string());
                            }
                        }
                        break;
                    }
                    AnyJsExpression::JsComputedMemberExpression(nested_member) => {
                        current = nested_member.object().ok();
                    }
                    _ => break,
                }
            }

            // Also extract the property if it's an identifier
            if let Ok(member) = member.member() {
                extract_dependencies_from_expression(&member, deps);
            }
        }
        // Object literals, arrays, etc. - recursively extract
        AnyJsExpression::JsObjectExpression(obj) => {
            for member in obj.members() {
                match member {
                    Ok(AnyJsObjectMember::JsPropertyObjectMember(prop)) => {
                        if let Ok(value) = prop.value() {
                            extract_dependencies_from_expression(&value, deps);
                        }
                    }
                    Ok(AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand)) => {
                        // Handle object shorthand properties like { userId }
                        if let Ok(name) = shorthand.name() {
                            if let Ok(name_token) = name.value_token() {
                                deps.insert(name_token.text_trimmed().to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        AnyJsExpression::JsArrayExpression(arr) => {
            for element in arr.elements().iter() {
                if let Ok(element) = element {
                    if let Some(expr) = element.as_any_js_expression() {
                        extract_dependencies_from_expression(expr, deps);
                    }
                }
            }
        }
        // Other expressions - for now, we don't extract dependencies
        _ => {}
    }
}

// Find the function that is calling the hook
fn function_of_hook_call(call: &JsCallExpression) -> Option<JsSyntaxNode> {
    call.syntax().ancestors().find(|x| {
        matches!(
            x.kind(),
            JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        )
    })
}
