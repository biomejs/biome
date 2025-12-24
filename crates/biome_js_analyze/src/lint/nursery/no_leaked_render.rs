use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsConditionalExpression, JsLogicalExpression,
    JsLogicalOperator, JsReferenceIdentifier, JsSyntaxNode, JsUnaryOperator, JsxExpressionChild,
    JsxTagExpression, binding_ext::AnyJsBindingDeclaration, jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, declare_node_union};
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
        version: "2.3.8",
        name: "noLeakedRender",
        language: "jsx",
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReact("jsx-no-leaked-render").inspired(),
        ],
        recommended: false,
    }
}

impl Rule for NoLeakedRender {
    type Query = Semantic<NoLeakedRenderQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoLeakedRenderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let model = ctx.model();

        if !is_inside_jsx_expression(query.syntax()).unwrap_or_default() {
            return None;
        }

        match query {
            NoLeakedRenderQuery::JsLogicalExpression(exp) => {
                let op = exp.operator().ok()?;

                if op != JsLogicalOperator::LogicalAnd {
                    return None;
                }
                let left = exp.left().ok()?;

                if is_left_hand_side_safe(&left)? {
                    return None;
                }

                let mut is_nested_left_hand_side_safe = false;

                let mut stack = vec![left.clone()];

                // Traverse the expression tree iteratively using a stack
                // This allows us to check nested expressions without recursion
                while let Some(current) = stack.pop() {
                    match current {
                        AnyJsExpression::JsLogicalExpression(expr) => {
                            let left = expr.left().ok()?.omit_parentheses();
                            let right = expr.right().ok()?.omit_parentheses();
                            stack.push(left);
                            stack.push(right);
                        }
                        AnyJsExpression::JsParenthesizedExpression(expr) => {
                            stack.push(expr.expression().ok()?.omit_parentheses());
                        }
                        // If we find expressions that coerce to boolean (unary, call, binary),
                        // then the entire expression is considered safe
                        expr => {
                            if is_left_hand_side_safe(&expr)? {
                                is_nested_left_hand_side_safe = true;
                                break;
                            }
                        }
                    }
                }

                if is_nested_left_hand_side_safe {
                    return None;
                }

                if let AnyJsExpression::JsIdentifierExpression(ident) = &left {
                    let name = ident.name().ok()?;
                    // Use the semantic model to resolve the variable binding and check
                    // if it's initialized with safe expressions. This allows us to
                    // handle cases like:
                    // let isOpen = false;  // This is safe
                    // let isError = errorCount > 0; // This is safe
                    // let isEqual = a === b; // This is safe
                    // let emptyStr = ''; // This is unsafe

                    if let Some(variable) = find_variable(model, &name) {
                        match variable {
                            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                                if is_unsafe_literal(&expr)? {
                                    return Some(());
                                }
                            }
                            AnyJsExpression::JsUnaryExpression(_)
                            | AnyJsExpression::JsBinaryExpression(_)
                            | AnyJsExpression::JsCallExpression(_) => return None,
                            _ => {}
                        }
                    }
                }

                if let AnyJsExpression::AnyJsLiteralExpression(literal) = left {
                    if is_unsafe_literal(&literal)? {
                        return Some(());
                    }
                }

                Some(())
            }
            NoLeakedRenderQuery::JsConditionalExpression(expr) => {
                let alternate = expr.alternate().ok()?;
                let is_alternate_identifier =
                    matches!(alternate, AnyJsExpression::JsIdentifierExpression(_));
                let is_jsx_element_alt = matches!(alternate, AnyJsExpression::JsxTagExpression(_));
                if !is_alternate_identifier || is_jsx_element_alt {
                    return None;
                }

                Some(())
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match node {
            NoLeakedRenderQuery::JsLogicalExpression(_) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Potential leaked value that might cause unintended rendering."
                        },
                    )
                    .note(markup! {
                        "JavaScript's && operator returns the left value when it's falsy (e.g., 0, NaN, ''). React will render that value, causing unexpected UI output."
                    })
                    .note(markup! {
                        "Make sure the condition is explicitly boolean.Use !!value, value > 0, or a ternary expression."
                    })
                )
            }
            NoLeakedRenderQuery::JsConditionalExpression(_) => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Potential leaked value that might cause unintended rendering."
                        },
                    )
                    .note(markup! {
                        "This happens when you use ternary operators in JSX with alternate values that could be variables."
                    })
                    .note(markup! {
                        "Replace with a safe alternate value like an empty string , null or another JSX element."
                    })
                )
            }
        }
    }
}

declare_node_union! {
    pub NoLeakedRenderQuery = JsLogicalExpression | JsConditionalExpression
}

fn is_inside_jsx_expression(node: &JsSyntaxNode) -> Option<bool> {
    let parent = node.parent()?;

    Some(
        JsxExpressionChild::can_cast(parent.kind())
            || JsxTagExpression::can_cast(parent.kind())
            || AnyJsxElement::can_cast(parent.kind()),
    )
}

fn find_variable(model: &SemanticModel, name: &JsReferenceIdentifier) -> Option<AnyJsExpression> {
    let binding = model.binding(name)?;
    let declaration = binding.tree().declaration()?;
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };

    let expr = declarator.initializer()?.expression().ok()?;
    Some(expr)
}

fn is_unsafe_literal(literal: &AnyJsLiteralExpression) -> Option<bool> {
    match literal {
        AnyJsLiteralExpression::JsStringLiteralExpression(str) => {
            if str.inner_string_text().ok()?.text().is_empty() {
                return Some(true);
            }
            return None;
        }
        AnyJsLiteralExpression::JsNullLiteralExpression(_) => {
            return Some(true);
        }
        AnyJsLiteralExpression::JsNumberLiteralExpression(num) => {
            let value = num.value_token().ok()?;

            if is_numeric_zero(value.text_trimmed()) {
                return Some(true);
            }

            return None;
        }

        AnyJsLiteralExpression::JsBigintLiteralExpression(bnum) => {
            let value = bnum.value_token().ok()?;
            let literal = value.text_trimmed();
            let num = literal.strip_suffix("n").unwrap_or(literal);

            if is_numeric_zero(num) {
                return Some(true);
            }
            return None;
        }
        _ => return None,
    };
}

fn is_numeric_zero(num: &str) -> bool {
    if let Ok(num) = num.parse::<i64>() {
        return num == 0;
    }

    if let Ok(num) = num.parse::<f64>() {
        return num == 0.0;
    }

    false
}

fn is_left_hand_side_safe(expr: &AnyJsExpression) -> Option<bool> {
    match expr {
        AnyJsExpression::JsCallExpression(_) | AnyJsExpression::JsBinaryExpression(_) => Some(true),
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? == JsUnaryOperator::Minus {
                Some(false)
            } else {
                Some(true)
            }
        }
        _ => Some(false),
    }
}
