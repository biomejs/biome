use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::{Scope, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsStatement, JsCallExpression, JsIfStatement,
    JsLogicalExpression, JsReturnStatement, JsSyntaxKind, JsSyntaxNode, OperatorPrecedence, T,
    global_identifier,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, Direction, TextRange};
use biome_rule_options::use_simplified_boolean_return::UseSimplifiedBooleanReturnOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Return conditions directly instead of using `if` statements that only return `true` or `false`.
    ///
    /// An `if` statement whose branches only return `true` and `false` is a long way of
    /// returning the condition itself. The same applies to an `if` statement that returns
    /// `true` or `false`, followed directly by a `return` of the opposite value.
    /// Returning the condition directly is shorter and easier to read.
    ///
    /// When the condition doesn't always produce a boolean, the fix wraps it in `Boolean(...)`
    /// so the function still returns `true` or `false`.
    ///
    /// A sequence of early returns that all return the same value is not reported, because
    /// rewriting only the last one would break up the group:
    ///
    /// ```js
    /// function isEmpty(value) {
    ///     if (value === null) {
    ///         return true;
    ///     }
    ///     if (value === "") {
    ///         return true;
    ///     }
    ///     return false;
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function isPositive(value) {
    ///     if (value > 0) {
    ///         return true;
    ///     }
    ///     return false;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function hasItems(items) {
    ///     if (items.length) {
    ///         return true;
    ///     } else {
    ///         return false;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function isNotPositive(value) {
    ///     if (value > 0) {
    ///         return false;
    ///     }
    ///     return true;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function isPositive(value) {
    ///     return value > 0;
    /// }
    ///
    /// function hasItems(items) {
    ///     return Boolean(items.length);
    /// }
    ///
    /// function check(value) {
    ///     if (value > 0) {
    ///         return true;
    ///     }
    ///     log(value);
    ///     return false;
    /// }
    /// ```
    ///
    pub UseSimplifiedBooleanReturn {
        version: "next",
        name: "useSimplifiedBooleanReturn",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-boolean-return").same()],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSimplifiedBooleanReturn {
    type Query = Semantic<JsIfStatement>;
    type State = UseSimplifiedBooleanReturnState;
    type Signals = Option<Self::State>;
    type Options = UseSimplifiedBooleanReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_statement = ctx.query();
        let test = if_statement.test().ok()?.omit_parentheses();
        if matches!(test, AnyJsExpression::JsConditionalExpression(_)) {
            return None;
        }

        let consequent_value = boolean_return_value(
            single_statement(if_statement.consequent().ok()?)?.as_js_return_statement()?,
        )?;

        let alternate = match if_statement.else_clause() {
            Some(else_clause) => single_statement(else_clause.alternate().ok()?)?,
            None => {
                if is_preceded_by_same_early_return(if_statement, consequent_value) {
                    return None;
                }
                AnyJsStatement::cast(if_statement.syntax().next_sibling()?)?
            }
        };
        let AnyJsStatement::JsReturnStatement(alternate) = alternate else {
            return None;
        };

        if boolean_return_value(&alternate)? == consequent_value {
            return None;
        }

        Some(UseSimplifiedBooleanReturnState {
            alternate,
            negate: !consequent_value,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let if_statement = ctx.query();
        let range = if if_statement.else_clause().is_some() {
            if_statement.range()
        } else {
            TextRange::new(if_statement.range().start(), state.alternate.range().end())
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This "<Emphasis>"if"</Emphasis>" statement only returns "<Emphasis>"true"</Emphasis>" or "<Emphasis>"false"</Emphasis>"."
                },
            )
            .note(markup! {
                "Returning the condition directly is shorter and easier to read."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let if_statement = ctx.query();
        let is_flat = if_statement.else_clause().is_none();

        if has_inner_comments(if_statement.syntax(), true, !is_flat)
            || (is_flat && has_inner_comments(state.alternate.syntax(), false, true))
        {
            return None;
        }

        let model = ctx.model();
        let test = if_statement.test().ok()?;
        // The semantic model can only resolve references in the original tree, so the
        // condition is classified before its trivia is trimmed.
        let is_boolean = !state.negate && is_boolean_expression(model, &test);
        let test = test.trim_trivia()?;
        let argument = if state.negate {
            let operand = if test.precedence().ok()? < OperatorPrecedence::Unary
                || matches!(test, AnyJsExpression::JsArrowFunctionExpression(_))
            {
                make::parenthesized(test).into()
            } else {
                test
            };
            make::js_unary_expression(make::token(T![!]), operand).into()
        } else if is_boolean {
            test
        } else {
            if is_boolean_shadowed(&model.scope(if_statement.syntax())) {
                return None;
            }
            let operand = if matches!(test, AnyJsExpression::JsSequenceExpression(_)) {
                make::parenthesized(test).into()
            } else {
                test
            };
            make::js_call_expression(
                make::js_identifier_expression(make::js_reference_identifier(make::ident(
                    "Boolean",
                )))
                .into(),
                make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(operand)], []),
                    make::token(T![')']),
                ),
            )
            .build()
            .into()
        };

        let last_token = if is_flat {
            state.alternate.syntax().last_token()?
        } else {
            if_statement.syntax().last_token()?
        };
        let return_token = make::token_with_trailing_space(T![return]).with_leading_trivia_pieces(
            if_statement
                .syntax()
                .first_token()?
                .leading_trivia()
                .pieces(),
        );
        let semicolon_token =
            make::token(T![;]).with_trailing_trivia_pieces(last_token.trailing_trivia().pieces());
        let return_statement = make::js_return_statement(return_token)
            .with_argument(argument)
            .with_semicolon_token(semicolon_token)
            .build();

        let mut mutation = ctx.root().begin();
        if is_flat {
            mutation.remove_node(state.alternate.clone());
        }
        mutation.replace_node_discard_trivia(
            AnyJsStatement::JsIfStatement(if_statement.clone()),
            AnyJsStatement::JsReturnStatement(return_statement),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Return the condition directly." }.to_owned(),
            mutation,
        ))
    }
}

pub struct UseSimplifiedBooleanReturnState {
    /// The statement that returns the opposite boolean: the `else` branch, or the
    /// `return` statement that directly follows the `if` statement.
    alternate: JsReturnStatement,
    /// Whether the `if` branch returns `false`, so the condition must be negated.
    negate: bool,
}

/// Returns the only statement of `statement`, looking through nested blocks and
/// ignoring empty statements.
fn single_statement(statement: AnyJsStatement) -> Option<AnyJsStatement> {
    let mut statement = statement;
    while let AnyJsStatement::JsBlockStatement(block) = &statement {
        let mut statements = block
            .statements()
            .into_iter()
            .filter(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)));
        let first = statements.next()?;
        if statements.next().is_some() {
            return None;
        }
        statement = first;
    }
    Some(statement)
}

/// Returns the boolean value of `statement` if it returns a `true` or `false` literal,
/// ignoring parentheses and TypeScript type assertions.
fn boolean_return_value(statement: &JsReturnStatement) -> Option<bool> {
    let mut argument = statement.argument()?;
    loop {
        argument = match argument {
            AnyJsExpression::JsParenthesizedExpression(expression) => {
                expression.expression().ok()?
            }
            AnyJsExpression::TsAsExpression(expression) => expression.expression().ok()?,
            AnyJsExpression::TsSatisfiesExpression(expression) => expression.expression().ok()?,
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                expression.expression().ok()?
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                expression.expression().ok()?
            }
            _ => break,
        };
    }
    let literal = argument
        .as_any_js_literal_expression()?
        .as_js_boolean_literal_expression()?;
    Some(literal.value_token().ok()?.kind() == T![true])
}

/// Whether `if_statement` follows an `if` statement without `else` whose branch
/// returns the same boolean, as in a sequence of early returns.
fn is_preceded_by_same_early_return(if_statement: &JsIfStatement, value: bool) -> bool {
    let Some(previous) = if_statement
        .syntax()
        .prev_sibling()
        .and_then(JsIfStatement::cast)
    else {
        return false;
    };
    previous.else_clause().is_none()
        && previous
            .consequent()
            .ok()
            .and_then(single_statement)
            .and_then(|statement| boolean_return_value(statement.as_js_return_statement()?))
            == Some(value)
}

/// Whether `node` contains comments. Comments before the node are ignored when
/// `allow_leading` is `true`, and comments after it on the same line are ignored when
/// `allow_trailing` is `true`.
fn has_inner_comments(node: &JsSyntaxNode, allow_leading: bool, allow_trailing: bool) -> bool {
    if !node.has_comments_descendants() {
        return false;
    }
    let first = node.first_token();
    let last = node.last_token();
    node.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() && !(allow_leading && Some(&token) == first.as_ref()))
            || (token.has_trailing_comments() && !(allow_trailing && Some(&token) == last.as_ref()))
    })
}

/// Whether `expression` always evaluates to `true` or `false`.
fn is_boolean_expression(model: &SemanticModel, expression: &AnyJsExpression) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.as_js_boolean_literal_expression().is_some()
        }
        AnyJsExpression::JsUnaryExpression(unary) => unary
            .operator_token()
            .is_ok_and(|operator| BOOLEAN_UNARY_OPERATORS.contains(operator.kind())),
        AnyJsExpression::JsBinaryExpression(binary) => binary.is_comparison_operator(),
        AnyJsExpression::JsInExpression(_) | AnyJsExpression::JsInstanceofExpression(_) => true,
        AnyJsExpression::JsLogicalExpression(logical) => {
            is_boolean_logical_expression(model, logical)
        }
        AnyJsExpression::JsCallExpression(call) => is_boolean_call(model, &call),
        _ => false,
    }
}

/// Whether both operands of `logical`, and of any logical expression nested on its
/// left side, always evaluate to `true` or `false`.
fn is_boolean_logical_expression(model: &SemanticModel, logical: JsLogicalExpression) -> bool {
    let mut logical = logical;
    loop {
        if !logical
            .right()
            .is_ok_and(|right| is_boolean_expression(model, &right))
        {
            return false;
        }
        match logical.left().map(AnyJsExpression::omit_parentheses) {
            Ok(AnyJsExpression::JsLogicalExpression(left)) => logical = left,
            Ok(left) => return is_boolean_expression(model, &left),
            Err(_) => return false,
        }
    }
}

/// Whether `call` is a call to the global `Boolean` or `Array.isArray` functions.
fn is_boolean_call(model: &SemanticModel, call: &JsCallExpression) -> bool {
    if call.is_optional_chain() {
        return false;
    }
    let Ok(callee) = call.callee().map(AnyJsExpression::omit_parentheses) else {
        return false;
    };
    if let AnyJsExpression::JsStaticMemberExpression(member) = &callee
        && !member.is_optional_chain()
        && member
            .member()
            .ok()
            .and_then(|member| member.as_js_name()?.value_token().ok())
            .is_some_and(|name| name.text_trimmed() == "isArray")
    {
        return member
            .object()
            .ok()
            .and_then(|object| is_global_reference_to(model, &object.omit_parentheses(), "Array"))
            .unwrap_or(false);
    }
    is_global_reference_to(model, &callee, "Boolean").unwrap_or(false)
}

/// Whether `expression` refers to the global variable `name`, possibly through
/// `globalThis` or `window`.
fn is_global_reference_to(
    model: &SemanticModel,
    expression: &AnyJsExpression,
    name: &str,
) -> Option<bool> {
    let (reference, reference_name) =
        global_identifier(&expression.as_any_global_identifier_expression()?)?;
    Some(reference_name.text() == name && model.binding(&reference).is_none())
}

/// Whether a variable named `Boolean` is declared in `scope` or one of its parent scopes.
fn is_boolean_shadowed(scope: &Scope) -> bool {
    scope
        .ancestors()
        .any(|scope| scope.get_binding("Boolean").is_some())
}

const BOOLEAN_UNARY_OPERATORS: TokenSet<JsSyntaxKind> = token_set![T![!], T![delete]];
