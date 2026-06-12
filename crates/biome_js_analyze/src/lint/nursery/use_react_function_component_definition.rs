use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsFunctionBody, AnyJsStatement, JsFunctionBody, JsSyntaxToken, JsVariableDeclaration,
    JsVariableStatement, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind};
use biome_rule_options::use_react_function_component_definition::{
    ComponentDefinitionStyle, UseReactFunctionComponentDefinitionOptions,
};

use crate::{
    JsRuleAction,
    react::components::{
        AnyPotentialReactComponentDeclaration, ReactComponentInfo, ReactComponentKind,
    },
};

declare_lint_rule! {
    /// Enforce a specific function type for React function components.
    ///
    /// This rule keeps function component definitions consistent. By default, named
    /// components must be written as function declarations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const MyComponent = (props) => {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// function MyComponent(props) {
    ///   return <div>{props.name}</div>;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `namedComponents`
    ///
    /// The function style to enforce for named React components.
    /// Accepted values are:
    /// - `"functionDeclaration"` (default): Enforce function declarations.
    /// - `"functionExpression"`: Enforce function expressions assigned to component bindings.
    /// - `"arrowFunction"`: Enforce arrow functions assigned to component bindings.
    ///
    /// #### `"functionDeclaration"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "namedComponents": "functionDeclaration"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// const MyComponent = (props) => {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```jsx,use_options
    /// function MyComponent(props) {
    ///   return <div>{props.name}</div>;
    /// }
    /// ```
    ///
    /// #### `"functionExpression"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "namedComponents": "functionExpression"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// function MyComponent(props) {
    ///   return <div>{props.name}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// const MyComponent = (props) => {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```jsx,use_options
    /// const MyComponent = function (props) {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// #### `"arrowFunction"`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "namedComponents": "arrowFunction"
    ///   }
    /// }
    /// ```
    ///
    /// ##### Invalid
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// function MyComponent(props) {
    ///   return <div>{props.name}</div>;
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diagnostic
    /// const MyComponent = function (props) {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    /// ##### Valid
    ///
    /// ```jsx,use_options
    /// const MyComponent = (props) => {
    ///   return <div>{props.name}</div>;
    /// };
    /// ```
    ///
    pub UseReactFunctionComponentDefinition {
        version: "next",
        name: "useReactFunctionComponentDefinition",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReact("function-component-definition").inspired()],
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Debug)]
pub struct UseReactFunctionComponentDefinitionState {
    component_name: JsSyntaxToken,
    actual_style: ComponentDefinitionStyle,
    preferred_style: ComponentDefinitionStyle,
    range: TextRange,
}

impl Rule for UseReactFunctionComponentDefinition {
    type Query = Ast<AnyPotentialReactComponentDeclaration>;
    type State = UseReactFunctionComponentDefinitionState;
    type Signals = Option<Self::State>;
    type Options = UseReactFunctionComponentDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let info = ReactComponentInfo::from_declaration(node.syntax())?;
        let ReactComponentKind::Function(_) = info.kind else {
            return None;
        };

        let component_name = info.name.clone()?;
        let range = info.declaration_highlight_range();
        let actual_style = component_definition_style(node)?;
        let preferred_style = ctx
            .options()
            .named_components
            .unwrap_or(ComponentDefinitionStyle::FunctionDeclaration);

        if actual_style == preferred_style {
            return None;
        }

        Some(UseReactFunctionComponentDefinitionState {
            component_name,
            actual_style,
            preferred_style,
            range,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The React component " {state.component_name.text_trimmed()} " is defined as " {state.actual_style.label()} "."
                },
            )
            .note(markup! {
                "Mixing component definition styles makes component declarations harder to scan."
            })
            .note(markup! {
                "Rewrite this component as " {state.preferred_style.label()} " or configure `namedComponents` to allow " {state.actual_style.label()} "."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        match state.preferred_style {
            ComponentDefinitionStyle::FunctionDeclaration => {
                let AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) = node
                else {
                    return None;
                };
                if declarator.variable_annotation().is_some() {
                    return None;
                }

                let declaration = declarator
                    .syntax()
                    .ancestors()
                    .find_map(JsVariableDeclaration::cast)?;
                if declaration.declarators().len() != 1 {
                    return None;
                }

                let statement = declaration.parent::<JsVariableStatement>()?;
                let id = declarator.id().ok()?.as_any_js_binding()?.clone();
                let id_name = id.as_js_identifier_binding()?.name_token().ok()?;

                let init_expr = declarator.initializer()?.expression().ok()?;

                let (async_token, star_token, type_parameters, return_type_annotation, params, body) =
                    match &init_expr {
                        AnyJsExpression::JsFunctionExpression(func) => {
                            if let Some(function_id) = func.id()
                                && function_id
                                    .as_js_identifier_binding()?
                                    .name_token()
                                    .ok()?
                                    .text_trimmed()
                                    != id_name.text_trimmed()
                            {
                                return None;
                            }
                            (
                                func.async_token(),
                                func.star_token(),
                                func.type_parameters(),
                                func.return_type_annotation(),
                                func.parameters().ok()?,
                                func.body().ok()?,
                            )
                        }
                        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                            // Arrow functions with a shorthand single-param can't be directly
                            // converted since we need JsParameters for the function declaration.
                            let params = arrow.parameters().ok()?.as_js_parameters()?.clone();
                            let body = arrow.body().ok()?;
                            let func_body = match body {
                                AnyJsFunctionBody::JsFunctionBody(block) => block,
                                AnyJsFunctionBody::AnyJsExpression(ref expr) => {
                                    arrow_expr_body_to_function_body(expr)?
                                }
                            };
                            (
                                arrow.async_token(),
                                None, // arrow functions cannot be generators
                                arrow.type_parameters(),
                                arrow.return_type_annotation(),
                                params,
                                func_body,
                            )
                        }
                        _ => return None,
                    };

                let mut function_declaration = make::js_function_declaration(
                    make::token(T![function])
                        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    id,
                    params,
                    body,
                );
                if let Some(t) = async_token {
                    function_declaration = function_declaration.with_async_token(t);
                }
                if let Some(t) = star_token {
                    function_declaration = function_declaration.with_star_token(t);
                }
                if let Some(t) = type_parameters {
                    function_declaration = function_declaration.with_type_parameters(t);
                }
                if let Some(t) = return_type_annotation {
                    function_declaration = function_declaration.with_return_type_annotation(t);
                }

                let mut mutation = ctx.root().begin();
                mutation.replace_node(
                    AnyJsStatement::JsVariableStatement(statement),
                    AnyJsStatement::JsFunctionDeclaration(function_declaration.build()),
                );

                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use a function declaration for this component." }.to_owned(),
                    mutation,
                ))
            }

            ComponentDefinitionStyle::ArrowFunction => match node {
                AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(func_decl) => {
                    if func_decl.star_token().is_some() {
                        return None;
                    }
                    let id_binding = func_decl.id().ok()?;
                    let params = func_decl.parameters().ok()?;
                    let body = func_decl.body().ok()?;

                    let mut arrow_builder = make::js_arrow_function_expression(
                        AnyJsArrowFunctionParameters::JsParameters(params),
                        make::token(T![=>])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        AnyJsFunctionBody::JsFunctionBody(body),
                    );
                    if let Some(t) = func_decl.async_token() {
                        arrow_builder = arrow_builder.with_async_token(t);
                    }
                    if let Some(t) = func_decl.type_parameters() {
                        arrow_builder = arrow_builder.with_type_parameters(t);
                    }
                    if let Some(t) = func_decl.return_type_annotation() {
                        arrow_builder = arrow_builder.with_return_type_annotation(t);
                    }

                    let new_statement = make_const_variable_statement(
                        id_binding,
                        AnyJsExpression::from(arrow_builder.build()),
                    );

                    let mut mutation = ctx.root().begin();
                    mutation.replace_node(
                        AnyJsStatement::JsFunctionDeclaration(func_decl.clone()),
                        AnyJsStatement::JsVariableStatement(new_statement),
                    );

                    Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        markup! { "Use an arrow function for this component." }.to_owned(),
                        mutation,
                    ))
                }

                AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => {
                    if declarator.variable_annotation().is_some() {
                        return None;
                    }
                    let func = declarator
                        .initializer()?
                        .expression()
                        .ok()?
                        .as_js_function_expression()?
                        .clone();
                    if func.star_token().is_some() {
                        return None;
                    }

                    let mut arrow_builder = make::js_arrow_function_expression(
                        AnyJsArrowFunctionParameters::JsParameters(func.parameters().ok()?),
                        make::token(T![=>])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        AnyJsFunctionBody::JsFunctionBody(func.body().ok()?),
                    );
                    if let Some(t) = func.async_token() {
                        arrow_builder = arrow_builder.with_async_token(t);
                    }
                    if let Some(t) = func.type_parameters() {
                        arrow_builder = arrow_builder.with_type_parameters(t);
                    }
                    if let Some(t) = func.return_type_annotation() {
                        arrow_builder = arrow_builder.with_return_type_annotation(t);
                    }

                    let mut mutation = ctx.root().begin();
                    mutation.replace_node(
                        AnyJsExpression::JsFunctionExpression(func),
                        AnyJsExpression::from(arrow_builder.build()),
                    );

                    Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        markup! { "Use an arrow function for this component." }.to_owned(),
                        mutation,
                    ))
                }

                _ => None,
            },

            ComponentDefinitionStyle::FunctionExpression => match node {
                AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(func_decl) => {
                    let id_binding = func_decl.id().ok()?;
                    let params = func_decl.parameters().ok()?;
                    let body = func_decl.body().ok()?;

                    let mut func_expr_builder = make::js_function_expression(
                        make::token(T![function])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        params,
                        body,
                    );
                    if let Some(t) = func_decl.async_token() {
                        func_expr_builder = func_expr_builder.with_async_token(t);
                    }
                    if let Some(t) = func_decl.star_token() {
                        func_expr_builder = func_expr_builder.with_star_token(t);
                    }
                    if let Some(t) = func_decl.type_parameters() {
                        func_expr_builder = func_expr_builder.with_type_parameters(t);
                    }
                    if let Some(t) = func_decl.return_type_annotation() {
                        func_expr_builder = func_expr_builder.with_return_type_annotation(t);
                    }

                    let new_statement = make_const_variable_statement(
                        id_binding,
                        AnyJsExpression::from(func_expr_builder.build()),
                    );

                    let mut mutation = ctx.root().begin();
                    mutation.replace_node(
                        AnyJsStatement::JsFunctionDeclaration(func_decl.clone()),
                        AnyJsStatement::JsVariableStatement(new_statement),
                    );

                    Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        markup! { "Use a function expression for this component." }.to_owned(),
                        mutation,
                    ))
                }

                AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => {
                    if declarator.variable_annotation().is_some() {
                        return None;
                    }
                    let arrow = declarator
                        .initializer()?
                        .expression()
                        .ok()?
                        .as_js_arrow_function_expression()?
                        .clone();
                    // Arrow functions with a shorthand single-param need JsParameters.
                    let params = arrow.parameters().ok()?.as_js_parameters()?.clone();
                    let body = arrow.body().ok()?;
                    let func_body = match body {
                        AnyJsFunctionBody::JsFunctionBody(block) => block,
                        AnyJsFunctionBody::AnyJsExpression(ref expr) => {
                            arrow_expr_body_to_function_body(expr)?
                        }
                    };

                    let mut func_expr_builder = make::js_function_expression(
                        make::token(T![function])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                        params,
                        func_body,
                    );
                    if let Some(t) = arrow.async_token() {
                        func_expr_builder = func_expr_builder.with_async_token(t);
                    }
                    if let Some(t) = arrow.type_parameters() {
                        func_expr_builder = func_expr_builder.with_type_parameters(t);
                    }
                    if let Some(t) = arrow.return_type_annotation() {
                        func_expr_builder = func_expr_builder.with_return_type_annotation(t);
                    }

                    let mut mutation = ctx.root().begin();
                    mutation.replace_node(
                        AnyJsExpression::JsArrowFunctionExpression(arrow),
                        AnyJsExpression::from(func_expr_builder.build()),
                    );

                    Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        markup! { "Use a function expression for this component." }.to_owned(),
                        mutation,
                    ))
                }

                _ => None,
            },
        }
    }
}

fn component_definition_style(
    node: &AnyPotentialReactComponentDeclaration,
) -> Option<ComponentDefinitionStyle> {
    match node {
        AnyPotentialReactComponentDeclaration::JsFunctionDeclaration(_)
        | AnyPotentialReactComponentDeclaration::JsFunctionExportDefaultDeclaration(_) => {
            Some(ComponentDefinitionStyle::FunctionDeclaration)
        }
        AnyPotentialReactComponentDeclaration::JsVariableDeclarator(declarator) => {
            expression_definition_style(&declarator.initializer()?.expression().ok()?)
        }
        AnyPotentialReactComponentDeclaration::JsAssignmentExpression(assignment) => {
            expression_definition_style(&assignment.right().ok()?)
        }
        _ => None,
    }
}

fn expression_definition_style(expression: &AnyJsExpression) -> Option<ComponentDefinitionStyle> {
    match expression {
        AnyJsExpression::JsFunctionExpression(_) => {
            Some(ComponentDefinitionStyle::FunctionExpression)
        }
        AnyJsExpression::JsArrowFunctionExpression(_) => Some(ComponentDefinitionStyle::ArrowFunction),
        AnyJsExpression::JsCallExpression(call) => {
            let callee_name = call.callee().ok()?.get_callee_member_name()?;
            let callee_member_name = callee_name.text_trimmed();
            let is_memo = callee_member_name == "memo";
            let is_forward_ref = callee_member_name == "forwardRef";
            if !is_memo && !is_forward_ref {
                return None;
            }

            let args = call.arguments().ok()?;
            let args_len = args.args().len();
            if (is_memo && !(1..=2).contains(&args_len)) || (is_forward_ref && args_len != 1) {
                return None;
            }

            let first_arg = args.args().into_iter().next()?.ok()?;
            expression_definition_style(first_arg.as_any_js_expression()?)
        }
        _ => None,
    }
}

/// Builds `const Name = expr;` from an existing binding and a new expression.
fn make_const_variable_statement(
    id_binding: AnyJsBinding,
    expr: AnyJsExpression,
) -> JsVariableStatement {
    let binding_pattern = AnyJsBindingPattern::AnyJsBinding(id_binding);
    let initializer = make::js_initializer_clause(
        make::token(T![=])
            .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        expr,
    );
    let declarator = make::js_variable_declarator(binding_pattern)
        .with_initializer(initializer)
        .build();
    let declaration = make::js_variable_declaration(
        make::token(T![const]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::js_variable_declarator_list([declarator], []),
    )
    .build();
    make::js_variable_statement(declaration)
        .with_semicolon_token(make::token(T![;]))
        .build()
}

/// Wraps an arrow function expression body in a block body with a `return` statement.
fn arrow_expr_body_to_function_body(expr: &AnyJsExpression) -> Option<JsFunctionBody> {
    let expr_to_return = if let Some(paren_expr) = expr.as_js_parenthesized_expression() {
        paren_expr.expression().ok()?
    } else {
        expr.clone()
    };
    let return_stmt = make::js_return_statement(
        make::token(T![return]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
    )
    .with_argument(expr_to_return)
    .with_semicolon_token(make::token(T![;]))
    .build();
    Some(make::js_function_body(
        make::token(T!['{'])
            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        make::js_directive_list([]),
        make::js_statement_list([AnyJsStatement::from(return_stmt)]),
        make::token(T!['}'])
            .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
    ))
}
