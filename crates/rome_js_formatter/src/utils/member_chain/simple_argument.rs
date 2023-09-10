use crate::utils::is_call_like_expression;
use biome_rowan::{AstSeparatedList, SyntaxResult};
use rome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, AnyJsName, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsTemplateElement, JsSpread, JsStaticMemberExpressionFields,
    JsTemplateExpression, JsUnaryOperator,
};

/// This enum tracks the arguments inside a call expressions and checks if they are
/// simple or not.
///
/// The heuristic changes based on its type and the depth of the expressions. For example
/// if we have expressions as arguments, having 2 or more them tags the first argument as "not simple".
///
/// Criteria are different:
/// - *complex*: if the chain of simple arguments exceeds the depth 2 or higher
/// - *simple*: the argument is a literal
/// - *simple*: the argument is a [JsThisExpression]
/// - *simple*: the argument is a [JsIdentifierExpression]
/// - *simple*: the argument is a [JsSuperExpression]
/// - *simple*: the argument is a [JsUnaryExpression], has `!` or `-` operator and the argument is simple
/// - *simple*: the argument is a [TsNonNullAssertionExpression] and the argument is simple
/// - if the argument is a template literal, check [is_simple_template_literal]
/// - if the argument is an object expression, all its members are checked if they are simple or not. Check [is_simple_static_member_expression]
/// - if the argument is an array expression, all its elements are checked if they are simple or not. Check [is_simple_array_expression]
///
/// This algorithm is inspired from [Prettier].
///
/// [JsThisExpression]: [rome_js_syntax::JsThisExpression]
/// [JsIdentifierExpression]: [rome_js_syntax::JsIdentifierExpression]
/// [JsSuperExpression]: [rome_js_syntax::JsSuperExpression]
/// [is_simple_static_member_expression]: [Simple::is_simple_static_member_expression]
/// [is_simple_array_expression]: [Simple::is_simple_array_expression]
/// [JsUnaryExpression]: [rome_js_syntax::JsUnaryExpression]
/// [TsNonNullAssertionExpression]: [rome_js_syntax::TsNonNullAssertionExpression]
/// [Prettier]: https://github.com/prettier/prettier/blob/a9de2a128cc8eea84ddd90efdc210378a894ab6b/src/language-js/utils/index.js#L802-L886
#[derive(Debug)]
pub(crate) enum SimpleArgument {
    Expression(AnyJsExpression),
    Name(AnyJsName),
    Spread,
}

impl SimpleArgument {
    pub fn new(node: AnyJsCallArgument) -> Self {
        match node {
            AnyJsCallArgument::AnyJsExpression(expr) => Self::Expression(expr),
            AnyJsCallArgument::JsSpread(_) => Self::Spread,
        }
    }

    pub fn is_simple(&self) -> bool {
        self.is_simple_impl(0)
    }

    fn is_simple_impl(&self, depth: u8) -> bool {
        if depth >= 2 {
            return false;
        }
        if self.is_simple_literal() {
            return true;
        }

        self.is_simple_template(depth)
            || self.is_simple_object_expression(depth)
            || self.is_simple_array_expression(depth)
            || self.is_simple_unary_expression(depth).unwrap_or(false)
            || self
                .is_simple_non_null_assertion_expression(depth)
                .unwrap_or(false)
            || self
                .is_simple_static_member_expression(depth)
                .unwrap_or(false)
            || self.is_simple_call_like_expression(depth).unwrap_or(false)
            || self.is_simple_object_expression(depth)
    }

    fn is_simple_call_like_expression(&self, depth: u8) -> SyntaxResult<bool> {
        let result = if let SimpleArgument::Expression(any_expression) = self {
            if is_call_like_expression(any_expression) {
                let mut is_import_call_expression = false;
                let mut is_simple_callee = false;
                let arguments = match any_expression {
                    AnyJsExpression::JsNewExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple_impl(depth);
                        expr.arguments()
                    }
                    AnyJsExpression::JsCallExpression(expr) => {
                        let callee = expr.callee()?;
                        is_simple_callee = SimpleArgument::from(callee).is_simple_impl(depth);
                        expr.arguments().ok()
                    }
                    AnyJsExpression::JsImportCallExpression(expr) => {
                        is_import_call_expression = true;
                        expr.arguments().ok()
                    }
                    _ => unreachable!("The check is done inside `is_call_like_expression`"),
                };

                let simple_arguments = if let Some(arguments) = arguments {
                    arguments.args().iter().all(|argument| {
                        argument.map_or(true, |argument| {
                            SimpleArgument::from(argument).is_simple_impl(depth + 1)
                        })
                    })
                } else {
                    true
                };

                (is_import_call_expression || is_simple_callee) && simple_arguments
            } else {
                false
            }
        } else {
            false
        };

        Ok(result)
    }

    fn is_simple_static_member_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(AnyJsExpression::JsStaticMemberExpression(
            static_expression,
        )) = self
        {
            let JsStaticMemberExpressionFields { member, object, .. } =
                static_expression.as_fields();

            Ok(member.is_ok() && SimpleArgument::from(object?).is_simple_impl(depth))
        } else {
            Ok(false)
        }
    }

    fn is_simple_non_null_assertion_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(AnyJsExpression::TsNonNullAssertionExpression(
            assertion,
        )) = self
        {
            Ok(SimpleArgument::from(assertion.expression()?).is_simple_impl(depth))
        } else {
            Ok(false)
        }
    }

    fn is_simple_unary_expression(&self, depth: u8) -> SyntaxResult<bool> {
        if let SimpleArgument::Expression(AnyJsExpression::JsUnaryExpression(unary_expression)) =
            self
        {
            if matches!(
                unary_expression.operator()?,
                JsUnaryOperator::LogicalNot | JsUnaryOperator::Minus
            ) {
                Ok(SimpleArgument::from(unary_expression.argument()?).is_simple_impl(depth))
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn is_simple_array_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(AnyJsExpression::JsArrayExpression(array_expression)) =
            self
        {
            array_expression
                .elements()
                .iter()
                .filter_map(|element| element.ok())
                .all(|element| match element {
                    AnyJsArrayElement::AnyJsExpression(expression) => {
                        SimpleArgument::from(expression).is_simple_impl(depth + 1)
                    }
                    _ => false,
                })
        } else {
            false
        }
    }

    fn is_simple_template(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(AnyJsExpression::JsTemplateExpression(template)) = self {
            is_simple_template_literal(template, depth + 1).unwrap_or(false)
        } else {
            false
        }
    }

    const fn is_simple_literal(&self) -> bool {
        if let SimpleArgument::Name(AnyJsName::JsPrivateName(_)) = self {
            return true;
        }

        matches!(
            self,
            SimpleArgument::Expression(
                AnyJsExpression::AnyJsLiteralExpression(_)
                    | AnyJsExpression::JsThisExpression(_)
                    | AnyJsExpression::JsIdentifierExpression(_)
                    | AnyJsExpression::JsSuperExpression(_),
            )
        )
    }

    fn is_simple_object_expression(&self, depth: u8) -> bool {
        if let SimpleArgument::Expression(AnyJsExpression::JsObjectExpression(object_expression)) =
            self
        {
            object_expression
                .members()
                .iter()
                .filter_map(|member| member.ok())
                .all(|member| {
                    use AnyJsObjectMember::*;

                    match member {
                        JsShorthandPropertyObjectMember(_) => true,
                        JsPropertyObjectMember(property) => {
                            let is_computed = matches!(
                                property.name(),
                                Ok(AnyJsObjectMemberName::JsComputedMemberName(_))
                            );

                            let is_simple = property.value().map_or(false, |value| {
                                SimpleArgument::from(value).is_simple_impl(depth + 1)
                            });

                            !is_computed && is_simple
                        }
                        _ => false,
                    }
                })
        } else {
            false
        }
    }
}

impl From<AnyJsExpression> for SimpleArgument {
    fn from(expr: AnyJsExpression) -> Self {
        Self::Expression(expr)
    }
}

impl From<AnyJsName> for SimpleArgument {
    fn from(name: AnyJsName) -> Self {
        Self::Name(name)
    }
}

impl From<JsSpread> for SimpleArgument {
    fn from(_: JsSpread) -> Self {
        Self::Spread
    }
}

impl From<AnyJsCallArgument> for SimpleArgument {
    fn from(call_argument: AnyJsCallArgument) -> Self {
        match call_argument {
            AnyJsCallArgument::AnyJsExpression(expr) => SimpleArgument::from(expr),
            AnyJsCallArgument::JsSpread(spread) => SimpleArgument::from(spread),
        }
    }
}

/// A template literal is simple when:
///
/// - all strings dont contain newlines
/// - the expressions contained in the template are considered as `is_simple_call_argument`. Check
/// [is_simple_call_argument].
pub fn is_simple_template_literal(
    template: &JsTemplateExpression,
    depth: u8,
) -> SyntaxResult<bool> {
    for element in template.elements() {
        match element {
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                if chunk.template_chunk_token()?.text_trimmed().contains('\n') {
                    return Ok(false);
                }
            }
            AnyJsTemplateElement::JsTemplateElement(element) => {
                let expression = element.expression()?;
                if !(SimpleArgument::from(expression).is_simple_impl(depth)) {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}
