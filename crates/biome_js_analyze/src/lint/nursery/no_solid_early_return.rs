use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsStatement, AnyJsxChild, AnyJsxTag, JsCallArgumentList,
    JsCallArguments, JsCallExpression, JsConditionalExpression, JsLogicalExpression,
    JsReturnStatement, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow early returns in Solid components.
    ///
    /// Solid components only run once, and so conditionals should be inside JSX.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function Component() {
    ///   if (condition) {
    ///     return <div />;
    ///   }
    ///   return <span />;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function Component() {
    ///   return <div />;
    /// }
    /// ```
    ///
    pub NoSolidEarlyReturn {
        version: "next",
        name: "noSolidEarlyReturn",
        language: "js",
        sources: &[RuleSource::EslintSolid("components-return-once").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Solid],
    }
}

impl Rule for NoSolidEarlyReturn {
    type Query = Ast<AnyJsFunction>;
    type State = (ReturnType, JsReturnStatement);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let func = ctx.query();

        let is_component = func
            .binding()
            .and_then(|b| b.as_js_identifier_binding().cloned())
            .and_then(|ident| ident.name_token().ok())
            .is_some_and(|name| is_component_name(name.text()))
            || is_argument_of_hoc(func);

        if !is_component {
            return Vec::new();
        }

        let Some(body) = func
            .body()
            .ok()
            .and_then(|b| b.as_js_function_body().cloned())
        else {
            return Vec::new();
        };

        let mut all_returns = vec![];

        for statement in body.statements() {
            collect_return_statements_shallow(&statement, &mut all_returns);
        }

        let mut problematic_returns = vec![];

        for ret in &all_returns {
            if let Some(arg) = ret.argument()
                && let Some(cond_type) = get_conditional_type(&arg)
            {
                problematic_returns.push((ReturnType::Conditional(cond_type), ret.clone()));
            }
        }

        if all_returns.len() > 1 {
            for statement in body.statements() {
                if let AnyJsStatement::JsIfStatement(if_stmt) = statement {
                    let mut if_returns = vec![];
                    collect_return_statements_shallow(
                        &AnyJsStatement::from(if_stmt.clone()),
                        &mut if_returns,
                    );

                    for ret in if_returns {
                        if !problematic_returns
                            .iter()
                            .any(|(_, r)| r.syntax() == ret.syntax())
                        {
                            problematic_returns.push((ReturnType::EarlyReturn, ret));
                        }
                    }
                }
            }
        }

        problematic_returns
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (return_type, statement) = state;
        let span = statement.return_token().ok()?.text_range();

        let message = match return_type {
            ReturnType::Conditional(_) => "This conditional return breaks reactivity.",
            ReturnType::EarlyReturn => "This early return breaks reactivity.",
        };

        Some(
            RuleDiagnostic::new(rule_category!(), span, message).note(
                "Solid components run once. Moving the condition inside JSX ensures reactivity is preserved.",
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let (return_type, ret_stmt) = state;

        // Only provide fixes for conditional expressions, not early returns
        let cond_type = match return_type {
            ReturnType::Conditional(cond_type) => cond_type,
            ReturnType::EarlyReturn => return None,
        };

        let arg = ret_stmt.argument()?;
        let mut mutation = ctx.root().begin();

        // Create a fragment wrapping the conditional expression: <>{expr}</>
        let opening_fragment = make::jsx_opening_fragment(make::token(T![<]), make::token(T![>]));
        let closing_fragment =
            make::jsx_closing_fragment(make::token(T![<]), make::token(T![/]), make::token(T![>]));

        // Wrap the expression in a JsxExpressionChild: {expr}
        let jsx_expr_child = make::jsx_expression_child(make::token(T!['{']), make::token(T!['}']))
            .with_expression(arg.clone())
            .build();

        let children = make::jsx_child_list([AnyJsxChild::JsxExpressionChild(jsx_expr_child)]);

        let fragment = make::jsx_fragment(opening_fragment, children, closing_fragment);

        // Create a JsxTagExpression to wrap the fragment
        let jsx_tag_expr = make::jsx_tag_expression(AnyJsxTag::JsxFragment(fragment));

        // Replace the expression in the return statement
        mutation.replace_node(arg, AnyJsExpression::JsxTagExpression(jsx_tag_expr));

        let message = match cond_type {
            ConditionalType::Ternary(_) => "Wrap the ternary expression in a fragment.",
            ConditionalType::Logical(_) => "Wrap the logical expression in a fragment.",
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { {message} }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if the function is (probably) a component (because it is PascalCase)
fn is_component_name(name: &str) -> bool {
    name.chars().next().is_some_and(|x| x.is_uppercase())
}

/// Check if the function is passed as an argument to a HOC (Higher Order Component)
/// A HOC is identified by a PascalCase function name, e.g. `HOC(() => { ... })`
fn is_argument_of_hoc(func: &AnyJsFunction) -> bool {
    func.syntax()
        .parent()
        .and_then(JsCallArgumentList::cast)
        .and_then(|list| list.parent::<JsCallArguments>())
        .and_then(|args| args.parent::<JsCallExpression>())
        .and_then(|call| call.callee().ok())
        .and_then(|callee| callee.as_js_reference_identifier())
        .is_some_and(|ident| ident.name().is_ok_and(|name| is_component_name(&name)))
}

pub enum ReturnType {
    /// Conditional expression (ternary or logical) in a return statement
    Conditional(ConditionalType),
    /// Early return in an if statement
    EarlyReturn,
}

/// The type of conditional expression found in a return statement
pub enum ConditionalType {
    /// A ternary expression: `condition ? consequent : alternate`
    Ternary(JsConditionalExpression),
    /// A logical expression: `a && b` or `a || b`
    Logical(JsLogicalExpression),
}

/// Check if an expression is a conditional (ternary) operator or logical operator
/// Returns the conditional type if found
fn get_conditional_type(expr: &AnyJsExpression) -> Option<ConditionalType> {
    match expr {
        AnyJsExpression::JsConditionalExpression(cond) => {
            Some(ConditionalType::Ternary(cond.clone()))
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            Some(ConditionalType::Logical(logical.clone()))
        }
        _ => None,
    }
}

/// Collect return statements from a statement WITHOUT recursing into nested functions
fn collect_return_statements_shallow(
    statement: &AnyJsStatement,
    returns: &mut Vec<JsReturnStatement>,
) {
    match statement {
        AnyJsStatement::JsReturnStatement(ret) => {
            returns.push(ret.clone());
        }
        AnyJsStatement::JsIfStatement(if_stmt) => {
            if let Ok(consequent) = if_stmt.consequent() {
                collect_return_statements_shallow(&consequent, returns);
            }
            if let Some(alternate) = if_stmt.else_clause()
                && let Ok(alternate_stmt) = alternate.alternate()
            {
                collect_return_statements_shallow(&alternate_stmt, returns);
            }
        }
        AnyJsStatement::JsBlockStatement(block) => {
            for stmt in block.statements() {
                if !matches!(stmt, AnyJsStatement::JsFunctionDeclaration(_)) {
                    collect_return_statements_shallow(&stmt, returns);
                }
            }
        }
        AnyJsStatement::JsVariableStatement(_) => {}
        _ => {}
    }
}
