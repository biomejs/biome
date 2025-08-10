use biome_analyze::{
    QueryMatch, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    JsClassExpression, JsFunctionExpression, JsIdentifierBinding, JsParameterList,
    JsVariableDeclarator, TsIdentifierBinding, TsPropertySignatureTypeMember,
    TsTypeAliasDeclaration, TsTypeParameter, TsTypeParameterName,
};
use biome_rowan::{AstNode, SyntaxNodeCast, TokenText, declare_node_union};
use biome_rule_options::no_shadow::NoShadowOptions;

use crate::services::semantic::SemanticServices;

declare_lint_rule! {
    /// Disallow variable declarations from shadowing variables declared in the outer scope.
    ///
    /// Shadowing is the process by which a local variable shares the same name as a variable in its containing scope. This can cause confusion while reading the code and make it impossible to access the global variable.
    ///
    /// See also: [`noShadowRestrictedNames`](http://biomejs.dev/linter/rules/no-shadow-restricted-names)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = "bar";
    /// if (true) {
    ///    const foo = "baz";
    /// }
    /// ```
    ///
    /// Variable declarations in functions can shadow variables in the outer scope:
    ///
    /// ```js,expect_diagnostic
    /// const foo = "bar";
    /// const bar = function () {
    ///     const foo = 10;
    /// }
    /// ```
    ///
    /// Function argument names can shadow variables in the outer scope:
    ///
    /// ```js,expect_diagnostic
    /// const foo = "bar";
    /// function bar(foo) {
    ///     foo = 10;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = "bar";
    /// if (true) {
    ///    const qux = "baz";
    /// }
    /// ```
    ///
    pub NoShadow {
        version: "2.0.0",
        name: "noShadow",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::Eslint("no-shadow").same(),
            // uncomment when we can handle the test cases from typescript-eslint
            // RuleSource::EslintTypeScript("no-shadow"),
        ],
    }
}

pub struct ShadowedBinding {
    /// The binding that is violating the rule.
    binding: Binding,
    /// The binding that is shadowed.
    shadowed_binding: Binding,
}

impl Rule for NoShadow {
    type Query = SemanticServices;
    type State = ShadowedBinding;
    type Signals = Box<[Self::State]>;
    type Options = NoShadowOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut shadowed_bindings = Vec::new();
        let model = ctx.query();

        for binding in ctx.query().all_bindings() {
            if let Some(shadowed_binding) = check_shadowing(model, binding) {
                shadowed_bindings.push(shadowed_binding);
            }
        }

        shadowed_bindings.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.binding.tree().range(),
                markup! {
                    "This variable shadows another variable with the same name in the outer scope."
                },
            )
            .detail(
                state.shadowed_binding.tree().range(),
                markup!(
                    "This is the shadowed variable, which is now inaccessible in the inner scope."
                ),
            )
            .note(markup! {
                "Consider renaming this variable. It's easy to confuse the origin of variables if they share the same name."
            }),
        )
    }
}

fn check_shadowing(model: &SemanticModel, binding: Binding) -> Option<ShadowedBinding> {
    if binding.scope().is_global_scope() {
        // global scope bindings can't shadow anything
        return None;
    }

    let name = get_binding_name(&binding)?;
    let binding_hoisted_scope = model
        .scope_hoisted_to(binding.syntax())
        .unwrap_or(binding.scope());

    for upper in binding_hoisted_scope.ancestors().skip(1) {
        if let Some(upper_binding) = upper.get_binding(name.clone())
            && evaluate_shadowing(model, &binding, &upper_binding)
        {
            // we found a shadowed binding
            return Some(ShadowedBinding {
                binding,
                shadowed_binding: upper_binding,
            });
        }
    }
    None
}

fn evaluate_shadowing(model: &SemanticModel, binding: &Binding, upper_binding: &Binding) -> bool {
    if binding.syntax() == upper_binding.syntax() {
        // a binding can't shadow itself
        return false;
    }
    if is_on_initializer(binding, upper_binding) {
        return false;
    }
    if is_declaration(binding) && is_declaration(upper_binding) {
        let binding_hoisted_scope = model
            .scope_hoisted_to(binding.syntax())
            .unwrap_or(binding.scope());
        let upper_binding_hoisted_scope = model
            .scope_hoisted_to(upper_binding.syntax())
            .unwrap_or(upper_binding.scope());
        if binding_hoisted_scope == upper_binding_hoisted_scope {
            // redeclarations are not shadowing, they get caught by `noRedeclare`
            return false;
        }
        if upper_binding.syntax().text_range().start() >= binding_hoisted_scope.range().end() {
            // the shadowed binding must be declared before the shadowing one
            return false;
        }
    } else if is_inside_function_parameters(binding)
        && (is_inside_type_parameter(binding) || is_inside_type_member(binding))
    {
        return false;
    }
    true
}

fn get_binding_name(binding: &Binding) -> Option<TokenText> {
    let node = binding.syntax();
    if let Some(ident) = node.clone().cast::<JsIdentifierBinding>() {
        let name = ident.name_token().ok()?;
        return Some(name.token_text_trimmed());
    }
    if let Some(ident) = node.clone().cast::<TsIdentifierBinding>() {
        let name = ident.name_token().ok()?;
        return Some(name.token_text_trimmed());
    }
    if let Some(ident) = node.clone().cast::<TsTypeParameterName>() {
        let name = ident.ident_token().ok()?;
        return Some(name.token_text_trimmed());
    }
    None
}

declare_node_union! {
    pub(crate) AnyIdentifiableExpression = JsFunctionExpression | JsClassExpression
}

/// Checks if a variable `a` is inside the initializer of variable `b`.
///
/// This is used to avoid false positives in cases like this:
/// ```js
/// const c = function c() {}
/// ```
///
/// But the rule should still trigger on these cases:
/// ```js
/// var a = function(a) {};
/// ```
///
/// ```js
/// var a = function() { function a() {} };
/// ```
fn is_on_initializer(a: &Binding, b: &Binding) -> bool {
    if let Some(b_initializer_expression) = b
        .tree()
        .parent::<JsVariableDeclarator>()
        .and_then(|d| d.initializer())
        .and_then(|i| i.expression().ok())
        && let Some(a_parent) = a.tree().parent::<AnyIdentifiableExpression>()
        && a_parent.syntax() == b_initializer_expression.syntax()
    {
        return true;
    }

    false
}

/// Whether the binding is a declaration or not.
///
/// Examples of declarations:
/// ```js
/// var a;
/// let b;
/// const c;
/// ```
fn is_declaration(binding: &Binding) -> bool {
    binding.tree().parent::<JsVariableDeclarator>().is_some()
        || binding.tree().parent::<TsTypeAliasDeclaration>().is_some()
}

fn is_inside_type_parameter(binding: &Binding) -> bool {
    binding
        .syntax()
        .ancestors()
        .any(|ancestor| ancestor.cast::<TsTypeParameter>().is_some())
}

fn is_inside_type_member(binding: &Binding) -> bool {
    binding
        .syntax()
        .ancestors()
        .any(|ancestor| ancestor.cast::<TsPropertySignatureTypeMember>().is_some())
}

fn is_inside_function_parameters(binding: &Binding) -> bool {
    binding
        .syntax()
        .ancestors()
        .any(|ancestor| ancestor.cast::<JsParameterList>().is_some())
}
