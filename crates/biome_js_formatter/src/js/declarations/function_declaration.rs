use crate::prelude::*;

use crate::js::expressions::call_arguments::GroupedCallArgumentLayout;
use crate::utils::function_body::{FormatMaybeCachedFunctionBody, FunctionBodyCacheMode};
use biome_formatter::{RemoveSoftLinesBuffer, write};
use biome_js_syntax::{
    AnyJsBinding, AnyTsReturnType, AnyTsType, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsParameters, JsSyntaxToken,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration,
    TsReturnTypeAnnotation, TsTypeParameters,
};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFunctionDeclaration;

impl FormatNodeRule<JsFunctionDeclaration> for FormatJsFunctionDeclaration {
    fn fmt_fields(&self, node: &JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [FormatFunction::from(node.clone())]]
    }
}

declare_node_union! {
    pub(crate) FormatFunction =
        JsFunctionDeclaration |
        JsFunctionExpression |
        JsFunctionExportDefaultDeclaration |
        TsDeclareFunctionDeclaration |
        TsDeclareFunctionExportDefaultDeclaration
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct FormatFunctionOptions {
    pub call_argument_layout: Option<GroupedCallArgumentLayout>,
    pub body_cache_mode: FunctionBodyCacheMode,
}

impl FormatFunction {
    fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.async_token(),
            Self::JsFunctionExpression(expression) => expression.async_token(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.async_token(),
            Self::TsDeclareFunctionDeclaration(member) => member.async_token(),
            Self::TsDeclareFunctionExportDefaultDeclaration(member) => member.async_token(),
        }
    }

    fn function_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.function_token(),
            Self::JsFunctionExpression(expression) => expression.function_token(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.function_token(),
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.function_token(),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token()
            }
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.star_token(),
            Self::JsFunctionExpression(expression) => expression.star_token(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.star_token(),
            Self::TsDeclareFunctionDeclaration(_) => None,
            Self::TsDeclareFunctionExportDefaultDeclaration(_) => None,
        }
    }

    fn id(&self) -> SyntaxResult<Option<AnyJsBinding>> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.id().map(Some),
            Self::JsFunctionExpression(expression) => Ok(expression.id()),
            Self::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.id().map(Some),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            Self::JsFunctionExpression(expression) => expression.type_parameters(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.type_parameters(),
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.type_parameters(),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                declaration.type_parameters()
            }
        }
    }

    fn parameters(&self) -> SyntaxResult<JsParameters> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.parameters(),
            Self::JsFunctionExpression(expression) => expression.parameters(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => declaration.parameters(),
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.parameters(),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                declaration.parameters()
            }
        }
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            Self::JsFunctionDeclaration(declaration) => declaration.return_type_annotation(),
            Self::JsFunctionExpression(expression) => expression.return_type_annotation(),
            Self::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            Self::TsDeclareFunctionDeclaration(declaration) => declaration.return_type_annotation(),
            Self::TsDeclareFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
        }
    }

    fn body(&self) -> SyntaxResult<Option<JsFunctionBody>> {
        Ok(match self {
            Self::JsFunctionDeclaration(declaration) => Some(declaration.body()?),
            Self::JsFunctionExpression(expression) => Some(expression.body()?),
            Self::JsFunctionExportDefaultDeclaration(declaration) => Some(declaration.body()?),
            Self::TsDeclareFunctionDeclaration(_) => None,
            Self::TsDeclareFunctionExportDefaultDeclaration(_) => None,
        })
    }

    /// Formats the function with the specified `options`.
    ///
    /// # Errors
    ///
    /// Returns [`FormatError::PoorLayout`] if [`call_argument_layout`](FormatFunctionOptions::call_argument_layout] is `Some`
    /// and the function parameters contain some content that [*force a group to break*](FormatElements::will_break).
    ///
    /// This error is handled by [FormatJsCallArguments].
    pub(crate) fn fmt_with_options(
        &self,
        f: &mut JsFormatter,
        options: &FormatFunctionOptions,
    ) -> FormatResult<()> {
        if let Some(async_token) = self.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        write!(
            f,
            [self.function_token().format(), self.star_token().format()]
        )?;

        match self.id()? {
            Some(id) => {
                write!(f, [space(), id.format()])?;
            }
            None => {
                write!(f, [space()])?;
            }
        }

        let type_parameters = self.type_parameters();
        let parameters = self.parameters()?;
        let return_type_annotation = self.return_type_annotation();

        write!(f, [type_parameters.format()])?;

        let format_parameters = format_with(|f: &mut JsFormatter| {
            if options.call_argument_layout.is_some() {
                let mut buffer = RemoveSoftLinesBuffer::new(f);

                let mut recording = buffer.start_recording();
                write!(recording, [parameters.format()])?;
                let recorded = recording.stop();

                if recorded.will_break() {
                    return Err(FormatError::PoorLayout);
                }
            } else {
                parameters.format().fmt(f)?;
            }

            Ok(())
        });

        write!(
            f,
            [group(&format_with(|f| {
                let mut format_return_type_annotation = return_type_annotation.format().memoized();
                let group_parameters = should_group_function_parameters(
                    type_parameters.as_ref(),
                    parameters.items().len(),
                    return_type_annotation
                        .as_ref()
                        .map(|annotation| annotation.ty()),
                    &mut format_return_type_annotation,
                    f,
                )?;

                if group_parameters {
                    write!(f, [group(&format_parameters)])?;
                } else {
                    write!(f, [format_parameters])?;
                }

                write!(f, [format_return_type_annotation])
            }))]
        )?;

        if let Some(body) = self.body()? {
            write!(
                f,
                [
                    space(),
                    FormatMaybeCachedFunctionBody {
                        body: &body.into(),
                        mode: options.body_cache_mode
                    }
                ]
            )?;
        }

        Ok(())
    }
}

impl Format<JsFormatContext> for FormatFunction {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        self.fmt_with_options(f, &FormatFunctionOptions::default())?;
        Ok(())
    }
}

/// Returns `true` if the function parameters should be grouped.
/// Grouping the parameters has the effect that the return type will break first.
pub(crate) fn should_group_function_parameters(
    type_parameters: Option<&TsTypeParameters>,
    parameter_count: usize,
    return_type: Option<SyntaxResult<AnyTsReturnType>>,
    formatted_return_type: &mut Memoized<impl Format<JsFormatContext>, JsFormatContext>,
    f: &mut JsFormatter,
) -> FormatResult<bool> {
    let return_type = match return_type {
        Some(return_type) => return_type?,
        None => return Ok(false),
    };

    if let Some(type_parameters) = type_parameters {
        match type_parameters.items().len() {
            0 => {} // fall through
            1 => {
                // SAFETY: Safe because the length is 1
                let first = type_parameters.items().iter().next().unwrap()?;

                if first.constraint().is_some() || first.default().is_some() {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }
    }

    let result = if parameter_count != 1 {
        false
    } else {
        matches!(
            return_type,
            AnyTsReturnType::AnyTsType(AnyTsType::TsObjectType(_) | AnyTsType::TsMappedType(_))
        ) || formatted_return_type.inspect(f)?.will_break()
    };

    Ok(result)
}
