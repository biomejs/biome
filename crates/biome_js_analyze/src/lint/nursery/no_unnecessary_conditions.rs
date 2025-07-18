use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::AnyJsStatement::{
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement, JsSwitchStatement,
    JsVariableStatement, JsWhileStatement,
};
use biome_js_syntax::JsSyntaxKind::QUESTIONDOT;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsBinaryExpression, JsBooleanLiteralExpression,
    JsFormalParameter, JsLogicalExpression, JsReferenceIdentifier, JsSyntaxNode,
    JsVariableDeclarator, TextRange,
};
use biome_rowan::{AstNode, SyntaxError, SyntaxNodeCast};
use biome_rule_options::no_unnecessary_conditions::NoUnnecessaryConditionsOptions;

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
    }
}

impl Rule for NoUnnecessaryConditions {
    type Query = Semantic<AnyJsStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnnecessaryConditionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        let model = ctx.model();
        println!("binding is {:?}", expression.syntax());

        match_js_statement(expression, model);
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

fn match_js_statement(expression: &AnyJsStatement, model: &SemanticModel) {
    match expression {
        JsVariableStatement(statement) => {
            println!("js variable statement: {:?}", statement);
            // let expression = statement
            //     .declaration()
            //     .ok()?
            //     .declarators()
            //     .iter()
            //     .find_map(|declarator| declarator.ok()?.initializer()?.expression().ok())?;
            // match_expression(&expression, model);
            println!(
                "js variable statement cover me {:?}",
                statement.declaration()
            );
            ()
        }
        // x.a.b?.c -> member is c
        JsExpressionStatement(statement) => {
            println!("expression: {:?}", statement.expression());
            if let Ok(expression) = statement.expression() {
                match_expression(&expression, model)
            }
            ()
        }
        JsDoWhileStatement(statement) => {
            println!("do while: {:?}", statement);
            ()
        }
        JsForStatement(statement) => {
            println!("for: {:?}", statement);
            ()
        }
        JsIfStatement(statement) => {
            println!("if: {:?}", statement);
            statement.test().ok().and_then(|test| {
                println!("if test statement {:?}", test);
                // single if (arg) {}
                test.as_js_reference_identifier().and_then(|identifier| {
                    // find definition
                    if let Some(binding) = model.binding(&identifier.clone()) {
                        if let Some(parent) = &binding.syntax().parent() {
                            println!("if test statement parent {:?}", binding.syntax().parent());
                            // let param: 'One' | 'two';
                            if let Some(parameter) = JsFormalParameter::cast_ref(parent) {
                                //todo extract all possible typs in union and pass to this method instead
                                TsUtil::process_definition(parameter.syntax());
                            }

                            if let Some(parameter) = JsVariableDeclarator::cast_ref(parent) {
                                //todo extract all possible typs in union and pass to this method instead
                                TsUtil::process_definition(parameter.syntax());
                            }
                        }
                    }

                    Some(identifier.value_token())
                });

                // if (arg1, arg2, ...) {}
                test.as_js_logical_expression()
                    .and_then(|js_logical_expression| {
                        let mut matches: Vec<TextRange> = Vec::new();
                        collect_boolean_literals_from_logical_expression(
                            js_logical_expression,
                            &mut matches,
                        );
                        matches.iter().for_each(|matched| {
                            println!("matched ${:?}", matched);
                        });
                        None::<&str>
                    });

                None::<&str>
            });

            ()
        }
        // corresponds to Switch Case
        JsSwitchStatement(statement) => {
            println!("switch: {:?}", statement);
            ()
        }
        JsWhileStatement(statement) => {
            println!("while: {:?}", statement);
            ()
        }
        _ => (),
    }
}

fn check_if_bool_expression_is_necessary_conditional(
    expression: &JsBinaryExpression,
    model: &SemanticModel,
) -> Option<String> {
    let left = expression.left().ok()?;
    let right = expression.right().ok()?;

    let left_value = left.as_static_value()?;
    let right_value = right.as_static_value()?;

    let arguments_reference = expression
        .syntax()
        .descendants()
        .filter_map(|x| x.cast::<JsReferenceIdentifier>())
        .find(|x| x.to_trimmed_string() == "arguments")
        .unwrap();
    let binding = model.binding(&arguments_reference);

    println!("model binding left: {:?}", binding);

    Some(format!(
        "Unnecessary conditional: Values for left `{}` and right `{}` never change.",
        left_value.text(),
        right_value.text()
    ))
}

fn match_expression(expression: &AnyJsExpression, model: &SemanticModel) {
    match expression {
        AnyJsExpression::JsAssignmentExpression(expression) => {
            println!("assignment expression: {:?}", expression);
        }
        AnyJsExpression::JsBinaryExpression(expression) => {
            if expression.is_comparison_operator() {
                let message: Result<_, SyntaxError> = Ok(
                    check_if_bool_expression_is_necessary_conditional(&expression, model),
                );
                println!("binary expression: {:?}", message);
            }
        }
        AnyJsExpression::JsCallExpression(expression) => {
            println!("call expression: {:?}", expression);
        }
        AnyJsExpression::JsConditionalExpression(expression) => {
            println!("conditional expression: {:?}", expression);
        }
        AnyJsExpression::JsLogicalExpression(expression) => {
            println!("logical expression: {:?}", expression);
        }
        AnyJsExpression::JsStaticMemberExpression(expression) => {
            if expression
                .operator_token()
                .is_ok_and(|token| token.kind().eq(&QUESTIONDOT))
            {
                println!("elvis member rules .?: {:?}", expression.member());
            }
            println!("static member expression: {:?}", expression);
        }
        // todo maybe not needed
        AnyJsExpression::JsComputedMemberExpression(expression) => {
            println!("computed member need??? expression: {:?}", expression);
        }
        _ => (),
    }
}

pub struct TsUtil;

impl TsUtil {
    // check type for 'one' | 'two' or Any[], etc, possibly just check negation - undefined, null, etc
    pub fn process_definition(argument: &JsSyntaxNode) {
        println!("process definition of: {:?}", argument);
    }
}

/// Recursively collects the text ranges of all boolean literal expressions and static values
/// from a given `JsLogicalExpression`.
///
/// This function traverses both the left and right sides of a logical expression.
/// It collects:
/// - Boolean literals like `true` or `false`.
/// - Any static value expressions (e.g., string, number literals).
/// - Nested logical expressions are visited recursively.
///
/// # Arguments
/// * `expression` - A reference to the `JsLogicalExpression` to search within.
/// * `booleans` - A mutable vector where the found `TextRange` values will be pushed.
///
/// # Example
/// ```rust
/// // For an input like:  if (arg && true && false && false || 1 || 'a' ...)
/// // This function will collect the ranges for: arg, true, false, 1, and 'a'. <- arg if it matches tsResolved
/// ```
fn collect_boolean_literals_from_logical_expression(
    expression: &JsLogicalExpression,
    booleans: &mut Vec<TextRange>,
) {
    let mut check = |expr: &AnyJsExpression| {
        if let Some(bool_expr) = JsBooleanLiteralExpression::cast_ref(expr.syntax()) {
            booleans.push(bool_expr.syntax().text_trimmed_range());
        } else if let Some(static_value) = expr.as_static_value() {
            booleans.push(static_value.range());
        } else if let Some(logical_expr) = JsLogicalExpression::cast_ref(expr.syntax()) {
            collect_boolean_literals_from_logical_expression(&logical_expr, booleans);
        }
    };

    if let Ok(left) = expression.left() {
        check(&left);
    }

    if let Ok(right) = expression.right() {
        check(&right);
    }
}
