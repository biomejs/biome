use super::*;

pub(super) fn convert_script(
    ctx: &ConvertCtx<'_>,
    script: &JsScript,
) -> Result<(Vec<Statement>, Vec<Directive>)> {
    let body = script
        .statements()
        .into_iter()
        .map(|statement| convert_statement(ctx, statement))
        .collect::<Result<Vec<_>>>()?;
    let directives = script
        .directives()
        .into_iter()
        .map(|directive| convert_directive(ctx, directive))
        .collect::<Result<Vec<_>>>()?;
    Ok((body, directives))
}

pub(super) fn convert_module_item(
    ctx: &ConvertCtx<'_>,
    item: AnyJsModuleItem,
) -> Result<Statement> {
    match item {
        AnyJsModuleItem::AnyJsStatement(statement) => convert_statement(ctx, statement),
        AnyJsModuleItem::JsImport(import) => {
            Ok(Statement::ImportDeclaration(convert_import(ctx, &import)?))
        }
        AnyJsModuleItem::JsExport(export) => convert_export(ctx, &export),
    }
}

pub(super) fn convert_statement(
    ctx: &ConvertCtx<'_>,
    statement: AnyJsStatement,
) -> Result<Statement> {
    match statement {
        AnyJsStatement::JsFunctionDeclaration(function) => Ok(Statement::FunctionDeclaration(
            convert_function_declaration(ctx, &function)?,
        )),
        AnyJsStatement::JsReturnStatement(statement) => Ok(Statement::ReturnStatement(
            convert_return_statement(ctx, &statement)?,
        )),
        AnyJsStatement::JsExpressionStatement(statement) => {
            Ok(Statement::ExpressionStatement(ExpressionStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    statement
                        .expression()
                        .map_err(|_| missing("JsExpressionStatement", "expression"))?,
                )?),
            }))
        }
        AnyJsStatement::JsVariableStatement(statement) => {
            convert_variable_statement(ctx, &statement)
        }
        AnyJsStatement::JsClassDeclaration(class) => Ok(Statement::ClassDeclaration(
            convert_class_declaration(ctx, &class)?,
        )),
        AnyJsStatement::JsBlockStatement(block) => Ok(Statement::BlockStatement(
            convert_block_statement(ctx, &block)?,
        )),
        AnyJsStatement::JsIfStatement(statement) => Ok(Statement::IfStatement(IfStatement {
            base: ctx.base(statement.syntax().text_trimmed_range()),
            test: Box::new(convert_expression(
                ctx,
                statement
                    .test()
                    .map_err(|_| missing("JsIfStatement", "test"))?,
            )?),
            consequent: Box::new(convert_statement(
                ctx,
                statement
                    .consequent()
                    .map_err(|_| missing("JsIfStatement", "consequent"))?,
            )?),
            alternate: statement
                .else_clause()
                .map(|else_clause| {
                    else_clause
                        .alternate()
                        .map_err(|_| missing("JsElseClause", "alternate"))
                        .and_then(|alternate| convert_statement(ctx, alternate))
                        .map(Box::new)
                })
                .transpose()?,
        })),
        AnyJsStatement::JsForStatement(statement) => Ok(Statement::ForStatement(
            convert_for_statement(ctx, &statement)?,
        )),
        AnyJsStatement::JsForInStatement(statement) => {
            Ok(Statement::ForInStatement(ForInStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                left: Box::new(convert_for_in_of_left(
                    ctx,
                    statement
                        .initializer()
                        .map_err(|_| missing("JsForInStatement", "initializer"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    statement
                        .expression()
                        .map_err(|_| missing("JsForInStatement", "expression"))?,
                )?),
                body: Box::new(convert_statement(
                    ctx,
                    statement
                        .body()
                        .map_err(|_| missing("JsForInStatement", "body"))?,
                )?),
            }))
        }
        AnyJsStatement::JsForOfStatement(statement) => {
            Ok(Statement::ForOfStatement(ForOfStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                left: Box::new(convert_for_in_of_left(
                    ctx,
                    statement
                        .initializer()
                        .map_err(|_| missing("JsForOfStatement", "initializer"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    statement
                        .expression()
                        .map_err(|_| missing("JsForOfStatement", "expression"))?,
                )?),
                body: Box::new(convert_statement(
                    ctx,
                    statement
                        .body()
                        .map_err(|_| missing("JsForOfStatement", "body"))?,
                )?),
                is_await: statement.await_token().is_some(),
            }))
        }
        AnyJsStatement::JsWhileStatement(statement) => {
            Ok(Statement::WhileStatement(WhileStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                test: Box::new(convert_expression(
                    ctx,
                    statement
                        .test()
                        .map_err(|_| missing("JsWhileStatement", "test"))?,
                )?),
                body: Box::new(convert_statement(
                    ctx,
                    statement
                        .body()
                        .map_err(|_| missing("JsWhileStatement", "body"))?,
                )?),
            }))
        }
        AnyJsStatement::JsDoWhileStatement(statement) => {
            Ok(Statement::DoWhileStatement(DoWhileStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                test: Box::new(convert_expression(
                    ctx,
                    statement
                        .test()
                        .map_err(|_| missing("JsDoWhileStatement", "test"))?,
                )?),
                body: Box::new(convert_statement(
                    ctx,
                    statement
                        .body()
                        .map_err(|_| missing("JsDoWhileStatement", "body"))?,
                )?),
            }))
        }
        AnyJsStatement::JsSwitchStatement(statement) => {
            Ok(Statement::SwitchStatement(SwitchStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                discriminant: Box::new(convert_expression(
                    ctx,
                    statement
                        .discriminant()
                        .map_err(|_| missing("JsSwitchStatement", "discriminant"))?,
                )?),
                cases: statement
                    .cases()
                    .into_iter()
                    .map(|case| convert_switch_case(ctx, case))
                    .collect::<Result<Vec<_>>>()?,
            }))
        }
        AnyJsStatement::JsThrowStatement(statement) => {
            Ok(Statement::ThrowStatement(ThrowStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                argument: Box::new(convert_expression(
                    ctx,
                    statement
                        .argument()
                        .map_err(|_| missing("JsThrowStatement", "argument"))?,
                )?),
            }))
        }
        AnyJsStatement::JsTryStatement(statement) => Ok(Statement::TryStatement(TryStatement {
            base: ctx.base(statement.syntax().text_trimmed_range()),
            block: convert_block_statement(
                ctx,
                &statement
                    .body()
                    .map_err(|_| missing("JsTryStatement", "body"))?,
            )?,
            handler: Some(convert_catch_clause(
                ctx,
                &statement
                    .catch_clause()
                    .map_err(|_| missing("JsTryStatement", "catch_clause"))?,
            )?),
            finalizer: None,
        })),
        AnyJsStatement::JsTryFinallyStatement(statement) => {
            Ok(Statement::TryStatement(TryStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                block: convert_block_statement(
                    ctx,
                    &statement
                        .body()
                        .map_err(|_| missing("JsTryFinallyStatement", "body"))?,
                )?,
                handler: statement
                    .catch_clause()
                    .map(|catch_clause| convert_catch_clause(ctx, &catch_clause))
                    .transpose()?,
                finalizer: Some(convert_block_statement(
                    ctx,
                    &statement
                        .finally_clause()
                        .map_err(|_| missing("JsTryFinallyStatement", "finally_clause"))?
                        .body()
                        .map_err(|_| missing("JsFinallyClause", "body"))?,
                )?),
            }))
        }
        AnyJsStatement::JsBreakStatement(statement) => {
            Ok(Statement::BreakStatement(BreakStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                label: statement
                    .label()
                    .map(|label| convert_label(ctx, &label))
                    .transpose()?,
            }))
        }
        AnyJsStatement::JsContinueStatement(statement) => {
            Ok(Statement::ContinueStatement(ContinueStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                label: statement
                    .label()
                    .map(|label| convert_label(ctx, &label))
                    .transpose()?,
            }))
        }
        AnyJsStatement::JsLabeledStatement(statement) => {
            Ok(Statement::LabeledStatement(LabeledStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
                label: convert_label(
                    ctx,
                    &statement
                        .label()
                        .map_err(|_| missing("JsLabeledStatement", "label"))?,
                )?,
                body: Box::new(convert_statement(
                    ctx,
                    statement
                        .body()
                        .map_err(|_| missing("JsLabeledStatement", "body"))?,
                )?),
            }))
        }
        AnyJsStatement::JsEmptyStatement(statement) => {
            Ok(Statement::EmptyStatement(EmptyStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
            }))
        }
        AnyJsStatement::JsDebuggerStatement(statement) => {
            Ok(Statement::DebuggerStatement(DebuggerStatement {
                base: ctx.base(statement.syntax().text_trimmed_range()),
            }))
        }
        AnyJsStatement::JsWithStatement(statement) => Ok(Statement::WithStatement(WithStatement {
            base: ctx.base(statement.syntax().text_trimmed_range()),
            object: Box::new(convert_expression(
                ctx,
                statement
                    .object()
                    .map_err(|_| missing("JsWithStatement", "object"))?,
            )?),
            body: Box::new(convert_statement(
                ctx,
                statement
                    .body()
                    .map_err(|_| missing("JsWithStatement", "body"))?,
            )?),
        })),
        _ => Err(unsupported(statement.syntax())),
    }
}

pub(super) fn convert_class_declaration(
    ctx: &ConvertCtx<'_>,
    class: &JsClassDeclaration,
) -> Result<ClassDeclaration> {
    Ok(ClassDeclaration {
        base: ctx.base(class.syntax().text_trimmed_range()),
        id: Some(convert_any_binding_identifier(
            ctx,
            &class
                .id()
                .map_err(|_| missing("JsClassDeclaration", "id"))?,
        )?),
        super_class: class
            .extends_clause()
            .map(|extends| {
                extends
                    .super_class()
                    .map_err(|_| missing("JsExtendsClause", "super_class"))
                    .and_then(|super_class| convert_expression(ctx, super_class))
                    .map(Box::new)
            })
            .transpose()?,
        body: convert_class_body(ctx, class.syntax(), class.members()),
        decorators: None,
        is_abstract: class.abstract_token().map(|_| true),
        declare: None,
        implements: None,
        super_type_parameters: None,
        type_parameters: None,
        mixins: None,
    })
}

pub(super) fn convert_class_body(
    ctx: &ConvertCtx<'_>,
    syntax: &JsSyntaxNode,
    members: biome_js_syntax::JsClassMemberList,
) -> ClassBody {
    ClassBody {
        base: ctx.base(syntax.text_trimmed_range()),
        body: members
            .into_iter()
            .map(|_| serde_json::Value::Null)
            .collect(),
    }
}

pub(super) fn convert_function_declaration(
    ctx: &ConvertCtx<'_>,
    function: &JsFunctionDeclaration,
) -> Result<FunctionDeclaration> {
    Ok(FunctionDeclaration {
        base: ctx.base(function.syntax().text_trimmed_range()),
        id: Some(convert_binding_identifier(
            ctx,
            &function
                .id()
                .map_err(|_| missing("JsFunctionDeclaration", "id"))?
                .syntax()
                .clone(),
        )?),
        params: convert_function_parameters(
            ctx,
            &function
                .parameters()
                .map_err(|_| missing("JsFunctionDeclaration", "parameters"))?,
        )?,
        body: convert_function_body(
            ctx,
            &function
                .body()
                .map_err(|_| missing("JsFunctionDeclaration", "body"))?,
        )?,
        generator: function.star_token().is_some(),
        is_async: function.async_token().is_some(),
        declare: None,
        return_type: None,
        type_parameters: None,
        predicate: None,
        component_declaration: false,
        hook_declaration: false,
    })
}

pub(super) fn convert_function_parameters(
    ctx: &ConvertCtx<'_>,
    parameters: &biome_js_syntax::JsParameters,
) -> Result<Vec<PatternLike>> {
    parameters
        .items()
        .into_iter()
        .map(|parameter| {
            let parameter = parameter.map_err(|_| missing("JsParameters", "parameter"))?;
            if let Some(rest) = parameter.as_js_rest_parameter() {
                return Ok(PatternLike::RestElement(RestElement {
                    base: ctx.base(rest.syntax().text_trimmed_range()),
                    argument: Box::new(convert_pattern(
                        ctx,
                        &rest
                            .binding()
                            .map_err(|_| missing("JsRestParameter", "binding"))?,
                    )?),
                    type_annotation: None,
                    decorators: None,
                }));
            }
            let binding = parameter
                .as_any_js_formal_parameter()
                .and_then(|parameter| parameter.as_js_formal_parameter())
                .ok_or_else(|| unsupported(parameter.syntax()))?
                .binding()
                .map_err(|_| missing("JsFormalParameter", "binding"))?;
            convert_pattern(ctx, &binding)
        })
        .collect()
}

pub(super) fn convert_function_body(
    ctx: &ConvertCtx<'_>,
    body: &JsFunctionBody,
) -> Result<BlockStatement> {
    Ok(BlockStatement {
        base: ctx.base(body.syntax().text_trimmed_range()),
        body: body
            .statements()
            .into_iter()
            .map(|statement| convert_statement(ctx, statement))
            .collect::<Result<Vec<_>>>()?,
        directives: body
            .directives()
            .into_iter()
            .map(|directive| convert_directive(ctx, directive))
            .collect::<Result<Vec<_>>>()?,
    })
}

pub(super) fn convert_block_statement(
    ctx: &ConvertCtx<'_>,
    block: &biome_js_syntax::JsBlockStatement,
) -> Result<BlockStatement> {
    Ok(BlockStatement {
        base: ctx.base(block.syntax().text_trimmed_range()),
        body: block
            .statements()
            .into_iter()
            .map(|statement| convert_statement(ctx, statement))
            .collect::<Result<Vec<_>>>()?,
        directives: Vec::new(),
    })
}

fn convert_for_statement(
    ctx: &ConvertCtx<'_>,
    statement: &biome_js_syntax::JsForStatement,
) -> Result<ForStatement> {
    Ok(ForStatement {
        base: ctx.base(statement.syntax().text_trimmed_range()),
        init: statement
            .initializer()
            .map(|initializer| convert_for_init(ctx, initializer).map(Box::new))
            .transpose()?,
        test: statement
            .test()
            .map(|test| convert_expression(ctx, test).map(Box::new))
            .transpose()?,
        update: statement
            .update()
            .map(|update| convert_expression(ctx, update).map(Box::new))
            .transpose()?,
        body: Box::new(convert_statement(
            ctx,
            statement
                .body()
                .map_err(|_| missing("JsForStatement", "body"))?,
        )?),
    })
}

fn convert_for_init(
    ctx: &ConvertCtx<'_>,
    initializer: biome_js_syntax::AnyJsForInitializer,
) -> Result<ForInit> {
    match initializer {
        biome_js_syntax::AnyJsForInitializer::AnyJsExpression(expression) => Ok(
            ForInit::Expression(Box::new(convert_expression(ctx, expression)?)),
        ),
        biome_js_syntax::AnyJsForInitializer::JsVariableDeclaration(declaration) => {
            let kind = convert_variable_declaration_kind(&declaration)?;
            Ok(ForInit::VariableDeclaration(convert_variable_declaration(
                ctx,
                &declaration,
                kind,
            )?))
        }
    }
}

fn convert_for_in_of_left(
    ctx: &ConvertCtx<'_>,
    initializer: biome_js_syntax::AnyJsForInOrOfInitializer,
) -> Result<ForInOfLeft> {
    match initializer {
        biome_js_syntax::AnyJsForInOrOfInitializer::AnyJsAssignmentPattern(pattern) => Ok(
            ForInOfLeft::Pattern(Box::new(convert_assignment_pattern(ctx, pattern)?)),
        ),
        biome_js_syntax::AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration) => {
            let kind = match declaration
                .kind_token()
                .map_err(|_| missing("JsForVariableDeclaration", "kind_token"))?
                .text_trimmed()
            {
                "const" => VariableDeclarationKind::Const,
                "let" => VariableDeclarationKind::Let,
                "var" => VariableDeclarationKind::Var,
                "using" => VariableDeclarationKind::Using,
                _ => return Err(unsupported(declaration.syntax())),
            };
            let declarator = declaration
                .declarator()
                .map_err(|_| missing("JsForVariableDeclaration", "declarator"))?;
            Ok(ForInOfLeft::VariableDeclaration(
                react_compiler_ast::statements::VariableDeclaration {
                    base: ctx.base(declaration.syntax().text_trimmed_range()),
                    declarations: vec![VariableDeclarator {
                        base: ctx.base(declarator.syntax().text_trimmed_range()),
                        id: convert_pattern(
                            ctx,
                            &declarator
                                .id()
                                .map_err(|_| missing("JsVariableDeclarator", "id"))?,
                        )?,
                        init: declarator
                            .initializer()
                            .map(|initializer| {
                                initializer
                                    .expression()
                                    .map_err(|_| missing("JsInitializerClause", "expression"))
                                    .and_then(|expression| convert_expression(ctx, expression))
                                    .map(Box::new)
                            })
                            .transpose()?,
                        definite: None,
                    }],
                    kind,
                    declare: None,
                },
            ))
        }
    }
}

fn convert_switch_case(
    ctx: &ConvertCtx<'_>,
    case: biome_js_syntax::AnyJsSwitchClause,
) -> Result<SwitchCase> {
    match case {
        biome_js_syntax::AnyJsSwitchClause::JsCaseClause(case) => Ok(SwitchCase {
            base: ctx.base(case.syntax().text_trimmed_range()),
            test: Some(Box::new(convert_expression(
                ctx,
                case.test().map_err(|_| missing("JsCaseClause", "test"))?,
            )?)),
            consequent: case
                .consequent()
                .into_iter()
                .map(|statement| convert_statement(ctx, statement))
                .collect::<Result<Vec<_>>>()?,
        }),
        biome_js_syntax::AnyJsSwitchClause::JsDefaultClause(case) => Ok(SwitchCase {
            base: ctx.base(case.syntax().text_trimmed_range()),
            test: None,
            consequent: case
                .consequent()
                .into_iter()
                .map(|statement| convert_statement(ctx, statement))
                .collect::<Result<Vec<_>>>()?,
        }),
    }
}

fn convert_catch_clause(
    ctx: &ConvertCtx<'_>,
    catch_clause: &biome_js_syntax::JsCatchClause,
) -> Result<CatchClause> {
    Ok(CatchClause {
        base: ctx.base(catch_clause.syntax().text_trimmed_range()),
        param: catch_clause
            .declaration()
            .map(|declaration| {
                declaration
                    .binding()
                    .map_err(|_| missing("JsCatchDeclaration", "binding"))
                    .and_then(|binding| convert_pattern(ctx, &binding))
            })
            .transpose()?,
        body: convert_block_statement(
            ctx,
            &catch_clause
                .body()
                .map_err(|_| missing("JsCatchClause", "body"))?,
        )?,
    })
}

fn convert_label(ctx: &ConvertCtx<'_>, label: &biome_js_syntax::JsLabel) -> Result<Identifier> {
    let token = label
        .value_token()
        .map_err(|_| missing("JsLabel", "value_token"))?;
    Ok(Identifier {
        base: ctx.base(label.syntax().text_trimmed_range()),
        name: token.text_trimmed().to_string(),
        type_annotation: None,
        optional: None,
        decorators: None,
    })
}

pub(super) fn convert_return_statement(
    ctx: &ConvertCtx<'_>,
    statement: &JsReturnStatement,
) -> Result<ReturnStatement> {
    Ok(ReturnStatement {
        base: ctx.base(statement.syntax().text_trimmed_range()),
        argument: statement
            .argument()
            .map(|argument| convert_expression(ctx, argument).map(Box::new))
            .transpose()?,
    })
}

pub(super) fn convert_variable_statement(
    ctx: &ConvertCtx<'_>,
    statement: &JsVariableStatement,
) -> Result<Statement> {
    let declaration = statement
        .declaration()
        .map_err(|_| missing("JsVariableStatement", "declaration"))?;
    let kind = convert_variable_declaration_kind(&declaration)?;

    Ok(Statement::VariableDeclaration(
        convert_variable_declaration(ctx, &declaration, kind)?,
    ))
}

fn convert_variable_declaration_kind(
    declaration: &biome_js_syntax::JsVariableDeclaration,
) -> Result<VariableDeclarationKind> {
    Ok(
        match declaration
            .kind()
            .map_err(|_| missing("JsVariableDeclaration", "kind"))?
            .text_trimmed()
        {
            "const" => VariableDeclarationKind::Const,
            "let" => VariableDeclarationKind::Let,
            "var" => VariableDeclarationKind::Var,
            "using" => VariableDeclarationKind::Using,
            _ => return Err(unsupported(declaration.syntax())),
        },
    )
}

pub(super) fn convert_variable_declaration(
    ctx: &ConvertCtx<'_>,
    declaration: &biome_js_syntax::JsVariableDeclaration,
    kind: VariableDeclarationKind,
) -> Result<react_compiler_ast::statements::VariableDeclaration> {
    Ok(react_compiler_ast::statements::VariableDeclaration {
        base: ctx.base(declaration.syntax().text_trimmed_range()),
        declarations: declaration
            .declarators()
            .into_iter()
            .map(|declarator| {
                let declarator =
                    declarator.map_err(|_| missing("JsVariableDeclaration", "declarator"))?;
                Ok(VariableDeclarator {
                    base: ctx.base(declarator.syntax().text_trimmed_range()),
                    id: convert_pattern(
                        ctx,
                        &declarator
                            .id()
                            .map_err(|_| missing("JsVariableDeclarator", "id"))?,
                    )?,
                    init: declarator
                        .initializer()
                        .map(|initializer| {
                            initializer
                                .expression()
                                .map_err(|_| missing("JsInitializerClause", "expression"))
                                .and_then(|expression| convert_expression(ctx, expression))
                                .map(Box::new)
                        })
                        .transpose()?,
                    definite: None,
                })
            })
            .collect::<Result<Vec<_>>>()?,
        kind,
        declare: None,
    })
}
