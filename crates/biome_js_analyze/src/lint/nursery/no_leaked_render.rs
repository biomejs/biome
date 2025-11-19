use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, JsConditionalExpression, JsLogicalExpression, JsLogicalOperator, JsSyntaxNode,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, SyntaxResult, declare_node_union};
use biome_rule_options::no_leaked_render::NoLeakedRenderOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Prevent problematic leaked values from being rendered.
    ///
    /// This rule prevents values that might cause unintentionally rendered values
    /// or rendering crashes in React JSX. When using conditional rendering with the
    /// logical AND operator (`&&`), if the left-hand side evaluates to a falsy value like
    /// `0`, `NaN`, or any empty string, these values will be rendered instead of rendering nothing.
    ///
    /// Similarly, when using ternary operators with problematic alternate values like `null`,
    /// `undefined`, or `false`, it can cause rendering issues or crashes.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const Component = () => {
    ///   const count = 0;
    ///   return <div>{count && <span>Count: {count}</span>}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const Component = () => {
    ///   const items = [];
    ///   return <div>{items.length && <List items={items} />}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const Component = () => {
    ///   const user = null;
    ///   return <div>{user && <Profile user={user} />}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// const Component = () => {
    ///   const condition = false;
    ///   return <div>{condition ? <Content /> : null}</div>;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const Component = () => {
    ///   const count = 0;
    ///   return <div>{count > 0 && <span>Count: {count}</span>}</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const Component = () => {
    ///   const items = [];
    ///   return <div>{!!items.length && <List items={items} />}</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const Component = () => {
    ///   const user = null;
    ///   return <div>{user ? <Profile user={user} /> : null}</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const Component = () => {
    ///   const condition = false;
    ///   return <div>{condition ? <Content /> : <Fallback />}</div>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// const Component = () => {
    ///   const isReady = true;
    ///   return <div>{isReady && <Content />}</div>;
    /// }
    /// ```

    pub NoLeakedRender{
        version: "next",
        name: "noLeakedRender",
        language: "js",
        recommended: false,
    }
}

const COERCE_STRATEGY: &str = "coerce";
const TERNARY_STRATEGY: &str = "ternary";
const TERNARY_INVALID_ALTERNATE_VALUES: &[&str] = &["null", "undefined", "false"];

const DEFAULT_VALID_STRATEGIES: &[&str] = &[TERNARY_STRATEGY, COERCE_STRATEGY];

pub enum NoLeakedRenderState {
    NoPotentialLeakedRender,
}

fn get_variable_from_context(
    model: &SemanticModel,
    node: &JsSyntaxNode,
    name: &str,
) -> Option<Binding> {
    let scope = model.scope(node);

    // Search through scope hierarchy
    for scope in scope.ancestors() {
        if let Some(binding) = scope.get_binding(name) {
            return Some(binding);
        }
    }

    None
}

declare_node_union! {
    pub Query = JsLogicalExpression | JsConditionalExpression
}

impl Rule for NoLeakedRender {
    type Query = Semantic<Query>;
    type State = NoLeakedRenderState;
    type Signals = Option<Self::State>;
    type Options = NoLeakedRenderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let model = ctx.model();

        let options = ctx.options();
        let valid_strategies: Vec<Box<str>> =
            if let Some(strategies) = options.valid_strategies.clone() {
                strategies.to_vec()
            } else {
                DEFAULT_VALID_STRATEGIES
                    .iter()
                    .map(|&str| str.into())
                    .collect()
            };
        match query {
            Query::JsLogicalExpression(exp) => {
                let op = exp.operator().ok()?;
                let left = exp.left().ok()?;

                if op != JsLogicalOperator::LogicalAnd {
                    return None;
                }

                let is_coerce_valid_left_side = matches!(
                    left,
                    AnyJsExpression::JsUnaryExpression(_)
                        | AnyJsExpression::JsCallExpression(_)
                        | AnyJsExpression::JsBinaryExpression(_)
                );

                if valid_strategies
                    .iter()
                    .any(|s| s.as_ref() == COERCE_STRATEGY)
                {
                    if is_coerce_valid_left_side
                        || get_is_coerce_valid_nested_logical_expression(exp.left())
                    {
                        return None;
                    }
                    let left_node = left.syntax();

                    if let AnyJsExpression::JsIdentifierExpression(ident) = &left {
                        let name = ident.name().ok()?;

                        let binding = get_variable_from_context(
                            model,
                            left_node,
                            name.to_trimmed_text().trim(),
                        )?;

                        let declaration = binding.tree().declaration()?;

                        if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
                            declaration
                        {
                            let initializer = declarator.initializer()?;
                            let initializer = initializer.expression().ok()?;

                            if let AnyJsExpression::AnyJsLiteralExpression(literal) = initializer {
                                let literal = literal.value_token().ok()?;

                                if matches!(literal.text_trimmed(), "true" | "false") {
                                    return None;
                                }
                            }
                        }
                    }
                }

                let is_literal = matches!(left, AnyJsExpression::AnyJsLiteralExpression(_));
                if is_literal && left.to_trimmed_text().is_empty() {
                    return None;
                }

                return Some(NoLeakedRenderState::NoPotentialLeakedRender);
            }
            Query::JsConditionalExpression(expr) => {
                if valid_strategies
                    .iter()
                    .any(|s| s.as_ref() == TERNARY_STRATEGY)
                {
                    return None;
                }
                let alternate = expr.alternate().ok()?;
                let is_problematic_alternate = TERNARY_INVALID_ALTERNATE_VALUES
                    .iter()
                    .any(|&s| alternate.to_trimmed_text() == s);

                let is_jsx_element_alt = matches!(alternate, AnyJsExpression::JsxTagExpression(_));

                if !is_problematic_alternate || is_jsx_element_alt {
                    return None;
                }

                return Some(NoLeakedRenderState::NoPotentialLeakedRender);
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match node {
            Query::JsLogicalExpression(_) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Potential leaked value that might cause unintended rendering."
                        },
                    )
                    .note(markup! {
                        "JavaScript's && operator returns the left value when it's falsy (e.g., 0, NaN, '').React will render that value, causing unexpected UI output."
                    })
                    .note(markup! {
                        "Make sure the condition is explicitly boolean.Use !!value, value > 0, or a ternary expression."
                    })
                )
            }
            Query::JsConditionalExpression(_) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Potential leaked value that might cause unintended rendering."
                        },
                    )
                    .note(markup! {
                        "This happens When you use ternary operators in JSX with alternate values like null,undefined, or false"
                    })
                    .note(markup! {
                        "Replace with a safe alternate value like an empty string or another JSX element"
                    })
                )
            }
        }
    }
}

fn get_is_coerce_valid_nested_logical_expression(node: SyntaxResult<AnyJsExpression>) -> bool {
    match node {
        Ok(AnyJsExpression::JsLogicalExpression(expr)) => {
            get_is_coerce_valid_nested_logical_expression(expr.left())
                && get_is_coerce_valid_nested_logical_expression(expr.right())
        }
        Ok(AnyJsExpression::JsParenthesizedExpression(expr)) => {
            get_is_coerce_valid_nested_logical_expression(expr.expression())
        }
        Ok(
            AnyJsExpression::JsUnaryExpression(_)
            | AnyJsExpression::JsCallExpression(_)
            | AnyJsExpression::JsBinaryExpression(_),
        ) => true,
        _ => false,
    }
}
