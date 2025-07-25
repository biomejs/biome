use crate::services::typed::Typed;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_js_syntax::AnyJsStatement::{
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement, JsReturnStatement,
    JsSwitchStatement, JsVariableStatement, JsWhileStatement,
};
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsBinaryExpression, JsBooleanLiteralExpression,
    JsConditionalExpression, JsIdentifierExpression, JsLogicalExpression,
    JsParenthesizedExpression, JsVariableDeclaration, TextRange,
};
use biome_js_type_info::{Literal, Type, TypeData};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, Text, declare_node_union};
use biome_rule_options::no_unnecessary_conditions::NoUnnecessaryConditionsOptions;
use std::ops::Deref;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoUnnecessaryConditions {
        version: "next",
        name: "noUnnecessaryConditions",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUnnecessaryConditions {
    type Query = Typed<AnyJsStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnnecessaryConditionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        match_any_js_statement(expression, ctx);

        Some(())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        // let node = ctx.query();
        // Some(
        //     RuleDiagnostic::new(
        //         rule_category!(),
        //         node.range(),
        //         markup! {
        //             "Variable is read here."
        //         },
        //     )
        //     .note(markup! {
        //         "This note will give you more information."
        //     }),
        // )
        None
    }
}

// TODO move in favour of pure expression matching later !!!
fn match_any_js_statement(
    any_js_statement: &AnyJsStatement,
    ctx: &RuleContext<NoUnnecessaryConditions>,
) {
    match any_js_statement {
        JsVariableStatement(statement) => {
            println!("js variable declaration {:?}", statement.declaration());

            let expression = statement.declaration().ok().and_then(|declaration| {
                JsVariableDeclaration::cast_ref(declaration.syntax()).and_then(|declaration| {
                    declaration.declarators().iter().find_map(|declarator| {
                        declarator
                            .ok()?
                            .initializer()
                            .and_then(|initializer| initializer.expression().ok())
                    })
                })
            });

            if let Some(expression) = expression {
                println!("expression in variable declaration: {:?}", expression);
                // Process the expression
                match_expression(&expression, ctx);
            }

            ()
        }
        // TODO this
        // x.a.b?.c -> member is c
        JsExpressionStatement(statement) => {
            println!("expression: {:?}", statement.expression());
            if let Ok(expression) = statement.expression() {
                match_expression(&expression, ctx);
            }
            ()
        }
        JsDoWhileStatement(statement) => {
            println!("do while: {:?}", statement);
            statement.test().ok().and_then(|expression| {
                println!("do while: {:?}", expression);
                match_expression(&expression, ctx);

                None::<&str>
            });
            ()
        }
        JsForStatement(statement) => {
            println!("for: {:?}", statement);
            if let Some(expression) = statement.test() {
                match_expression(&expression, ctx);
            }
            ()
        }
        JsIfStatement(statement) => {
            println!("if: {:?}", statement);
            if let Some(expression) = statement.test().ok() {
                match_expression(&expression, ctx);
            }

            ()
        }
        // corresponds to Switch Case todo add visit fn instead
        JsSwitchStatement(statement) => {
            println!("switch: {:?}", statement);
            if let Some(discriminant) = statement.discriminant().ok() {
                // also cover if NON static value, but its binding is either static or matches non null type ts
                if discriminant.as_static_value().is_some() {
                    statement.cases().iter().for_each(|case| {
                        if let Some(case_clause) = case.as_js_case_clause()
                            && let Some(test) = case_clause.test().ok()
                        {
                            match_expression(&test, ctx);
                        }
                    });
                } else {
                    match_expression(&discriminant, ctx);
                }
            }
            ()
        }
        JsWhileStatement(statement) => {
            println!("while: {:?}", statement);
            if let Some(expression) = statement.test().ok() {
                match_expression(&expression, ctx);
            }
            ()
        }
        JsReturnStatement(statement) => {
            println!("return stmt: {:?}", statement);
            if let Some(expression) = statement.argument() {
                if let Some(expression) = expression.as_js_static_member_expression() {
                    if let Ok(expression) = expression.object() {
                        match_expression(&expression, ctx);
                    }
                }
            }
            ()
        }
        _ => (),
    }
}

// pass third optional parameter to match a value - for case statement for instance
fn match_expression(expression: &AnyJsExpression, ctx: &RuleContext<NoUnnecessaryConditions>) {
    // println!("match_expression {:?}", expression);

    // should not allow anything but static values, identifiers, and expressions - add hard type check
    let mut collect_static_values_or_identifiers = |expression: &AnyJsExpression| {
        // Do something with expression
        let mut matches: Vec<DiagnosticMessageRange> = Vec::new();
        // println!("Visited: {:?}", expression.syntax());

        if let Some(static_value) = expression.as_static_value() {
            println!("static_value is: {:?} ", static_value);
            matches.push(DiagnosticMessageRange {
                range: static_value.range(),
                message: Text::from(static_value.text().to_string()),
            });
        } else if let Some(js_identifier_expression) =
            JsIdentifierExpression::cast_ref(expression.syntax())
        {
            let expression_type = ctx.type_of_expression(expression);

            if is_literal_or_null_or_undefined(&expression_type) {
                matches.push(DiagnosticMessageRange {
                    range: js_identifier_expression.range(),
                    message: js_identifier_expression.to_trimmed_text(),
                });
            }

            // expression.syntax().kind() == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION
            println!(
                "JsIdentifierExpression is: {:?} {:?}  {:?}  {:?}  {:?} {:?}",
                expression_type.conditional_semantics().is_falsy(),
                expression_type.conditional_semantics().is_truthy(),
                expression_type.conditional_semantics().is_inferred(),
                expression_type,
                expression,
                is_literal_or_null_or_undefined(&expression_type),
            );
            // JsParameterOrVariableDeclarator::try_cast_ref_and_process(&expression, &ctx);
            // todo push diagnostic message
            // parenthesized expressions, proceed with the inner expression
        }

        // matches.iter().for_each(|matched| {
        //     println!("matched ${:?}", matched);
        // });
    };

    if let Some(conditional_expression) = expression.as_js_conditional_expression() {
        // if (arg) {} else {}
        println!("conditional expression: {:?}", conditional_expression);
        JsConditionalExpressionVisit::visit(
            &conditional_expression,
            &mut collect_static_values_or_identifiers,
        );
    }
    if let Some(arrow_fn) = expression.as_js_arrow_function_expression() {
        println!("arrow_fn is: {:?} ", arrow_fn);
        // matches.push(DiagnosticMessageRange {
        //     range: static_value.range(),
        //     message: Text::from(static_value.text().to_string()),
        // });
    } else if let Some(call_expression) = expression.as_js_call_expression() {
        println!("call expression: {:?}", call_expression);
        // can be a function call, or [].each call
        println!(
            "call expression arguments: {:?}",
            call_expression
                .callee()
                .ok()
                .unwrap()
                .as_js_call_expression()
        );
        // collect_static_values_or_identifiers(&expression);
    }

    if expression.as_static_value().is_some() {
        collect_static_values_or_identifiers(&expression);
    }

    // single if (arg) {}
    expression
        .as_js_reference_identifier()
        .and_then(|identifier| {
            println!(
                "TODO implement this branch of logic if test as_js_reference_identifier {:?}",
                identifier
            );

            Some(identifier.value_token())
        });

    // if (arg1, arg2, ...) {}
    if let Some(any_js_binary_expression) =
        AnyJsBinaryOrLogicalExpression::cast_ref(expression.syntax())
    {
        AnyJsBinaryOrLogicalExpression::visit(
            &any_js_binary_expression,
            &mut collect_static_values_or_identifiers,
        );
    }
}

#[derive(Debug)]
pub struct DiagnosticMessageRange {
    pub message: Text,
    pub range: TextRange,
}

declare_node_union! {
    pub AnyJsBinaryOrLogicalExpression = JsLogicalExpression | JsBinaryExpression
}

#[derive(Debug, Clone)]
pub enum AnyJsBinaryOrLogicalCallbackArgument {
    Identifier(JsIdentifierExpression),
    BooleanLiteral(JsBooleanLiteralExpression),
    Static(StaticValue),
}

pub struct JsConditionalExpressionVisit;
impl JsConditionalExpressionVisit {
    pub fn visit(expression: &JsConditionalExpression, callback: &mut dyn FnMut(&AnyJsExpression)) {
        // Visit test
        if let Ok(test) = expression.test() {
            Self::visit_descendant(&test, callback);
        }

        // Visit consequent
        if let Ok(consequent) = expression.consequent() {
            Self::visit_descendant(&consequent, callback);
        }

        // Visit alternate
        if let Ok(alternate) = expression.alternate() {
            Self::visit_descendant(&alternate, callback);
        }
    }

    fn visit_descendant(expr: &AnyJsExpression, callback: &mut dyn FnMut(&AnyJsExpression)) {
        if let Some(logical_or_binary) = AnyJsBinaryOrLogicalExpression::cast_ref(expr.syntax()) {
            AnyJsBinaryOrLogicalExpression::visit(&logical_or_binary, callback);
        } else if let Some(nested_conditional) = JsConditionalExpression::cast_ref(expr.syntax()) {
            Self::visit(&nested_conditional, callback);
        } else if let Some(paren_expr) = JsParenthesizedExpression::cast_ref(expr.syntax()) {
            if let Ok(inner) = paren_expr.expression() {
                Self::visit_descendant(&inner, callback);
            }
        }
    }
}

impl AnyJsBinaryOrLogicalExpression {
    fn left(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsLogicalExpression(logical) => logical.left(),
            Self::JsBinaryExpression(binary) => binary.left(),
        }
    }

    fn right(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsLogicalExpression(logical) => logical.right(),
            Self::JsBinaryExpression(binary) => binary.right(),
        }
    }

    fn visit_descendant(expr: &AnyJsExpression, callback: &mut dyn FnMut(&AnyJsExpression)) {
        // println!("visit descendant: {:?}", expr.syntax());
        if let Some(logical_expr) = Self::cast_ref(expr.syntax()) {
            Self::visit(&logical_expr, callback);
        } else if JsIdentifierExpression::cast_ref(expr.syntax()).is_some()
            || JsBooleanLiteralExpression::cast_ref(expr.syntax()).is_some()
            || expr.as_static_value().is_some()
        {
            callback(&expr);
        } else if let Some(paren_expr) = JsParenthesizedExpression::cast_ref(expr.syntax()) {
            if let Ok(inner_expr) = paren_expr.expression() {
                Self::visit_descendant(&inner_expr, callback);
            }
        } else if let Some(expr) = JsConditionalExpression::cast_ref(expr.syntax()) {
            println!("must visit conditional expression: {:?}", expr);
            JsConditionalExpressionVisit::visit(&expr, callback);
        }
    }

    pub fn visit(
        expression: &AnyJsBinaryOrLogicalExpression,
        callback: &mut dyn FnMut(&AnyJsExpression),
    ) {
        if let Ok(left) = expression.left() {
            Self::visit_descendant(&left, callback);
        }

        if let Ok(right) = expression.right() {
            Self::visit_descendant(&right, callback);
        }
    }
}

// need to add more types / non literals see if we can use alternative to this
fn is_literal_or_null_or_undefined(ty: &Type) -> bool {
    match ty.deref() {
        TypeData::Literal(_) => true,
        // todo decide about null and undefined
        TypeData::Null | TypeData::Undefined => true,
        _ => false,
    }
}

fn type_to_string(ty: &Type) -> String {
    match ty.deref() {
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::Boolean(b) => b.as_bool().to_string(),
            Literal::Number(n) => n.text().to_string(),
            Literal::String(s) => format!("\"{}\"", s.as_str()),
            _ => "unknown".to_string(),
        },
        TypeData::Null => "null".to_string(),
        TypeData::Undefined => "undefined".to_string(),
        _ => "unknown".to_string(),
    }
}
