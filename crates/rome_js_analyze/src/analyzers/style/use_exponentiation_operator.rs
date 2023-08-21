use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::{make, syntax::T};
use rome_js_syntax::{
    global_identifier, AnyJsExpression, AnyJsMemberExpression, JsBinaryOperator, JsCallExpression,
    JsClassDeclaration, JsClassExpression, JsExtendsClause, JsInExpression, OperatorPrecedence,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_rule! {
    /// Disallow the use of `Math.pow` in favor of the `**` operator.
    ///
    /// > Introduced in ES2016, the infix exponentiation operator ** is an alternative for the standard Math.pow function.
    /// > Infix notation is considered to be more readable and thus more preferable than the function notation.
    ///
    /// Source: https://eslint.org/docs/latest/rules/prefer-exponentiation-operator
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = Math.pow(2, 8);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const bar = Math.pow(a, b);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let baz = Math.pow(a + b, c + d);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let quux = Math.pow(-1, n);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 2 ** 8;
    ///
    /// const bar = a ** b;
    ///
    /// let baz = (a + b) ** (c + d);
    ///
    /// let quux = (-1) ** n;
    /// ```
    ///
    pub(crate) UseExponentiationOperator {
        version: "1.0.0",
        name: "useExponentiationOperator",
        recommended: true,
    }
}

pub struct MathPowCall {
    base: AnyJsExpression,
    exponent: AnyJsExpression,
}

impl MathPowCall {
    fn make_base(&self) -> Option<AnyJsExpression> {
        Some(if self.does_base_need_parens()? {
            parenthesize_any_js_expression(&self.base)
        } else {
            self.base.clone()
        })
    }

    fn make_exponent(&self) -> Option<AnyJsExpression> {
        Some(if self.does_exponent_need_parens()? {
            parenthesize_any_js_expression(&self.exponent)
        } else {
            self.exponent.clone()
        })
    }

    /// Determines whether the base expression needs parens in an exponentiation binary expression.
    fn does_base_need_parens(&self) -> Option<bool> {
        Some(
            // '**' is right-associative, parens are needed when Math.pow(a ** b, c) is converted to (a ** b) ** c
            self.base.precedence().ok()? <= OperatorPrecedence::Exponential
				// An unary operator cannot be used immediately before an exponentiation expression
				|| self.base.as_js_unary_expression().is_some()
				|| self.base.as_js_await_expression().is_some(),
        )
    }

    /// Determines whether the exponent expression needs parens in an exponentiation binary expression.
    fn does_exponent_need_parens(&self) -> Option<bool> {
        Some(self.exponent.precedence().ok()? < OperatorPrecedence::Exponential)
    }
}

impl Rule for UseExponentiationOperator {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let callee = node.callee().ok()?.omit_parentheses();
        let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        if member_expr.member_name()?.text() != "pow" {
            return None;
        }
        let object = member_expr.object().ok()?.omit_parentheses();
        let (reference, name) = global_identifier(&object)?;
        if name.text() != "Math" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Use the '**' operator instead of 'Math.pow'.",
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if !should_suggest_fix(node)? {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let [base, exponent] = node.get_arguments_by_index([0, 1]);

        let math_pow_call = MathPowCall {
            base: base?.as_any_js_expression()?.clone().omit_parentheses(),
            exponent: exponent?.as_any_js_expression()?.clone().omit_parentheses(),
        };

        let new_node = make::js_binary_expression(
            math_pow_call.make_base()?,
            make::token(T![**]),
            math_pow_call.make_exponent()?,
        );

        if let Some((needs_parens, parent)) = does_exponentiation_expression_need_parens(node) {
            if needs_parens && parent.is_some() {
                mutation.replace_node(parent.clone()?, parenthesize_any_js_expression(&parent?));
            }

            mutation.replace_node(
                AnyJsExpression::from(node.clone()),
                parenthesize_any_js_expression(&AnyJsExpression::from(new_node)),
            );
        } else {
            mutation.replace_node(
                AnyJsExpression::from(node.clone()),
                AnyJsExpression::from(new_node),
            );
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use the '**' operator instead of 'Math.pow'." }.to_owned(),
            mutation,
        })
    }
}

/// Verify if the autofix is safe to be applied and won't remove comments.
/// Argument list is considered valid if there's no spread arg and leading/trailing comments.
fn should_suggest_fix(node: &JsCallExpression) -> Option<bool> {
    let arguments = node.arguments().ok()?;
    let args_count = arguments.args().len();

    Some(
        args_count == 2
            && !arguments.l_paren_token().ok()?.has_leading_comments()
            && !arguments.l_paren_token().ok()?.has_trailing_comments()
            && !arguments.r_paren_token().ok()?.has_leading_comments()
            && !arguments.r_paren_token().ok()?.has_trailing_comments()
            && arguments.args().into_iter().flatten().all(|arg| {
                !arg.syntax().has_leading_comments()
                    && !arg.syntax().has_trailing_comments()
                    && arg.as_js_spread().is_none()
            }),
    )
}

/// Wraps a [AnyJsExpression] in paretheses
fn parenthesize_any_js_expression(expr: &AnyJsExpression) -> AnyJsExpression {
    AnyJsExpression::from(make::js_parenthesized_expression(
        make::token(T!['(']),
        expr.clone(),
        make::token(T![')']),
    ))
}

/// Determines whether the given parent node needs parens if used as the exponent in an exponentiation binary expression.
fn does_exponentiation_expression_need_parens(
    node: &JsCallExpression,
) -> Option<(bool, Option<AnyJsExpression>)> {
    if let Some(parent) = node.parent::<AnyJsExpression>() {
        if does_expression_need_parens(node, &parent)? {
            return Some((true, Some(parent)));
        }
    } else if let Some(extends_clause) = node.parent::<JsExtendsClause>() {
        if extends_clause.parent::<JsClassDeclaration>().is_some() {
            return Some((true, None));
        }

        if let Some(class_expr) = extends_clause.parent::<JsClassExpression>() {
            let class_expr = AnyJsExpression::from(class_expr);
            if does_expression_need_parens(node, &class_expr)? {
                return Some((true, Some(class_expr)));
            }
        }
    }

    None
}

/// Determines whether the given expression needs parens when used in an exponentiation binary expression.
fn does_expression_need_parens(
    node: &JsCallExpression,
    expression: &AnyJsExpression,
) -> Option<bool> {
    let needs_parentheses = match &expression {
        // Skips already parenthesized expressions
        AnyJsExpression::JsParenthesizedExpression(_) => return None,
        AnyJsExpression::JsBinaryExpression(bin_expr) => {
            if bin_expr.parent::<JsInExpression>().is_some() {
                return Some(true);
            }

            let binding = bin_expr.right().ok()?;
            let call_expr = binding.as_js_call_expression();

            bin_expr.operator().ok()? != JsBinaryOperator::Exponent
                || call_expr.is_none()
                || call_expr? != node
        }
        AnyJsExpression::JsCallExpression(call_expr) => !call_expr
            .arguments()
            .ok()?
            .args()
            .iter()
            .filter_map(|arg| {
                let binding = arg.ok()?;
                return binding
                    .as_any_js_expression()?
                    .as_js_call_expression()
                    .cloned();
            })
            .any(|arg| &arg == node),
        AnyJsExpression::JsNewExpression(new_expr) => !new_expr
            .arguments()?
            .args()
            .iter()
            .filter_map(|arg| {
                let binding = arg.ok()?;
                return binding
                    .as_any_js_expression()?
                    .as_js_call_expression()
                    .cloned();
            })
            .any(|arg| &arg == node),
        AnyJsExpression::JsComputedMemberExpression(member_expr) => {
            let binding = member_expr.member().ok()?;
            let call_expr = binding.as_js_call_expression();

            call_expr.is_none() || call_expr? != node
        }
        AnyJsExpression::JsInExpression(_) => return Some(true),
        AnyJsExpression::JsClassExpression(_)
        | AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsUnaryExpression(_)
        | AnyJsExpression::JsTemplateExpression(_) => true,
        _ => false,
    };

    Some(needs_parentheses && expression.precedence().ok()? >= OperatorPrecedence::Exponential)
}
