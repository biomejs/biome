use super::*;

pub(super) fn convert_pattern(
    ctx: &ConvertCtx<'_>,
    pattern: &AnyJsBindingPattern,
) -> Result<PatternLike> {
    match pattern {
        AnyJsBindingPattern::AnyJsBinding(binding) => Ok(PatternLike::Identifier(
            convert_binding_identifier(ctx, binding.syntax())?,
        )),
        AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Ok(PatternLike::ObjectPattern(
            convert_object_binding_pattern(ctx, pattern)?,
        )),
        AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Ok(PatternLike::ArrayPattern(
            ArrayPattern {
                base: ctx.base(pattern.syntax().text_trimmed_range()),
                elements: pattern
                    .elements()
                    .into_iter()
                    .map(|element| {
                        match element.map_err(|_| missing("JsArrayBindingPattern", "element"))? {
                            biome_js_syntax::AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) => {
                                let converted = convert_pattern(
                                    ctx,
                                    &element.pattern().map_err(|_| {
                                        missing("JsArrayBindingPatternElement", "pattern")
                                    })?,
                                )?;
                                convert_optional_default_pattern(ctx, converted, element.init())
                                    .map(Some)
                            }
                            biome_js_syntax::AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(element) => {
                                Ok(Some(PatternLike::RestElement(RestElement {
                                    base: ctx.base(element.syntax().text_trimmed_range()),
                                    argument: Box::new(convert_pattern(
                                        ctx,
                                        &element.pattern().map_err(|_| {
                                            missing("JsArrayBindingPatternRestElement", "pattern")
                                        })?,
                                    )?),
                                    type_annotation: None,
                                    decorators: None,
                                })))
                            }
                            biome_js_syntax::AnyJsArrayBindingPatternElement::JsArrayHole(_) => Ok(None),
                        }
                    })
                    .collect::<Result<Vec<_>>>()?,
                type_annotation: None,
                decorators: None,
            },
        )),
    }
}

pub(super) fn convert_object_binding_pattern(
    ctx: &ConvertCtx<'_>,
    pattern: &JsObjectBindingPattern,
) -> Result<ObjectPattern> {
    Ok(ObjectPattern {
        base: ctx.base(pattern.syntax().text_trimmed_range()),
        properties: pattern
            .properties()
            .into_iter()
            .map(|property| {
                match property.map_err(|_| missing("JsObjectBindingPattern", "property"))? {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                        let member = property
                            .member()
                            .map_err(|_| missing("JsObjectBindingPatternProperty", "member"))?;
                        let computed =
                            matches!(member, AnyJsObjectMemberName::JsComputedMemberName(_));
                        let converted = convert_pattern(
                            ctx,
                            &property.pattern().map_err(|_| {
                                missing("JsObjectBindingPatternProperty", "pattern")
                            })?,
                        )?;
                        Ok(ObjectPatternProperty::ObjectProperty(ObjectPatternProp {
                            base: ctx.base(property.syntax().text_trimmed_range()),
                            key: Box::new(convert_object_member_name(ctx, member)?),
                            value: Box::new(convert_optional_default_pattern(
                                ctx,
                                converted,
                                property.init(),
                            )?),
                            computed,
                            shorthand: false,
                            decorators: None,
                            method: None,
                        }))
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        property,
                    ) => {
                        let identifier = convert_any_binding_identifier(
                            ctx,
                            &property.identifier().map_err(|_| {
                                missing("JsObjectBindingPatternShorthandProperty", "identifier")
                            })?,
                        )?;
                        Ok(ObjectPatternProperty::ObjectProperty(ObjectPatternProp {
                            base: ctx.base(property.syntax().text_trimmed_range()),
                            key: Box::new(Expression::Identifier(identifier.clone())),
                            value: Box::new(convert_optional_default_pattern(
                                ctx,
                                PatternLike::Identifier(identifier),
                                property.init(),
                            )?),
                            computed: false,
                            shorthand: true,
                            decorators: None,
                            method: None,
                        }))
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        Ok(ObjectPatternProperty::RestElement(RestElement {
                            base: ctx.base(rest.syntax().text_trimmed_range()),
                            argument: Box::new(PatternLike::Identifier(
                                convert_any_binding_identifier(
                                    ctx,
                                    &rest.binding().map_err(|_| {
                                        missing("JsObjectBindingPatternRest", "binding")
                                    })?,
                                )?,
                            )),
                            type_annotation: None,
                            decorators: None,
                        }))
                    }
                    property => Err(unsupported(property.syntax())),
                }
            })
            .collect::<Result<Vec<_>>>()?,
        type_annotation: None,
        decorators: None,
    })
}

pub(super) fn convert_optional_default_pattern(
    ctx: &ConvertCtx<'_>,
    left: PatternLike,
    init: Option<biome_js_syntax::JsInitializerClause>,
) -> Result<PatternLike> {
    match init {
        Some(init) => Ok(PatternLike::AssignmentPattern(AssignmentPattern {
            base: ctx.base(init.syntax().text_trimmed_range()),
            left: Box::new(left),
            right: Box::new(convert_expression(
                ctx,
                init.expression()
                    .map_err(|_| missing("JsInitializerClause", "expression"))?,
            )?),
            type_annotation: None,
            decorators: None,
        })),
        None => Ok(left),
    }
}

pub(super) fn convert_assignment_pattern(
    ctx: &ConvertCtx<'_>,
    pattern: AnyJsAssignmentPattern,
) -> Result<PatternLike> {
    match pattern {
        AnyJsAssignmentPattern::AnyJsAssignment(assignment) => convert_assignment(ctx, assignment)
            .and_then(|expression| match expression {
                Expression::Identifier(identifier) => Ok(PatternLike::Identifier(identifier)),
                Expression::MemberExpression(member) => Ok(PatternLike::MemberExpression(member)),
                _ => Err(ReactCompilerError::CompilerOutput(
                    "unsupported assignment target".to_string(),
                )),
            }),
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(pattern) => {
            Ok(PatternLike::ArrayPattern(ArrayPattern {
                base: ctx.base(pattern.syntax().text_trimmed_range()),
                elements: pattern
                    .elements()
                    .into_iter()
                    .map(|element| {
                        match element.map_err(|_| missing("JsArrayAssignmentPattern", "element"))? {
                            biome_js_syntax::AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(element) => {
                                let converted = convert_assignment_pattern(
                                    ctx,
                                    element.pattern().map_err(|_| {
                                        missing("JsArrayAssignmentPatternElement", "pattern")
                                    })?,
                                )?;
                                convert_optional_default_pattern(ctx, converted, element.init())
                                    .map(Some)
                            }
                            biome_js_syntax::AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(element) => {
                                Ok(Some(PatternLike::RestElement(RestElement {
                                    base: ctx.base(element.syntax().text_trimmed_range()),
                                    argument: Box::new(convert_assignment_pattern(
                                        ctx,
                                        element.pattern().map_err(|_| {
                                            missing("JsArrayAssignmentPatternRestElement", "pattern")
                                        })?,
                                    )?),
                                    type_annotation: None,
                                    decorators: None,
                                })))
                            }
                            biome_js_syntax::AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => Ok(None),
                        }
                    })
                    .collect::<Result<Vec<_>>>()?,
                type_annotation: None,
                decorators: None,
            }))
        }
        AnyJsAssignmentPattern::JsObjectAssignmentPattern(pattern) => {
            Ok(PatternLike::ObjectPattern(ObjectPattern {
                base: ctx.base(pattern.syntax().text_trimmed_range()),
                properties: pattern
                    .properties()
                    .into_iter()
                    .map(|property| convert_object_assignment_pattern_member(ctx, property.map_err(|_| missing("JsObjectAssignmentPattern", "property"))?))
                    .collect::<Result<Vec<_>>>()?,
                type_annotation: None,
                decorators: None,
            }))
        }
    }
}

fn convert_object_assignment_pattern_member(
    ctx: &ConvertCtx<'_>,
    property: biome_js_syntax::AnyJsObjectAssignmentPatternMember,
) -> Result<ObjectPatternProperty> {
    match property {
        biome_js_syntax::AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
            let member = property
                .member()
                .map_err(|_| missing("JsObjectAssignmentPatternProperty", "member"))?;
            let computed = matches!(member, AnyJsObjectMemberName::JsComputedMemberName(_));
            let converted = convert_assignment_pattern(
                ctx,
                property
                    .pattern()
                    .map_err(|_| missing("JsObjectAssignmentPatternProperty", "pattern"))?,
            )?;
            Ok(ObjectPatternProperty::ObjectProperty(ObjectPatternProp {
                base: ctx.base(property.syntax().text_trimmed_range()),
                key: Box::new(convert_object_member_name(ctx, member)?),
                value: Box::new(convert_optional_default_pattern(ctx, converted, property.init())?),
                computed,
                shorthand: false,
                decorators: None,
                method: None,
            }))
        }
        biome_js_syntax::AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(property) => {
            let identifier = match convert_assignment(
                ctx,
                AnyJsAssignment::JsIdentifierAssignment(
                    property
                        .identifier()
                        .map_err(|_| missing("JsObjectAssignmentPatternShorthandProperty", "identifier"))?,
                ),
            )? {
                Expression::Identifier(identifier) => identifier,
                _ => return Err(unsupported(property.syntax())),
            };
            Ok(ObjectPatternProperty::ObjectProperty(ObjectPatternProp {
                base: ctx.base(property.syntax().text_trimmed_range()),
                key: Box::new(Expression::Identifier(identifier.clone())),
                value: Box::new(convert_optional_default_pattern(
                    ctx,
                    PatternLike::Identifier(identifier),
                    property.init(),
                )?),
                computed: false,
                shorthand: true,
                decorators: None,
                method: None,
            }))
        }
        biome_js_syntax::AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
            let target = convert_assignment(ctx, rest.target().map_err(|_| missing("JsObjectAssignmentPatternRest", "target"))?)?;
            let argument = match target {
                Expression::Identifier(identifier) => PatternLike::Identifier(identifier),
                Expression::MemberExpression(member) => PatternLike::MemberExpression(member),
                _ => return Err(unsupported(rest.syntax())),
            };
            Ok(ObjectPatternProperty::RestElement(RestElement {
                base: ctx.base(rest.syntax().text_trimmed_range()),
                argument: Box::new(argument),
                type_annotation: None,
                decorators: None,
            }))
        }
        property => Err(unsupported(property.syntax())),
    }
}

pub(super) fn convert_assignment(
    ctx: &ConvertCtx<'_>,
    assignment: AnyJsAssignment,
) -> Result<Expression> {
    match assignment {
        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
            let token = identifier
                .name_token()
                .map_err(|_| missing("JsIdentifierAssignment", "name_token"))?;
            Ok(Expression::Identifier(Identifier {
                base: ctx.base(identifier.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
                type_annotation: None,
                optional: None,
                decorators: None,
            }))
        }
        AnyJsAssignment::JsStaticMemberAssignment(member) => {
            Ok(Expression::MemberExpression(MemberExpression {
                base: ctx.base(member.syntax().text_trimmed_range()),
                object: Box::new(convert_expression(
                    ctx,
                    member
                        .object()
                        .map_err(|_| missing("JsStaticMemberAssignment", "object"))?,
                )?),
                property: Box::new(convert_static_member_name(
                    ctx,
                    member
                        .member()
                        .map_err(|_| missing("JsStaticMemberAssignment", "member"))?,
                )?),
                computed: false,
            }))
        }
        AnyJsAssignment::JsComputedMemberAssignment(member) => {
            Ok(Expression::MemberExpression(MemberExpression {
                base: ctx.base(member.syntax().text_trimmed_range()),
                object: Box::new(convert_expression(
                    ctx,
                    member
                        .object()
                        .map_err(|_| missing("JsComputedMemberAssignment", "object"))?,
                )?),
                property: Box::new(convert_expression(
                    ctx,
                    member
                        .member()
                        .map_err(|_| missing("JsComputedMemberAssignment", "member"))?,
                )?),
                computed: true,
            }))
        }
        AnyJsAssignment::JsParenthesizedAssignment(assignment) => convert_assignment(
            ctx,
            assignment
                .assignment()
                .map_err(|_| missing("JsParenthesizedAssignment", "assignment"))?,
        ),
        assignment => Err(unsupported(assignment.syntax())),
    }
}

pub(super) fn convert_reference_identifier(
    ctx: &ConvertCtx<'_>,
    identifier: &JsIdentifierExpression,
) -> Result<Identifier> {
    let name = identifier
        .name()
        .map_err(|_| missing("JsIdentifierExpression", "name"))?;
    let name_token = name
        .value_token()
        .map_err(|_| missing("JsReferenceIdentifier", "value_token"))?;
    Ok(Identifier {
        base: ctx.base(identifier.syntax().text_trimmed_range()),
        name: name_token.text_trimmed().to_string(),
        type_annotation: None,
        optional: None,
        decorators: None,
    })
}

pub(super) fn convert_binding_identifier(
    ctx: &ConvertCtx<'_>,
    syntax: &JsSyntaxNode,
) -> Result<Identifier> {
    let binding = JsIdentifierBinding::cast(syntax.clone()).ok_or_else(|| unsupported(syntax))?;
    let name = binding
        .name_token()
        .map_err(|_| missing("JsIdentifierBinding", "name_token"))?;
    Ok(Identifier {
        base: ctx.base(binding.syntax().text_trimmed_range()),
        name: name.text_trimmed().to_string(),
        type_annotation: None,
        optional: None,
        decorators: None,
    })
}

pub(super) fn convert_any_binding_identifier(
    ctx: &ConvertCtx<'_>,
    binding: &AnyJsBinding,
) -> Result<Identifier> {
    convert_binding_identifier(ctx, binding.syntax())
}
