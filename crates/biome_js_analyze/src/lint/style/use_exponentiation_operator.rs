use crate::services::semantic::Semantic;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::{make, syntax::T};
use biome_js_syntax::{
    global_identifier, AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsBinaryOperator,
    JsCallExpression, JsClassDeclaration, JsClassExpression, JsExtendsClause, JsInExpression,
    OperatorPrecedence,
};
use biome_rowan::{
    chain_trivia_pieces, trim_leading_trivia_pieces, AstNode, AstSeparatedList, BatchMutationExt,
    SyntaxResult,
};

declare_lint_rule! {
    /// Disallow the use of `Math.pow` in favor of the `**` operator.
    ///
    /// Introduced in ES2016, the infix exponentiation operator `**` is an alternative for the standard `Math.pow` function.
    /// Infix notation is considered to be more readable and thus more preferable than the function notation.
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
    pub UseExponentiationOperator {
        version: "1.0.0",
        name: "useExponentiationOperator",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-exponentiation-operator")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
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
        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;
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
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Use the '**' operator instead of 'Math.pow'.",
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let args = node.arguments().ok()?;
        let [Some(AnyJsCallArgument::AnyJsExpression(base)), Some(AnyJsCallArgument::AnyJsExpression(exponent)), None] =
            node.arguments().ok()?.get_arguments_by_index([0, 1, 2])
        else {
            return None;
        };
        let base = if does_base_need_parens(&base).ok()? {
            make::parenthesized(base).into()
        } else {
            base
        };
        let exponent = if does_exponent_need_parens(&exponent).ok()? {
            make::parenthesized(exponent).into()
        } else {
            exponent
        };
        let l_paren = args.l_paren_token().ok()?;
        let mut separators = args.args().separators();
        let separator = separators.next()?.ok()?;
        let trailing_separator = separators.next();
        let r_paren = args.r_paren_token().ok()?;
        // Transfer comments before and after `base` and `exponent`
        // which are associated with the comma or a paren.
        let base = base
            .prepend_trivia_pieces(chain_trivia_pieces(
                l_paren.leading_trivia().pieces(),
                l_paren.trailing_trivia().pieces(),
            ))?
            .append_trivia_pieces(chain_trivia_pieces(
                separator.leading_trivia().pieces(),
                separator.leading_trivia().pieces(),
            ))?;
        let exponent = if let Some(Ok(trailing_separator)) = trailing_separator {
            exponent.append_trivia_pieces(chain_trivia_pieces(
                trailing_separator.leading_trivia().pieces(),
                trailing_separator.trailing_trivia().pieces(),
            ))?
        } else {
            exponent
        };
        let exponent = exponent
            .prepend_trivia_pieces(trim_leading_trivia_pieces(
                separator.trailing_trivia().pieces(),
            ))?
            .append_trivia_pieces(r_paren.leading_trivia().pieces())?;
        let mut mutation = ctx.root().begin();
        let new_node = AnyJsExpression::from(make::js_binary_expression(
            base,
            make::token_decorated_with_space(T![**]),
            exponent,
        ));
        let new_node = if let Some((needs_parens, parent)) =
            does_exponentiation_expression_need_parens(node)
        {
            if needs_parens && parent.is_some() {
                mutation.replace_node(parent.clone()?, make::parenthesized(parent?).into());
            }
            make::parenthesized(new_node).into()
        } else {
            new_node
        };
        // Transfer leading and trailing comments
        let new_node = new_node
            .prepend_trivia_pieces(node.syntax().first_leading_trivia()?.pieces())?
            .append_trivia_pieces(node.syntax().last_trailing_trivia()?.pieces())?;
        mutation.replace_node_discard_trivia(AnyJsExpression::from(node.clone()), new_node);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the '**' operator instead of 'Math.pow'." }.to_owned(),
            mutation,
        ))
    }
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
        AnyJsExpression::JsParenthesizedExpression(_) => return Some(false),
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
        AnyJsExpression::JsCallExpression(call_expr) => call_expr
            .arguments()
            .ok()?
            .args()
            .iter()
            .find_map(|arg| {
                Some(arg.ok()?.as_any_js_expression()?.as_js_call_expression()? == node)
            })
            .is_none(),
        AnyJsExpression::JsNewExpression(new_expr) => new_expr
            .arguments()?
            .args()
            .iter()
            .find_map(|arg| {
                Some(arg.ok()?.as_any_js_expression()?.as_js_call_expression()? == node)
            })
            .is_none(),
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

fn does_base_need_parens(base: &AnyJsExpression) -> SyntaxResult<bool> {
    // '**' is right-associative, parens are needed when Math.pow(a ** b, c) is converted to (a ** b) ** c
    Ok(base.precedence()? <= OperatorPrecedence::Exponential
        // An unary operator cannot be used immediately before an exponentiation expression
        || base.as_js_unary_expression().is_some()
        || base.as_js_await_expression().is_some()
        // Parenthesis could be avoided in the following cases.
        // However, this improves readability.
        || base.as_js_pre_update_expression().is_some()
        || base.as_js_post_update_expression().is_some())
}

fn does_exponent_need_parens(exponent: &AnyJsExpression) -> SyntaxResult<bool> {
    Ok(exponent.precedence()? < OperatorPrecedence::Exponential)
}
