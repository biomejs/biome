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
use biome_rowan::{SyntaxResult, declare_node_union};

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
            Self::JsMethodClassMember(member) => member.async_token(),
            Self::JsMethodObjectMember(member) => member.async_token(),
            Self::JsConstructorClassMember(_) => None,
            Self::TsMethodSignatureClassMember(signature) => signature.async_token(),
            Self::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsMethodClassMember(member) => member.star_token(),
            Self::JsMethodObjectMember(member) => member.star_token(),
            Self::JsConstructorClassMember(_) => None,
            Self::TsMethodSignatureClassMember(_) => None,
            Self::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn name(&self) -> SyntaxResult<AnyJsMemberName> {
        Ok(match self {
            Self::JsMethodClassMember(member) => member.name()?.into(),
            Self::JsMethodObjectMember(member) => member.name()?.into(),
            Self::JsConstructorClassMember(member) => {
                AnyJsMemberName::from(AnyJsClassMemberName::from(member.name()?))
            }
            Self::TsMethodSignatureClassMember(signature) => signature.name()?.into(),
            Self::TsMethodSignatureTypeMember(member) => member.name()?.into(),
        })
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            Self::JsMethodClassMember(member) => member.type_parameters(),
            Self::JsMethodObjectMember(member) => member.type_parameters(),
            Self::JsConstructorClassMember(_) => None,
            Self::TsMethodSignatureClassMember(signature) => signature.type_parameters(),
            Self::TsMethodSignatureTypeMember(member) => member.type_parameters(),
        }
    }

    fn parameters(&self) -> SyntaxResult<MethodParameters> {
        Ok(match self {
            Self::JsMethodClassMember(member) => member.parameters()?.into(),
            Self::JsMethodObjectMember(member) => member.parameters()?.into(),
            Self::JsConstructorClassMember(member) => member.parameters()?.into(),
            Self::TsMethodSignatureClassMember(signature) => signature.parameters()?.into(),
            Self::TsMethodSignatureTypeMember(member) => member.parameters()?.into(),
        })
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            Self::JsMethodClassMember(member) => member.return_type_annotation(),
            Self::JsMethodObjectMember(member) => member.return_type_annotation(),
            Self::JsConstructorClassMember(_) => None,
            Self::TsMethodSignatureClassMember(signature) => signature.return_type_annotation(),
            Self::TsMethodSignatureTypeMember(member) => member.return_type_annotation(),
        }
    }

    fn question_mark_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsMethodClassMember(member) => member.question_mark_token(),
            Self::JsMethodObjectMember(_) => None,
            Self::JsConstructorClassMember(_) => None,
            Self::TsMethodSignatureClassMember(signature) => signature.question_mark_token(),
            Self::TsMethodSignatureTypeMember(member) => member.optional_token(),
        }
    }

    fn body(&self) -> SyntaxResult<Option<JsFunctionBody>> {
        Ok(match self {
            Self::JsMethodClassMember(member) => Some(member.body()?),
            Self::JsMethodObjectMember(member) => Some(member.body()?),
            Self::JsConstructorClassMember(member) => Some(member.body()?),
            Self::TsMethodSignatureClassMember(_) => None,
            Self::TsMethodSignatureTypeMember(_) => None,
        })
    }
}

declare_node_union! {
    MethodParameters = JsParameters | JsConstructorParameters
}

impl MethodParameters {
    pub fn len(&self) -> usize {
        match self {
            Self::JsParameters(parameters) => parameters.items().len(),
            Self::JsConstructorParameters(parameters) => parameters.parameters().len(),
        }
    }
}

impl Format<JsFormatContext> for MethodParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            Self::JsParameters(parameters) => parameters.format().fmt(f),
            Self::JsConstructorParameters(parameters) => parameters.format().fmt(f),
        }
    }
}
