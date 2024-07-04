use crate::prelude::*;

use crate::js::declarations::function_declaration::should_group_function_parameters;
use crate::utils::object::AnyJsMemberName;
use biome_formatter::write;
use biome_js_syntax::{
    AnyJsClassMemberName, JsConstructorClassMember, JsConstructorParameters, JsFunctionBody,
    JsParameters, TsMethodSignatureClassMember, TsMethodSignatureTypeMember,
    TsReturnTypeAnnotation, TsTypeParameters,
};
use biome_js_syntax::{JsMethodClassMember, JsMethodObjectMember, JsSyntaxToken};
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodClassMember;

impl FormatNodeRule<JsMethodClassMember> for FormatJsMethodClassMember {
    fn fmt_fields(&self, node: &JsMethodClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [
                node.modifiers().format(),
                space(),
                FormatAnyJsMethodMember::from(node.clone())
            ]
        ]
    }
}

declare_node_union! {
    /// Formats the type parameters, parameters, and return type annotation of a method
    pub(crate) FormatAnyJsMethodMember =
        JsMethodClassMember |
        JsMethodObjectMember |
        JsConstructorClassMember |
        TsMethodSignatureClassMember |
        TsMethodSignatureTypeMember
}

impl Format<JsFormatContext> for FormatAnyJsMethodMember {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if let Some(async_token) = self.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        let type_parameters = self.type_parameters();

        write!(
            f,
            [
                self.star_token().format(),
                self.name(),
                self.question_mark_token().format(),
                type_parameters.format(),
            ]
        )?;

        write!(
            f,
            [group(&format_with(|f| {
                let parameters = self.parameters()?;
                let return_type_annotation = self.return_type_annotation();
                let mut format_return_type_annotation = return_type_annotation.format().memoized();

                if should_group_function_parameters(
                    type_parameters.as_ref(),
                    parameters.len(),
                    return_type_annotation
                        .as_ref()
                        .map(|annotation| annotation.ty()),
                    &mut format_return_type_annotation,
                    f,
                )? {
                    write!(f, [group(&parameters)])?;
                } else {
                    write!(f, [parameters])?;
                }

                write!(f, [format_return_type_annotation])
            }))]
        )?;

        if let Some(body) = self.body()? {
            write!(f, [space(), body.format()])?;
        }

        Ok(())
    }
}

impl FormatAnyJsMethodMember {
    fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.async_token(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => member.async_token(),
            FormatAnyJsMethodMember::JsConstructorClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.async_token()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.star_token(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => member.star_token(),
            FormatAnyJsMethodMember::JsConstructorClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn name(&self) -> SyntaxResult<AnyJsMemberName> {
        Ok(match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.name()?.into(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => member.name()?.into(),
            FormatAnyJsMethodMember::JsConstructorClassMember(member) => {
                AnyJsMemberName::from(AnyJsClassMemberName::from(member.name()?))
            }
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.name()?.into()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(member) => member.name()?.into(),
        })
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.type_parameters(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => member.type_parameters(),
            FormatAnyJsMethodMember::JsConstructorClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.type_parameters()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(member) => {
                member.type_parameters()
            }
        }
    }

    fn parameters(&self) -> SyntaxResult<MethodParameters> {
        Ok(match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.parameters()?.into(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => member.parameters()?.into(),
            FormatAnyJsMethodMember::JsConstructorClassMember(member) => {
                member.parameters()?.into()
            }
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.parameters()?.into()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(member) => {
                member.parameters()?.into()
            }
        })
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.return_type_annotation(),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => {
                member.return_type_annotation()
            }
            FormatAnyJsMethodMember::JsConstructorClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.return_type_annotation()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(member) => {
                member.return_type_annotation()
            }
        }
    }

    fn question_mark_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => member.question_mark_token(),
            FormatAnyJsMethodMember::JsMethodObjectMember(_) => None,
            FormatAnyJsMethodMember::JsConstructorClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.question_mark_token()
            }
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(member) => member.optional_token(),
        }
    }

    fn body(&self) -> SyntaxResult<Option<JsFunctionBody>> {
        Ok(match self {
            FormatAnyJsMethodMember::JsMethodClassMember(member) => Some(member.body()?),
            FormatAnyJsMethodMember::JsMethodObjectMember(member) => Some(member.body()?),
            FormatAnyJsMethodMember::JsConstructorClassMember(member) => Some(member.body()?),
            FormatAnyJsMethodMember::TsMethodSignatureClassMember(_) => None,
            FormatAnyJsMethodMember::TsMethodSignatureTypeMember(_) => None,
        })
    }
}

declare_node_union! {
    MethodParameters = JsParameters | JsConstructorParameters
}

impl MethodParameters {
    pub fn len(&self) -> usize {
        match self {
            MethodParameters::JsParameters(parameters) => parameters.items().len(),
            MethodParameters::JsConstructorParameters(parameters) => parameters.parameters().len(),
        }
    }
}

impl Format<JsFormatContext> for MethodParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            MethodParameters::JsParameters(parameters) => parameters.format().fmt(f),
            MethodParameters::JsConstructorParameters(parameters) => parameters.format().fmt(f),
        }
    }
}
