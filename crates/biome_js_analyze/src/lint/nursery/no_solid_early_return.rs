use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule
};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsStatement, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsReturnStatement,
};
use biome_rowan::AstNode;

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

/// Returns `true` if the function is (probably) a component (because it is PascalCase)
fn is_component_name(name: &str) -> bool {
    name.chars()
        .next()
        .map(|x| x.is_uppercase())
        .unwrap_or_default()
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
    Conditional,
    EarlyReturn,
}

impl ReturnType {
    pub fn get_message(&self) -> &str {
        match self {
            Self::Conditional => {
                "Solid components run once, so a conditional return breaks reactivity. Move the condition inside a JSX element, such as a fragment or <Show />."
            }
            Self::EarlyReturn => {
                "Solid components run once, so an early return breaks reactivity. Move the condition inside a JSX element, such as a fragment or <Show />."
            }
        }
    }
}

/// Check if an expression contains a conditional (ternary) operator or logical operators
fn has_conditional_expression(expr: &AnyJsExpression) -> bool {
    matches!(
        expr,
        AnyJsExpression::JsConditionalExpression(_) | AnyJsExpression::JsLogicalExpression(_)
    )
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
            if let Some(alternate) = if_stmt.else_clause() {
                if let Ok(alternate_stmt) = alternate.alternate() {
                    collect_return_statements_shallow(&alternate_stmt, returns);
                }
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

impl Rule for NoSolidEarlyReturn {
    type Query = Ast<AnyJsFunction>;
    type State = Vec<(ReturnType, JsReturnStatement)>;
    type Signals = Option<Self::State>;
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
            return None;
        }

        let body = func.body().ok()?;
        let body = body.as_js_function_body()?;

        let mut all_returns = vec![];

        for statement in body.statements() {
            collect_return_statements_shallow(&statement, &mut all_returns);
        }

        let mut problematic_returns = vec![];

        for ret in &all_returns {
            if let Some(arg) = ret.argument() {
                if has_conditional_expression(&arg) {
                    problematic_returns.push((ReturnType::Conditional, ret.clone()));
                }
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

                    if !if_returns.is_empty() {
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
        }

        if problematic_returns.is_empty() {
            None
        } else {
            Some(problematic_returns)
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut issues = state.iter();

        if let Some((return_type, statement)) = issues.next() {
            let span = statement.return_token().ok()?.text_range();

            let mut diagnostic =
                RuleDiagnostic::new(rule_category!(), span, return_type.get_message());

            for (return_type, statement) in issues {
                if let Ok(token) = statement.return_token() {
                    diagnostic = diagnostic.detail(token.text_range(), return_type.get_message());
                }
            }

            Some(diagnostic)
        } else {
            None
        }
    }
}
