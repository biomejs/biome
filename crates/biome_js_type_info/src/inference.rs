use std::str::FromStr;

use biome_js_syntax::{
    AnyJsClassMember, AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsFormalParameter,
    AnyJsParameter, AnyTsName, AnyTsReturnType, AnyTsTupleTypeElement, AnyTsType, AnyTsTypeMember,
    AnyTsTypePredicateParameterName, JsLanguage, JsParameters, TsReferenceType, TsTypeArguments,
    TsTypeParameter, TsTypeParameters, TsTypeofType,
};
use biome_rowan::{SyntaxResult, SyntaxToken, Text, TokenText};

use crate::{
    AssertsReturnType, CallSignatureTypeMember, Class, ClassMember, Constructor,
    ConstructorTypeMember, Function, FunctionParameter, GenericTypeParameter, Intersection,
    Literal, MethodTypeMember, Object, PredicateReturnType, PropertyTypeMember, ReturnType, Tuple,
    TupleElementType, Type, TypeMember, TypeOperator, TypeOperatorType, TypeReference,
    TypeReferenceQualifier, Union,
};

impl Type {
    pub fn from_any_js_declaration_clause(decl: &AnyJsDeclarationClause) -> Self {
        match decl {
            AnyJsDeclarationClause::JsClassDeclaration(decl) => Self::Class(Box::new(Class {
                name: decl
                    .id()
                    .ok()
                    .as_ref()
                    .and_then(|id| id.as_js_identifier_binding())
                    .and_then(|id| id.name_token().ok())
                    .map(|token| token.token_text_trimmed().into()),
                members: decl
                    .members()
                    .into_iter()
                    .filter_map(|member| ClassMember::from_any_js_class_member(&member))
                    .collect(),
            })),
            AnyJsDeclarationClause::JsFunctionDeclaration(decl) => {
                Self::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: decl
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    name: decl
                        .id()
                        .ok()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: decl
                        .parameters()
                        .ok()
                        .map(|params| FunctionParameter::params_from_js_parameters(&params))
                        .unwrap_or_default(),
                    return_type: decl
                        .return_type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                        .unwrap_or_default(),
                }))
            }
            AnyJsDeclarationClause::JsVariableDeclarationClause(_) => {
                // Variable declarations don't have a type;
                // only their inner declarators have.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsDeclareFunctionDeclaration(_decl) => {
                // TODO: Handle module declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsEnumDeclaration(_decl) => {
                // TODO: Handle enum declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsExternalModuleDeclaration(_decl) => {
                // TODO: Handle external module declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsGlobalDeclaration(_decl) => {
                // TODO: Handle global declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsImportEqualsDeclaration(_decl) => {
                // TODO: Handle `import T = Name` syntax.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsModuleDeclaration(_decl) => {
                // TODO: Handle module declarations.
                Self::Unknown
            }
            AnyJsDeclarationClause::TsTypeAliasDeclaration(decl) => decl
                .ty()
                .map(|ty| Self::Alias(Box::new(Self::from_any_ts_type(&ty))))
                .unwrap_or_default(),
        }
    }

    pub fn from_any_js_export_default_declaration(decl: &AnyJsExportDefaultDeclaration) -> Self {
        match decl {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(decl) => {
                Self::Class(Box::new(Class {
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    members: decl
                        .members()
                        .into_iter()
                        .filter_map(|member| ClassMember::from_any_js_class_member(&member))
                        .collect(),
                }))
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(decl) => {
                Self::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: decl
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: decl
                        .parameters()
                        .ok()
                        .map(|params| FunctionParameter::params_from_js_parameters(&params))
                        .unwrap_or_default(),
                    return_type: decl
                        .return_type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                        .unwrap_or_default(),
                }))
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                Self::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: decl
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: decl
                        .parameters()
                        .ok()
                        .map(|params| FunctionParameter::params_from_js_parameters(&params))
                        .unwrap_or_default(),
                    return_type: decl
                        .return_type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                        .unwrap_or_default(),
                }))
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::Unknown
            }
        }
    }

    pub fn from_any_ts_type(ty: &AnyTsType) -> Self {
        match ty {
            AnyTsType::JsMetavariable(_) => Self::Unknown,
            AnyTsType::TsAnyType(_) => Self::AnyKeyword,
            AnyTsType::TsArrayType(ty) => Self::Array(Box::new(
                ty.element_type()
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .unwrap_or_default(),
            )),
            AnyTsType::TsBigintLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    Text::Owned(format!("{minus_token}{literal_token}")),
                ))),
                (None, Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => Self::Unknown,
            },
            AnyTsType::TsBigintType(_) => Self::BigInt,
            AnyTsType::TsBogusType(_) => Self::Unknown,
            AnyTsType::TsBooleanLiteralType(ty) => match ty.literal() {
                Ok(token) => Self::Literal(Box::new(Literal::Boolean(
                    token.token_text_trimmed().into(),
                ))),
                Err(_) => Self::Unknown,
            },
            AnyTsType::TsBooleanType(_) => Self::Boolean,
            AnyTsType::TsConditionalType(_) => {
                // TODO: Handle conditional types (`T extends U ? V : W`).
                Self::Unknown
            }
            AnyTsType::TsConstructorType(ty) => Self::Constructor(Box::new(Constructor {
                type_parameters: ty
                    .type_parameters()
                    .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                    .unwrap_or_default(),
                parameters: ty
                    .parameters()
                    .map(|params| FunctionParameter::params_from_js_parameters(&params))
                    .unwrap_or_default(),
                return_type: ty.return_type().ok().map(|ty| Type::from_any_ts_type(&ty)),
            })),
            AnyTsType::TsFunctionType(ty) => Self::Function(Box::new(Function {
                is_async: false,
                type_parameters: ty
                    .type_parameters()
                    .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                    .unwrap_or_default(),
                name: None,
                parameters: ty
                    .parameters()
                    .map(|params| FunctionParameter::params_from_js_parameters(&params))
                    .unwrap_or_default(),
                return_type: ty
                    .return_type()
                    .ok()
                    .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                    .unwrap_or_default(),
            })),
            AnyTsType::TsImportType(_) => {
                // TODO: Handle import types (`import("./module").T`).
                Self::Unknown
            }
            AnyTsType::TsIndexedAccessType(_) => {
                // TODO: Handle type indexing (`T[U]`).
                Self::Unknown
            }
            AnyTsType::TsInferType(_) => {
                // TODO: Handle `infer T` syntax.
                Self::Unknown
            }
            AnyTsType::TsIntersectionType(ty) => Self::Intersection(Box::new(Intersection(
                ty.types()
                    .into_iter()
                    .filter_map(|ty| ty.ok())
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .collect(),
            ))),
            AnyTsType::TsMappedType(_) => {
                // TODO: Handle mapped types (`type T<U> = { [K in keyof U]: V }`).
                Self::Unknown
            }
            AnyTsType::TsNeverType(_) => Self::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => Self::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => Self::Literal(Box::new(Literal::Null)),
            AnyTsType::TsNumberLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => Self::Literal(Box::new(Literal::Number(
                    Text::Owned(format!("{minus_token}{literal_token}")),
                ))),
                (None, Ok(literal_token)) => Self::Literal(Box::new(Literal::Number(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => Self::Unknown,
            },
            AnyTsType::TsNumberType(_) => Self::Number,
            AnyTsType::TsObjectType(ty) => Self::Object(Box::new(Object(
                ty.members()
                    .into_iter()
                    .filter_map(|member| TypeMember::from_any_ts_type_member(&member))
                    .collect(),
            ))),
            AnyTsType::TsParenthesizedType(ty) => ty
                .ty()
                .map(|ty| Self::from_any_ts_type(&ty))
                .unwrap_or_default(),
            AnyTsType::TsReferenceType(ty) => Self::from_ts_reference_type(ty),
            AnyTsType::TsStringLiteralType(ty) => match ty.literal_token() {
                Ok(token) => {
                    Self::Literal(Box::new(Literal::String(token.token_text_trimmed().into())))
                }
                Err(_) => Self::Unknown,
            },
            AnyTsType::TsStringType(_) => Self::String,
            AnyTsType::TsSymbolType(_) => Self::Symbol,
            AnyTsType::TsTemplateLiteralType(ty) => {
                Self::Literal(Box::new(Literal::Template(Text::Owned(ty.to_string()))))
            }
            AnyTsType::TsThisType(_) => Self::ThisKeyword,
            AnyTsType::TsTupleType(ty) => {
                let elements: SyntaxResult<Box<_>> = ty
                    .elements()
                    .into_iter()
                    .map(|el| el.map(|el| TupleElementType::from_any_ts_tuple_type_element(&el)))
                    .collect();
                match elements {
                    Ok(elements) => Self::Tuple(Box::new(Tuple(elements))),
                    Err(_) => Self::Unknown,
                }
            }
            AnyTsType::TsTypeOperatorType(ty) => match (ty.operator_token(), ty.ty()) {
                (Ok(operator_token), Ok(ty)) => TypeOperator::from_str(
                    operator_token.text_trimmed(),
                )
                .map_or(Self::Unknown, |operator| {
                    Self::TypeOperator(Box::new(TypeOperatorType {
                        operator,
                        ty: Self::from_any_ts_type(&ty),
                    }))
                }),
                _ => Self::Unknown,
            },
            AnyTsType::TsTypeofType(ty) => Self::from_ts_typeof_type(ty),
            AnyTsType::TsUndefinedType(_) => Self::Undefined,
            AnyTsType::TsUnionType(ty) => Self::Union(Box::new(Union(
                ty.types()
                    .into_iter()
                    .filter_map(|ty| ty.ok())
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .collect(),
            ))),
            AnyTsType::TsUnknownType(_) => Self::UnknownKeyword,
            AnyTsType::TsVoidType(_) => Self::VoidKeyword,
        }
    }

    pub fn from_ts_reference_type(ty: &TsReferenceType) -> Self {
        ty.name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                Self::Reference(Box::new(TypeReference {
                    qualifier,
                    type_parameters: Self::types_from_ts_type_arguments(ty.type_arguments()),
                }))
            })
            .unwrap_or_default()
    }

    pub fn from_ts_typeof_type(ty: &TsTypeofType) -> Self {
        ty.expression_name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                Self::TypeofType(Box::new(TypeReference {
                    qualifier,
                    type_parameters: Self::types_from_ts_type_arguments(ty.type_arguments()),
                }))
            })
            .unwrap_or_default()
    }

    pub fn types_from_ts_type_arguments(arguments: Option<TsTypeArguments>) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.ts_type_argument_list()
                    .into_iter()
                    .filter_map(|arg| arg.ok().map(|ty| Self::from_any_ts_type(&ty)))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl ClassMember {
    pub fn from_any_js_class_member(member: &AnyJsClassMember) -> Option<Self> {
        match member {
            AnyJsClassMember::JsMethodClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Method(MethodTypeMember {
                        type_parameters: member
                            .type_parameters()
                            .map(|params| {
                                GenericTypeParameter::params_from_ts_type_parameters(&params)
                            })
                            .unwrap_or_default(),
                        name: TokenText::from(name).into(),
                        parameters: member
                            .parameters()
                            .map(|params| FunctionParameter::params_from_js_parameters(&params))
                            .unwrap_or_default(),
                        return_type: member
                            .return_type_annotation()
                            .and_then(|annotation| annotation.ty().ok())
                            .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                            .unwrap_or_default(),
                        is_optional: false,
                    })
                })
            }
            AnyJsClassMember::JsPropertyClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: TokenText::from(name).into(),
                        ty: member
                            .property_annotation()
                            .and_then(|annotation| annotation.type_annotation().ok())
                            .flatten()
                            .and_then(|annotation| annotation.ty().ok())
                            .map(|ty| Type::from_any_ts_type(&ty))
                            .unwrap_or_default(),
                        is_optional: member
                            .property_annotation()
                            .as_ref()
                            .and_then(|annotation| annotation.as_ts_optional_property_annotation())
                            .is_some(),
                    })
                })
            }
            _ => {
                // TODO: Handle more variants
                None
            }
        }
    }
}

impl FunctionParameter {
    pub fn from_any_js_parameter(param: &AnyJsParameter) -> Self {
        match param {
            AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                param,
            )) => Self {
                name: param
                    .binding()
                    .ok()
                    .as_ref()
                    .and_then(|binding| binding.as_any_js_binding())
                    .and_then(|binding| binding.as_js_identifier_binding())
                    .and_then(|identifier| identifier.name_token().ok())
                    .map(|token| token.token_text_trimmed().into()),
                ty: param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                is_optional: param.question_mark_token().is_some(),
                is_rest: false,
            },
            AnyJsParameter::AnyJsFormalParameter(_) => Self {
                name: None,
                ty: Type::Unknown,
                is_optional: false,
                is_rest: false,
            },
            AnyJsParameter::JsRestParameter(param) => Self {
                name: None,
                ty: param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                is_optional: false,
                is_rest: true,
            },
            AnyJsParameter::TsThisParameter(param) => Self {
                name: Some(Text::Static("this")),
                ty: param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                is_optional: false,
                is_rest: false,
            },
        }
    }

    pub fn params_from_js_parameters(params: &JsParameters) -> Box<[Self]> {
        params
            .as_fields()
            .items
            .into_iter()
            .filter_map(|param| param.ok().map(|param| Self::from_any_js_parameter(&param)))
            .collect()
    }
}

impl GenericTypeParameter {
    pub fn from_ts_type_parameter(param: &TsTypeParameter) -> Option<Self> {
        param
            .name()
            .and_then(|name| name.ident_token())
            .map(|name| Self {
                name: name.token_text_trimmed().into(),
                default_ty: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map(|default_ty| Type::from_any_ts_type(&default_ty))
                    .unwrap_or_default(),
            })
            .ok()
    }

    pub fn params_from_ts_type_parameters(params: &TsTypeParameters) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .filter_map(|param| param.ok())
            .filter_map(|param| Self::from_ts_type_parameter(&param))
            .collect()
    }
}

impl ReturnType {
    pub fn from_any_ts_return_type(ty: &AnyTsReturnType) -> Option<Self> {
        match ty {
            AnyTsReturnType::AnyTsType(ty) => Some(Self::Type(Type::from_any_ts_type(ty))),
            AnyTsReturnType::TsAssertsReturnType(ty) => {
                ty.parameter_name().ok().and_then(|parameter_name| {
                    Some(Self::Asserts(AssertsReturnType {
                        parameter_name: match parameter_name {
                            AnyTsTypePredicateParameterName::JsReferenceIdentifier(identifier) => {
                                text_from_value_token(identifier.value_token())?
                            }
                            AnyTsTypePredicateParameterName::TsThisType(_) => Text::Static("text"),
                        },
                        ty: ty
                            .predicate()
                            .and_then(|asserts| asserts.ty().ok())
                            .map(|ty| Type::from_any_ts_type(&ty))
                            .unwrap_or_default(),
                    }))
                })
            }
            AnyTsReturnType::TsPredicateReturnType(ty) => {
                ty.parameter_name().ok().and_then(|parameter_name| {
                    Some(Self::Predicate(PredicateReturnType {
                        parameter_name: match parameter_name {
                            AnyTsTypePredicateParameterName::JsReferenceIdentifier(identifier) => {
                                text_from_value_token(identifier.value_token())?
                            }
                            AnyTsTypePredicateParameterName::TsThisType(_) => Text::Static("text"),
                        },
                        ty: ty
                            .ty()
                            .map(|ty| Type::from_any_ts_type(&ty))
                            .unwrap_or_default(),
                    }))
                })
            }
        }
    }
}

impl TupleElementType {
    pub fn from_any_ts_tuple_type_element(el: &AnyTsTupleTypeElement) -> Self {
        match el {
            AnyTsTupleTypeElement::AnyTsType(ty) => Self {
                ty: Type::from_any_ts_type(ty),
                name: None,
                is_optional: false,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsNamedTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                name: el
                    .name()
                    .ok()
                    .and_then(|name| text_from_value_token(name.value_token())),
                is_optional: el.question_mark_token().is_some(),
                is_rest: el.dotdotdot_token().is_some(),
            },
            AnyTsTupleTypeElement::TsOptionalTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: true,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsRestTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| Type::from_any_ts_type(&ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: false,
                is_rest: true,
            },
        }
    }
}

impl TypeMember {
    pub fn from_any_ts_type_member(member: &AnyTsTypeMember) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_) => None,
            AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                Some(Self::CallSignature(CallSignatureTypeMember {
                    type_parameters: member
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    parameters: member
                        .parameters()
                        .map(|params| FunctionParameter::params_from_js_parameters(&params))
                        .unwrap_or_default(),
                    return_type: member
                        .return_type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                        .unwrap_or_default(),
                }))
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                Some(Self::Constructor(ConstructorTypeMember {
                    type_parameters: member
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    parameters: member
                        .parameters()
                        .map(|params| FunctionParameter::params_from_js_parameters(&params))
                        .unwrap_or_default(),
                    return_type: member
                        .type_annotation()
                        .and_then(|annotation| annotation.ty().ok())
                        .map(|ty| Type::from_any_ts_type(&ty)),
                }))
            }
            AnyTsTypeMember::TsGetterSignatureTypeMember(_member) => {
                // TODO: Handle getters
                None
            }
            AnyTsTypeMember::TsIndexSignatureTypeMember(_member) => {
                // TODO: Handle index signatures
                None
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Method(MethodTypeMember {
                        type_parameters: member
                            .type_parameters()
                            .map(|params| {
                                GenericTypeParameter::params_from_ts_type_parameters(&params)
                            })
                            .unwrap_or_default(),
                        name: name.into(),
                        parameters: member
                            .parameters()
                            .map(|params| FunctionParameter::params_from_js_parameters(&params))
                            .unwrap_or_default(),
                        return_type: member
                            .return_type_annotation()
                            .and_then(|annotation| annotation.ty().ok())
                            .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                            .unwrap_or_default(),
                        is_optional: member.optional_token().is_some(),
                    })
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: name.into(),
                        ty: member
                            .type_annotation()
                            .and_then(|annotation| annotation.ty().ok())
                            .map(|ty| Type::from_any_ts_type(&ty))
                            .unwrap_or_default(),
                        is_optional: member.optional_token().is_some(),
                    })
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_member) => {
                // TODO: Handle setters
                None
            }
        }
    }
}

impl TypeReferenceQualifier {
    pub fn from_any_ts_name(name: &AnyTsName) -> Option<Self> {
        match name {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                text_from_value_token(identifier.value_token()).map(|text| Self(Box::new([text])))
            }
            AnyTsName::TsQualifiedName(name) => {
                let mut fields = name.as_fields();
                let mut identifiers = Vec::new();
                loop {
                    identifiers.insert(0, text_from_value_token(fields.right.ok()?.value_token())?);

                    match fields.left.ok()? {
                        AnyTsName::JsReferenceIdentifier(identifier) => {
                            identifiers.insert(0, text_from_value_token(identifier.value_token())?);
                            break;
                        }
                        AnyTsName::TsQualifiedName(name) => {
                            fields = name.as_fields();
                        }
                    }
                }
                Some(Self(identifiers.into()))
            }
        }
    }
}

fn text_from_value_token(value_token: SyntaxResult<SyntaxToken<JsLanguage>>) -> Option<Text> {
    Some(value_token.ok()?.token_text_trimmed().into())
}
