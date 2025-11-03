use crate::{JsRuleAction, services::semantic::Semantic, utils::rename::RenameSymbolExtensions};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsCallExpression, JsIdentifierBinding,
    JsVariableDeclarator, binding_ext::AnyJsParameterParentFunction, function_ext::AnyFunctionLike,
};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};

declare_lint_rule! {
    /// Disallow function parameters that are only used in recursive calls.
    ///
    /// A parameter that is only passed to recursive calls is effectively unused
    /// and can be removed or replaced with a constant, simplifying the function.
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
    /// ```js,expect_diagnostic
    /// function fn(n, acc) {
    ///     if (n === 0) return 0;
    ///     return fn(n - 1, acc || 0);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Counter {
    ///     count(n, acc) {
    ///         if (n === 0) return 0;
    ///         return this?.count(n - 1, acc);
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
        version: "2.3.3",
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
                function_name.as_ref(),
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

fn get_function_name(parent_function: &AnyJsParameterParentFunction) -> Option<TokenText> {
    match parent_function {
        AnyJsParameterParentFunction::JsFunctionDeclaration(decl) => decl
            .id()
            .ok()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .and_then(|id| id.name_token().ok())
            .map(|t| t.token_text_trimmed()),
        AnyJsParameterParentFunction::JsFunctionExpression(expr) => expr
            .id()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .and_then(|id| id.name_token().ok())
            .map(|t| t.token_text_trimmed()),
        AnyJsParameterParentFunction::JsMethodClassMember(method) => method
            .name()
            .ok()
            .and_then(|name| name.as_js_literal_member_name().cloned())
            .and_then(|lit| lit.value().ok())
            .map(|t| t.token_text_trimmed()),
        AnyJsParameterParentFunction::JsMethodObjectMember(method) => method
            .name()
            .ok()
            .and_then(|name| name.as_js_literal_member_name().cloned())
            .and_then(|lit| lit.value().ok())
            .map(|t| t.token_text_trimmed()),
        // Arrow functions: extract name from surrounding variable declarator or assignment
        AnyJsParameterParentFunction::JsArrowFunctionExpression(arrow) => {
            get_arrow_function_name(arrow)
        }
        _ => None,
    }
}

/// Extracts the name of an arrow function from its surrounding context.
/// Handles cases like:
/// - `const foo = () => ...` (variable declarator)
/// - `foo = () => ...` (assignment expression)
///
/// Returns `None` for anonymous arrow functions that cannot be recursively called.
fn get_arrow_function_name(
    arrow_fn: &biome_js_syntax::JsArrowFunctionExpression,
) -> Option<TokenText> {
    let arrow_syntax = arrow_fn.syntax();

    // Walk up the syntax tree to find a variable declarator or assignment
    for ancestor in arrow_syntax.ancestors().skip(1) {
        // Check for variable declarator: const foo = () => ...
        if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor) {
            return declarator
                .id()
                .ok()?
                .as_any_js_binding()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()
                .map(|t| t.token_text_trimmed());
        }

        // Check for assignment expression: foo = () => ...
        if let Some(assignment) = JsAssignmentExpression::cast_ref(&ancestor) {
            return assignment
                .left()
                .ok()?
                .as_any_js_assignment()?
                .as_js_identifier_assignment()?
                .name_token()
                .ok()
                .map(|t| t.token_text_trimmed());
        }

        // Stop searching if we hit a function boundary
        // (prevents extracting wrong name from outer scope)
        if AnyFunctionLike::cast_ref(&ancestor).is_some() {
            break;
        }
    }

    None
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

fn is_recursive_call(call: &JsCallExpression, function_name: &TokenText) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };

    let expr = callee.omit_parentheses();

    // Simple identifier: foo()
    if let Some(ref_id) = expr.as_js_reference_identifier() {
        return ref_id
            .name()
            .ok()
            .is_some_and(|n| n.text() == function_name.text());
    }

    // Member expression: this.foo() or this?.foo()
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
                .is_some_and(|t| t.text_trimmed() == function_name.text())
        });

        return member_name_matches;
    }

    // Computed member expression: this["foo"]() or this?.["foo"]()
    if let Some(computed) = expr.as_js_computed_member_expression() {
        // Check if object is 'this' (for method calls)
        let is_this_call = computed
            .object()
            .ok()
            .is_some_and(|obj| obj.as_js_this_expression().is_some());

        if !is_this_call {
            return false;
        }

        // Conservative approach: only handle string literal members
        if let Ok(member_expr) = computed.member()
            && let Some(lit) = member_expr.as_any_js_literal_expression()
            && let Some(string_lit) = lit.as_js_string_literal_expression()
            && let Ok(text) = string_lit.inner_string_text()
        {
            return text.text() == function_name.text();
        }

        return false;
    }

    false
}

fn is_reference_in_recursive_call(
    reference: &Reference,
    function_name: Option<&TokenText>,
    parent_function: &AnyJsParameterParentFunction,
    param_name: &str,
) -> bool {
    // Early return if no function name (cannot be recursive)
    let Some(name) = function_name else {
        return false;
    };

    let ref_node = reference.syntax();

    // Walk up the tree to find if we're inside a call expression
    let mut current = ref_node.parent();
    while let Some(node) = current {
        // Check if this is a call expression
        if let Some(call_expr) = JsCallExpression::cast_ref(&node) {
            // Check if this call is recursive AND uses our parameter
            if is_recursive_call_with_param_usage(&call_expr, name, param_name) {
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
    // Direct syntax node comparison
    node == parent_function.syntax()
}

/// Checks if an expression traces back to a specific parameter
/// through "safe" operations (arithmetic, unary, field access)
///
/// Uses an iterative approach with a worklist to avoid stack overflow
/// on deeply nested expressions.
fn traces_to_parameter(expr: &AnyJsExpression, param_name: &str) -> bool {
    // Worklist of expressions to examine
    let mut to_check = vec![expr.clone()];

    while let Some(current_expr) = to_check.pop() {
        // Omit parentheses
        let current_expr = current_expr.omit_parentheses();

        if let Some(ref_id) = current_expr.as_js_reference_identifier() {
            if ref_id.name().ok().is_some_and(|n| n.text() == param_name) {
                // Found direct parameter reference
                return true;
            }
            continue;
        }

        // Binary operations: a + 1, a - b
        // Add both sides to worklist
        if let Some(bin_expr) = current_expr.as_js_binary_expression() {
            if let Ok(left) = bin_expr.left() {
                to_check.push(left);
            }
            if let Ok(right) = bin_expr.right() {
                to_check.push(right);
            }
            continue;
        }

        // Logical operations: a && b, a || b, a ?? b
        // Add both sides to worklist
        if let Some(logical_expr) = current_expr.as_js_logical_expression() {
            if let Ok(left) = logical_expr.left() {
                to_check.push(left);
            }
            if let Ok(right) = logical_expr.right() {
                to_check.push(right);
            }
            continue;
        }

        // Conditional expression: cond ? a : b
        // Add all three parts to worklist (test, consequent, alternate)
        if let Some(cond_expr) = current_expr.as_js_conditional_expression() {
            if let Ok(test) = cond_expr.test() {
                to_check.push(test);
            }
            if let Ok(consequent) = cond_expr.consequent() {
                to_check.push(consequent);
            }
            if let Ok(alternate) = cond_expr.alternate() {
                to_check.push(alternate);
            }
            continue;
        }

        // Unary operations: -a, !flag
        // Add argument to worklist
        if let Some(unary_expr) = current_expr.as_js_unary_expression() {
            if let Ok(arg) = unary_expr.argument() {
                to_check.push(arg);
            }
            continue;
        }

        // Static member access: obj.field
        // Add object to worklist
        if let Some(member_expr) = current_expr.as_js_static_member_expression()
            && let Ok(obj) = member_expr.object()
        {
            to_check.push(obj);
        }

        // Any other expression - not safe to trace
        // Just continue to next item in worklist
    }

    // Didn't find the parameter anywhere
    false
}

/// Enhanced version that checks if any argument traces to parameters
fn is_recursive_call_with_param_usage(
    call: &JsCallExpression,
    function_name: &TokenText,
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
