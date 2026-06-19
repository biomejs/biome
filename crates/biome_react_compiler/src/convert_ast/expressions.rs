use super::*;

pub(super) fn convert_expression(
    ctx: &ConvertCtx<'_>,
    expression: AnyJsExpression,
) -> Result<Expression> {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => Ok(Expression::Identifier(
            convert_reference_identifier(ctx, &identifier)?,
        )),
        AnyJsExpression::AnyJsLiteralExpression(literal) => convert_literal(ctx, literal),
        AnyJsExpression::JsArrayExpression(array) => {
            Ok(Expression::ArrayExpression(ArrayExpression {
                base: ctx.base(array.syntax().text_trimmed_range()),
                elements: array
                    .elements()
                    .into_iter()
                    .map(|element| {
                        match element.map_err(|_| missing("JsArrayExpression", "element"))? {
                            biome_js_syntax::AnyJsArrayElement::AnyJsExpression(expression) => {
                                convert_expression(ctx, expression).map(Some)
                            }
                            biome_js_syntax::AnyJsArrayElement::JsArrayHole(_) => Ok(None),
                            biome_js_syntax::AnyJsArrayElement::JsSpread(spread) => {
                                convert_spread(ctx, &spread)
                                    .map(Expression::SpreadElement)
                                    .map(Some)
                            }
                        }
                    })
                    .collect::<Result<Vec<_>>>()?,
            }))
        }
        AnyJsExpression::JsObjectExpression(object) => {
            Ok(Expression::ObjectExpression(ObjectExpression {
                base: ctx.base(object.syntax().text_trimmed_range()),
                properties: object
                    .members()
                    .into_iter()
                    .map(|member| {
                        convert_object_member(
                            ctx,
                            member.map_err(|_| missing("JsObjectExpression", "member"))?,
                        )
                    })
                    .collect::<Result<Vec<_>>>()?,
            }))
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            Ok(Expression::BinaryExpression(BinaryExpression {
                base: ctx.base(binary.syntax().text_trimmed_range()),
                operator: convert_binary_operator(
                    binary
                        .operator_token()
                        .map_err(|_| missing("JsBinaryExpression", "operator_token"))?
                        .text_trimmed(),
                )?,
                left: Box::new(convert_expression(
                    ctx,
                    binary
                        .left()
                        .map_err(|_| missing("JsBinaryExpression", "left"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    binary
                        .right()
                        .map_err(|_| missing("JsBinaryExpression", "right"))?,
                )?),
            }))
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            Ok(Expression::LogicalExpression(LogicalExpression {
                base: ctx.base(logical.syntax().text_trimmed_range()),
                operator: convert_logical_operator(
                    logical
                        .operator_token()
                        .map_err(|_| missing("JsLogicalExpression", "operator_token"))?
                        .text_trimmed(),
                )?,
                left: Box::new(convert_expression(
                    ctx,
                    logical
                        .left()
                        .map_err(|_| missing("JsLogicalExpression", "left"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    logical
                        .right()
                        .map_err(|_| missing("JsLogicalExpression", "right"))?,
                )?),
            }))
        }
        AnyJsExpression::JsInExpression(in_expression) => {
            Ok(Expression::BinaryExpression(BinaryExpression {
                base: ctx.base(in_expression.syntax().text_trimmed_range()),
                operator: BinaryOperator::In,
                left: Box::new(
                    match in_expression
                        .property()
                        .map_err(|_| missing("JsInExpression", "property"))?
                    {
                        biome_js_syntax::AnyJsInProperty::AnyJsExpression(expression) => {
                            convert_expression(ctx, expression)?
                        }
                        biome_js_syntax::AnyJsInProperty::JsPrivateName(name) => {
                            convert_private_name(ctx, &name)?
                        }
                    },
                ),
                right: Box::new(convert_expression(
                    ctx,
                    in_expression
                        .object()
                        .map_err(|_| missing("JsInExpression", "object"))?,
                )?),
            }))
        }
        AnyJsExpression::JsInstanceofExpression(instanceof) => {
            Ok(Expression::BinaryExpression(BinaryExpression {
                base: ctx.base(instanceof.syntax().text_trimmed_range()),
                operator: BinaryOperator::Instanceof,
                left: Box::new(convert_expression(
                    ctx,
                    instanceof
                        .left()
                        .map_err(|_| missing("JsInstanceofExpression", "left"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    instanceof
                        .right()
                        .map_err(|_| missing("JsInstanceofExpression", "right"))?,
                )?),
            }))
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            Ok(Expression::UnaryExpression(UnaryExpression {
                base: ctx.base(unary.syntax().text_trimmed_range()),
                operator: convert_unary_operator(
                    unary
                        .operator_token()
                        .map_err(|_| missing("JsUnaryExpression", "operator_token"))?
                        .text_trimmed(),
                )?,
                prefix: true,
                argument: Box::new(convert_expression(
                    ctx,
                    unary
                        .argument()
                        .map_err(|_| missing("JsUnaryExpression", "argument"))?,
                )?),
            }))
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            Ok(Expression::ConditionalExpression(ConditionalExpression {
                base: ctx.base(conditional.syntax().text_trimmed_range()),
                test: Box::new(convert_expression(
                    ctx,
                    conditional
                        .test()
                        .map_err(|_| missing("JsConditionalExpression", "test"))?,
                )?),
                consequent: Box::new(convert_expression(
                    ctx,
                    conditional
                        .consequent()
                        .map_err(|_| missing("JsConditionalExpression", "consequent"))?,
                )?),
                alternate: Box::new(convert_expression(
                    ctx,
                    conditional
                        .alternate()
                        .map_err(|_| missing("JsConditionalExpression", "alternate"))?,
                )?),
            }))
        }
        AnyJsExpression::JsArrowFunctionExpression(arrow) => Ok(
            Expression::ArrowFunctionExpression(ArrowFunctionExpression {
                base: ctx.base(arrow.syntax().text_trimmed_range()),
                params: convert_arrow_parameters(
                    ctx,
                    arrow
                        .parameters()
                        .map_err(|_| missing("JsArrowFunctionExpression", "parameters"))?,
                )?,
                body: Box::new(
                    match arrow
                        .body()
                        .map_err(|_| missing("JsArrowFunctionExpression", "body"))?
                    {
                        AnyJsFunctionBody::AnyJsExpression(expression) => {
                            ArrowFunctionBody::Expression(Box::new(convert_expression(
                                ctx, expression,
                            )?))
                        }
                        AnyJsFunctionBody::JsFunctionBody(body) => {
                            ArrowFunctionBody::BlockStatement(convert_function_body(ctx, &body)?)
                        }
                    },
                ),
                id: None,
                generator: false,
                is_async: arrow.async_token().is_some(),
                expression: None,
                return_type: None,
                type_parameters: None,
                predicate: None,
            }),
        ),
        AnyJsExpression::JsFunctionExpression(function) => Ok(Expression::FunctionExpression(
            convert_function_expression(ctx, &function)?,
        )),
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            Ok(Expression::AssignmentExpression(AssignmentExpression {
                base: ctx.base(assignment.syntax().text_trimmed_range()),
                operator: convert_assignment_operator(
                    assignment
                        .operator_token()
                        .map_err(|_| missing("JsAssignmentExpression", "operator_token"))?
                        .text_trimmed(),
                )?,
                left: Box::new(convert_assignment_pattern(
                    ctx,
                    assignment
                        .left()
                        .map_err(|_| missing("JsAssignmentExpression", "left"))?,
                )?),
                right: Box::new(convert_expression(
                    ctx,
                    assignment
                        .right()
                        .map_err(|_| missing("JsAssignmentExpression", "right"))?,
                )?),
            }))
        }
        AnyJsExpression::JsPostUpdateExpression(update) => {
            convert_post_update_expression(ctx, &update)
        }
        AnyJsExpression::JsPreUpdateExpression(update) => {
            convert_pre_update_expression(ctx, &update)
        }
        AnyJsExpression::JsNewExpression(new) => Ok(Expression::NewExpression(NewExpression {
            base: ctx.base(new.syntax().text_trimmed_range()),
            callee: Box::new(convert_expression(
                ctx,
                new.callee()
                    .map_err(|_| missing("JsNewExpression", "callee"))?,
            )?),
            arguments: new
                .arguments()
                .map(|arguments| {
                    arguments
                        .args()
                        .into_iter()
                        .map(|argument| convert_call_argument(ctx, argument))
                        .collect::<Result<Vec<_>>>()
                })
                .transpose()?
                .unwrap_or_default(),
            type_parameters: None,
            type_arguments: None,
        })),
        AnyJsExpression::JsAwaitExpression(await_expression) => {
            Ok(Expression::AwaitExpression(AwaitExpression {
                base: ctx.base(await_expression.syntax().text_trimmed_range()),
                argument: Box::new(convert_expression(
                    ctx,
                    await_expression
                        .argument()
                        .map_err(|_| missing("JsAwaitExpression", "argument"))?,
                )?),
            }))
        }
        AnyJsExpression::JsSequenceExpression(sequence) => {
            Ok(Expression::SequenceExpression(SequenceExpression {
                base: ctx.base(sequence.syntax().text_trimmed_range()),
                expressions: flatten_sequence_expression(ctx, sequence)?,
            }))
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            convert_template_expression(ctx, &template)
        }
        AnyJsExpression::JsYieldExpression(yield_expression) => {
            let argument = yield_expression.argument();
            let delegate = argument
                .as_ref()
                .and_then(|argument| argument.star_token())
                .is_some();
            Ok(Expression::YieldExpression(YieldExpression {
                base: ctx.base(yield_expression.syntax().text_trimmed_range()),
                argument: argument
                    .map(|argument| {
                        argument
                            .expression()
                            .map_err(|_| missing("JsYieldArgument", "expression"))
                            .and_then(|expression| convert_expression(ctx, expression))
                            .map(Box::new)
                    })
                    .transpose()?,
                delegate,
            }))
        }
        AnyJsExpression::JsSuperExpression(super_expression) => Ok(Expression::Super(Super {
            base: ctx.base(super_expression.syntax().text_trimmed_range()),
        })),
        AnyJsExpression::JsImportMetaExpression(import_meta) => {
            Ok(Expression::MetaProperty(MetaProperty {
                base: ctx.base(import_meta.syntax().text_trimmed_range()),
                meta: Identifier {
                    base: ctx.base(import_meta.syntax().text_trimmed_range()),
                    name: "import".to_string(),
                    type_annotation: None,
                    optional: None,
                    decorators: None,
                },
                property: Identifier {
                    base: ctx.base(import_meta.syntax().text_trimmed_range()),
                    name: "meta".to_string(),
                    type_annotation: None,
                    optional: None,
                    decorators: None,
                },
            }))
        }
        AnyJsExpression::JsImportCallExpression(import_call) => {
            Ok(Expression::CallExpression(CallExpression {
                base: ctx.base(import_call.syntax().text_trimmed_range()),
                callee: Box::new(Expression::Import(Import {
                    base: ctx.base(import_call.syntax().text_trimmed_range()),
                })),
                arguments: import_call
                    .arguments()
                    .map_err(|_| missing("JsImportCallExpression", "arguments"))?
                    .args()
                    .into_iter()
                    .map(|argument| convert_call_argument(ctx, argument))
                    .collect::<Result<Vec<_>>>()?,
                type_parameters: None,
                type_arguments: None,
                optional: None,
            }))
        }
        AnyJsExpression::JsClassExpression(class) => Ok(Expression::ClassExpression(
            convert_class_expression(ctx, &class)?,
        )),
        AnyJsExpression::JsCallExpression(call) => {
            let base = ctx.base(call.syntax().text_trimmed_range());
            let callee = Box::new(convert_expression(
                ctx,
                call.callee()
                    .map_err(|_| missing("JsCallExpression", "callee"))?,
            )?);
            let arguments = call
                .arguments()
                .map_err(|_| missing("JsCallExpression", "arguments"))?
                .args()
                .into_iter()
                .map(|argument| convert_call_argument(ctx, argument))
                .collect::<Result<Vec<_>>>()?;
            if call.optional_chain_token().is_some() {
                Ok(Expression::OptionalCallExpression(OptionalCallExpression {
                    base,
                    callee,
                    arguments,
                    optional: true,
                    type_parameters: None,
                    type_arguments: None,
                }))
            } else {
                Ok(Expression::CallExpression(CallExpression {
                    base,
                    callee,
                    arguments,
                    type_parameters: None,
                    type_arguments: None,
                    optional: None,
                }))
            }
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let base = ctx.base(member.syntax().text_trimmed_range());
            let object = Box::new(convert_expression(
                ctx,
                member
                    .object()
                    .map_err(|_| missing("JsStaticMemberExpression", "object"))?,
            )?);
            let property = Box::new(convert_static_member_name(
                ctx,
                member
                    .member()
                    .map_err(|_| missing("JsStaticMemberExpression", "member"))?,
            )?);
            if member
                .operator_token()
                .map_err(|_| missing("JsStaticMemberExpression", "operator_token"))?
                .text_trimmed()
                == "?."
            {
                Ok(Expression::OptionalMemberExpression(
                    OptionalMemberExpression {
                        base,
                        object,
                        property,
                        computed: false,
                        optional: true,
                    },
                ))
            } else {
                Ok(Expression::MemberExpression(MemberExpression {
                    base,
                    object,
                    property,
                    computed: false,
                }))
            }
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let base = ctx.base(member.syntax().text_trimmed_range());
            let object = Box::new(convert_expression(
                ctx,
                member
                    .object()
                    .map_err(|_| missing("JsComputedMemberExpression", "object"))?,
            )?);
            let property = Box::new(convert_expression(
                ctx,
                member
                    .member()
                    .map_err(|_| missing("JsComputedMemberExpression", "member"))?,
            )?);
            if member.optional_chain_token().is_some() {
                Ok(Expression::OptionalMemberExpression(
                    OptionalMemberExpression {
                        base,
                        object,
                        property,
                        computed: true,
                        optional: true,
                    },
                ))
            } else {
                Ok(Expression::MemberExpression(MemberExpression {
                    base,
                    object,
                    property,
                    computed: true,
                }))
            }
        }
        AnyJsExpression::JsThisExpression(this) => Ok(Expression::ThisExpression(
            react_compiler_ast::expressions::ThisExpression {
                base: ctx.base(this.syntax().text_trimmed_range()),
            },
        )),
        AnyJsExpression::JsxTagExpression(tag) => convert_jsx_tag_expression(
            ctx,
            tag.tag().map_err(|_| missing("JsxTagExpression", "tag"))?,
        ),
        AnyJsExpression::JsParenthesizedExpression(expression) => Ok(
            Expression::ParenthesizedExpression(ParenthesizedExpression {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("JsParenthesizedExpression", "expression"))?,
                )?),
            }),
        ),
        AnyJsExpression::TsAsExpression(expression) => {
            Ok(Expression::TSAsExpression(TSAsExpression {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("TsAsExpression", "expression"))?,
                )?),
                type_annotation: react_compiler_ast::common::RawNode::null(),
            }))
        }
        AnyJsExpression::TsSatisfiesExpression(expression) => {
            Ok(Expression::TSSatisfiesExpression(TSSatisfiesExpression {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("TsSatisfiesExpression", "expression"))?,
                )?),
                type_annotation: react_compiler_ast::common::RawNode::null(),
            }))
        }
        AnyJsExpression::TsNonNullAssertionExpression(expression) => {
            Ok(Expression::TSNonNullExpression(TSNonNullExpression {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("TsNonNullAssertionExpression", "expression"))?,
                )?),
            }))
        }
        AnyJsExpression::TsTypeAssertionExpression(expression) => {
            Ok(Expression::TSTypeAssertion(TSTypeAssertion {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("TsTypeAssertionExpression", "expression"))?,
                )?),
                type_annotation: react_compiler_ast::common::RawNode::null(),
            }))
        }
        AnyJsExpression::TsInstantiationExpression(expression) => Ok(
            Expression::TSInstantiationExpression(TSInstantiationExpression {
                base: ctx.base(expression.syntax().text_trimmed_range()),
                expression: Box::new(convert_expression(
                    ctx,
                    expression
                        .expression()
                        .map_err(|_| missing("TsInstantiationExpression", "expression"))?,
                )?),
                type_parameters: react_compiler_ast::common::RawNode::null(),
            }),
        ),
        AnyJsExpression::JsNewTargetExpression(expression) => {
            let range = expression.syntax().text_trimmed_range();
            let meta = expression
                .new_token()
                .map_err(|_| missing("JsNewTargetExpression", "new_token"))?;
            let property = expression
                .target_token()
                .map_err(|_| missing("JsNewTargetExpression", "target_token"))?;
            Ok(Expression::MetaProperty(MetaProperty {
                base: ctx.base(range),
                meta: Identifier {
                    base: ctx.base(meta.text_trimmed_range()),
                    name: meta.text_trimmed().to_string(),
                    type_annotation: None,
                    optional: None,
                    decorators: None,
                },
                property: Identifier {
                    base: ctx.base(property.text_trimmed_range()),
                    name: property.text_trimmed().to_string(),
                    type_annotation: None,
                    optional: None,
                    decorators: None,
                },
            }))
        }
        _ => Err(unsupported(expression.syntax())),
    }
}

pub(super) fn flatten_sequence_expression(
    ctx: &ConvertCtx<'_>,
    sequence: biome_js_syntax::JsSequenceExpression,
) -> Result<Vec<Expression>> {
    let mut expressions = Vec::new();
    let left = sequence
        .left()
        .map_err(|_| missing("JsSequenceExpression", "left"))?;
    match left {
        AnyJsExpression::JsSequenceExpression(sequence) => {
            expressions.extend(flatten_sequence_expression(ctx, sequence)?);
        }
        expression => expressions.push(convert_expression(ctx, expression)?),
    }
    expressions.push(convert_expression(
        ctx,
        sequence
            .right()
            .map_err(|_| missing("JsSequenceExpression", "right"))?,
    )?);
    Ok(expressions)
}

pub(super) fn convert_template_expression(
    ctx: &ConvertCtx<'_>,
    template: &JsTemplateExpression,
) -> Result<Expression> {
    let (quasis, expressions) = convert_template_literal_parts(ctx, template)?;
    let literal = TemplateLiteral {
        base: ctx.base(template.syntax().text_trimmed_range()),
        quasis,
        expressions,
    };

    match template.tag() {
        Some(tag) => Ok(Expression::TaggedTemplateExpression(
            TaggedTemplateExpression {
                base: ctx.base(template.syntax().text_trimmed_range()),
                tag: Box::new(convert_expression(ctx, tag)?),
                quasi: literal,
                type_parameters: None,
            },
        )),
        None => Ok(Expression::TemplateLiteral(literal)),
    }
}

pub(super) fn convert_template_literal_parts(
    ctx: &ConvertCtx<'_>,
    template: &JsTemplateExpression,
) -> Result<(Vec<TemplateElement>, Vec<Expression>)> {
    let mut quasis = Vec::new();
    let mut expressions = Vec::new();
    let mut pending_raw = String::new();
    let mut pending_base = ctx.base(template.syntax().text_trimmed_range());

    for element in template.elements() {
        match element {
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                let token = chunk
                    .template_chunk_token()
                    .map_err(|_| missing("JsTemplateChunkElement", "template_chunk_token"))?;
                if pending_raw.is_empty() {
                    pending_base = ctx.base(chunk.syntax().text_trimmed_range());
                }
                pending_raw.push_str(token.text_trimmed());
            }
            AnyJsTemplateElement::JsTemplateElement(element) => {
                quasis.push(TemplateElement {
                    base: pending_base.clone(),
                    value: TemplateElementValue {
                        raw: pending_raw.clone(),
                        cooked: Some(pending_raw.clone()),
                    },
                    tail: false,
                });
                pending_raw.clear();
                pending_base = ctx.base(element.syntax().text_trimmed_range());
                expressions.push(convert_expression(
                    ctx,
                    element
                        .expression()
                        .map_err(|_| missing("JsTemplateElement", "expression"))?,
                )?);
            }
        }
    }

    quasis.push(TemplateElement {
        base: pending_base,
        value: TemplateElementValue {
            raw: pending_raw.clone(),
            cooked: Some(pending_raw),
        },
        tail: true,
    });
    Ok((quasis, expressions))
}

pub(super) fn convert_call_argument(
    ctx: &ConvertCtx<'_>,
    argument: biome_rowan::SyntaxResult<AnyJsCallArgument>,
) -> Result<Expression> {
    match argument.map_err(|_| missing("JsCallArguments", "argument"))? {
        AnyJsCallArgument::AnyJsExpression(expression) => convert_expression(ctx, expression),
        AnyJsCallArgument::JsSpread(spread) => {
            convert_spread(ctx, &spread).map(Expression::SpreadElement)
        }
    }
}

pub(super) fn convert_static_member_name(
    ctx: &ConvertCtx<'_>,
    name: AnyJsName,
) -> Result<Expression> {
    match name {
        AnyJsName::JsName(name) => Ok(Expression::Identifier(Identifier {
            base: ctx.base(name.syntax().text_trimmed_range()),
            name: name
                .value_token()
                .map_err(|_| missing("JsName", "value_token"))?
                .text_trimmed()
                .to_string(),
            type_annotation: None,
            optional: None,
            decorators: None,
        })),
        _ => Err(unsupported(name.syntax())),
    }
}

pub(super) fn convert_private_name(
    ctx: &ConvertCtx<'_>,
    name: &biome_js_syntax::JsPrivateName,
) -> Result<Expression> {
    let token = name
        .value_token()
        .map_err(|_| missing("JsPrivateName", "value_token"))?;
    Ok(Expression::PrivateName(
        react_compiler_ast::expressions::PrivateName {
            base: ctx.base(name.syntax().text_trimmed_range()),
            id: Identifier {
                base: ctx.base(token.text_trimmed_range()),
                name: token.text_trimmed().to_string(),
                type_annotation: None,
                optional: None,
                decorators: None,
            },
        },
    ))
}

pub(super) fn convert_object_member(
    ctx: &ConvertCtx<'_>,
    member: AnyJsObjectMember,
) -> Result<ObjectExpressionProperty> {
    match member {
        AnyJsObjectMember::JsPropertyObjectMember(member) => {
            let name = member
                .name()
                .map_err(|_| missing("JsPropertyObjectMember", "name"))?;
            let computed = is_computed_object_member_name(&name);
            Ok(ObjectExpressionProperty::ObjectProperty(ObjectProperty {
                base: ctx.base(member.syntax().text_trimmed_range()),
                key: Box::new(convert_object_member_name(ctx, name)?),
                value: Box::new(convert_expression(
                    ctx,
                    member
                        .value()
                        .map_err(|_| missing("JsPropertyObjectMember", "value"))?,
                )?),
                computed,
                shorthand: false,
                decorators: None,
                method: None,
            }))
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
            let name = member
                .name()
                .map_err(|_| missing("JsShorthandPropertyObjectMember", "name"))?;
            let token = name
                .value_token()
                .map_err(|_| missing("JsReferenceIdentifier", "value_token"))?;
            let identifier = Expression::Identifier(Identifier {
                base: ctx.base(name.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
                type_annotation: None,
                optional: None,
                decorators: None,
            });
            Ok(ObjectExpressionProperty::ObjectProperty(ObjectProperty {
                base: ctx.base(member.syntax().text_trimmed_range()),
                key: Box::new(identifier.clone()),
                value: Box::new(identifier),
                computed: false,
                shorthand: true,
                decorators: None,
                method: None,
            }))
        }
        AnyJsObjectMember::JsSpread(spread) => Ok(ObjectExpressionProperty::SpreadElement(
            convert_spread(ctx, &spread)?,
        )),
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            let name = method
                .name()
                .map_err(|_| missing("JsMethodObjectMember", "name"))?;
            let computed = is_computed_object_member_name(&name);
            Ok(ObjectExpressionProperty::ObjectMethod(ObjectMethod {
                base: ctx.base(method.syntax().text_trimmed_range()),
                method: true,
                kind: ObjectMethodKind::Method,
                key: Box::new(convert_object_member_name(ctx, name)?),
                params: convert_function_parameters(
                    ctx,
                    &method
                        .parameters()
                        .map_err(|_| missing("JsMethodObjectMember", "parameters"))?,
                )?,
                body: convert_function_body(
                    ctx,
                    &method
                        .body()
                        .map_err(|_| missing("JsMethodObjectMember", "body"))?,
                )?,
                computed,
                id: None,
                generator: method.star_token().is_some(),
                is_async: method.async_token().is_some(),
                decorators: None,
                return_type: None,
                type_parameters: None,
                predicate: None,
            }))
        }
        AnyJsObjectMember::JsGetterObjectMember(getter) => {
            let name = getter
                .name()
                .map_err(|_| missing("JsGetterObjectMember", "name"))?;
            let computed = is_computed_object_member_name(&name);
            Ok(ObjectExpressionProperty::ObjectMethod(ObjectMethod {
                base: ctx.base(getter.syntax().text_trimmed_range()),
                method: false,
                kind: ObjectMethodKind::Get,
                key: Box::new(convert_object_member_name(ctx, name)?),
                params: Vec::new(),
                body: convert_function_body(
                    ctx,
                    &getter
                        .body()
                        .map_err(|_| missing("JsGetterObjectMember", "body"))?,
                )?,
                computed,
                id: None,
                generator: false,
                is_async: false,
                decorators: None,
                return_type: None,
                type_parameters: None,
                predicate: None,
            }))
        }
        AnyJsObjectMember::JsSetterObjectMember(setter) => {
            let name = setter
                .name()
                .map_err(|_| missing("JsSetterObjectMember", "name"))?;
            let computed = is_computed_object_member_name(&name);
            let parameter = setter
                .parameter()
                .map_err(|_| missing("JsSetterObjectMember", "parameter"))?;
            let binding = parameter
                .as_js_formal_parameter()
                .ok_or_else(|| unsupported(parameter.syntax()))?
                .binding()
                .map_err(|_| missing("JsFormalParameter", "binding"))?;
            Ok(ObjectExpressionProperty::ObjectMethod(ObjectMethod {
                base: ctx.base(setter.syntax().text_trimmed_range()),
                method: false,
                kind: ObjectMethodKind::Set,
                key: Box::new(convert_object_member_name(ctx, name)?),
                params: vec![convert_pattern(ctx, &binding)?],
                body: convert_function_body(
                    ctx,
                    &setter
                        .body()
                        .map_err(|_| missing("JsSetterObjectMember", "body"))?,
                )?,
                computed,
                id: None,
                generator: false,
                is_async: false,
                decorators: None,
                return_type: None,
                type_parameters: None,
                predicate: None,
            }))
        }
        member => Err(unsupported(member.syntax())),
    }
}

pub(super) fn convert_class_expression(
    ctx: &ConvertCtx<'_>,
    class: &JsClassExpression,
) -> Result<ClassExpression> {
    Ok(ClassExpression {
        base: ctx.base(class.syntax().text_trimmed_range()),
        id: class
            .id()
            .map(|id| convert_any_binding_identifier(ctx, &id))
            .transpose()?,
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
        implements: None,
        super_type_parameters: None,
        type_parameters: None,
    })
}

pub(super) fn convert_function_expression(
    ctx: &ConvertCtx<'_>,
    function: &JsFunctionExpression,
) -> Result<FunctionExpression> {
    Ok(FunctionExpression {
        base: ctx.base(function.syntax().text_trimmed_range()),
        params: convert_function_parameters(
            ctx,
            &function
                .parameters()
                .map_err(|_| missing("JsFunctionExpression", "parameters"))?,
        )?,
        body: convert_function_body(
            ctx,
            &function
                .body()
                .map_err(|_| missing("JsFunctionExpression", "body"))?,
        )?,
        id: function
            .id()
            .map(|id| convert_any_binding_identifier(ctx, &id))
            .transpose()?,
        generator: function.star_token().is_some(),
        is_async: function.async_token().is_some(),
        return_type: None,
        type_parameters: None,
        predicate: None,
    })
}

pub(super) fn convert_spread(ctx: &ConvertCtx<'_>, spread: &JsSpread) -> Result<SpreadElement> {
    Ok(SpreadElement {
        base: ctx.base(spread.syntax().text_trimmed_range()),
        argument: Box::new(convert_expression(
            ctx,
            spread
                .argument()
                .map_err(|_| missing("JsSpread", "argument"))?,
        )?),
    })
}

pub(super) fn convert_post_update_expression(
    ctx: &ConvertCtx<'_>,
    update: &JsPostUpdateExpression,
) -> Result<Expression> {
    Ok(Expression::UpdateExpression(UpdateExpression {
        base: ctx.base(update.syntax().text_trimmed_range()),
        operator: convert_update_operator(
            update
                .operator_token()
                .map_err(|_| missing("JsPostUpdateExpression", "operator_token"))?
                .text_trimmed(),
        )?,
        argument: Box::new(convert_assignment(
            ctx,
            update
                .operand()
                .map_err(|_| missing("JsPostUpdateExpression", "operand"))?,
        )?),
        prefix: false,
    }))
}

pub(super) fn convert_pre_update_expression(
    ctx: &ConvertCtx<'_>,
    update: &JsPreUpdateExpression,
) -> Result<Expression> {
    Ok(Expression::UpdateExpression(UpdateExpression {
        base: ctx.base(update.syntax().text_trimmed_range()),
        operator: convert_update_operator(
            update
                .operator_token()
                .map_err(|_| missing("JsPreUpdateExpression", "operator_token"))?
                .text_trimmed(),
        )?,
        argument: Box::new(convert_assignment(
            ctx,
            update
                .operand()
                .map_err(|_| missing("JsPreUpdateExpression", "operand"))?,
        )?),
        prefix: true,
    }))
}

pub(super) fn convert_object_member_name(
    ctx: &ConvertCtx<'_>,
    name: AnyJsObjectMemberName,
) -> Result<Expression> {
    match name {
        AnyJsObjectMemberName::JsLiteralMemberName(name) => {
            let token = name
                .value()
                .map_err(|_| missing("JsLiteralMemberName", "value"))?;
            Ok(Expression::Identifier(Identifier {
                base: ctx.base(name.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
                type_annotation: None,
                optional: None,
                decorators: None,
            }))
        }
        AnyJsObjectMemberName::JsComputedMemberName(name) => convert_expression(
            ctx,
            name.expression()
                .map_err(|_| missing("JsComputedMemberName", "expression"))?,
        ),
        AnyJsObjectMemberName::JsMetavariable(name) => Err(unsupported(name.syntax())),
    }
}

fn is_computed_object_member_name(name: &AnyJsObjectMemberName) -> bool {
    matches!(name, AnyJsObjectMemberName::JsComputedMemberName(_))
}

pub(super) fn convert_arrow_parameters(
    ctx: &ConvertCtx<'_>,
    parameters: biome_js_syntax::AnyJsArrowFunctionParameters,
) -> Result<Vec<PatternLike>> {
    match parameters {
        biome_js_syntax::AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
            Ok(vec![PatternLike::Identifier(
                convert_any_binding_identifier(ctx, &binding)?,
            )])
        }
        biome_js_syntax::AnyJsArrowFunctionParameters::JsParameters(parameters) => {
            convert_function_parameters(ctx, &parameters)
        }
    }
}
