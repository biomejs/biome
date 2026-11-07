use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsArrayBindingPatternElementList, JsBinaryOperator,
    JsLogicalExpression, JsLogicalOperator, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode,
    JsUnaryOperator, JsVariableDeclarator, JsxExpressionAttributeValue, JsxExpressionChild,
    JsxTagExpression, binding_ext::AnyJsBindingDeclaration, jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, AstSeparatedList};
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
        severity: Severity::Warning,
    }
}

impl Rule for NoLeakedRender {
    type Query = Semantic<JsLogicalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoLeakedRenderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let exp = ctx.query();
        let model = ctx.model();

        if !is_inside_jsx_child(exp.syntax()) {
            return None;
        }

        let op = exp.operator().ok()?;
        if op != JsLogicalOperator::LogicalAnd {
            return None;
        }

        let left = exp.left().ok()?.omit_parentheses();

        if is_left_hand_side_safe(&left, model).unwrap_or(false) {
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
                    if is_left_hand_side_safe(&expr, model).unwrap_or(false) {
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
            // if it's initialized with safe expressions.
            // Safe: let isOpen = false; let isError = errorCount > 0; let isEqual = a === b;
            // Unsafe: let emptyStr = '';

            if let Some(variable) = find_variable(model, &name) {
                let variable = variable.omit_parentheses();
                match variable {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => {
                        if is_unsafe_literal(&expr).unwrap_or(false) {
                            return Some(());
                        } else {
                            return None;
                        }
                    }
                    AnyJsExpression::JsUnaryExpression(_)
                    | AnyJsExpression::JsBinaryExpression(_)
                    | AnyJsExpression::JsCallExpression(_) => return None,
                    _ => {}
                }
            }
        }

        if let AnyJsExpression::AnyJsLiteralExpression(literal) = &left {
            if is_unsafe_literal(literal).unwrap_or(false) {
                return Some(());
            } else {
                return None;
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

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
                "Make sure the condition is explicitly boolean. Use !!value, value > 0, or a ternary expression."
            }),
        )
    }
}

fn is_inside_jsx_child(node: &JsSyntaxNode) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if JsxExpressionChild::can_cast(parent.kind()) {
            return true;
        }
        if JsxExpressionAttributeValue::can_cast(parent.kind())
            || JsxTagExpression::can_cast(parent.kind())
            || AnyJsxElement::can_cast(parent.kind())
        {
            return false;
        }
        if biome_js_syntax::JsUnaryExpression::cast(parent.clone())
            .and_then(|unary| unary.operator().ok())
            == Some(JsUnaryOperator::LogicalNot)
        {
            return false;
        }
        if !matches!(
            parent.kind(),
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                | JsSyntaxKind::JS_UNARY_EXPRESSION
        ) {
            return false;
        }
        current = parent.parent();
    }
    false
}

fn is_boolean_state_binding(model: &SemanticModel, ident: &JsReferenceIdentifier) -> Option<bool> {
    let binding = model.binding(ident)?;
    let decl = binding.tree().declaration()?;
    let AnyJsBindingDeclaration::JsArrayBindingPatternElement(elem) = decl else {
        return None;
    };
    let pattern = elem.parent::<JsArrayBindingPatternElementList>()?;
    let first_elem = pattern.first()?.ok()?;
    if first_elem.syntax() != elem.syntax() {
        return None;
    }
    let declarator = pattern
        .syntax()
        .ancestors()
        .find_map(JsVariableDeclarator::cast)?;
    let init = declarator.initializer()?.expression().ok()?.omit_parentheses();
    let call = init.as_js_call_expression()?;

    let callee = call.callee().ok()?.omit_parentheses();
    let is_use_state = match callee {
        AnyJsExpression::JsIdentifierExpression(ident) => {
            ident.name().ok()?.value_token().ok()?.text_trimmed() == "useState"
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            member.member().ok()?.value_token().ok()?.text_trimmed() == "useState"
        }
        _ => false,
    };
    if !is_use_state {
        return None;
    }

    if let Some(type_args) = call.type_arguments() {
        for arg in type_args.ts_type_argument_list().into_iter().flatten() {
            if arg.syntax().text_trimmed() == "boolean" {
                return Some(true);
            }
        }
    }

    let args = call.arguments().ok()?.args();
    let first_arg = args.into_iter().next()?.ok()?;
    if let Some(expr) = first_arg.as_any_js_expression() {
        let expr = expr.clone().omit_parentheses();
        match expr {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_),
            ) => return Some(true),
            AnyJsExpression::JsUnaryExpression(unary) => {
                if unary.operator().ok()? == JsUnaryOperator::LogicalNot {
                    return Some(true);
                }
            }
            AnyJsExpression::JsBinaryExpression(binary) => {
                let op = binary.operator().ok()?;
                if matches!(
                    op,
                    JsBinaryOperator::Equality
                        | JsBinaryOperator::Inequality
                        | JsBinaryOperator::StrictEquality
                        | JsBinaryOperator::StrictInequality
                        | JsBinaryOperator::LessThan
                        | JsBinaryOperator::LessThanOrEqual
                        | JsBinaryOperator::GreaterThan
                        | JsBinaryOperator::GreaterThanOrEqual
                ) {
                    return Some(true);
                }
            }
            _ => {}
        }
    }

    None
}

fn find_variable(model: &SemanticModel, name: &JsReferenceIdentifier) -> Option<AnyJsExpression> {
    let binding = model.binding(name)?;
    let declaration = binding.tree().declaration()?;
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };

    declarator.initializer()?.expression().ok()
}

fn is_unsafe_literal(literal: &AnyJsLiteralExpression) -> Option<bool> {
    match literal {
        AnyJsLiteralExpression::JsStringLiteralExpression(str) => {
            if str.inner_string_text().ok()?.text().is_empty() {
                return Some(true);
            }
            None
        }
        AnyJsLiteralExpression::JsNumberLiteralExpression(num) => {
            let value = num.value_token().ok()?;

            if is_numeric_zero(value.text_trimmed()) {
                return Some(true);
            }

            None
        }

        AnyJsLiteralExpression::JsBigintLiteralExpression(bnum) => {
            let value = bnum.value_token().ok()?;
            let literal = value.text_trimmed();
            let num = literal.strip_suffix('n').unwrap_or(literal);

            if is_numeric_zero(num) {
                return Some(true);
            }
            None
        }
        _ => None,
    }
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

fn is_left_hand_side_safe(expr: &AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    let expr = expr.clone().omit_parentheses();
    match &expr {
        AnyJsExpression::JsCallExpression(_) | AnyJsExpression::JsBinaryExpression(_) => Some(true),
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? == JsUnaryOperator::Minus {
                Some(false)
            } else {
                Some(true)
            }
        }
        AnyJsExpression::AnyJsLiteralExpression(lit) => match lit {
            AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
            | AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(true),
            _ => {
                if is_unsafe_literal(lit).unwrap_or(false) {
                    Some(false)
                } else {
                    Some(true)
                }
            }
        },
        AnyJsExpression::JsIdentifierExpression(ident) => {
            let name_ident = ident.name().ok()?;
            let name_token = name_ident.value_token().ok()?;
            let name_text = name_token.text_trimmed();
            if name_text == "undefined" {
                return Some(true);
            }
            if name_text == "NaN" {
                return Some(false);
            }
            if is_boolean_state_binding(model, &name_ident).unwrap_or(false) {
                return Some(true);
            }
            None
        }
        _ => Some(false),
    }
}
