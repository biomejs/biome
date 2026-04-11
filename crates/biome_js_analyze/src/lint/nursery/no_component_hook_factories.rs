use crate::react::components::{
    AnyPotentialReactComponentDeclaration, ReactComponentInfo, is_react_component_name,
};
use crate::react::hooks::is_react_hook_name;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsFunction, AnyJsParameter, JsAssignmentExpression,
    JsCallExpression, JsFunctionExpression, JsInitializerClause, JsPropertyClassMember,
    JsPropertyObjectMember, JsSyntaxNode, JsSyntaxToken,
};
use biome_rowan::{AstNode, SyntaxResult, declare_node_union};
use biome_rule_options::no_component_hook_factories::NoComponentHookFactoriesOptions;

declare_lint_rule! {
    /// Disallows defining React components or custom hooks inside other functions.
    ///
    /// Defining components or hooks inside other functions creates new instances on every call.
    /// React treats each new instance as a completely different component, which destroys and
    /// recreates the entire component subtree on each render and causes all state to be lost.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// A component is defined inside a factory function:
    ///
    /// ```jsx,expect_diagnostic
    /// function makeComponent(label) {
    ///   function MyComponent() {
    ///     return <div>{label}</div>;
    ///   }
    ///   return MyComponent;
    /// }
    /// ```
    ///
    /// A hook is defined inside a factory function:
    ///
    /// ```jsx,expect_diagnostic
    /// function makeHook(key) {
    ///   function useMyHook() {
    ///     return useState(key);
    ///   }
    ///   return useMyHook;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Components and hooks defined at the module level:
    ///
    /// ```jsx
    /// function MyComponent() {
    ///   return <div>Hello</div>;
    /// }
    ///
    /// function useMyHook() {
    ///   return useState(0);
    /// }
    /// ```
    ///
    /// Higher-order components that receive a component as a parameter are allowed:
    ///
    /// ```jsx
    /// function withAuth(WrappedComponent) {
    ///   function AuthenticatedComponent(props) {
    ///     return <WrappedComponent {...props} />;
    ///   }
    ///   return AuthenticatedComponent;
    /// }
    /// ```
    ///
    pub NoComponentHookFactories {
        version: "next",
        name: "noComponentHookFactories",
        language: "jsx",
        sources: &[RuleSource::EslintReactHooks("component-hook-factories").same(), RuleSource::EslintReactX("component-hook-factories").same(), RuleSource::EslintReactXyz("component-hook-factories").same()],
        recommended: false,
        domains: &[RuleDomain::React],
        severity: Severity::Error,
    }
}

// Extends `AnyPotentialReactComponentDeclaration` with `JsFunctionExpression`
// to also catch named function expressions in non-declaration contexts
// such as `return function MyComponent() {}`.
declare_node_union! {
    pub AnyComponentLikeDeclaration = AnyPotentialReactComponentDeclaration | JsFunctionExpression
}

pub enum RuleState {
    Component(JsSyntaxToken),
    Hook(JsSyntaxToken),
}

impl Rule for NoComponentHookFactories {
    type Query = Ast<AnyComponentLikeDeclaration>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoComponentHookFactoriesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyComponentLikeDeclaration::AnyPotentialReactComponentDeclaration(decl) => {
                let syntax = decl.syntax();

                // Hooks are cheap to detect by name; check before the heavier component detection.
                if let Some(token) =
                    get_simple_binding_token(decl).filter(|t| is_react_hook_name(t.text_trimmed()))
                {
                    let parent_fn = find_parent_function(syntax)?;
                    if is_hoc_like(&parent_fn) || is_inside_test_mock_callback(syntax) {
                        return None;
                    }
                    return Some(RuleState::Hook(token));
                }

                let component_info = ReactComponentInfo::from_declaration(syntax)?;
                let name_token = component_info.name.clone()?;

                let parent_fn = find_parent_function(syntax)?;
                if is_hoc_like(&parent_fn) || is_inside_test_mock_callback(syntax) {
                    return None;
                }

                Some(RuleState::Component(name_token))
            }

            // Named function expressions in return/yield/call positions:
            // `return function MyComponent() {}` — not covered by AnyPotentialReactComponentDeclaration.
            AnyComponentLikeDeclaration::JsFunctionExpression(expr) => {
                let token = expr.id()?.as_js_identifier_binding()?.name_token().ok()?;
                let name = token.text_trimmed();

                let parent = expr.syntax().parent()?;
                if is_declarative_context(&parent) {
                    return None;
                }

                let is_hook = is_react_hook_name(name);
                if !is_hook && !is_react_component_name(name) {
                    return None;
                }

                let parent_fn = find_parent_function(expr.syntax())?;
                if is_hoc_like(&parent_fn) || is_inside_test_mock_callback(expr.syntax()) {
                    return None;
                }

                if is_hook {
                    Some(RuleState::Hook(token))
                } else {
                    Some(RuleState::Component(token))
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            RuleState::Component(token) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    token.text_trimmed_range(),
                    markup! {
                        "Do not define component "<Emphasis>{token.text_trimmed()}</Emphasis>" inside a function."
                    },
                )
                .note(markup! {
                    "React treats components defined inside functions as completely different on each call, destroying the entire subtree and losing all state. Move it to the module level."
                }),
            ),
            RuleState::Hook(token) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    token.text_trimmed_range(),
                    markup! {
                        "Do not define hook "<Emphasis>{token.text_trimmed()}</Emphasis>" inside a function."
                    },
                )
                .note(markup! {
                    "Each call creates a new hook identity, causing React to lose its state across renders. Move it to the module level."
                }),
            ),
        }
    }
}

/// Returns the identifier token of the declared name for simple function/variable declarations.
/// Used to detect hooks by naming convention before heavier component detection.
fn get_simple_binding_token(node: &AnyPotentialReactComponentDeclaration) -> Option<JsSyntaxToken> {
    match node {
        AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(decl) => decl
            .id()
            .ok()?
            .as_js_identifier_binding()?
            .name_token()
            .ok(),
        AnyPotentialReactComponentDeclaration::JsVariableDeclarator(decl) => decl
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok(),
        _ => None,
    }
}

/// Returns `true` if `parent` is a declarative context already handled by
/// `AnyPotentialReactComponentDeclaration` (variable initializer, assignment,
/// object/class property). Used to avoid double-reporting for named function
/// expressions like `const X = function Y() {}`.
fn is_declarative_context(parent: &JsSyntaxNode) -> bool {
    JsInitializerClause::can_cast(parent.kind())
        || JsAssignmentExpression::can_cast(parent.kind())
        || JsPropertyObjectMember::can_cast(parent.kind())
        || JsPropertyClassMember::can_cast(parent.kind())
}

fn find_parent_function(syntax: &JsSyntaxNode) -> Option<AnyJsFunction> {
    syntax.ancestors().skip(1).find_map(AnyJsFunction::cast)
}

/// Returns `true` if the node is nested inside a `vi.mock(...)` or `jest.mock(...)`
/// call, which are test utility calls where defining components inline is acceptable.
fn is_inside_test_mock_callback(syntax: &JsSyntaxNode) -> bool {
    syntax
        .ancestors()
        .skip(1)
        .any(|ancestor| is_test_mock_call(&ancestor).unwrap_or(false))
}

fn is_test_mock_call(ancestor: &JsSyntaxNode) -> Option<bool> {
    let call_expr = JsCallExpression::cast_ref(ancestor)?;
    let member = call_expr
        .callee()
        .ok()?
        .as_js_static_member_expression()?
        .clone();
    let method_token = member.member().ok()?.as_js_name()?.value_token().ok()?;
    if method_token.text_trimmed() != "mock" {
        return None;
    }
    let object_token = member
        .object()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()?;
    Some(matches!(object_token.text_trimmed(), "vi" | "jest"))
}

/// Heuristic for Higher-Order Component detection.
///
/// A function is treated as an HOC if any of its parameters has a PascalCase name,
/// which is the conventional way to signal that a function accepts a React component
/// (e.g. `withAuth(WrappedComponent)`).
fn is_hoc_like(function: &AnyJsFunction) -> bool {
    let Ok(params) = function.parameters() else {
        return false;
    };
    let items = match params {
        AnyJsArrowFunctionParameters::JsParameters(params) => params.items(),
        AnyJsArrowFunctionParameters::AnyJsBinding(_) => return false,
    };

    items.into_iter().any(|param| {
        is_pascal_case_param(param).unwrap_or(false)
    })
}

fn is_pascal_case_param(param: SyntaxResult<AnyJsParameter>) -> Option<bool> {
    let formal = param
        .ok()?
        .as_any_js_formal_parameter()?
        .as_js_formal_parameter()?
        .clone();
    let name_token = formal
        .binding()
        .ok()?
        .as_any_js_binding()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()?;
    Some(
        name_token
            .text_trimmed()
            .chars()
            .next()
            .is_some_and(char::is_uppercase),
    )
}
