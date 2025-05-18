use crate::react::hooks::is_react_component;
use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsFunction, JsAssignmentExpression,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsInitializerClause, JsSyntaxToken,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, declare_node_union};

declare_lint_rule! {
    /// Disallows defining React components inside other components.
    ///
    /// Component definitions inside other components cause them to be recreated on every render,
    /// which can lead to performance issues and unexpected behavior.
    ///
    /// When a component is defined inside another component:
    /// - It gets recreated on every render of the parent component
    /// - It loses its internal state when the parent rerenders
    /// - It defeats props memoization and optimization techniques
    /// - It creates new function references on every render
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// A new component is created every time ParentComponent renders:
    /// ```jsx,expect_diagnostic
    /// function ParentComponent() {
    ///   function ChildComponent() {
    ///     return <div>Hello</div>;
    ///   }
    ///
    ///   return <ChildComponent />;
    /// }
    /// ```
    ///
    /// Even with memo, a new component is still created on each render:
    /// ```jsx,expect_diagnostic
    /// function ParentComponent() {
    ///   const MemoizedChild = memo(() => {
    ///     return <div>Hello</div>;
    ///   });
    ///
    ///   return <MemoizedChild />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// Component is defined outside other components:
    /// ```jsx
    /// function ChildComponent() {
    ///   return <div>Hello</div>;
    /// }
    ///
    /// function ParentComponent() {
    ///   return <ChildComponent />;
    /// }
    /// ```
    ///
    /// ## Correct approaches
    ///
    /// 1. Move the component definition outside:
    ///    ```jsx
    ///    function ChildComponent() {
    ///      return <div>Hello</div>;
    ///    }
    ///
    ///    function ParentComponent() {
    ///      return <ChildComponent />;
    ///    }
    ///    ```
    ///
    /// 2. Pass components as props:
    ///    ```jsx
    ///    function ParentComponent({ CustomComponent }) {
    ///      return <CustomComponent />;
    ///    }
    ///    ```
    ///
    /// 3. Use React's Children API:
    ///    ```jsx
    ///    function ParentComponent({ children }) {
    ///      return <div>{children}</div>;
    ///    }
    ///    ```
    pub NoNestedComponentDefinitions {
        version: "2.0.0",
        name: "noNestedComponentDefinitions",
        language: "jsx",
        sources: &[RuleSource::EslintReactXyz("no-nested-components")],
        recommended: false,
        domains: &[RuleDomain::React],
        severity: Severity::Error,
    }
}

impl Rule for NoNestedComponentDefinitions {
    type Query = Ast<AnyJsFunction>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if let Some(component_name) = get_function_component_info(node) {
            if let Some(parent_component_name) = node
                .syntax()
                .ancestors()
                .skip_while(|ancestor| ancestor.eq(node.syntax()))
                .find_map(|ancestor| {
                    AnyJsFunction::cast(ancestor).and_then(|n| get_function_component_info(&n))
                })
            {
                return Some(RuleState {
                    component_name,
                    parent_component_name,
                });
            }
        };
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.component_name.text_range(),
                markup! {
                    "Components should not be defined inside other components."
                },
            )
            .detail(
                state.parent_component_name.text_range(),
                markup! {
                    "Move it outside of the parent component or pass it as a prop."
                },
            )
            .note(markup! {
                "Component definitions inside other components cause them to be recreated on every render, which can lead to performance issues and unexpected behavior."
            }),
        )
    }
}

pub struct RuleState {
    /// The name of the component.
    pub component_name: JsSyntaxToken,
    /// The parent component name.
    pub parent_component_name: JsSyntaxToken,
}

// React built-in wrappers that are used to create components.
const REACT_HOOKS: [&str; 2] = ["memo", "forwardRef"];

/// Returns React component name for a function if it is a component.
/// This relies on function naming and usage of React wrappers.
/// It might be required in the future to use a more sophisticated approach,
/// such as analyzing function return values.
fn get_function_component_info(func: &AnyJsFunction) -> Option<JsSyntaxToken> {
    // Extracting function name if possible. For function expressions the name can later
    // be extracted from the assignment.
    let mut name: Option<JsSyntaxToken> = func.id().ok().and_then(|id| id).and_then(|id| {
        if let AnyJsBinding::JsIdentifierBinding(id) = id {
            id.name_token().ok()
        } else {
            None
        }
    });
    let mut max_parameter_count = Some(1);
    let is_expression = matches!(
        func,
        AnyJsFunction::JsFunctionExpression(_) | AnyJsFunction::JsArrowFunctionExpression(_)
    );

    // If this is a function expression, it should be either directly assigned to a variable:
    //   const MyComponent = () => {};
    // or assigned to a variable after being wrapped in one of the wrappers:
    //   const MyComponent = memo(() => {});
    if is_expression {
        let FunctionExpressionInfo {
            assignment,
            is_wrapped,
        } = get_function_expression_info(func)?;

        if is_wrapped {
            // No need to check parameter count if the function is wrapped in React wrappers.
            max_parameter_count = None;
        }

        name = get_function_name_from_assignment(assignment?);
    }

    if let (Ok(parameters), Some(max_parameter_count)) = (func.parameters(), max_parameter_count) {
        if parameters.len() > max_parameter_count {
            // If the function has more than one parameter, we assume that it is not a component.
            return None;
        }
    }

    name.filter(|name| is_react_component(name.text_trimmed()))
}

struct FunctionExpressionInfo {
    /// Potential assignment node.
    assignment: Option<AnyAssignment>,
    /// Is the function wrapped (e.g. `memo` or `forwardRef`).
    is_wrapped: bool,
}

/// Tries to extract the assignment node of the function expression.
/// Also checks if the function is wrapped in a React wrapper (e.g. `memo` or `forwardRef`).
fn get_function_expression_info(func: &AnyJsFunction) -> Option<FunctionExpressionInfo> {
    let wrapper_call = func
        .parent::<JsCallArgumentList>()
        .and_then(|args_list| args_list.parent::<JsCallArguments>())
        .and_then(|args| args.parent::<JsCallExpression>());
    if let Some(wrapper_call) = wrapper_call {
        // We don't check that wrappers are necessarily React built-in wrappers,
        // we assume that `SomeOtherLib.memo(...)` is a valid wrapper same as `React.memo(...)`
        // and `memo(...)`.
        let wrapper_method_name = wrapper_call
            .callee()
            .ok()
            .and_then(|c| c.get_callee_member_name());
        if let Some(method_name) = wrapper_method_name {
            if !REACT_HOOKS.contains(&method_name.text()) {
                // If the function is wrapped in a non-React wrapper,
                // we assume that it is not a component.
                return None;
            }
            // If the function is wrapped in a React wrapper,
            // return it for further analysis.
            Some(FunctionExpressionInfo {
                assignment: wrapper_call.parent::<AnyAssignment>(),
                is_wrapped: true,
            })
        } else {
            // If the function is wrapped in an unknown wrapper,
            // we assume that it is not a component.
            None
        }
    } else {
        // If the function is not wrapped, return the potential assignment node
        // for further analysis.
        Some(FunctionExpressionInfo {
            assignment: func.parent::<AnyAssignment>(),
            is_wrapped: false,
        })
    }
}

/// Tries to extract the function name from the assignment node.
fn get_function_name_from_assignment(assignment: AnyAssignment) -> Option<JsSyntaxToken> {
    match assignment {
        AnyAssignment::JsInitializerClause(initializer) => {
            let variable_declarator = initializer.parent::<JsVariableDeclarator>()?;
            variable_declarator
                .id()
                .ok()?
                .as_any_js_binding()?
                .as_js_identifier_binding()?
                .name_token()
                .ok()
        }
        AnyAssignment::JsAssignmentExpression(assignment) => {
            if let Ok(AnyJsAssignmentPattern::AnyJsAssignment(
                AnyJsAssignment::JsIdentifierAssignment(name),
            )) = assignment.left()
            {
                name.name_token().ok()
            } else {
                None
            }
        }
    }
}

declare_node_union! {
    pub AnyAssignment = JsInitializerClause | JsAssignmentExpression
}
