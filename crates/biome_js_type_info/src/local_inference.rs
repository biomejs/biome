use std::{str::FromStr, sync::Arc};

use biome_js_syntax::{
    AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsClassMember,
    AnyJsDeclaration, AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsExpression,
    AnyJsFormalParameter, AnyJsLiteralExpression, AnyJsName, AnyJsObjectMember, AnyJsParameter,
    AnyTsName, AnyTsReturnType, AnyTsTupleTypeElement, AnyTsType, AnyTsTypeMember,
    AnyTsTypePredicateParameterName, ClassMemberName, JsArrowFunctionExpression, JsCallArguments,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression, JsFunctionDeclaration,
    JsFunctionExpression, JsNewExpression, JsObjectExpression, JsParameters, JsReferenceIdentifier,
    JsSyntaxToken, JsVariableDeclarator, TsReferenceType, TsReturnTypeAnnotation,
    TsTypeAliasDeclaration, TsTypeAnnotation, TsTypeArguments, TsTypeParameter, TsTypeParameters,
    TsTypeofType, inner_string_text, unescape_js_string,
};
use biome_rowan::{AstNode, SyntaxResult, Text, TokenText};

use crate::{
    AssertsReturnType, CallArgumentType, CallSignatureTypeMember, Class, Constructor,
    ConstructorTypeMember, Function, FunctionParameter, GenericTypeParameter, Intersection,
    Literal, MethodTypeMember, Object, PredicateReturnType, PropertyTypeMember, ReturnType, Tuple,
    TupleElementType, Type, TypeAlias, TypeId, TypeInner, TypeMember, TypeOperator,
    TypeOperatorType, TypeReference, TypeReferenceQualifier, TypeofCallExpression,
    TypeofExpression, TypeofNewExpression, TypeofStaticMemberExpression,
    TypeofThisOrSuperExpression, TypeofValue, Union,
    globals::{ARRAY_TYPE, PROMISE_TYPE},
};

impl Type {
    pub fn from_any_js_declaration(decl: &AnyJsDeclaration) -> Self {
        match decl {
            AnyJsDeclaration::JsClassDeclaration(decl) => Self::from_js_class_declaration(decl),
            AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                Self::from_js_function_declaration(decl)
            }
            AnyJsDeclaration::JsVariableDeclaration(_) => {
                // Variable declarations don't have a type;
                // only their inner declarators have.
                Self::unknown()
            }
            AnyJsDeclaration::TsDeclareFunctionDeclaration(_decl) => {
                // TODO: Handle module declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsEnumDeclaration(_decl) => {
                // TODO: Handle enum declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsExternalModuleDeclaration(_decl) => {
                // TODO: Handle external module declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsGlobalDeclaration(_decl) => {
                // TODO: Handle global declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsImportEqualsDeclaration(_decl) => {
                // TODO: Handle `import T = Name` syntax.
                Self::unknown()
            }
            AnyJsDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsModuleDeclaration(_decl) => {
                // TODO: Handle module declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                Self::from_ts_type_alias_declaration(decl).unwrap_or_default()
            }
        }
    }

    pub fn from_any_js_declaration_clause(decl: AnyJsDeclarationClause) -> Self {
        decl.into_declaration()
            .map(|decl| Self::from_any_js_declaration(&decl))
            .unwrap_or_default()
    }

    pub fn from_any_js_export_default_declaration(decl: &AnyJsExportDefaultDeclaration) -> Self {
        match decl {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(decl) => {
                TypeInner::Class(Box::new(Class {
                    id: TypeId::new(),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    type_parameters: decl
                        .type_parameters()
                        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                        .unwrap_or_default(),
                    extends: decl
                        .extends_clause()
                        .and_then(|extends| extends.super_class().ok())
                        .map(|super_class| Self::from_any_js_expression(&super_class)),
                    members: decl
                        .members()
                        .into_iter()
                        .filter_map(|member| TypeMember::from_any_js_class_member(&member))
                        .collect(),
                }))
                .into()
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(decl) => {
                TypeInner::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: generic_params_from_ts_type_params(decl.type_parameters()),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(decl.parameters()),
                    return_type: return_type_from_annotation(decl.return_type_annotation())
                        .unwrap_or_else(|| return_type_from_async_token(decl.async_token())),
                }))
                .into()
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                TypeInner::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: generic_params_from_ts_type_params(decl.type_parameters()),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(decl.parameters()),
                    return_type: return_type_from_annotation(decl.return_type_annotation())
                        .unwrap_or_else(|| return_type_from_async_token(decl.async_token())),
                }))
                .into()
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::unknown()
            }
        }
    }

    pub fn from_any_js_expression(expr: &AnyJsExpression) -> Self {
        match expr {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                Self::from_any_js_literal_expression(expr).unwrap_or_default()
            }
            AnyJsExpression::JsArrayExpression(expr) => TypeInner::Tuple(Box::new(Tuple(
                expr.elements()
                    .into_iter()
                    .map(|el| match el {
                        Ok(AnyJsArrayElement::AnyJsExpression(expr)) => TupleElementType {
                            ty: Type::from_any_js_expression(&expr),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        },
                        Ok(AnyJsArrayElement::JsSpread(_spread)) => TupleElementType {
                            // TODO: We can definitely be smarter about this one.
                            ty: Type::unknown(),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        },
                        Ok(AnyJsArrayElement::JsArrayHole(_)) | Err(_) => TupleElementType {
                            ty: Type::unknown(),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        },
                    })
                    .collect(),
            )))
            .into(),
            AnyJsExpression::JsArrowFunctionExpression(expr) => {
                Self::from_js_arrow_function_expression(expr)
            }
            AnyJsExpression::JsCallExpression(expr) => match expr.callee() {
                Ok(callee) => TypeInner::TypeofExpression(Box::new(TypeofExpression::Call(
                    TypeofCallExpression {
                        callee: Self::from_any_js_expression(&callee),
                        arguments: CallArgumentType::types_from_js_call_arguments(
                            expr.arguments().ok(),
                        ),
                    },
                )))
                .into(),
                Err(_) => Self::unknown(),
            },
            AnyJsExpression::JsClassExpression(expr) => Self::from_js_class_expression(expr),
            AnyJsExpression::JsComputedMemberExpression(expr) => {
                match (expr.object(), expr.member()) {
                    (
                        Ok(object),
                        Ok(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(member),
                        )),
                    ) => unescaped_text_from_token(member.value_token())
                        .map(|member| {
                            TypeInner::TypeofExpression(Box::new(TypeofExpression::StaticMember(
                                TypeofStaticMemberExpression {
                                    object: Self::from_any_js_expression(&object),
                                    member,
                                },
                            )))
                        })
                        .unwrap_or_default()
                        .into(),
                    _ => Self::unknown(),
                }
            }
            AnyJsExpression::JsFunctionExpression(expr) => Self::from_js_function_expression(expr),
            AnyJsExpression::JsIdentifierExpression(expr) => expr
                .name()
                .map(|name| Self::from_js_reference_identifier(&name))
                .unwrap_or_default(),
            AnyJsExpression::JsInstanceofExpression(_expr) => Self::boolean(),
            AnyJsExpression::JsNewExpression(expr) => {
                Self::from_js_new_expression(expr).unwrap_or_default()
            }
            AnyJsExpression::JsObjectExpression(expr) => TypeInner::Object(Box::new(Object {
                prototype: None,
                members: expr
                    .members()
                    .into_iter()
                    .filter_map(|member| member.ok())
                    .filter_map(|member| TypeMember::from_any_js_object_member(&member))
                    .collect(),
            }))
            .into(),
            AnyJsExpression::JsParenthesizedExpression(expr) => expr
                .expression()
                .map(|expr| Self::from_any_js_expression(&expr))
                .unwrap_or_default(),
            AnyJsExpression::JsStaticMemberExpression(expr) => match (expr.object(), expr.member())
            {
                (Ok(object), Ok(member)) => text_from_any_js_name(member)
                    .map(|member| {
                        TypeInner::TypeofExpression(Box::new(TypeofExpression::StaticMember(
                            TypeofStaticMemberExpression {
                                object: Self::from_any_js_expression(&object),
                                member,
                            },
                        )))
                    })
                    .unwrap_or_default()
                    .into(),
                _ => Self::unknown(),
            },
            AnyJsExpression::JsSuperExpression(_) => TypeInner::TypeofExpression(Box::new(
                TypeofExpression::Super(TypeofThisOrSuperExpression::from_any_js_expression(expr)),
            ))
            .into(),
            AnyJsExpression::JsThisExpression(_) => TypeInner::TypeofExpression(Box::new(
                TypeofExpression::This(TypeofThisOrSuperExpression::from_any_js_expression(expr)),
            ))
            .into(),
            _ => {
                // TODO: Much
                Self::unknown()
            }
        }
    }

    pub fn from_any_js_literal_expression(expr: &AnyJsLiteralExpression) -> Option<Self> {
        let literal = match expr {
            AnyJsLiteralExpression::JsBigintLiteralExpression(expr) => {
                Literal::BigInt(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsBooleanLiteralExpression(expr) => {
                Literal::Boolean(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => Literal::Null,
            AnyJsLiteralExpression::JsNumberLiteralExpression(expr) => {
                Literal::Number(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(expr) => {
                Literal::RegExp(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(expr) => {
                Literal::String(text_from_token(expr.value_token())?)
            }
        };

        Some(TypeInner::Literal(Box::new(literal)).into())
    }

    pub fn from_any_ts_type(ty: &AnyTsType) -> Self {
        let inner = match ty {
            AnyTsType::JsMetavariable(_) => TypeInner::Unknown,
            AnyTsType::TsAnyType(_) => TypeInner::AnyKeyword,
            AnyTsType::TsArrayType(ty) => {
                return Self::instance_of(
                    ARRAY_TYPE.with_type_parameters(&[ty
                        .element_type()
                        .map(|ty| Self::from_any_ts_type(&ty))
                        .unwrap_or_default()]),
                );
            }
            AnyTsType::TsBigintLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => TypeInner::Literal(Box::new(
                    Literal::BigInt(Text::Owned(format!("{minus_token}{literal_token}"))),
                )),
                (None, Ok(literal_token)) => TypeInner::Literal(Box::new(Literal::BigInt(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => TypeInner::Unknown,
            },
            AnyTsType::TsBigintType(_) => TypeInner::BigInt,
            AnyTsType::TsBogusType(_) => TypeInner::Unknown,
            AnyTsType::TsBooleanLiteralType(ty) => match ty.literal() {
                Ok(token) => TypeInner::Literal(Box::new(Literal::Boolean(
                    token.token_text_trimmed().into(),
                ))),
                Err(_) => TypeInner::Unknown,
            },
            AnyTsType::TsBooleanType(_) => TypeInner::Boolean,
            AnyTsType::TsConditionalType(_) => {
                // TODO: Handle conditional types (`T extends U ? V : W`).
                TypeInner::Unknown
            }
            AnyTsType::TsConstructorType(ty) => TypeInner::Constructor(Box::new(Constructor {
                type_parameters: generic_params_from_ts_type_params(ty.type_parameters()),
                parameters: function_params_from_js_params(ty.parameters()),
                return_type: ty.return_type().ok().map(|ty| Type::from_any_ts_type(&ty)),
            })),
            AnyTsType::TsFunctionType(ty) => TypeInner::Function(Box::new(Function {
                is_async: false,
                type_parameters: generic_params_from_ts_type_params(ty.type_parameters()),
                name: None,
                parameters: function_params_from_js_params(ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
                    .unwrap_or_default(),
            })),
            AnyTsType::TsImportType(_) => {
                // TODO: Handle import types (`import("./module").T`).
                TypeInner::Unknown
            }
            AnyTsType::TsIndexedAccessType(_) => {
                // TODO: Handle type indexing (`T[U]`).
                TypeInner::Unknown
            }
            AnyTsType::TsInferType(_) => {
                // TODO: Handle `infer T` syntax.
                TypeInner::Unknown
            }
            AnyTsType::TsIntersectionType(ty) => TypeInner::Intersection(Box::new(Intersection(
                ty.types()
                    .into_iter()
                    .filter_map(|ty| ty.ok())
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .collect(),
            ))),
            AnyTsType::TsMappedType(_) => {
                // TODO: Handle mapped types (`type T<U> = { [K in keyof U]: V }`).
                TypeInner::Unknown
            }
            AnyTsType::TsNeverType(_) => TypeInner::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => TypeInner::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => TypeInner::Literal(Box::new(Literal::Null)),
            AnyTsType::TsNumberLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => TypeInner::Literal(Box::new(
                    Literal::Number(Text::Owned(format!("{minus_token}{literal_token}"))),
                )),
                (None, Ok(literal_token)) => TypeInner::Literal(Box::new(Literal::Number(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => TypeInner::Unknown,
            },
            AnyTsType::TsNumberType(_) => TypeInner::Number,
            AnyTsType::TsObjectType(ty) => TypeInner::Object(Box::new(Object {
                prototype: None,
                members: ty
                    .members()
                    .into_iter()
                    .filter_map(|member| TypeMember::from_any_ts_type_member(&member))
                    .collect(),
            })),
            AnyTsType::TsParenthesizedType(ty) => {
                return ty
                    .ty()
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .unwrap_or_default();
            }
            AnyTsType::TsReferenceType(ty) => {
                return Self::from_ts_reference_type(ty);
            }
            AnyTsType::TsStringLiteralType(ty) => match ty.literal_token() {
                Ok(token) => {
                    TypeInner::Literal(Box::new(Literal::String(token.token_text_trimmed().into())))
                }
                Err(_) => TypeInner::Unknown,
            },
            AnyTsType::TsStringType(_) => TypeInner::String,
            AnyTsType::TsSymbolType(_) => TypeInner::Symbol,
            AnyTsType::TsTemplateLiteralType(ty) => {
                TypeInner::Literal(Box::new(Literal::Template(Text::Owned(ty.to_string()))))
            }
            AnyTsType::TsThisType(_) => TypeInner::ThisKeyword,
            AnyTsType::TsTupleType(ty) => {
                let elements: SyntaxResult<Box<_>> = ty
                    .elements()
                    .into_iter()
                    .map(|el| el.map(|el| TupleElementType::from_any_ts_tuple_type_element(&el)))
                    .collect();
                match elements {
                    Ok(elements) => TypeInner::Tuple(Box::new(Tuple(elements))),
                    Err(_) => TypeInner::Unknown,
                }
            }
            AnyTsType::TsTypeOperatorType(ty) => match (ty.operator_token(), ty.ty()) {
                (Ok(operator_token), Ok(ty)) => TypeOperator::from_str(
                    operator_token.text_trimmed(),
                )
                .map_or(TypeInner::Unknown, |operator| {
                    TypeInner::TypeOperator(Box::new(TypeOperatorType {
                        operator,
                        ty: Self::from_any_ts_type(&ty),
                    }))
                }),
                _ => TypeInner::Unknown,
            },
            AnyTsType::TsTypeofType(ty) => {
                return Self::from_ts_typeof_type(ty);
            }
            AnyTsType::TsUndefinedType(_) => TypeInner::Undefined,
            AnyTsType::TsUnionType(ty) => TypeInner::Union(Box::new(Union(
                ty.types()
                    .into_iter()
                    .filter_map(|ty| ty.ok())
                    .map(|ty| Self::from_any_ts_type(&ty))
                    .collect(),
            ))),
            AnyTsType::TsUnknownType(_) => TypeInner::UnknownKeyword,
            AnyTsType::TsVoidType(_) => TypeInner::VoidKeyword,
        };

        inner.into()
    }

    pub fn from_any_ts_type_result(ty: SyntaxResult<AnyTsType>) -> Self {
        ty.map(|ty| Self::from_any_ts_type(&ty)).unwrap_or_default()
    }

    pub fn from_js_arrow_function_expression(expr: &JsArrowFunctionExpression) -> Self {
        TypeInner::Function(Box::new(Function {
            is_async: expr.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(expr.type_parameters()),
            name: None,
            parameters: match expr.parameters() {
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(binding)) => {
                    Arc::new([FunctionParameter {
                        name: binding
                            .as_js_identifier_binding()
                            .and_then(|binding| text_from_token(binding.name_token())),
                        ty: Type::unknown(),
                        is_optional: false,
                        is_rest: false,
                    }])
                }
                Ok(AnyJsArrowFunctionParameters::JsParameters(params)) => {
                    function_params_from_js_params(Ok(params))
                }
                Err(_) => Arc::default(),
            },
            return_type: return_type_from_annotation(expr.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(expr.async_token())),
        }))
        .into()
    }

    pub fn from_js_class_declaration(decl: &JsClassDeclaration) -> Self {
        TypeInner::Class(Box::new(Class {
            id: TypeId::new(),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|id| id.as_js_identifier_binding())
                .and_then(|id| id.name_token().ok())
                .map(|token| token.token_text_trimmed().into()),
            type_parameters: decl
                .type_parameters()
                .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                .unwrap_or_default(),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| Self::from_any_js_expression(&super_class)),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| TypeMember::from_any_js_class_member(&member))
                .collect(),
        }))
        .into()
    }

    pub fn from_js_class_expression(decl: &JsClassExpression) -> Self {
        TypeInner::Class(Box::new(Class {
            id: TypeId::new(),
            name: decl
                .id()
                .as_ref()
                .and_then(|id| id.as_js_identifier_binding())
                .and_then(|id| id.name_token().ok())
                .map(|token| token.token_text_trimmed().into()),
            type_parameters: decl
                .type_parameters()
                .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
                .unwrap_or_default(),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| Self::from_any_js_expression(&super_class)),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| TypeMember::from_any_js_class_member(&member))
                .collect(),
        }))
        .into()
    }

    pub fn from_js_function_declaration(decl: &JsFunctionDeclaration) -> Self {
        TypeInner::Function(Box::new(Function {
            is_async: decl.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(decl.type_parameters()),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(decl.parameters()),
            return_type: return_type_from_annotation(decl.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(decl.async_token())),
        }))
        .into()
    }

    pub fn from_js_function_expression(expr: &JsFunctionExpression) -> Self {
        TypeInner::Function(Box::new(Function {
            is_async: expr.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(expr.type_parameters()),
            name: expr
                .id()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(expr.parameters()),
            return_type: return_type_from_annotation(expr.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(expr.async_token())),
        }))
        .into()
    }

    pub fn from_js_new_expression(expr: &JsNewExpression) -> Option<Self> {
        Some(
            TypeInner::TypeofExpression(Box::new(TypeofExpression::New(TypeofNewExpression {
                callee: Self::from_any_js_expression(&expr.callee().ok()?),
                arguments: CallArgumentType::types_from_js_call_arguments(expr.arguments()),
            })))
            .into(),
        )
    }

    pub fn from_js_object_expression(expr: &JsObjectExpression) -> Self {
        TypeInner::Object(Box::new(Object {
            prototype: None,
            members: expr
                .members()
                .into_iter()
                .filter_map(|member| {
                    member
                        .ok()
                        .and_then(|member| TypeMember::from_any_js_object_member(&member))
                })
                .collect(),
        }))
        .into()
    }

    pub fn from_js_reference_identifier(id: &JsReferenceIdentifier) -> Self {
        if id.is_undefined() {
            Self::undefined()
        } else {
            id.name()
                .map(|name| TypeInner::Reference(Box::new(TypeReference::from_name(name))))
                .map(Into::into)
                .unwrap_or_default()
        }
    }

    pub fn from_js_variable_declarator(decl: &JsVariableDeclarator) -> Option<Self> {
        let ty = match decl.variable_annotation() {
            Some(annotation) => {
                let annotation = annotation.type_annotation().ok()??;
                Self::from_any_ts_type(&annotation.ty().ok()?)
            }
            None => Self::from_any_js_expression(&decl.initializer()?.expression().ok()?),
        };

        Some(ty)
    }

    pub fn from_ts_reference_type(ty: &TsReferenceType) -> Self {
        ty.name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                TypeInner::Reference(Box::new(TypeReference {
                    qualifier,
                    ty: Type::unknown(),
                    type_parameters: Self::types_from_ts_type_arguments(ty.type_arguments()),
                }))
            })
            .map(Into::into)
            .unwrap_or_default()
    }

    pub fn from_ts_type_alias_declaration(decl: &TsTypeAliasDeclaration) -> Option<Self> {
        Some(match decl.type_parameters() {
            Some(params) => TypeInner::Alias(Box::new(TypeAlias {
                ty: Self::from_any_ts_type(&decl.ty().ok()?),
                type_parameters: GenericTypeParameter::params_from_ts_type_parameters(&params),
            }))
            .into(),
            None => Self::from_any_ts_type(&decl.ty().ok()?),
        })
    }

    pub fn from_ts_typeof_type(ty: &TsTypeofType) -> Self {
        ty.expression_name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                TypeInner::TypeofType(Box::new(
                    TypeInner::Reference(Box::new(TypeReference {
                        qualifier,
                        ty: Type::unknown(),
                        type_parameters: Self::types_from_ts_type_arguments(ty.type_arguments()),
                    }))
                    .into(),
                ))
            })
            .map(Into::into)
            .unwrap_or_default()
    }

    pub fn types_from_ts_type_arguments(arguments: Option<TsTypeArguments>) -> Arc<[Self]> {
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

impl CallArgumentType {
    pub fn types_from_js_call_arguments(
        arguments: Option<JsCallArguments>,
    ) -> Arc<[CallArgumentType]> {
        arguments
            .map(|args| {
                args.args()
                    .into_iter()
                    .filter_map(|arg| {
                        arg.ok()
                            .map(|arg| CallArgumentType::from_any_js_call_argument(&arg))
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn from_any_js_call_argument(arg: &AnyJsCallArgument) -> Self {
        match arg {
            AnyJsCallArgument::AnyJsExpression(expr) => {
                Self::Argument(Type::from_any_js_expression(expr))
            }
            AnyJsCallArgument::JsSpread(spread) => Self::Spread(
                spread
                    .argument()
                    .map(|arg| Type::from_any_js_expression(&arg))
                    .unwrap_or_default(),
            ),
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
                ty: Type::unknown(),
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

    pub fn params_from_js_parameters(params: &JsParameters) -> Arc<[Self]> {
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
                ty: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map(|default_ty| Type::from_any_ts_type(&default_ty))
                    .unwrap_or_default(),
            })
            .ok()
    }

    pub fn params_from_ts_type_parameters(params: &TsTypeParameters) -> Arc<[Self]> {
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
                                text_from_token(identifier.value_token())?
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
                                text_from_token(identifier.value_token())?
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
                    .and_then(|name| text_from_token(name.value_token())),
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
    pub fn from_any_js_class_member(member: &AnyJsClassMember) -> Option<Self> {
        match member {
            AnyJsClassMember::JsMethodClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Method(MethodTypeMember {
                        is_async: member.async_token().is_some(),
                        type_parameters: generic_params_from_ts_type_params(
                            member.type_parameters(),
                        ),
                        name: text_from_class_member_name(name),
                        parameters: function_params_from_js_params(member.parameters()),
                        return_type: return_type_from_annotation(member.return_type_annotation())
                            .unwrap_or_else(|| return_type_from_async_token(member.async_token())),
                        is_optional: false,
                        is_static: member
                            .modifiers()
                            .into_iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    })
                })
            }
            AnyJsClassMember::JsPropertyClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: text_from_class_member_name(name),
                        ty: member
                            .property_annotation()
                            .and_then(|annotation| annotation.type_annotation().ok())
                            .flatten()
                            .and_then(|annotation| annotation.ty().ok())
                            .map_or_else(
                                || {
                                    member
                                        .value()
                                        .and_then(|initializer| initializer.expression().ok())
                                        .map(|expr| Type::from_any_js_expression(&expr))
                                        .unwrap_or_default()
                                },
                                |ty| Type::from_any_ts_type(&ty),
                            ),
                        is_optional: member
                            .property_annotation()
                            .as_ref()
                            .and_then(|annotation| annotation.as_ts_optional_property_annotation())
                            .is_some(),
                        is_static: member
                            .modifiers()
                            .into_iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    })
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(member) => member
                .name()
                .ok()
                .and_then(|name| name.name())
                .and_then(|name| {
                    Some(Self::Property(PropertyTypeMember {
                        name: text_from_class_member_name(name),
                        ty: Type::from_any_js_expression(&member.value().ok()?.expression().ok()?),
                        is_optional: member.question_mark_token().is_some(),
                        is_static: member
                            .modifiers()
                            .into_iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    }))
                }),
            AnyJsClassMember::TsPropertySignatureClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: text_from_class_member_name(name),
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
                        is_static: member
                            .modifiers()
                            .into_iter()
                            .any(|modifier| modifier.as_js_static_modifier().is_some()),
                    })
                })
            }
            _ => {
                // TODO: Handle more variants
                None
            }
        }
    }

    pub fn from_any_js_object_member(member: &AnyJsObjectMember) -> Option<Self> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => {
                // TODO: Handle getters
                None
            }
            AnyJsObjectMember::JsMethodObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    TypeMember::Method(MethodTypeMember {
                        is_async: member.async_token().is_some(),
                        type_parameters: generic_params_from_ts_type_params(
                            member.type_parameters(),
                        ),
                        name: name.into(),
                        parameters: function_params_from_js_params(member.parameters()),
                        return_type: return_type_from_annotation(member.return_type_annotation())
                            .unwrap_or_else(|| return_type_from_async_token(member.async_token())),
                        is_optional: false,
                        is_static: false,
                    })
                })
            }
            AnyJsObjectMember::JsPropertyObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    TypeMember::Property(PropertyTypeMember {
                        name: name.into(),
                        ty: member
                            .value()
                            .map(|value| Type::from_any_js_expression(&value))
                            .unwrap_or_default(),
                        is_optional: false,
                        is_static: false,
                    })
                })
            }
            AnyJsObjectMember::JsSetterObjectMember(_) => {
                // TODO: Handle setters
                None
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => member
                .name()
                .ok()
                .and_then(|name| text_from_token(name.value_token()))
                .map(|name| {
                    TypeMember::Property(PropertyTypeMember {
                        name: name.clone(),
                        ty: TypeInner::TypeofValue(Box::new(TypeofValue {
                            identifier: name,
                            ty: Type::unknown(),
                        }))
                        .into(),
                        is_optional: false,
                        is_static: false,
                    })
                }),
            AnyJsObjectMember::JsSpread(_) => {
                // TODO: Handle spread operator
                None
            }
        }
    }

    pub fn from_any_ts_type_member(member: &AnyTsTypeMember) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_) => None,
            AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                Some(Self::CallSignature(CallSignatureTypeMember {
                    type_parameters: generic_params_from_ts_type_params(member.type_parameters()),
                    parameters: function_params_from_js_params(member.parameters()),
                    return_type: return_type_from_annotation(member.return_type_annotation())
                        .unwrap_or_default(),
                }))
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                Some(Self::Constructor(ConstructorTypeMember {
                    type_parameters: generic_params_from_ts_type_params(member.type_parameters()),
                    parameters: function_params_from_js_params(member.parameters()),
                    return_type: type_from_annotation(member.type_annotation()),
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
                        is_async: false,
                        type_parameters: generic_params_from_ts_type_params(
                            member.type_parameters(),
                        ),
                        name: name.into(),
                        parameters: function_params_from_js_params(member.parameters()),
                        return_type: return_type_from_annotation(member.return_type_annotation())
                            .unwrap_or_default(),
                        is_optional: member.optional_token().is_some(),
                        is_static: false,
                    })
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: name.into(),
                        ty: type_from_annotation(member.type_annotation()).unwrap_or_default(),
                        is_optional: member.optional_token().is_some(),
                        is_static: false,
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

impl TypeReference {
    pub fn from_name(name: TokenText) -> Self {
        Self::from_qualifier(TypeReferenceQualifier::from_name(name.into()))
    }

    pub fn from_qualifier(qualifier: TypeReferenceQualifier) -> Self {
        Self {
            qualifier,
            ty: Type::unknown(),
            type_parameters: Arc::new([]),
        }
    }
}

impl TypeReferenceQualifier {
    pub fn from_any_ts_name(name: &AnyTsName) -> Option<Self> {
        match name {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                text_from_token(identifier.value_token()).map(|text| Self(Box::new([text])))
            }
            AnyTsName::TsQualifiedName(name) => {
                let mut fields = name.as_fields();
                let mut identifiers = Vec::new();
                loop {
                    identifiers.insert(0, text_from_token(fields.right.ok()?.value_token())?);

                    match fields.left.ok()? {
                        AnyTsName::JsReferenceIdentifier(identifier) => {
                            identifiers.insert(0, text_from_token(identifier.value_token())?);
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

    pub fn from_name(name: Text) -> Self {
        Self(Box::new([name]))
    }
}

impl TypeofThisOrSuperExpression {
    fn from_any_js_expression(expr: &AnyJsExpression) -> Self {
        let parent = expr
            .syntax()
            .ancestors()
            .find_map(|node| {
                let binding = if let Some(class) = JsClassDeclaration::cast_ref(&node) {
                    class.id().ok()
                } else if let Some(class) = JsClassExpression::cast_ref(&node) {
                    if let Some(declarator) = class
                        .syntax()
                        .ancestors()
                        .find_map(JsVariableDeclarator::cast)
                        .filter(|declarator| {
                            declarator.initializer().is_some_and(|initializer| {
                                initializer.expression().is_ok_and(|expr| {
                                    matches!(expr, AnyJsExpression::JsClassExpression(_))
                                })
                            })
                        })
                    {
                        let pattern = declarator.id().ok();
                        pattern.and_then(|pattern| pattern.as_any_js_binding().cloned())
                    } else {
                        class.id()
                    }
                } else if let Some(class) = JsClassExportDefaultDeclaration::cast_ref(&node) {
                    class.id()
                } else if let Some(object) = JsObjectExpression::cast(node) {
                    object
                        .syntax()
                        .ancestors()
                        .find_map(JsVariableDeclarator::cast)
                        .filter(|declarator| {
                            declarator.initializer().is_some_and(|initializer| {
                                initializer.expression().is_ok_and(|expr| {
                                    matches!(expr, AnyJsExpression::JsObjectExpression(_))
                                })
                            })
                        })
                        .and_then(|declarator| declarator.id().ok())
                        .and_then(|pattern| pattern.as_any_js_binding().cloned())
                } else {
                    None
                }?;

                let binding = binding.as_js_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                let ty = TypeInner::Reference(Box::new(TypeReference {
                    qualifier: TypeReferenceQualifier::from_name(name),
                    ty: Type::unknown(),
                    type_parameters: Arc::new([]),
                }));
                Some(ty.into())
            })
            .unwrap_or_default();

        Self { parent }
    }
}

fn function_params_from_js_params(params: SyntaxResult<JsParameters>) -> Arc<[FunctionParameter]> {
    params
        .ok()
        .map(|params| FunctionParameter::params_from_js_parameters(&params))
        .unwrap_or_default()
}

fn generic_params_from_ts_type_params(
    params: Option<TsTypeParameters>,
) -> Arc<[GenericTypeParameter]> {
    params
        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(&params))
        .unwrap_or_default()
}

fn return_type_from_annotation(annotation: Option<TsReturnTypeAnnotation>) -> Option<ReturnType> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .and_then(|ty| ReturnType::from_any_ts_return_type(&ty))
}

fn return_type_from_async_token(async_token: Option<JsSyntaxToken>) -> ReturnType {
    ReturnType::Type(match async_token {
        Some(_) => PROMISE_TYPE.clone(),
        None => Type::unknown(),
    })
}

fn text_from_any_js_name(name: AnyJsName) -> Option<Text> {
    match name {
        AnyJsName::JsMetavariable(_) => None,
        AnyJsName::JsName(name) => text_from_token(name.value_token()),
        AnyJsName::JsPrivateName(name) => name
            .value_token()
            .ok()
            .map(|token| Text::Owned(format!("#{}", token.token_text_trimmed()))),
    }
}

fn text_from_class_member_name(name: ClassMemberName) -> Text {
    match name {
        ClassMemberName::Private(name) => Text::Owned(format!("#{name}")),
        ClassMemberName::Public(name) => Text::Borrowed(name),
    }
}

fn text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(token.ok()?.token_text_trimmed().into())
}

fn type_from_annotation(annotation: Option<TsTypeAnnotation>) -> Option<Type> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .map(|ty| Type::from_any_ts_type(&ty))
}

fn unescaped_text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(unescape_js_string(inner_string_text(&token.ok()?)))
}
