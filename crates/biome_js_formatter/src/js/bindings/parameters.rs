use crate::prelude::*;
use biome_formatter::{write, CstFormatContext};

use crate::js::expressions::arrow_function_expression::can_avoid_parentheses;
use crate::js::lists::parameter_list::FormatJsAnyParameterList;
use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
use biome_js_syntax::{
    is_test_call_argument, AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter,
    AnyJsExpression, AnyJsFormalParameter, AnyJsParameter, AnyTsType, JsArrowFunctionExpression,
    JsConstructorParameters, JsParameters, JsSyntaxNode, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsParameters();

impl FormatNodeRule<JsParameters> for FormatJsParameters {
    fn fmt_fields(&self, node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatAnyJsParameters::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &JsParameters, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `FormatJsAnyParameters`
        Ok(())
    }
}

declare_node_union! {
    pub(crate) FormatAnyJsParameters = JsParameters | JsConstructorParameters
}

impl Format<JsFormatContext> for FormatAnyJsParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let list = self.list();

        let parentheses_not_needed = self
            .as_arrow_function_expression()
            .is_some_and(|expression| can_avoid_parentheses(&expression, f));
        let has_any_decorated_parameter = list.has_any_decorated_parameter();

        let can_hug =
            should_hug_function_parameters(self, f.context().comments(), parentheses_not_needed)?
                && !has_any_decorated_parameter;

        let layout = if list.is_empty() {
            ParameterLayout::NoParameters
        } else if can_hug || self.is_in_test_call()? {
            ParameterLayout::Hug
        } else {
            ParameterLayout::Default
        };

        let l_paren_token = self.l_paren_token()?;
        let r_paren_token = self.r_paren_token()?;

        match layout {
            ParameterLayout::NoParameters => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        format_dangling_comments(self.syntax()).with_soft_block_indent(),
                        r_paren_token.format()
                    ]
                )
            }
            ParameterLayout::Hug => {
                if !parentheses_not_needed {
                    write!(f, [l_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&l_paren_token)])?;
                }

                write!(
                    f,
                    [FormatJsAnyParameterList::with_layout(
                        &list,
                        ParameterLayout::Hug
                    )]
                )?;

                if !parentheses_not_needed {
                    write!(f, [&r_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&r_paren_token)])?;
                }

                Ok(())
            }
            ParameterLayout::Default => {
                if !parentheses_not_needed {
                    write!(f, [l_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&l_paren_token)])?;
                }

                write!(
                    f,
                    [soft_block_indent(&FormatJsAnyParameterList::with_layout(
                        &list,
                        ParameterLayout::Default,
                    ))]
                )?;

                if !parentheses_not_needed {
                    write!(f, [r_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&r_paren_token)])?;
                }

                Ok(())
            }
        }
    }
}

impl FormatAnyJsParameters {
    fn l_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatAnyJsParameters::JsParameters(parameters) => parameters.l_paren_token(),
            FormatAnyJsParameters::JsConstructorParameters(parameters) => {
                parameters.l_paren_token()
            }
        }
    }

    fn list(&self) -> AnyJsParameterList {
        match self {
            FormatAnyJsParameters::JsParameters(parameters) => {
                AnyJsParameterList::from(parameters.items())
            }
            FormatAnyJsParameters::JsConstructorParameters(parameters) => {
                AnyJsParameterList::from(parameters.parameters())
            }
        }
    }

    fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatAnyJsParameters::JsParameters(parameters) => parameters.r_paren_token(),
            FormatAnyJsParameters::JsConstructorParameters(parameters) => {
                parameters.r_paren_token()
            }
        }
    }

    /// Returns `true` for function parameters if the function is an argument of a [test `CallExpression`](is_test_call_expression).
    fn is_in_test_call(&self) -> SyntaxResult<bool> {
        let result = match self {
            FormatAnyJsParameters::JsParameters(parameters) => match parameters.syntax().parent() {
                Some(function) => is_test_call_argument(&function)?,
                None => false,
            },
            FormatAnyJsParameters::JsConstructorParameters(_) => false,
        };

        Ok(result)
    }

    fn as_arrow_function_expression(&self) -> Option<JsArrowFunctionExpression> {
        match self {
            FormatAnyJsParameters::JsParameters(parameters) => parameters
                .syntax()
                .parent()
                .and_then(JsArrowFunctionExpression::cast),
            FormatAnyJsParameters::JsConstructorParameters(_) => None,
        }
    }

    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            FormatAnyJsParameters::JsParameters(parameters) => parameters.syntax(),
            FormatAnyJsParameters::JsConstructorParameters(parameters) => parameters.syntax(),
        }
    }
}

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum ParameterLayout {
    /// ```javascript
    /// function test() {}
    /// ```
    NoParameters,

    /// Enforce that the opening and closing parentheses aren't separated from the first token of the parameter.
    /// For example, to enforce that the `{`  and `}` of an object expression are formatted on the same line
    /// as the `(` and `)` tokens even IF the object expression itself breaks across multiple lines.
    ///
    /// ```javascript
    /// function test({
    ///     aVeryLongObjectBinding,
    ///     thatContinuesAndExceeds,
    ///     theLineWidth
    /// }) {}
    /// ```
    Hug,

    /// The default layout formats all parameters on the same line if they fit or breaks after the `(`
    /// and before the `)`.
    ///
    /// ```javascript
    /// function test(
    ///     firstParameter,
    ///     secondParameter,
    ///     thirdParameter
    /// ) {}
    /// ```
    Default,
}

pub(crate) fn should_hug_function_parameters(
    parameters: &FormatAnyJsParameters,
    comments: &JsComments,
    parentheses_not_needed: bool,
) -> FormatResult<bool> {
    /// Returns true if the first parameter should be forced onto the same line as the `(` and `)` parentheses.
    /// See the `[ParameterLayout::Hug] documentation.
    ///
    /// parameter `should_hug_formal_parameter` is a bool value used to determine whether the parenthesized arrow function parameters
    /// should be on the same line as the arrow function after removing the parentheses.
    fn hug_formal_parameter(
        parameter: &self::AnyJsFormalParameter,
        should_hug_formal_parameter: bool,
    ) -> FormatResult<bool> {
        let result = match parameter {
            AnyJsFormalParameter::JsFormalParameter(parameter) => {
                match parameter.initializer() {
                    None => {
                        match parameter.binding()? {
                            // always true for `[a]` or `{a}`
                            AnyJsBindingPattern::JsArrayBindingPattern(_)
                            | AnyJsBindingPattern::JsObjectBindingPattern(_)
                            | AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsMetavariable(_)) => {
                                true
                            }
                            // if the type parameter is an object type
                            // `a: { prop: string }`
                            // or parameter is an arrow function parameter
                            // (a) => {}
                            AnyJsBindingPattern::AnyJsBinding(
                                AnyJsBinding::JsIdentifierBinding(_),
                            ) => {
                                if should_hug_formal_parameter {
                                    true
                                } else {
                                    parameter.type_annotation().is_some_and(|type_annotation| {
                                        matches!(
                                            type_annotation.ty(),
                                            Ok(AnyTsType::TsObjectType(_))
                                        )
                                    })
                                }
                            }
                            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsBogusBinding(_)) => {
                                return Err(FormatError::SyntaxError);
                            }
                        }
                    }

                    Some(initializer) => {
                        // only for `[a] = []`, `{a} = {}`
                        let object_or_array_binding = matches!(
                            parameter.binding()?,
                            AnyJsBindingPattern::JsArrayBindingPattern(_)
                                | AnyJsBindingPattern::JsObjectBindingPattern(_)
                        );
                        let should_hug_right = match initializer.expression()? {
                            AnyJsExpression::JsObjectExpression(object) => {
                                object.members().is_empty()
                            }
                            AnyJsExpression::JsArrayExpression(array) => {
                                array.elements().is_empty()
                            }
                            AnyJsExpression::JsIdentifierExpression(_) => true,
                            _ => false,
                        };

                        object_or_array_binding && should_hug_right
                    }
                }
            }
            AnyJsFormalParameter::JsBogusParameter(_) | AnyJsFormalParameter::JsMetavariable(_) => {
                return Err(FormatError::SyntaxError)
            }
        };

        Ok(result)
    }

    let list = parameters.list();
    if list.len() != 1 {
        return Ok(false);
    }

    // SAFETY: Safe because of the length check above
    let only_parameter = list.first().unwrap()?;

    if comments.has_comments(only_parameter.syntax()) {
        return Ok(false);
    }

    let has_parentheses = parameters.l_paren_token().is_ok() && parameters.r_paren_token().is_ok();
    let from_arrow_function = parameters.as_arrow_function_expression().is_some();
    let should_hug_formal_parameter =
        has_parentheses && from_arrow_function && parentheses_not_needed;

    let result = match only_parameter {
        AnyParameter::AnyJsParameter(parameter) => match parameter {
            AnyJsParameter::AnyJsFormalParameter(formal_parameter) => {
                hug_formal_parameter(&formal_parameter, should_hug_formal_parameter)?
            }
            AnyJsParameter::JsRestParameter(_) => false,
            AnyJsParameter::TsThisParameter(this) => {
                this.type_annotation().is_some_and(|type_annotation| {
                    matches!(type_annotation.ty(), Ok(AnyTsType::TsObjectType(_)))
                })
            }
        },
        AnyParameter::AnyJsConstructorParameter(constructor_parameter) => {
            match constructor_parameter {
                AnyJsConstructorParameter::AnyJsFormalParameter(formal_parameter) => {
                    hug_formal_parameter(&formal_parameter, should_hug_formal_parameter)?
                }
                AnyJsConstructorParameter::JsRestParameter(_)
                | AnyJsConstructorParameter::TsPropertyParameter(_) => false,
            }
        }
    };

    Ok(result)
}

/// Tests if all of the parameters of `expression` are simple enough to allow
/// a function to group.
pub(crate) fn has_only_simple_parameters(
    parameters: &JsParameters,
    allow_type_annotations: bool,
) -> bool {
    parameters
        .items()
        .into_iter()
        .flatten()
        .all(|parameter| is_simple_parameter(&parameter, allow_type_annotations))
}

/// Tests if the single parameter is "simple", as in a plain identifier with no
/// explicit type annotation and no initializer:
///
/// Examples:
/// foo             => true
/// foo?            => true
/// foo = 'bar'     => false
/// foo: string     => false
/// {a, b}          => false
/// {a, b} = {}     => false
/// [a, b]          => false
///
pub(crate) fn is_simple_parameter(
    parameter: &AnyJsParameter,
    allow_type_annotations: bool,
) -> bool {
    match parameter {
        AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(param)) => {
            matches!(
                param.binding(),
                Ok(AnyJsBindingPattern::AnyJsBinding(
                    AnyJsBinding::JsIdentifierBinding(_)
                ))
            ) && (allow_type_annotations || param.type_annotation().is_none())
                && param.initializer().is_none()
        }
        _ => false,
    }
}
