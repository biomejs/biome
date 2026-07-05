use super::*;

pub(super) fn convert_jsx_tag_expression(
    ctx: &ConvertCtx<'_>,
    tag: AnyJsxTag,
) -> Result<Expression> {
    match tag {
        AnyJsxTag::JsxElement(element) => Ok(Expression::JSXElement(Box::new(
            convert_jsx_element(ctx, &element)?,
        ))),
        AnyJsxTag::JsxSelfClosingElement(element) => Ok(Expression::JSXElement(Box::new(
            convert_jsx_self_closing_element(ctx, &element)?,
        ))),
        AnyJsxTag::JsxFragment(fragment) => Ok(Expression::JSXFragment(convert_jsx_fragment(
            ctx, &fragment,
        )?)),
    }
}

pub(super) fn convert_jsx_element(
    ctx: &ConvertCtx<'_>,
    element: &JsxElement,
) -> Result<JSXElement> {
    let opening = element
        .opening_element()
        .map_err(|_| missing("JsxElement", "opening_element"))?;
    let closing = element
        .closing_element()
        .map_err(|_| missing("JsxElement", "closing_element"))?;
    Ok(JSXElement {
        base: ctx.base(element.syntax().text_trimmed_range()),
        opening_element: JSXOpeningElement {
            base: ctx.base(opening.syntax().text_trimmed_range()),
            name: convert_jsx_element_name(
                ctx,
                opening
                    .name()
                    .map_err(|_| missing("JsxOpeningElement", "name"))?,
            )?,
            attributes: convert_jsx_attributes(ctx, opening.attributes())?,
            self_closing: false,
            type_parameters: None,
        },
        closing_element: Some(JSXClosingElement {
            base: ctx.base(closing.syntax().text_trimmed_range()),
            name: convert_jsx_element_name(
                ctx,
                closing
                    .name()
                    .map_err(|_| missing("JsxClosingElement", "name"))?,
            )?,
        }),
        children: element
            .children()
            .into_iter()
            .map(|child| convert_jsx_child(ctx, child))
            .collect::<Result<Vec<_>>>()?,
        self_closing: Some(false),
    })
}

pub(super) fn convert_jsx_self_closing_element(
    ctx: &ConvertCtx<'_>,
    element: &JsxSelfClosingElement,
) -> Result<JSXElement> {
    Ok(JSXElement {
        base: ctx.base(element.syntax().text_trimmed_range()),
        opening_element: JSXOpeningElement {
            base: ctx.base(element.syntax().text_trimmed_range()),
            name: convert_jsx_element_name(
                ctx,
                element
                    .name()
                    .map_err(|_| missing("JsxSelfClosingElement", "name"))?,
            )?,
            attributes: convert_jsx_attributes(ctx, element.attributes())?,
            self_closing: true,
            type_parameters: None,
        },
        closing_element: None,
        children: Vec::new(),
        self_closing: Some(true),
    })
}

pub(super) fn convert_jsx_child(ctx: &ConvertCtx<'_>, child: AnyJsxChild) -> Result<JSXChild> {
    match child {
        AnyJsxChild::JsxElement(element) => Ok(JSXChild::JSXElement(Box::new(
            convert_jsx_element(ctx, &element)?,
        ))),
        AnyJsxChild::JsxSelfClosingElement(element) => Ok(JSXChild::JSXElement(Box::new(
            convert_jsx_self_closing_element(ctx, &element)?,
        ))),
        AnyJsxChild::JsxFragment(fragment) => {
            Ok(JSXChild::JSXFragment(convert_jsx_fragment(ctx, &fragment)?))
        }
        AnyJsxChild::JsxSpreadChild(child) => Ok(JSXChild::JSXSpreadChild(JSXSpreadChild {
            base: ctx.base(child.syntax().text_trimmed_range()),
            expression: Box::new(convert_expression(
                ctx,
                child
                    .expression()
                    .map_err(|_| missing("JsxSpreadChild", "expression"))?,
            )?),
        })),
        AnyJsxChild::JsxExpressionChild(child) => Ok(JSXChild::JSXExpressionContainer(
            convert_jsx_expression_child(ctx, &child)?,
        )),
        AnyJsxChild::JsxText(text) => Ok(JSXChild::JSXText(convert_jsx_text(ctx, &text)?)),
        child => Err(unsupported(child.syntax())),
    }
}

pub(super) fn convert_jsx_fragment(
    ctx: &ConvertCtx<'_>,
    fragment: &JsxFragment,
) -> Result<JSXFragment> {
    let opening = fragment
        .opening_fragment()
        .map_err(|_| missing("JsxFragment", "opening_fragment"))?;
    let closing = fragment
        .closing_fragment()
        .map_err(|_| missing("JsxFragment", "closing_fragment"))?;
    Ok(JSXFragment {
        base: ctx.base(fragment.syntax().text_trimmed_range()),
        opening_fragment: JSXOpeningFragment {
            base: ctx.base(opening.syntax().text_trimmed_range()),
        },
        closing_fragment: JSXClosingFragment {
            base: ctx.base(closing.syntax().text_trimmed_range()),
        },
        children: fragment
            .children()
            .into_iter()
            .map(|child| convert_jsx_child(ctx, child))
            .collect::<Result<Vec<_>>>()?,
    })
}

pub(super) fn convert_jsx_expression_child(
    ctx: &ConvertCtx<'_>,
    child: &JsxExpressionChild,
) -> Result<JSXExpressionContainer> {
    Ok(JSXExpressionContainer {
        base: ctx.base(child.syntax().text_trimmed_range()),
        expression: match child.expression() {
            Some(expression) => JSXExpressionContainerExpr::Expression(Box::new(
                convert_expression(ctx, expression)?,
            )),
            None => JSXExpressionContainerExpr::JSXEmptyExpression(JSXEmptyExpression {
                base: ctx.base(child.syntax().text_trimmed_range()),
            }),
        },
    })
}

pub(super) fn convert_jsx_text(ctx: &ConvertCtx<'_>, text: &JsxText) -> Result<JSXText> {
    let token = text
        .value_token()
        .map_err(|_| missing("JsxText", "value_token"))?;
    Ok(JSXText {
        base: ctx.base(text.syntax().text_trimmed_range()),
        value: token.text_trimmed().to_string(),
    })
}

pub(super) fn convert_jsx_attributes(
    ctx: &ConvertCtx<'_>,
    attributes: biome_js_syntax::JsxAttributeList,
) -> Result<Vec<JSXAttributeItem>> {
    attributes
        .into_iter()
        .map(|attribute| match attribute {
            AnyJsxAttribute::JsxAttribute(attribute) => Ok(JSXAttributeItem::JSXAttribute(
                convert_jsx_attribute(ctx, &attribute)?,
            )),
            AnyJsxAttribute::JsxSpreadAttribute(spread) => Ok(
                JSXAttributeItem::JSXSpreadAttribute(convert_jsx_spread_attribute(ctx, &spread)?),
            ),
            attribute => Err(unsupported(attribute.syntax())),
        })
        .collect()
}

pub(super) fn convert_jsx_spread_attribute(
    ctx: &ConvertCtx<'_>,
    attribute: &JsxSpreadAttribute,
) -> Result<JSXSpreadAttribute> {
    Ok(JSXSpreadAttribute {
        base: ctx.base(attribute.syntax().text_trimmed_range()),
        argument: Box::new(convert_expression(
            ctx,
            attribute
                .argument()
                .map_err(|_| missing("JsxSpreadAttribute", "argument"))?,
        )?),
    })
}

pub(super) fn convert_jsx_attribute(
    ctx: &ConvertCtx<'_>,
    attribute: &JsxAttribute,
) -> Result<JSXAttribute> {
    Ok(JSXAttribute {
        base: ctx.base(attribute.syntax().text_trimmed_range()),
        name: convert_jsx_attribute_name(
            ctx,
            attribute
                .name()
                .map_err(|_| missing("JsxAttribute", "name"))?,
        )?,
        value: attribute
            .initializer()
            .map(|initializer| {
                initializer
                    .value()
                    .map_err(|_| missing("JsxAttributeInitializerClause", "value"))
                    .and_then(|value| convert_jsx_attribute_value(ctx, value))
            })
            .transpose()?,
    })
}

pub(super) fn convert_jsx_attribute_value(
    ctx: &ConvertCtx<'_>,
    value: AnyJsxAttributeValue,
) -> Result<JSXAttributeValue> {
    match value {
        AnyJsxAttributeValue::JsxString(string) => Ok(JSXAttributeValue::StringLiteral(
            convert_jsx_string_literal(ctx, &string)?,
        )),
        AnyJsxAttributeValue::JsxExpressionAttributeValue(expression) => {
            Ok(JSXAttributeValue::JSXExpressionContainer(
                convert_jsx_expression_attribute_value(ctx, &expression)?,
            ))
        }
        AnyJsxAttributeValue::AnyJsxTag(AnyJsxTag::JsxElement(element)) => Ok(
            JSXAttributeValue::JSXElement(Box::new(convert_jsx_element(ctx, &element)?)),
        ),
        value => Err(unsupported(value.syntax())),
    }
}

pub(super) fn convert_jsx_expression_attribute_value(
    ctx: &ConvertCtx<'_>,
    value: &JsxExpressionAttributeValue,
) -> Result<JSXExpressionContainer> {
    Ok(JSXExpressionContainer {
        base: ctx.base(value.syntax().text_trimmed_range()),
        expression: JSXExpressionContainerExpr::Expression(Box::new(convert_expression(
            ctx,
            value
                .expression()
                .map_err(|_| missing("JsxExpressionAttributeValue", "expression"))?,
        )?)),
    })
}

pub(super) fn convert_jsx_string_literal(
    ctx: &ConvertCtx<'_>,
    string: &JsxString,
) -> Result<StringLiteral> {
    let token = string
        .value_token()
        .map_err(|_| missing("JsxString", "value_token"))?;
    Ok(StringLiteral {
        base: ctx.base(string.syntax().text_trimmed_range()),
        value: inner_string_text(&token).to_string().into(),
    })
}

pub(super) fn convert_jsx_attribute_name(
    ctx: &ConvertCtx<'_>,
    name: AnyJsxAttributeName,
) -> Result<JSXAttributeName> {
    match name {
        AnyJsxAttributeName::JsxName(name) => Ok(JSXAttributeName::JSXIdentifier(
            convert_jsx_identifier(ctx, &name)?,
        )),
        AnyJsxAttributeName::JsxNamespaceName(name) => Ok(JSXAttributeName::JSXNamespacedName(
            convert_jsx_namespace_name(ctx, &name)?,
        )),
    }
}

pub(super) fn convert_jsx_namespace_name(
    ctx: &ConvertCtx<'_>,
    name: &biome_js_syntax::JsxNamespaceName,
) -> Result<JSXNamespacedName> {
    let namespace = name
        .namespace()
        .map_err(|_| missing("JsxNamespaceName", "namespace"))?;
    let local = name
        .name()
        .map_err(|_| missing("JsxNamespaceName", "name"))?;
    Ok(JSXNamespacedName {
        base: ctx.base(name.syntax().text_trimmed_range()),
        namespace: convert_jsx_identifier(ctx, &namespace)?,
        name: convert_jsx_identifier(ctx, &local)?,
    })
}

pub(super) fn convert_jsx_element_name(
    ctx: &ConvertCtx<'_>,
    name: AnyJsxElementName,
) -> Result<JSXElementName> {
    match name {
        AnyJsxElementName::JsxName(name) => Ok(JSXElementName::JSXIdentifier(
            convert_jsx_identifier(ctx, &name)?,
        )),
        AnyJsxElementName::JsxReferenceIdentifier(name) => {
            let token = name
                .value_token()
                .map_err(|_| missing("JsxReferenceIdentifier", "value_token"))?;
            Ok(JSXElementName::JSXIdentifier(JSXIdentifier {
                base: ctx.base(name.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
            }))
        }
        AnyJsxElementName::JsxMemberName(name) => Ok(JSXElementName::JSXMemberExpression(
            convert_jsx_member_name(ctx, &name)?,
        )),
        AnyJsxElementName::JsxNamespaceName(name) => Ok(JSXElementName::JSXNamespacedName(
            convert_jsx_namespace_name(ctx, &name)?,
        )),
        _ => Err(unsupported(name.syntax())),
    }
}

pub(super) fn convert_jsx_member_name(
    ctx: &ConvertCtx<'_>,
    name: &JsxMemberName,
) -> Result<JSXMemberExpression> {
    let object = match name
        .object()
        .map_err(|_| missing("JsxMemberName", "object"))?
    {
        AnyJsxObjectName::JsxReferenceIdentifier(object) => {
            let token = object
                .value_token()
                .map_err(|_| missing("JsxReferenceIdentifier", "value_token"))?;
            JSXMemberExprObject::JSXIdentifier(JSXIdentifier {
                base: ctx.base(object.syntax().text_trimmed_range()),
                name: token.text_trimmed().to_string(),
            })
        }
        AnyJsxObjectName::JsxMemberName(object) => JSXMemberExprObject::JSXMemberExpression(
            Box::new(convert_jsx_member_name(ctx, &object)?),
        ),
        object => return Err(unsupported(object.syntax())),
    };
    let member = name
        .member()
        .map_err(|_| missing("JsxMemberName", "member"))?;
    let token = member
        .value_token()
        .map_err(|_| missing("JsName", "value_token"))?;
    Ok(JSXMemberExpression {
        base: ctx.base(name.syntax().text_trimmed_range()),
        object: Box::new(object),
        property: JSXIdentifier {
            base: ctx.base(member.syntax().text_trimmed_range()),
            name: token.text_trimmed().to_string(),
        },
    })
}

pub(super) fn convert_jsx_identifier(
    ctx: &ConvertCtx<'_>,
    name: &JsxName,
) -> Result<JSXIdentifier> {
    let token = name
        .value_token()
        .map_err(|_| missing("JsxName", "value_token"))?;
    Ok(JSXIdentifier {
        base: ctx.base(name.syntax().text_trimmed_range()),
        name: token.text_trimmed().to_string(),
    })
}
