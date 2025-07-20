use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::AnyJsStatement::{
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement, JsSwitchStatement,
    JsVariableStatement, JsWhileStatement,
};
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsBinaryExpression, JsBooleanLiteralExpression,
    JsFormalParameter, JsIdentifierExpression, JsLogicalExpression, JsParenthesizedExpression,
    JsSyntaxNode, JsVariableDeclaration, JsVariableDeclarator, TextRange,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, Text, declare_node_union};
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

        match_any_js_statement(expression, model);
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
fn match_any_js_statement(any_js_statement: &AnyJsStatement, model: &SemanticModel) {
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

            // println!("expression in variable declaration: {:?}", expression);
            if let Some(expression) = expression {
                // Process the expression
                match_expression(&expression, model);
            }

            ()
        }
        // TODO this
        // x.a.b?.c -> member is c
        JsExpressionStatement(statement) => {
            println!("expression: {:?}", statement.expression());
            if let Ok(expression) = statement.expression() {
                match_expression(&expression, model);
            }
            ()
        }
        JsDoWhileStatement(statement) => {
            println!("do while: {:?}", statement);
            statement.test().ok().and_then(|expression| {
                println!("do while: {:?}", expression);
                match_expression(&expression, model);

                None::<&str>
            });
            ()
        }
        JsForStatement(statement) => {
            println!("for: {:?}", statement);
            if let Some(expression) = statement.test() {
                match_expression(&expression, model);
            }
            ()
        }
        JsIfStatement(statement) => {
            println!("if: {:?}", statement);
            if let Some(expression) = statement.test().ok() {
                match_expression(&expression, model);
            }

            ()
        }
        // corresponds to Switch Case
        JsSwitchStatement(statement) => {
            println!("switch: {:?}", statement);
            if let Some(discriminant) = statement.discriminant().ok() {
                // also cover if NON static value, but its binding is either static or matches non null type ts
                if discriminant.as_static_value().is_some() {
                    statement.cases().iter().for_each(|case| {
                        if let Some(case_clause) = case.as_js_case_clause()
                            && let Some(test) = case_clause.test().ok()
                        {
                            match_expression(&test, model);
                        }
                    });
                } else {
                    // TODO check if discriminant is 2>3 or has error
                    match_expression(&discriminant, model);
                }
                // match_expression(&discriminant, model);
            }
            ()
        }
        JsWhileStatement(statement) => {
            println!("while: {:?}", statement);
            if let Some(expression) = statement.test().ok() {
                match_expression(&expression, model);
            }
            ()
        }
        _ => (),
    }
}

// pass third optional parameter to match a value - for case statement for instance
fn match_expression(expression: &AnyJsExpression, model: &SemanticModel) {
    println!("match_expression {:?}", expression);

    let mut visitor = |expression: &AnyJsExpression| {
        // Do something with expression
        let mut matches: Vec<DiagnosticMessageRange> = Vec::new();
        println!("Visited: {:?}", expression.syntax().text_trimmed_range());

        if let Some(static_value) = expression.as_static_value() {
            matches.push(DiagnosticMessageRange {
                range: static_value.range(),
                message: Text::Owned(static_value.text().to_string()),
            });
        }

        if let Some(binding) = JsIdentifierExpression::cast_ref(expression.syntax())
            .and_then(|expression| {
                let name = expression.name().ok()?;
                Some(name)
            })
            .and_then(|js_reference_identifier| model.binding(&js_reference_identifier))
            && let Some(parent) = binding.syntax().parent()
        {
            JsParameterOrVariableDeclarator::try_cast_ref_and_process(&parent);
            // todo push diagnostic message
            // parenthesized expressions, proceed with the inner expression
        }

        matches.iter().for_each(|matched| {
            println!("matched ${:?}", matched);
        });
    };

    if expression.as_static_value().is_some() {
        visitor(&expression);
    }

    // single if (arg) {}
    expression
        .as_js_reference_identifier()
        .and_then(|identifier| {
            println!("if test as_js_reference_identifier {:?}", identifier);
            // find definition

            // if let Some(binding) = model.binding(&identifier.clone()) {
            //     if let Some(parent) = &binding.syntax().parent() {
            //         println!("if test statement parent {:?}", binding.syntax().parent());
            //         visitor(&binding.syntax().parent());
            //         JsParameterOrVariableDeclarator::try_cast_ref_and_process(parent);
            //     }
            // }

            Some(identifier.value_token())
        });

    // if (arg1, arg2, ...) {}
    expression
        .as_js_logical_expression()
        .and_then(|js_logical_expression| {
            if let Some(any_js_binary_expression) =
                AnyJsBinaryOrLogicalExpression::cast_ref(js_logical_expression.syntax())
            {
                AnyJsBinaryOrLogicalExpression::visit(&any_js_binary_expression, &mut visitor);
            }
            None::<&str>
        });

    expression
        .as_js_binary_expression()
        .and_then(|js_binary_expression| {
            if let Some(any_js_binary_expression) =
                AnyJsBinaryOrLogicalExpression::cast_ref(js_binary_expression.syntax())
            {
                println!(
                    "if test any_js_binary_expression {:?}",
                    any_js_binary_expression
                );

                AnyJsBinaryOrLogicalExpression::visit(&any_js_binary_expression, &mut visitor);
            }
            None::<&str>
        });
}

declare_node_union! {
    pub JsParameterOrVariableDeclarator = JsFormalParameter | JsVariableDeclarator
}

impl JsParameterOrVariableDeclarator {
    // todo clean up return type!!
    pub fn try_cast_ref_and_process(argument: &JsSyntaxNode) -> Option<()> {
        // println!("try cast and process with argument: {:?}", argument);
        if let Some(parameter) = JsParameterOrVariableDeclarator::cast_ref(argument) {
            // println!("try cast success: {:?}", parameter);
            Self::process_argument_or_variable_initialization(&parameter);
            Some(())
        } else {
            None
        }
    }

    // check type for 'one' | 'two' or Any[], etc, possibly just check negation - undefined, null, etc
    // cover classes here!!!
    fn process_argument_or_variable_initialization(argument: &JsParameterOrVariableDeclarator) {
        // println!("enter process_initialization with argument: {:?}", argument);
        if let Some(statement) = JsVariableDeclarator::cast_ref(argument.syntax()) {
            // println!("variable declarator: {:?}", statement);
            let declaration_option = statement.declaration();

            let expression = declaration_option.and_then(|declaration| {
                if declaration.is_const().eq(&false) {
                    return None;
                }
                let declaration = JsVariableDeclaration::cast_ref(declaration.syntax())?;
                let declarators = declaration.declarators();

                declarators.iter().find_map(|declarator| {
                    declarator
                        .ok()?
                        .initializer()
                        .and_then(|initializer| initializer.expression().ok())
                })
            });

            if let Some(expr) = expression {
                println!("Found expression in variable declaration: {:?}", expr);
                // should return name of variable and type
                // match_any_js_expression(&expr, model);
            }
        } else if let Some(statement) = JsFormalParameter::cast_ref(argument.syntax()) {
            println!("processing JsFormalParameter: {:?}", statement);
        }

        // println!(
        //     "process definition of: {:?}",
        //     DiagnosticMessageRange {
        //         message: Text::from("Processing definition..."),
        //         range: argument.text_trimmed_range(),
        //     }
        // );
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
        }
    }
    // todo tighten the callback fn type to just AnyJsBinaryOrLogicalCallbackArgument
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
