use crate::{JsRuleAction, services::semantic::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsIdentifierBinding,
    binding_ext::AnyJsParameterParentFunction,
};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow function parameters that are only used in recursive calls.
    ///
    /// A parameter that is only passed to recursive calls is effectively unused
    /// and can be removed or replaced with a constant, simplifying the function.
    ///
    /// This rule is inspired by Rust Clippy's `only_used_in_recursion` lint.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function factorial(n, acc) {
    ///     if (n === 0) return 1;
    ///     return factorial(n - 1, acc);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function countdown(n, step) {
    ///     if (n === 0) return 0;
    ///     return countdown(n - step, step);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Counter {
    ///     count(n, acc) {
    ///         if (n === 0) return 0;
    ///         return this.count(n - 1, acc);
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function factorial(n, acc) {
    ///     if (n === 0) return acc;
    ///     return factorial(n - 1, acc * n);
    /// }
    /// ```
    ///
    /// ```js
    /// function countdown(n, step) {
    ///     console.log(step);
    ///     if (n === 0) return 0;
    ///     return countdown(n - step, step);
    /// }
    /// ```
    ///
    /// ```js
    /// function fn(n, threshold) {
    ///     if (n > threshold) return n;
    ///     return fn(n + 1, threshold);
    /// }
    /// ```
    pub NoParametersOnlyUsedInRecursion {
        version: "next",
        name: "noParametersOnlyUsedInRecursion",
        language: "js",
        sources: &[RuleSource::Clippy("only_used_in_recursion").inspired()],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoParametersOnlyUsedInRecursion {
    type Query = Semantic<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let model = ctx.model();

        // Get parameter name
        let name = binding.name_token().ok()?;
        let name_text = name.text_trimmed();

        // Skip parameters starting with underscore (intentionally unused)
        if name_text.starts_with('_') {
            return None;
        }

        // Check if this is a function parameter
        let parent_function = get_parent_function(binding)?;

        // Skip TypeScript signatures (same logic as noUnusedFunctionParameters)
        if is_function_signature(&parent_function) {
            return None;
        }

        // Get function name for recursion detection
        let function_name = get_function_name(&parent_function);

        // Get all references to this parameter
        let all_refs: Vec<_> = binding.all_references(model).collect();

        // If no references, let noUnusedFunctionParameters handle it
        if all_refs.is_empty() {
            return None;
        }

        // Classify references
        let mut refs_in_recursion = 0;
        let mut refs_elsewhere = 0;

        for reference in all_refs {
            if is_reference_in_recursive_call(
                &reference,
                function_name.as_deref(),
                &parent_function,
                name_text,
            ) {
                refs_in_recursion += 1;
            } else {
                refs_elsewhere += 1;
            }
        }

        // Only report if ALL references are in recursive calls
        if refs_in_recursion > 0 && refs_elsewhere == 0 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                binding.range(),
                markup! {
                    "This "<Emphasis>"parameter"</Emphasis>" is only used in recursive calls."
                },
            )
            .note(markup! {
                "Parameters that are only used in recursive calls are effectively unused and can be removed."
            })
            .note(markup! {
                "If the parameter is needed for the recursion to work, consider if the function can be refactored to avoid it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let binding = ctx.query();
        let model = ctx.model();
        let mut mutation = ctx.root().begin();

        let name = binding.name_token().ok()?;
        let name_trimmed = name.text_trimmed();
        let new_name = format!("_{name_trimmed}");

        // Rename the parameter and all its references
        if !mutation.rename_node_declaration(model, binding, &new_name) {
            return None;
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "If this is intentional, prepend "<Emphasis>{name_trimmed}</Emphasis>" with an underscore."
            }
            .to_owned(),
            mutation,
        ))
    }
}

fn get_parent_function(binding: &JsIdentifierBinding) -> Option<AnyJsParameterParentFunction> {
    let declaration = binding.declaration()?;
    match declaration {
        biome_js_syntax::binding_ext::AnyJsBindingDeclaration::JsFormalParameter(param) => {
            param.parent_function()
        }
        biome_js_syntax::binding_ext::AnyJsBindingDeclaration::JsRestParameter(param) => {
            param.parent_function()
        }
        _ => None,
    }
}

fn get_function_name(parent_function: &AnyJsParameterParentFunction) -> Option<String> {
    match parent_function {
        AnyJsParameterParentFunction::JsFunctionDeclaration(decl) => decl
            .id()
            .ok()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .and_then(|id| id.name_token().ok())
            .map(|t| t.text_trimmed().to_string()),
        AnyJsParameterParentFunction::JsFunctionExpression(expr) => expr
            .id()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .and_then(|id| id.name_token().ok())
            .map(|t| t.text_trimmed().to_string()),
        AnyJsParameterParentFunction::JsMethodClassMember(method) => method
            .name()
            .ok()
            .and_then(|name| name.as_js_literal_member_name().cloned())
            .and_then(|lit| lit.value().ok())
            .map(|t| t.text_trimmed().to_string()),
        AnyJsParameterParentFunction::JsMethodObjectMember(method) => method
            .name()
            .ok()
            .and_then(|name| name.as_js_literal_member_name().cloned())
            .and_then(|lit| lit.value().ok())
            .map(|t| t.text_trimmed().to_string()),
        // Arrow functions don't have names, use None
        AnyJsParameterParentFunction::JsArrowFunctionExpression(_) => None,
        _ => None,
    }
}

fn is_function_signature(parent_function: &AnyJsParameterParentFunction) -> bool {
    matches!(
        parent_function,
        AnyJsParameterParentFunction::TsMethodSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsCallSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsConstructSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsConstructorSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsMethodSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsSetterSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsSetterSignatureTypeMember(_)
            | AnyJsParameterParentFunction::TsIndexSignatureClassMember(_)
            | AnyJsParameterParentFunction::TsFunctionType(_)
            | AnyJsParameterParentFunction::TsConstructorType(_)
            | AnyJsParameterParentFunction::TsDeclareFunctionDeclaration(_)
            | AnyJsParameterParentFunction::TsDeclareFunctionExportDefaultDeclaration(_)
    )
}

fn is_recursive_call(call: &JsCallExpression, function_name: Option<&str>) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };

    let Some(name) = function_name else {
        return false;
    };

    let expr = callee.omit_parentheses();

    // Simple identifier: foo()
    if let Some(ref_id) = expr.as_js_reference_identifier() {
        return ref_id.name().ok().is_some_and(|n| n.text() == name);
    }

    // Member expression: this.foo() or obj.foo()
    if let Some(member) = expr.as_js_static_member_expression() {
        // Check if object is 'this' (for method calls)
        let is_this_call = member
            .object()
            .ok()
            .is_some_and(|obj| obj.as_js_this_expression().is_some());

        if !is_this_call {
            return false;
        }

        // Check if member name matches function name
        let member_name_matches = member.member().ok().is_some_and(|m| {
            m.as_js_name()
                .and_then(|n| n.value_token().ok())
                .is_some_and(|t| t.text_trimmed() == name)
        });

        return member_name_matches;
    }

    false
}

fn is_reference_in_recursive_call(
    reference: &biome_js_semantic::Reference,
    function_name: Option<&str>,
    parent_function: &AnyJsParameterParentFunction,
    param_name: &str,
) -> bool {
    let ref_node = reference.syntax();

    // Walk up the tree to find if we're inside a call expression
    let mut current = ref_node.parent();
    while let Some(node) = current {
        // Check if this is a call expression
        if let Some(call_expr) = JsCallExpression::cast_ref(&node) {
            // Check if this call is recursive AND uses our parameter
            if is_recursive_call_with_param_usage(&call_expr, function_name, param_name) {
                return true;
            }
        }

        // Stop if we reach the function boundary
        if is_function_boundary(&node, parent_function) {
            break;
        }

        current = node.parent();
    }

    false
}

fn is_function_boundary(
    node: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
    parent_function: &AnyJsParameterParentFunction,
) -> bool {
    // Check if this node matches our parent function by comparing text ranges
    node.text_trimmed_range() == parent_function.syntax().text_trimmed_range()
}

/// Checks if an expression traces back to a specific parameter
/// through "safe" operations (arithmetic, unary, field access)
fn traces_to_parameter(expr: &AnyJsExpression, param_name: &str) -> bool {
    // Direct parameter reference
    if let Some(ref_id) = expr.as_js_reference_identifier() {
        return ref_id.name().ok().is_some_and(|n| n.text() == param_name);
    }

    // Binary operations: a + 1, a - b
    if let Some(bin_expr) = expr.as_js_binary_expression() {
        let left = bin_expr.left().ok();
        let right = bin_expr.right().ok();

        return left.is_some_and(|l| traces_to_parameter(&l, param_name))
            || right.is_some_and(|r| traces_to_parameter(&r, param_name));
    }

    // Unary operations: -a, !flag
    if let Some(unary_expr) = expr.as_js_unary_expression() {
        return unary_expr
            .argument()
            .ok()
            .is_some_and(|arg| traces_to_parameter(&arg, param_name));
    }

    // Parenthesized: (a + 1)
    if let Some(paren_expr) = expr.as_js_parenthesized_expression() {
        return paren_expr
            .expression()
            .ok()
            .is_some_and(|e| traces_to_parameter(&e, param_name));
    }

    // Static member access: obj.field (conservative: only trace if primitive)
    if let Some(member_expr) = expr.as_js_static_member_expression() {
        return member_expr
            .object()
            .ok()
            .is_some_and(|obj| traces_to_parameter(&obj, param_name));
    }

    // Any other expression (function calls, etc.) - not safe to trace
    false
}

/// Enhanced version that checks if any argument traces to parameters
fn is_recursive_call_with_param_usage(
    call: &JsCallExpression,
    function_name: Option<&str>,
    param_name: &str,
) -> bool {
    // First check if this is a recursive call at all
    if !is_recursive_call(call, function_name) {
        return false;
    }

    // Check if any argument uses the parameter
    let Ok(arguments) = call.arguments() else {
        return false;
    };

    for arg in arguments.args() {
        let Ok(arg_node) = arg else { continue };

        // Skip spread arguments (conservative)
        if arg_node.as_js_spread().is_some() {
            continue;
        }

        // Check if argument expression uses the parameter
        if let Some(expr) = arg_node.as_any_js_expression()
            && traces_to_parameter(expr, param_name)
        {
            return true;
        }
    }

    false
}
