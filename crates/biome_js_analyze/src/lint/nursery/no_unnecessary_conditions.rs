use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::AnyJsStatement::{
    JsDoWhileStatement, JsExpressionStatement, JsForStatement, JsIfStatement, JsSwitchStatement,
    JsVariableStatement, JsWhileStatement,
};
use biome_js_syntax::static_value::StaticValue;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, AnyTsType, AnyTsVariableAnnotation, JsBinaryExpression,
    JsBooleanLiteralExpression, JsConditionalExpression, JsFormalParameter, JsIdentifierExpression,
    JsLogicalExpression, JsParenthesizedExpression, JsSyntaxNode, JsVariableDeclaration,
    JsVariableDeclarator, TextRange,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxResult, Text, declare_node_union,
};
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
         println!("root is {:?}", expression.syntax());

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
            // println!("js variable declaration {:?}", statement.declaration());

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
                            match_expression(&test, model);
                        }
                    });
                } else {
                    match_expression(&discriminant, model);
                }
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
    // println!("match_expression {:?}", expression);

    // should not allow anything but static values, identifiers, and expressions - add hard type check
    let mut collect_static_values_or_identifiers = |expression: &AnyJsExpression| {
        // Do something with expression
        let mut matches: Vec<DiagnosticMessageRange> = Vec::new();
        // println!("Visited: {:?}", expression.syntax());

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
            JsParameterOrVariableDeclarator::try_cast_ref_and_process(&parent, model);
            // todo push diagnostic message
            // parenthesized expressions, proceed with the inner expression
        }

        matches.iter().for_each(|matched| {
            println!("matched ${:?}", matched);
        });
    };

    if let Some(conditional_expression) = expression.as_js_conditional_expression() {
        // if (arg) {} else {}
        println!("conditional expression: {:?}", conditional_expression);
        JsConditionalExpressionVisit::visit(
            &conditional_expression,
            &mut collect_static_values_or_identifiers,
        );
    }

    if let Some(call_expression) = expression.as_js_call_expression() {
        println!("call expression: {:?}", call_expression);
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
    if let Some(any_js_binary_expression) =
        AnyJsBinaryOrLogicalExpression::cast_ref(expression.syntax())
    {
        AnyJsBinaryOrLogicalExpression::visit(
            &any_js_binary_expression,
            &mut collect_static_values_or_identifiers,
        );
    }
}

declare_node_union! {
    pub JsParameterOrVariableDeclarator = JsFormalParameter | JsVariableDeclarator
}

impl JsParameterOrVariableDeclarator {
    // todo clean up return type!!
    pub fn try_cast_ref_and_process(argument: &JsSyntaxNode, model: &SemanticModel) -> Option<()> {
         println!("try cast and process with argument: {:?}", argument);
        if let Some(parameter) = JsParameterOrVariableDeclarator::cast_ref(argument) {
            // println!("try cast success: {:?}", parameter);
            Self::process_argument_or_variable_initialization(&parameter, model);
            Some(())
        } else {
            None
        }
    }

    // check type for 'one' | 'two' or Any[], etc, possibly just check negation - undefined, null, etc
    // cover classes here!!!
    fn process_argument_or_variable_initialization(
        argument: &JsParameterOrVariableDeclarator,
        model: &SemanticModel,
    ) {
        fn resolve_expression_recursively(
            expression: AnyJsExpression,
            model: &SemanticModel,
            depth: usize,
        ) {
            if depth > 10 {
                println!("Max recursion depth reached");
                return;
            }

            if let Some(static_value) = expression.as_static_value() {
                println!(
                    "Resolved static value: {} {:?}",
                    static_value.text(),
                    expression.syntax().grand_parent().unwrap()
                );
                if let Some(grand_parent) = expression.syntax().grand_parent() {
                    if let Some(variable_declarator) = JsVariableDeclarator::cast_ref(&grand_parent)
                    {
                        println!("Variable declarator: {:?}", variable_declarator);

                        if let Some(annotation) = variable_declarator.variable_annotation() {
                            TsUtil::process_identifier(&annotation);
                        }
                    }
                }
                return;
            }

            if let Some(identifier) = JsIdentifierExpression::cast_ref(expression.syntax()) {
                if let Ok(name) = identifier.name() {
                    if let Some(binding) = model.binding(&name) {
                        if let Some(parent) = binding.syntax().parent() {
                            if let Some(declarator) =
                                JsParameterOrVariableDeclarator::cast_ref(&parent)
                            {
                                // Recurse into initializer
                                if let Some(var_decl) =
                                    JsVariableDeclarator::cast_ref(declarator.syntax())
                                {
                                    if let Some(init_expr) =
                                        var_decl.initializer().and_then(|i| i.expression().ok())
                                    {
                                        // println!(
                                        //     "Resolved identifier '{:?}' to initializer {:?}",
                                        //     name, init_expr
                                        // );
                                        resolve_expression_recursively(init_expr, model, depth + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                println!(
                    "Unhandled expression type: {:?}",
                    expression.syntax().kind()
                );
            }
        }

        if let Some(statement) = JsVariableDeclarator::cast_ref(argument.syntax()) {
            if let Some(declaration) = statement.declaration() {
                if !declaration.is_const() {
                    return;
                }

                if let Some(declaration) = JsVariableDeclaration::cast_ref(declaration.syntax()) {
                    for declarator in declaration.declarators() {
                        if let Ok(declarator) = declarator {
                            if let Some(initializer) = declarator.initializer() {
                                if let Ok(expression) = initializer.expression() {
                                    println!(
                                        "Starting from expression in JsVariableDeclaration: {:?}",
                                        expression
                                    );
                                    resolve_expression_recursively(expression, model, 0);
                                }
                            }
                        }
                    }
                }
            }
        } else if let Some(parameter) = JsFormalParameter::cast_ref(argument.syntax()) {
            println!("Formal parameter found: {:?}", parameter);
        }
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

pub struct TsUtil;

impl TsUtil {
    fn is_always_falsy(ty: &AnyTsType) -> bool {
        match ty {
            AnyTsType::TsNumberLiteralType(number) => {
                number
                    .literal_token()
                    .ok()
                    .and_then(|token| token.text().parse::<f64>().ok())
                    == Some(0.0)
            }
            AnyTsType::TsStringLiteralType(str_) => str_.to_trimmed_text().is_empty(),
            AnyTsType::TsBooleanLiteralType(boolean) => {
                if let Ok(literal) = boolean.literal() {
                    return true;
                } else {
                    false
                }
            }
            AnyTsType::TsBigintLiteralType(bigint) => bigint
                .literal_token()
                .ok()
                .map(|token| token.text_trimmed() == "0n")
                .unwrap_or(false),
            AnyTsType::TsUndefinedType(_) => true,
            AnyTsType::TsNullLiteralType(_) => true,
            AnyTsType::TsNeverType(_) => true,
            AnyTsType::TsUnionType(union) => union.types().iter().all(|arg0| {
                if let Some(arg) = arg0.ok() {
                    Self::is_always_falsy(&arg)
                } else {
                    false
                }
            }),
            _ => false,
        }
    }
    //
    // fn is_possibly_falsy(ty: &AnyTsType) -> bool {
    //     match ty {
    //         AnyTsType::TsNumberLiteralType(number) => number.value().ok() == Some(0.0),
    //         AnyTsType::TsStringLiteralType(str_) => str_.value_text().is_empty(),
    //         // AnyTsType::TsBooleanLiteralType(boolean) => boolean.kind() == TsBooleanLiteralKind::False,
    //         AnyTsType::TsBigintLiteralType(bigint) => bigint.text() == "0n",
    //         AnyTsType::TsUndefinedType(_) => true,
    //         AnyTsType::TsNullLiteralType(_) => true,
    //         AnyTsType::TsUnionType(union) => union.types().iter().any(Self::is_possibly_falsy),
    //         _ => false,
    //     }
    // }
    //
    // fn is_always_truthy(ty: &AnyTsType) -> bool {
    //     match ty {
    //         AnyTsType::TsUnionType(union) => union.types().iter().all(Self::is_always_truthy),
    //         _ => !Self::is_possibly_falsy(ty),
    //     }
    // }
    fn analyze(identifier: &AnyTsVariableAnnotation) -> bool {
        if let Ok(Some(type_annotation)) = identifier.type_annotation() {
            if let Some(ty) = type_annotation.ty().ok() {
                println!("Found union type: {:?}", ty);
                ty.as_ts_any_type();
                println!("is always falsy: {}", Self::is_always_falsy(&ty));
                println!("is always truthy: {}", Self::is_always_falsy(&ty));

                return true;
            }
        }
        false
    }

    pub fn process_identifier(
        identifier: &AnyTsVariableAnnotation,
    ) -> Option<JsParameterOrVariableDeclarator> {
        Self::analyze(identifier);
        println!("Variable annotation: {:?}", identifier.type_annotation());
        None
    }
}
