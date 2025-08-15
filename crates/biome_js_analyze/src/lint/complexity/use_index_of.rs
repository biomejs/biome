use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    AnyJsMemberExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsBinaryExpression,
    JsCallExpression, JsFunctionExpression, JsLogicalExpression, JsParameterList,
    JsReturnStatement, JsSyntaxNode, JsSyntaxToken, JsVariableDeclaration, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxToken, Text};
use biome_rule_options::use_index_of::UseIndexOfOptions;

declare_lint_rule! {
    /// Prefer `Array#{indexOf,lastIndexOf}()` over `Array#{findIndex,findLastIndex}()` when looking for the index of an item.
    ///
    /// `Array#findIndex()` and `Array#findLastIndex()` are intended for more complex needs.
    /// If you are just looking for the index where the given item is present, then the code can be simplified to use Array#indexOf() or Array#lastIndexOf().
    /// This applies to any search with a literal, a variable, or any expression that doesn't have any explicit side effects.
    /// However, if the expression you are looking for relies on an item related to the function (its arguments, the function self, etc.), the case is still valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => x === 'foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => 'foo' === x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findIndex(x => {
    ///      return x === 'foo';
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(x => 'foo' === x);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(x => {
    ///      return x === 'bar';
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const index = foo.findLastIndex(function(x) {
    ///      return x === 'foo';
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const index = foo.indexOf('foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => x !== 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex((x, index) => x === index);
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => (x === 'foo') && isValid());
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => y === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => y.x === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(x => {
    ///     const bar = getBar();
    ///     return x === bar;
    /// });
    /// ```
    ///
    /// ```js
    /// const index = foo.findIndex(function(x) {
    ///     const bar = getBar();
    ///     return x === bar;
    /// });
    /// ```
    ///
    /// ```js
    /// const index = foo.lastIndexOf('foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => x !== 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex((x, index) => x === index);
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => (x === 'foo') && isValid());
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => y === 'foo');
    /// ```
    ///
    /// ```js
    /// const index = foo.findLastIndex(x => y.x === 'foo');
    /// ```
    ///
    pub UseIndexOf {
        version: "2.0.0",
        name: "useIndexOf",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintUnicorn("prefer-array-index-of").same()],
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseIndexOf {
    type Query = Ast<JsCallExpression>;
    type State = JsSyntaxMatchPair;
    type Signals = Option<Self::State>;
    type Options = UseIndexOfOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let member_expression = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
        let member_name = member_expression.member_name()?;

        member_name_remap(member_name.text())?;

        let member_name_token = member_expression.syntax().last_token()?;
        let callback_function = call.arguments().ok()?.args().first()?.ok()?;
        match callback_function {
            AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsFunctionExpression(function)) => {
                callback_function_match(&function, member_name_token)
            }
            AnyJsCallArgument::AnyJsExpression(AnyJsExpression::JsArrowFunctionExpression(
                function,
            )) => callback_arrow_function_match(&function, member_name_token),
            _ => None,
        }
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        matched_expression: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let JsSyntaxMatchPair {
            matching_array_element: _,
            member_name,
        } = matched_expression;

        let old_member_name = member_name.text_trimmed();
        let new_member_name = member_name_remap(old_member_name)?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Prefer "<Emphasis>"Array#"{new_member_name}"()"</Emphasis>" over "<Emphasis>"Array#"{old_member_name}"()"</Emphasis>" when looking for the index of an item."
                },
            ).note(markup! {
                "This callback only tests for equality against a single value. This value can be passed directly to "<Emphasis>""{new_member_name}"()"</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, matched_expression: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        let mut mutation = ctx.root().begin();
        let JsSyntaxMatchPair {
            matching_array_element,
            member_name,
        } = matched_expression;

        let old_member_name = member_name.text_trimmed();
        let new_member_name = member_name_remap(old_member_name)?;

        mutation.replace_token_discard_trivia(
            member_name.clone(),
            SyntaxToken::new_detached(member_name.kind(), new_member_name, [], []),
        );

        let arguments = call.arguments().ok()?;
        let old_arg = arguments.args().first()?.ok()?;

        mutation.replace_element_discard_trivia(
            old_arg.into_syntax().into(),
            matching_array_element.clone().into(),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
              "Replace "<Emphasis>"Array#"{old_member_name}"()"</Emphasis>" with "<Emphasis>"Array#"{new_member_name}"()"</Emphasis>""
            },
            mutation,
        ))
    }
}

pub struct JsSyntaxMatchPair {
    pub member_name: JsSyntaxToken,
    pub matching_array_element: JsSyntaxNode,
}

fn member_name_remap(old_member_name: &str) -> Option<&str> {
    match old_member_name {
        "findIndex" => Some("indexOf"),
        "findLastIndex" => Some("lastIndexOf"),
        _ => None,
    }
}

fn extract_simple_compare_match(
    expression: &JsBinaryExpression,
    parameter_name: &Text,
) -> Option<JsSyntaxNode> {
    if expression.operator_token().ok()?.kind() != T![===] {
        return None;
    }

    let (left, right) = (expression.left().ok()?, expression.right().ok()?);

    let matching_side = if left.to_trimmed_text() == *parameter_name {
        right
    } else if right.to_trimmed_text() == *parameter_name {
        left
    } else {
        return None;
    };

    Some(matching_side.into_syntax())
}

fn find_index_comparable_expression(
    body: &AnyJsFunctionBody,
    parameter_name: &Text,
    return_statement_required: bool,
) -> Option<JsSyntaxNode> {
    let has_invalid_expression = body.syntax().descendants().find(|node| {
        JsAssignmentExpression::can_cast(node.kind())
            || JsVariableDeclaration::can_cast(node.kind())
            || JsLogicalExpression::can_cast(node.kind())
    });

    if has_invalid_expression.is_some() {
        return None;
    }

    let mut binary_expressions = body
        .syntax()
        .descendants()
        .filter_map(JsBinaryExpression::cast);

    let binary_expression = binary_expressions.next()?;
    if binary_expressions.next().is_some() {
        return None;
    }

    let mut return_statements = body
        .syntax()
        .descendants()
        .filter_map(JsReturnStatement::cast);
    let has_one_or_more_return_statements = return_statements.next().is_some();
    let has_two_or_more_return_statements = return_statements.next().is_some();

    if has_two_or_more_return_statements {
        return None;
    }

    if return_statement_required && !has_one_or_more_return_statements {
        return None;
    }

    extract_simple_compare_match(&binary_expression, parameter_name)
}

fn extract_function_parameter_name(parameters: &JsParameterList) -> Option<Text> {
    if parameters.len() != 1 {
        return None;
    }

    Some(parameters.first()?.ok()?.to_trimmed_text())
}

fn callback_function_match(
    function: &JsFunctionExpression,
    member_name_token: JsSyntaxToken,
) -> Option<JsSyntaxMatchPair> {
    if function.async_token().is_some() || function.star_token().is_some() {
        return None;
    }

    let function_parameters = function.parameters().ok()?.items();
    let parameter_name = extract_function_parameter_name(&function_parameters)?;
    let binding = function.body().ok()?;
    let body = binding
        .syntax()
        .descendants()
        .find_map(AnyJsFunctionBody::cast)?;

    let matched = find_index_comparable_expression(&body, &parameter_name, true);

    matched.as_ref().map(|token_match| JsSyntaxMatchPair {
        matching_array_element: token_match.clone(),
        member_name: member_name_token,
    })
}

fn extract_parameter_name(parameters: &AnyJsArrowFunctionParameters) -> Option<Text> {
    if parameters.len() != 1 {
        return None;
    }

    match parameters {
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => Some(binding.to_trimmed_text()),
        AnyJsArrowFunctionParameters::JsParameters(param) => param
            .items()
            .first()?
            .ok()
            .map(|item| item.to_trimmed_text()),
    }
}

fn callback_arrow_function_match(
    function: &JsArrowFunctionExpression,
    member_name_token: JsSyntaxToken,
) -> Option<JsSyntaxMatchPair> {
    if function.async_token().is_some() {
        return None;
    }

    let parameters = function.parameters().ok()?;
    let parameter_name = extract_parameter_name(&parameters)?;
    let body = function.body().ok()?;

    let matched = find_index_comparable_expression(&body, &parameter_name, false);

    matched.as_ref().map(|token_match| JsSyntaxMatchPair {
        matching_array_element: token_match.clone(),
        member_name: member_name_token,
    })
}
