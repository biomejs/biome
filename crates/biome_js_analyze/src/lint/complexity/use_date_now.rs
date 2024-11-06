use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentOperator, JsBinaryOperator, JsCallExpression, JsNewExpression,
    JsNewOrCallExpression, JsSyntaxKind, JsSyntaxNode, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

use crate::lint::style::use_explicit_length_check::does_node_needs_space_before_child;

declare_lint_rule! {
    /// Use `Date.now()` to get the number of milliseconds since the Unix Epoch.
    ///
    /// `Date.now()` is more readable than `new Date().getTime()` and its variants,
    /// it also avoids unnecessary instantiation of `Date` object.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = new Date().getTime();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = new Date().valueOf();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = +new Date();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = Number(new Date());
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = new Date() * 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = Date.now();
    /// ```
    /// ```js
    /// const foo = Date.now() * 2;
    /// ```
    ///
    pub UseDateNow {
        version: "1.8.0",
        name: "useDateNow",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-date-now")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseDateNow {
    type Query = Ast<JsNewOrCallExpression>;
    type State = (AnyJsExpression, UseDateNowIssueKind);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();

        match expr {
            JsNewOrCallExpression::JsCallExpression(call_expr) => get_date_method_issue(call_expr),
            JsNewOrCallExpression::JsNewExpression(expr) => get_new_date_issue(expr),
        }
    }

    fn diagnostic(_: &RuleContext<Self>, (node, kind): &Self::State) -> Option<RuleDiagnostic> {
        let message = match kind {
            UseDateNowIssueKind::ReplaceMethod(method) => format!("new Date().{method}"),
            UseDateNowIssueKind::ReplaceConstructor => "new Date()".to_string(),
            UseDateNowIssueKind::ReplaceNumberConstructor => "Number(new Date())".to_string(),
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>"Date.now()"</Emphasis>" instead of "<Emphasis>{message}</Emphasis>"."
            },
        ).note(
            markup! {
                <Emphasis>"Date.now()"</Emphasis>" is more readable and also avoids unnecessary instantiation of "<Emphasis>"Date"</Emphasis>"object."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, (node, _): &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let args = make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list([], []),
            make::token(T![')']),
        );

        let mut date_now_expr = make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Date")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident("now")).into(),
        );

        if does_node_needs_space_before_child(&node.syntax().parent()?) {
            // Make fake token to get leading trivia
            let leading_trivia = make::token_decorated_with_space(T![=])
                .leading_trivia()
                .pieces();

            // In case `await +a` this produces double  space `await  +a`.
            // TODO. Find a way to avoid this.
            date_now_expr = date_now_expr
                .trim_leading_trivia()?
                .prepend_trivia_pieces(leading_trivia)?;
        }

        let new_call_expr = make::js_call_expression(date_now_expr.into(), args).build();

        mutation.replace_node_discard_trivia::<AnyJsExpression>(node.clone(), new_call_expr.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Replace with "<Emphasis>"Date.now()"</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub enum UseDateNowIssueKind {
    ReplaceMethod(String),
    ReplaceConstructor,
    ReplaceNumberConstructor,
}

fn get_date_method_issue(
    call_expr: &JsCallExpression,
) -> Option<(AnyJsExpression, UseDateNowIssueKind)> {
    let callee = call_expr.callee().ok()?.omit_parentheses();

    let member_name = callee.get_callee_member_name()?.token_text_trimmed();
    if member_name != "getTime" && member_name != "valueOf"
        || call_expr.is_optional()
        || call_expr.arguments().ok()?.args().len() > 0
    {
        return None;
    }

    let object = callee
        .as_js_static_member_expression()?
        .object()
        .ok()?
        .omit_parentheses();

    let new_expr = object.as_js_new_expression()?;
    let object_name = new_expr
        .callee()
        .ok()?
        .get_callee_object_name()?
        .token_text_trimmed();

    if object_name != "Date" || new_expr.arguments()?.args().len() > 0 {
        return None;
    }

    Some((
        AnyJsExpression::cast_ref(call_expr.syntax())?,
        UseDateNowIssueKind::ReplaceMethod(member_name.to_string()),
    ))
}

fn get_new_date_issue(expr: &JsNewExpression) -> Option<(AnyJsExpression, UseDateNowIssueKind)> {
    let callee = expr.callee().ok()?.omit_parentheses();

    if callee.get_callee_member_name()?.token_text_trimmed() != "Date"
        || expr.arguments()?.args().len() > 0
    {
        return None;
    }

    let parent = get_parent_without_parenthesis(expr.syntax())?;
    match parent {
        AnyJsExpression::JsBinaryExpression(binary_expr) => {
            let operator = binary_expr.operator().ok()?;

            if matches!(operator, |JsBinaryOperator::Minus| JsBinaryOperator::Times
                | JsBinaryOperator::Divide
                | JsBinaryOperator::Remainder
                | JsBinaryOperator::Exponent)
            {
                let any_expr = AnyJsExpression::cast_ref(expr.syntax())?;

                return Some((any_expr, UseDateNowIssueKind::ReplaceConstructor));
            }

            None
        }
        AnyJsExpression::JsAssignmentExpression(expr) => {
            let token = expr.operator().ok()?;

            if matches!(
                token,
                JsAssignmentOperator::SubtractAssign
                    | JsAssignmentOperator::TimesAssign
                    | JsAssignmentOperator::SlashAssign
                    | JsAssignmentOperator::RemainderAssign
                    | JsAssignmentOperator::ExponentAssign
            ) {
                let any_expr = AnyJsExpression::cast(expr.right().ok()?.into_syntax())?;

                return Some((any_expr, UseDateNowIssueKind::ReplaceConstructor));
            }

            None
        }
        AnyJsExpression::JsUnaryExpression(unary_expr) => {
            let operator = unary_expr.operator().ok()?;

            let syntax = match operator {
                JsUnaryOperator::Plus => unary_expr.into_syntax(),
                JsUnaryOperator::Minus => expr.syntax().clone(),
                _ => return None,
            };

            Some((
                AnyJsExpression::cast(syntax)?,
                UseDateNowIssueKind::ReplaceConstructor,
            ))
        }
        AnyJsExpression::JsCallExpression(call_expr) => {
            if call_expr.is_optional() || call_expr.arguments().ok()?.args().len() != 1 {
                return None;
            }

            let call_name = call_expr
                .callee()
                .ok()?
                .omit_parentheses()
                .get_callee_member_name()?
                .token_text_trimmed();

            if call_name == "Number" || call_name == "BigInt" {
                return Some((
                    AnyJsExpression::cast(call_expr.into_syntax())?,
                    UseDateNowIssueKind::ReplaceNumberConstructor,
                ));
            }

            None
        }
        _ => None,
    }
}

fn get_parent_without_parenthesis(node: &JsSyntaxNode) -> Option<AnyJsExpression> {
    node.ancestors()
        .skip(1)
        .find(|ancestor| {
            ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                && AnyJsExpression::can_cast(ancestor.kind())
        })
        .and_then(AnyJsExpression::cast)
}
