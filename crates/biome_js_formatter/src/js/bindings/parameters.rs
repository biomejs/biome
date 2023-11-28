use crate::prelude::*;
use biome_formatter::{write, CstFormatContext, FormatRuleWithOptions};

use crate::js::expressions::arrow_function_expression::can_avoid_parentheses;
use crate::js::lists::parameter_list::FormatJsAnyParameterList;
use crate::utils::test_call::is_test_call_argument;
use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsExpression,
    AnyJsFormalParameter, AnyJsParameter, AnyTsType, JsArrowFunctionExpression,
    JsConstructorParameters, JsParameters, JsSyntaxNode, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsParameters {
    options: FormatJsParametersOptions,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatJsParametersOptions {
    /// Whether the parameters should include soft line breaks to allow them to
    /// break naturally over multiple lines when they can't fit on one line.
    ///
    /// This is particularly important for arrow chains passed as arguments in
    /// call expressions, where it must be set to false to avoid having the
    /// parameters break onto lines before the entire expression expands.
    ///
    /// When `true`, parameters will _not_ include any soft line break points.
    pub prevent_soft_line_breaks: bool,
}

impl FormatRuleWithOptions<JsParameters> for FormatJsParameters {
    type Options = FormatJsParametersOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<JsParameters> for FormatJsParameters {
    fn fmt_fields(&self, node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatAnyJsParameters::new(AnyJsParameters::JsParameters(node.clone()), self.options).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &JsParameters, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `FormatJsAnyParameters
        Ok(())
    }
}

declare_node_union! {
    pub(crate) AnyJsParameters = JsParameters | JsConstructorParameters
}

pub(crate) struct FormatAnyJsParameters {
    pub(crate) parameters: AnyJsParameters,
    pub(crate) options: FormatJsParametersOptions,
}

impl FormatAnyJsParameters {
    pub(crate) fn new(parameters: AnyJsParameters, options: FormatJsParametersOptions) -> Self {
        Self {
            parameters,
            options,
        }
    }
}

impl Format<JsFormatContext> for FormatAnyJsParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let list = self.list();

        let has_any_decorated_parameter = list.has_any_decorated_parameter();

        let can_hug = should_hug_function_parameters(self, f.context().comments())?
            && !has_any_decorated_parameter;

        let layout = if list.is_empty() {
            ParameterLayout::NoParameters
        } else if can_hug || self.is_in_test_call()? {
            ParameterLayout::Hug
        } else if self.options.prevent_soft_line_breaks {
            ParameterLayout::Compact
        } else {
            ParameterLayout::Default
        };

        let l_paren_token = self.l_paren_token()?;
        let r_paren_token = self.r_paren_token()?;

        let parentheses_not_needed = self
            .as_arrow_function_expression()
            .map_or(false, |expression| can_avoid_parentheses(&expression, f));

        match layout {
            ParameterLayout::NoParameters => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        format_dangling_comments(self.parameters_syntax()).with_soft_block_indent(),
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
            ParameterLayout::Compact => {
                if !parentheses_not_needed {
                    write!(f, [l_paren_token.format()])?;
                } else {
                    write!(f, [format_removed(&l_paren_token)])?;
                }

                write!(
                    f,
                    [FormatJsAnyParameterList::with_layout(
                        &list,
                        ParameterLayout::Compact
                    )]
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
        match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => parameters.l_paren_token(),
            AnyJsParameters::JsConstructorParameters(parameters) => parameters.l_paren_token(),
        }
    }

    fn list(&self) -> AnyJsParameterList {
        match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => {
                AnyJsParameterList::from(parameters.items())
            }
            AnyJsParameters::JsConstructorParameters(parameters) => {
                AnyJsParameterList::from(parameters.parameters())
            }
        }
    }

    fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => parameters.r_paren_token(),
            AnyJsParameters::JsConstructorParameters(parameters) => parameters.r_paren_token(),
        }
    }

    /// Returns `true` for function parameters if the function is an argument of a [test `CallExpression`](is_test_call_expression).
    fn is_in_test_call(&self) -> SyntaxResult<bool> {
        let result = match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => match parameters.syntax().parent() {
                Some(function) => is_test_call_argument(&function)?,
                None => false,
            },
            AnyJsParameters::JsConstructorParameters(_) => false,
        };

        Ok(result)
    }

    fn as_arrow_function_expression(&self) -> Option<JsArrowFunctionExpression> {
        match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => parameters
                .syntax()
                .parent()
                .and_then(JsArrowFunctionExpression::cast),
            AnyJsParameters::JsConstructorParameters(_) => None,
        }
    }

    fn parameters_syntax(&self) -> &JsSyntaxNode {
        match &self.parameters {
            AnyJsParameters::JsParameters(parameters) => parameters.syntax(),
            AnyJsParameters::JsConstructorParameters(parameters) => parameters.syntax(),
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

    /// Compact layout forces all parameters to try to render on the same line,
    /// with no breaks added around the brackets. This should likely only be
    /// used in a `best_fitting!` context where one variant attempts to fit the
    /// parameters on a single line, and a default expanded version that is
    /// used in case that does not fit.
    ///
    /// ```javascript
    /// function test(firstParameter, secondParameter, thirdParameter, evenOverlyLong) {}
    /// ```
    Compact,
}

pub(crate) fn should_hug_function_parameters(
    parameters: &FormatAnyJsParameters,
    comments: &JsComments,
) -> FormatResult<bool> {
    /// Returns true if the first parameter should be forced onto the same line as the `(` and `)` parentheses.
    /// See the `[ParameterLayout::Hug] documentation.
    fn hug_formal_parameter(parameter: &self::AnyJsFormalParameter) -> FormatResult<bool> {
        let result = match parameter {
            AnyJsFormalParameter::JsFormalParameter(parameter) => {
                match parameter.initializer() {
                    None => {
                        match parameter.binding()? {
                            // always true for `[a]` or `{a}`
                            AnyJsBindingPattern::JsArrayBindingPattern(_)
                            | AnyJsBindingPattern::JsObjectBindingPattern(_) => true,
                            // only if the type parameter is an object type
                            // `a: { prop: string }`
                            AnyJsBindingPattern::AnyJsBinding(
                                AnyJsBinding::JsIdentifierBinding(_),
                            ) => parameter
                                .type_annotation()
                                .map_or(false, |type_annotation| {
                                    matches!(type_annotation.ty(), Ok(AnyTsType::TsObjectType(_)))
                                }),
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
            AnyJsFormalParameter::JsBogusParameter(_) => return Err(FormatError::SyntaxError),
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

    let result = match only_parameter {
        AnyParameter::AnyJsParameter(parameter) => match parameter {
            AnyJsParameter::AnyJsFormalParameter(formal_parameter) => {
                hug_formal_parameter(&formal_parameter)?
            }
            AnyJsParameter::JsRestParameter(_) => false,
            AnyJsParameter::TsThisParameter(this) => {
                this.type_annotation().map_or(false, |type_annotation| {
                    matches!(type_annotation.ty(), Ok(AnyTsType::TsObjectType(_)))
                })
            }
        },
        AnyParameter::AnyJsConstructorParameter(constructor_parameter) => {
            match constructor_parameter {
                AnyJsConstructorParameter::AnyJsFormalParameter(formal_parameter) => {
                    hug_formal_parameter(&formal_parameter)?
                }
                AnyJsConstructorParameter::JsRestParameter(_)
                | AnyJsConstructorParameter::TsPropertyParameter(_) => false,
            }
        }
    };

    Ok(result)
}
