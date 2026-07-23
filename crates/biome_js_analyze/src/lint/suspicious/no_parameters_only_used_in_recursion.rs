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
use biome_rule_options::no_parameters_only_used_in_recursion::NoParametersOnlyUsedInRecursionOptions;

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
    type Options = NoParametersOnlyUsedInRecursionOptions;

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
        let function_name = get_function_name(&parent_function)?;

        // Get function binding for semantic comparison
        let parent_function_binding = get_function_binding(&parent_function, model);

        // Classify references
        let mut refs_in_recursion = 0;
        let mut refs_elsewhere = 0;

        for reference in binding.all_references(model) {
            if is_reference_in_recursive_call(
                &reference,
                &function_name,
                &parent_function,
                name_text,
                model,
                parent_function_binding.as_ref(),
            )
            .unwrap_or_default()
            {
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

/// Gets the binding for a function declaration/expression, if available
fn get_function_binding(
    parent_function: &AnyJsParameterParentFunction,
    model: &biome_js_semantic::SemanticModel,
) -> Option<biome_js_semantic::Binding> {
    match parent_function {
        AnyJsParameterParentFunction::JsFunctionDeclaration(decl) => decl
            .id()
            .ok()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .map(|id| model.as_binding(&id)),
        AnyJsParameterParentFunction::JsFunctionExpression(expr) => expr
            .id()
            .and_then(|any_binding| any_binding.as_js_identifier_binding().cloned())
            .map(|id| model.as_binding(&id)),
        AnyJsParameterParentFunction::JsArrowFunctionExpression(arrow) => {
            // For arrow functions, find the binding from the surrounding context
            let arrow_syntax = arrow.syntax();
            for ancestor in arrow_syntax.ancestors().skip(1) {
                // Check for variable declarator: const foo = () => ...
                if let Some(declarator) = JsVariableDeclarator::cast_ref(&ancestor)
                    && let Ok(id) = declarator.id()
                    && let Some(any_binding) = id.as_any_js_binding()
                    && let Some(js_id_binding) = any_binding.as_js_identifier_binding()
                {
                    return Some(model.as_binding(js_id_binding));
                }

                // Check for assignment expression: foo = () => ...
                if let Some(assignment) = JsAssignmentExpression::cast_ref(&ancestor)
                    && let Ok(left) = assignment.left()
                    && let Some(id_assignment) = left.as_any_js_assignment()
                    && let Some(js_id_assignment) = id_assignment.as_js_identifier_assignment()
                {
                    // Resolve assignment target to its binding
                    return model.binding(js_id_assignment);
                }

                if AnyFunctionLike::can_cast(ancestor.kind()) {
                    break;
                }
            }
            None
        }
        // Methods are property names, not bindings - use name-based comparison
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
        if AnyFunctionLike::can_cast(ancestor.kind()) {
            break;
        }
    }

    None
}

/// Returns true if the function is a TypeScript signature without an implementation body.
/// Matches interface method signatures, call signatures, function types, and declared functions.
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

/// Checks if a call expression is a recursive call to the current function.
/// Handles direct calls (`foo()`), method calls (`this.foo()`), and computed members (`this["foo"]()`).
/// Uses a conservative approach to avoid false positives.
fn is_recursive_call(
    call: &JsCallExpression,
    function_name: Option<&TokenText>,
    model: &biome_js_semantic::SemanticModel,
    parent_function_binding: Option<&biome_js_semantic::Binding>,
) -> Option<bool> {
    let callee = call.callee().ok()?;

    let expr = callee.omit_parentheses();

    // Simple identifier: foo()
    if let Some(ref_id) = expr.as_js_reference_identifier() {
        let name = ref_id.value_token().ok()?;
        let name_matches = name.token_text_trimmed() == *function_name?;
        if !name_matches {
            return Some(false);
        }

        let called_binding = model.binding(&ref_id);

        match (parent_function_binding, called_binding) {
            // Both have bindings - compare them directly
            (Some(parent_binding), Some(called_binding)) => {
                return Some(called_binding == *parent_binding);
            }
            // Parent has no binding (e.g. in the case of a method),
            // but call resolves to a binding
            (None, Some(_)) => {
                return Some(false);
            }
            // Parent has binding but call doesn't resolve
            (Some(_), None) => {
                return Some(false);
            }
            // Neither has a binding. Fall back to name comparison
            (None, None) => {
                return Some(name_matches);
            }
        }
    }

    // Member expression: this.foo() or this?.foo()
    if let Some(member) = expr.as_js_static_member_expression() {
        // Check if object is 'this' (for method calls)
        let object = member.object().ok()?;
        if object.as_js_this_expression().is_none() {
            return Some(false);
        }

        // Check if member name matches function name
        let member_node = member.member().ok()?;
        let name = member_node.as_js_name()?;
        let token = name.value_token().ok()?;
        return Some(token.token_text_trimmed() == *function_name?);
    }

    // Computed member expression: this["foo"]() or this?.["foo"]()
    if let Some(computed) = expr.as_js_computed_member_expression() {
        // Check if object is 'this' (for method calls)
        let object = computed.object().ok()?;
        if object.as_js_this_expression().is_none() {
            return Some(false);
        }

        // Conservative approach: only handle string literal members
        let member_expr = computed.member().ok()?;
        let lit = member_expr.as_any_js_literal_expression()?;
        let string_lit = lit.as_js_string_literal_expression()?;
        let text = string_lit.inner_string_text().ok()?;
        return Some(text == *function_name?);
    }

    Some(false)
}

/// Checks if a parameter reference occurs within a recursive call expression.
/// Walks up the syntax tree from the reference to find a recursive call that uses the parameter,
/// stopping at the function boundary.
fn is_reference_in_recursive_call(
    reference: &Reference,
    function_name: &TokenText,
    parent_function: &AnyJsParameterParentFunction,
    param_name: &str,
    model: &biome_js_semantic::SemanticModel,
    parent_function_binding: Option<&biome_js_semantic::Binding>,
) -> Option<bool> {
    let ref_node = reference.syntax();

    // Walk up the tree to find if we're inside a call expression
    let mut current = ref_node.parent();
    while let Some(node) = current {
        // Check if this is a call expression
        if let Some(call_expr) = JsCallExpression::cast_ref(&node) {
            // Check if this call is recursive AND uses our parameter
            if let Some(true) = is_recursive_call_with_param_usage(
                &call_expr,
                function_name,
                param_name,
                model,
                parent_function_binding,
            ) {
                return Some(true);
            }
        }

        // Stop if we reach the function boundary
        if is_function_boundary(&node, parent_function) {
            break;
        }

        current = node.parent();
    }

    Some(false)
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
fn traces_to_parameter(expr: &AnyJsExpression, param_name: &str) -> Option<bool> {
    // Worklist of expressions to examine
    let mut to_check = vec![expr.clone()];

    while let Some(current_expr) = to_check.pop() {
        // Omit parentheses
        let current_expr = current_expr.omit_parentheses();

        if let Some(ref_id) = current_expr.as_js_reference_identifier() {
            if ref_id.name().ok()?.text() == param_name {
                // Found direct parameter reference
                return Some(true);
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
            if let Ok(argument) = unary_expr.argument() {
                to_check.push(argument);
            }
            continue;
        }

        // Static member access: obj.field
        // Add object to worklist
        if let Some(member_expr) = current_expr.as_js_static_member_expression()
            && let Ok(object) = member_expr.object()
        {
            to_check.push(object);
        }

        // Any other expression - not safe to trace
        // Just continue to next item in worklist
    }

    // Didn't find the parameter anywhere
    Some(false)
}

/// Checks if a recursive call uses a specific parameter in its arguments.
/// Examines each argument to see if it traces back to the parameter through transformations
/// like arithmetic operations, unary operations, or member access.
fn is_recursive_call_with_param_usage(
    call: &JsCallExpression,
    function_name: &TokenText,
    param_name: &str,
    model: &biome_js_semantic::SemanticModel,
    parent_function_binding: Option<&biome_js_semantic::Binding>,
) -> Option<bool> {
    // First check if this is a recursive call at all
    if !is_recursive_call(call, Some(function_name), model, parent_function_binding)? {
        return Some(false);
    }

    // Check if any argument uses the parameter
    let arguments = call.arguments().ok()?;

    for arg in arguments.args() {
        let Some(arg_node) = arg.ok() else {
            continue;
        };

        // Skip spread arguments (conservative)
        if arg_node.as_js_spread().is_some() {
            continue;
        }

        // Check if argument expression uses the parameter
        if let Some(expr) = arg_node.as_any_js_expression()
            && traces_to_parameter(expr, param_name)?
        {
            return Some(true);
        }
    }

    Some(false)
}
