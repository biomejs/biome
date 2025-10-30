use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{binding_ext::AnyJsParameterParentFunction, JsCallExpression, JsIdentifierBinding};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow function parameters that are only used in recursive calls.
    ///
    /// A parameter that is only passed to recursive calls is effectively unused
    /// and can be removed, simplifying the function signature.
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
    /// ### Valid
    ///
    /// ```js
    /// function factorial(n, acc) {
    ///     console.log(acc);
    ///     if (n === 0) return acc;
    ///     return factorial(n - 1, acc * n);
    /// }
    /// ```
    pub NoParametersOnlyUsedInRecursion {
        version: "next",
        name: "noParametersOnlyUsedInRecursion",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
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
            if is_reference_in_recursive_call(&reference, function_name.as_deref(), &parent_function)
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
                    "This parameter is only used in recursive calls."
                },
            )
            .note(markup! {
                "Parameters only used in recursion can be removed to simplify the function."
            }),
        )
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

    // For now, handle simple identifier calls: foo()
    let Some(identifier) = callee.omit_parentheses().as_js_reference_identifier() else {
        return false;
    };

    // Simple name comparison
    identifier
        .name()
        .ok()
        .map(|n| n.text() == name)
        .unwrap_or(false)
}

fn is_reference_in_recursive_call(
    reference: &biome_js_semantic::Reference,
    function_name: Option<&str>,
    parent_function: &AnyJsParameterParentFunction,
) -> bool {
    let ref_node = reference.syntax();

    // Walk up the tree to find if we're inside a call expression
    let mut current = ref_node.parent();
    while let Some(node) = current {
        // Check if this is a call expression
        if let Some(call_expr) = JsCallExpression::cast_ref(&node) {
            // Check if this call is recursive
            if is_recursive_call(&call_expr, function_name) {
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
