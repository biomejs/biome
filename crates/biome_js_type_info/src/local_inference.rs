use std::str::FromStr;

use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsClassMember, AnyJsDeclaration,
    AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsFormalParameter,
    AnyJsLiteralExpression, AnyJsName, AnyJsObjectBindingPatternMember, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsParameter, AnyTsName, AnyTsReturnType, AnyTsTupleTypeElement,
    AnyTsType, AnyTsTypeMember, AnyTsTypePredicateParameterName, ClassMemberName,
    JsArrayBindingPattern, JsArrowFunctionExpression, JsBinaryExpression, JsBinaryOperator,
    JsCallArguments, JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression,
    JsFormalParameter, JsFunctionDeclaration, JsFunctionExpression, JsNewExpression,
    JsObjectBindingPattern, JsObjectExpression, JsParameters, JsReferenceIdentifier, JsSyntaxToken,
    JsVariableDeclaration, JsVariableDeclarator, TsReferenceType, TsReturnTypeAnnotation,
    TsTypeAliasDeclaration, TsTypeAnnotation, TsTypeArguments, TsTypeParameter, TsTypeParameters,
    TsTypeofType, inner_string_text, unescape_js_string,
};
use biome_rowan::{AstNode, SyntaxResult, Text, TokenText};

use crate::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
use crate::{
    AssertsReturnType, CallArgumentType, CallSignatureTypeMember, Class, Constructor,
    ConstructorTypeMember, DestructureField, Function, FunctionParameter, FunctionParameterBinding,
    GenericTypeParameter, Intersection, Literal, MethodTypeMember, Object, PredicateReturnType,
    PropertyTypeMember, ReturnType, Tuple, TupleElementType, TypeData, TypeInstance, TypeMember,
    TypeOperator, TypeOperatorType, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeofCallExpression, TypeofExpression, TypeofNewExpression, TypeofStaticMemberExpression,
    TypeofThisOrSuperExpression, TypeofValue, Union,
};

impl TypeData {
    /// Applies the given `pattern` and returns the named bindings, and their
    /// associated types.
    pub fn apply_array_binding_pattern(
        &self,
        resolver: &mut dyn TypeResolver,
        pattern: &JsArrayBindingPattern,
    ) -> Box<[(Text, Self)]> {
        pattern
            .elements()
            .into_iter()
            .enumerate()
            .filter_map(|(i, elem)| elem.ok().map(|elem| (i, elem)))
            .filter_map(|(i, elem)| self.apply_array_binding_pattern_element(resolver, i, elem))
            .flatten()
            .collect()
    }

    fn apply_array_binding_pattern_element(
        &self,
        resolver: &mut dyn TypeResolver,
        i: usize,
        elem: AnyJsArrayBindingPatternElement,
    ) -> Option<Box<[(Text, Self)]>> {
        let reference = resolver.reference_to_registered_data(self.clone());
        match elem {
            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(elem) => {
                match elem.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            resolver.destructuring_of(reference, DestructureField::Index(i)),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = resolver.destructuring_of(reference, DestructureField::Index(i));
                        data.apply_array_binding_pattern(resolver, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = resolver.destructuring_of(reference, DestructureField::Index(i));
                        data.apply_object_binding_pattern(resolver, &pattern)
                    }),
                }
            }
            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(elem) => {
                match elem.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            resolver
                                .destructuring_of(reference, DestructureField::RestFrom(i))
                                .clone(),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data =
                            resolver.destructuring_of(reference, DestructureField::RestFrom(i));
                        data.apply_array_binding_pattern(resolver, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(_pattern) => {
                        // An array rest element destructures into another array,
                        // so applying an object pattern would give nonsensical
                        // results.
                        None
                    }
                }
            }
            AnyJsArrayBindingPatternElement::JsArrayHole(_) => None,
        }
    }

    /// Applies the given `pattern` and returns the named bindings, and their
    /// associated types.
    pub fn apply_object_binding_pattern(
        &self,
        resolver: &mut dyn TypeResolver,
        pattern: &JsObjectBindingPattern,
    ) -> Box<[(Text, Self)]> {
        // Accumulate names to exclude from the rest operator.
        let mut names = Vec::new();

        pattern
            .properties()
            .into_iter()
            .filter_map(|member| member.ok())
            .filter_map(|member| {
                let name = match &member {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(prop) => {
                        match prop.member().ok()? {
                            AnyJsObjectMemberName::JsComputedMemberName(name) => {
                                match name.expression() {
                                    Ok(AnyJsExpression::AnyJsLiteralExpression(
                                        AnyJsLiteralExpression::JsStringLiteralExpression(member),
                                    )) => unescaped_text_from_token(member.value_token()),
                                    // TODO: Support dynamic destructuring fields
                                    _ => None,
                                }
                            }
                            AnyJsObjectMemberName::JsLiteralMemberName(name) => {
                                text_from_token(name.value())
                            }
                            AnyJsObjectMemberName::JsMetavariable(_) => None,
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        prop,
                    ) => {
                        let binding = prop.identifier().ok()?;
                        let binding = binding.as_js_identifier_binding()?;
                        text_from_token(binding.name_token())
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_)
                    | AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                    | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
                };

                if let Some(name) = &name {
                    names.push(name.clone());
                }

                self.apply_object_binding_pattern_member(resolver, &names, name, member)
            })
            .flatten()
            .collect()
    }

    fn apply_object_binding_pattern_member(
        &self,
        resolver: &mut dyn TypeResolver,
        names: &[Text],
        member_name: Option<Text>,
        member: AnyJsObjectBindingPatternMember,
    ) -> Option<Box<[(Text, Self)]>> {
        let reference = resolver.reference_to_registered_data(self.clone());
        match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(prop) => {
                let member_name = member_name?;
                match prop.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            resolver
                                .destructuring_of(reference, DestructureField::Name(member_name))
                                .clone(),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = resolver
                            .destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_array_binding_pattern(resolver, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = resolver
                            .destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_object_binding_pattern(resolver, &pattern)
                    }),
                }
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(_) => Some({
                let member_name = member_name?;
                Box::new([(
                    member_name.clone(),
                    resolver.destructuring_of(reference, DestructureField::Name(member_name)),
                )])
            }),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => Some({
                let binding = rest.binding().ok()?;
                let binding = binding.as_js_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                Box::new([(
                    name,
                    resolver.destructuring_of(
                        reference,
                        DestructureField::RestExcept(names.iter().cloned().collect()),
                    ),
                )])
            }),
            AnyJsObjectBindingPatternMember::JsBogusBinding(_)
            | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
        }
    }

    pub fn array_of(ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::Qualifier(TypeReferenceQualifier {
            path: [Text::Static("Array")].into(),
            type_parameters: [ty].into(),
        }))
    }

    pub fn from_any_js_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &AnyJsDeclaration,
    ) -> Self {
        match decl {
            AnyJsDeclaration::JsClassDeclaration(decl) => {
                Self::from_js_class_declaration(resolver, decl)
            }
            AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                Self::from_js_function_declaration(resolver, decl)
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
                Self::from_ts_type_alias_declaration(resolver, decl).unwrap_or_default()
            }
        }
    }

    pub fn from_any_js_declaration_clause(
        resolver: &mut dyn TypeResolver,
        decl: AnyJsDeclarationClause,
    ) -> Self {
        decl.into_declaration()
            .map(|decl| Self::from_any_js_declaration(resolver, &decl))
            .unwrap_or_default()
    }

    pub fn from_any_js_export_default_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &AnyJsExportDefaultDeclaration,
    ) -> Self {
        match decl {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(decl) => {
                Self::Class(Box::new(Class {
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    type_parameters: decl
                        .type_parameters()
                        .map(|params| {
                            GenericTypeParameter::params_from_ts_type_parameters(resolver, &params)
                        })
                        .unwrap_or_default(),
                    extends: decl
                        .extends_clause()
                        .and_then(|extends| extends.super_class().ok())
                        .map(|super_class| {
                            TypeReference::from_any_js_expression(resolver, &super_class)
                        }),
                    members: decl
                        .members()
                        .into_iter()
                        .filter_map(|member| {
                            TypeMember::from_any_js_class_member(resolver, &member)
                        })
                        .collect(),
                }))
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(decl) => {
                Self::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        decl.type_parameters(),
                    ),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(resolver, decl.parameters()),
                    return_type: return_type_from_annotation(
                        resolver,
                        decl.return_type_annotation(),
                    )
                    .unwrap_or_else(|| return_type_from_async_token(resolver, decl.async_token())),
                }))
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                Self::Function(Box::new(Function {
                    is_async: decl.async_token().is_some(),
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        decl.type_parameters(),
                    ),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(resolver, decl.parameters()),
                    return_type: return_type_from_annotation(
                        resolver,
                        decl.return_type_annotation(),
                    )
                    .unwrap_or_else(|| return_type_from_async_token(resolver, decl.async_token())),
                }))
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::unknown()
            }
        }
    }

    pub fn from_any_js_expression(resolver: &mut dyn TypeResolver, expr: &AnyJsExpression) -> Self {
        match expr {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                Self::from_any_js_literal_expression(expr).unwrap_or_default()
            }
            AnyJsExpression::JsArrayExpression(expr) => Self::Tuple(Box::new(Tuple(
                expr.elements()
                    .into_iter()
                    .filter_map(|el| match el {
                        Ok(AnyJsArrayElement::AnyJsExpression(expr)) => Some(TupleElementType {
                            ty: TypeReference::from_any_js_expression(resolver, &expr),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        }),
                        Ok(AnyJsArrayElement::JsSpread(spread)) => spread
                            .argument()
                            .ok()
                            .map(|expr| TypeReference::from_any_js_expression(resolver, &expr))
                            .map(|ty| TupleElementType {
                                ty,
                                name: None,
                                is_optional: false,
                                is_rest: true,
                            }),
                        Ok(AnyJsArrayElement::JsArrayHole(_)) | Err(_) => Some(TupleElementType {
                            ty: TypeReference::Unknown,
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        }),
                    })
                    .collect(),
            ))),
            AnyJsExpression::JsArrowFunctionExpression(expr) => {
                Self::from_js_arrow_function_expression(resolver, expr)
            }
            AnyJsExpression::JsBinaryExpression(expr) => {
                Self::from_js_binary_expression(resolver, expr)
            }
            AnyJsExpression::JsCallExpression(expr) => match expr.callee() {
                Ok(callee) => {
                    Self::TypeofExpression(Box::new(TypeofExpression::Call(TypeofCallExpression {
                        callee: TypeReference::from_any_js_expression(resolver, &callee),
                        arguments: CallArgumentType::types_from_js_call_arguments(
                            resolver,
                            expr.arguments().ok(),
                        ),
                    })))
                }
                Err(_) => Self::unknown(),
            },
            AnyJsExpression::JsClassExpression(expr) => {
                Self::from_js_class_expression(resolver, expr)
            }
            AnyJsExpression::JsComputedMemberExpression(expr) => {
                match (expr.object(), expr.member()) {
                    (
                        Ok(object),
                        Ok(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(member),
                        )),
                    ) => unescaped_text_from_token(member.value_token())
                        .map(|member| {
                            Self::TypeofExpression(Box::new(TypeofExpression::StaticMember(
                                TypeofStaticMemberExpression {
                                    object: TypeReference::from_any_js_expression(
                                        resolver, &object,
                                    ),
                                    member,
                                },
                            )))
                        })
                        .unwrap_or_default(),
                    _ => Self::unknown(),
                }
            }
            AnyJsExpression::JsFunctionExpression(expr) => {
                Self::from_js_function_expression(resolver, expr)
            }
            AnyJsExpression::JsIdentifierExpression(expr) => expr
                .name()
                .map(|name| Self::from_js_reference_identifier(&name))
                .unwrap_or_default(),
            AnyJsExpression::JsInstanceofExpression(_expr) => Self::Boolean,
            AnyJsExpression::JsNewExpression(expr) => {
                Self::from_js_new_expression(resolver, expr).unwrap_or_default()
            }
            AnyJsExpression::JsObjectExpression(expr) => Self::object_with_members(
                expr.members()
                    .into_iter()
                    .filter_map(|member| member.ok())
                    .filter_map(|member| TypeMember::from_any_js_object_member(resolver, &member))
                    .collect(),
            ),
            AnyJsExpression::JsParenthesizedExpression(expr) => expr
                .expression()
                .map(|expr| Self::from_any_js_expression(resolver, &expr))
                .unwrap_or_default(),
            AnyJsExpression::JsStaticMemberExpression(expr) => match (expr.object(), expr.member())
            {
                (Ok(object), Ok(member)) => text_from_any_js_name(member)
                    .map(|member| {
                        Self::TypeofExpression(Box::new(TypeofExpression::StaticMember(
                            TypeofStaticMemberExpression {
                                object: TypeReference::from_any_js_expression(resolver, &object),
                                member,
                            },
                        )))
                    })
                    .unwrap_or_default(),
                _ => Self::unknown(),
            },
            AnyJsExpression::JsSuperExpression(_) => Self::TypeofExpression(Box::new(
                TypeofExpression::Super(TypeofThisOrSuperExpression::from_any_js_expression(expr)),
            )),
            AnyJsExpression::JsThisExpression(_) => Self::TypeofExpression(Box::new(
                TypeofExpression::This(TypeofThisOrSuperExpression::from_any_js_expression(expr)),
            )),
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
            AnyJsLiteralExpression::JsBooleanLiteralExpression(expr) => Literal::Boolean(
                BooleanLiteral::parse(text_from_token(expr.value_token())?.text())?,
            ),
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => Literal::Null,
            AnyJsLiteralExpression::JsNumberLiteralExpression(expr) => Literal::Number(
                NumberLiteral::parse(text_from_token(expr.value_token())?.text())?,
            ),
            AnyJsLiteralExpression::JsRegexLiteralExpression(expr) => {
                Literal::RegExp(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(expr) => Literal::String(
                StringLiteral::from(Text::Borrowed(expr.inner_string_text().ok()?)),
            ),
        };

        Some(Self::Literal(Box::new(literal)))
    }

    pub fn from_any_ts_type(resolver: &mut dyn TypeResolver, ty: &AnyTsType) -> Self {
        match ty {
            AnyTsType::JsMetavariable(_) => Self::Unknown,
            AnyTsType::TsAnyType(_) => Self::AnyKeyword,
            AnyTsType::TsArrayType(ty) => Self::array_of(
                ty.element_type()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .unwrap_or_default(),
            ),
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
                    BooleanLiteral::parse(token.text_trimmed()).unwrap(),
                ))),
                Err(_) => Self::Unknown,
            },
            AnyTsType::TsBooleanType(_) => Self::Boolean,
            AnyTsType::TsConditionalType(_) => {
                // TODO: Handle conditional types (`T extends U ? V : W`).
                Self::Unknown
            }
            AnyTsType::TsConstructorType(ty) => Self::Constructor(Box::new(Constructor {
                type_parameters: generic_params_from_ts_type_params(resolver, ty.type_parameters()),
                parameters: function_params_from_js_params(resolver, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty)),
            })),
            AnyTsType::TsFunctionType(ty) => Self::Function(Box::new(Function {
                is_async: false,
                type_parameters: generic_params_from_ts_type_params(resolver, ty.type_parameters()),
                name: None,
                parameters: function_params_from_js_params(resolver, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .and_then(|ty| ReturnType::from_any_ts_return_type(resolver, &ty))
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
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .collect(),
            ))),
            AnyTsType::TsMappedType(_) => {
                // TODO: Handle mapped types (`type T<U> = { [K in keyof U]: V }`).
                Self::Unknown
            }
            AnyTsType::TsNeverType(_) => Self::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => Self::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => Self::Literal(Box::new(Literal::Null)),
            AnyTsType::TsNumberLiteralType(ty) => {
                let Ok(literal_token) = ty.literal_token() else {
                    return Self::unknown();
                };

                let Some(lit) = NumberLiteral::parse(literal_token.text_trimmed()) else {
                    return Self::unknown();
                };

                Literal::Number(match ty.minus_token() {
                    Some(_) => lit.inverse(),
                    _ => lit,
                })
                .into()
            }
            AnyTsType::TsNumberType(_) => Self::Number,
            AnyTsType::TsObjectType(ty) => Self::object_with_members(
                ty.members()
                    .into_iter()
                    .filter_map(|member| TypeMember::from_any_ts_type_member(resolver, &member))
                    .collect(),
            ),
            AnyTsType::TsParenthesizedType(ty) => ty
                .ty()
                .map(|ty| Self::from_any_ts_type(resolver, &ty))
                .unwrap_or_default(),
            AnyTsType::TsReferenceType(ty) => Self::from_ts_reference_type(resolver, ty),
            AnyTsType::TsStringLiteralType(ty) => match ty.inner_string_text() {
                Ok(token) => Literal::String(token.text().into()).into(),
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
                    .map(|el| {
                        el.map(|el| TupleElementType::from_any_ts_tuple_type_element(resolver, &el))
                    })
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
                        ty: TypeReference::from_any_ts_type(resolver, &ty),
                    }))
                }),
                _ => Self::Unknown,
            },
            AnyTsType::TsTypeofType(ty) => Self::from_ts_typeof_type(resolver, ty),
            AnyTsType::TsUndefinedType(_) => Self::Undefined,
            AnyTsType::TsUnionType(ty) => Self::Union(Box::new(Union(
                ty.types()
                    .into_iter()
                    .filter_map(|ty| ty.ok())
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .collect(),
            ))),
            AnyTsType::TsUnknownType(_) => Self::UnknownKeyword,
            AnyTsType::TsVoidType(_) => Self::VoidKeyword,
        }
    }

    pub fn from_any_ts_type_result(
        resolver: &mut dyn TypeResolver,
        ty: SyntaxResult<AnyTsType>,
    ) -> Self {
        ty.map(|ty| Self::from_any_ts_type(resolver, &ty))
            .unwrap_or_default()
    }

    pub fn from_js_arrow_function_expression(
        resolver: &mut dyn TypeResolver,
        expr: &JsArrowFunctionExpression,
    ) -> Self {
        Self::Function(Box::new(Function {
            is_async: expr.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(resolver, expr.type_parameters()),
            name: None,
            parameters: match expr.parameters() {
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(binding)) => {
                    let name = binding
                        .as_js_identifier_binding()
                        .and_then(|binding| text_from_token(binding.name_token()));
                    Box::new([FunctionParameter {
                        bindings: name
                            .iter()
                            .map(|name| FunctionParameterBinding {
                                name: name.clone(),
                                ty: Self::unknown(),
                            })
                            .collect(),
                        name,
                        ty: TypeReference::Unknown,
                        is_optional: false,
                        is_rest: false,
                    }])
                }
                Ok(AnyJsArrowFunctionParameters::JsParameters(params)) => {
                    function_params_from_js_params(resolver, Ok(params))
                }
                Err(_) => Box::default(),
            },
            return_type: return_type_from_annotation(resolver, expr.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(resolver, expr.async_token())),
        }))
    }

    pub fn from_js_binary_expression(
        resolver: &mut dyn TypeResolver,
        expr: &JsBinaryExpression,
    ) -> Self {
        let (Ok(left), Ok(operator), Ok(right)) = (expr.left(), expr.operator(), expr.right())
        else {
            return Self::unknown();
        };

        let left = Self::from_any_js_expression(resolver, &left);
        let right = Self::from_any_js_expression(resolver, &right);

        match operator {
            JsBinaryOperator::StrictEquality => match (left, right) {
                (Self::Literal(left), Self::Literal(right)) => {
                    Literal::Boolean((left == right).into()).into()
                }
                _ => Self::boolean(),
            },
            JsBinaryOperator::StrictInequality => match (left, right) {
                (Self::Literal(left), Self::Literal(right)) => {
                    Literal::Boolean((left != right).into()).into()
                }
                _ => Self::boolean(),
            },
            _ => Self::unknown(), // TODO
        }
    }

    pub fn from_js_class_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &JsClassDeclaration,
    ) -> Self {
        Self::Class(Box::new(Class {
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|id| id.as_js_identifier_binding())
                .and_then(|id| id.name_token().ok())
                .map(|token| token.token_text_trimmed().into()),
            type_parameters: decl
                .type_parameters()
                .map(|params| {
                    GenericTypeParameter::params_from_ts_type_parameters(resolver, &params)
                })
                .unwrap_or_default(),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| TypeReference::from_any_js_expression(resolver, &super_class)),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| TypeMember::from_any_js_class_member(resolver, &member))
                .collect(),
        }))
    }

    pub fn from_js_class_expression(
        resolver: &mut dyn TypeResolver,
        decl: &JsClassExpression,
    ) -> Self {
        Self::Class(Box::new(Class {
            name: decl
                .id()
                .as_ref()
                .and_then(|id| id.as_js_identifier_binding())
                .and_then(|id| id.name_token().ok())
                .map(|token| token.token_text_trimmed().into()),
            type_parameters: decl
                .type_parameters()
                .map(|params| {
                    GenericTypeParameter::params_from_ts_type_parameters(resolver, &params)
                })
                .unwrap_or_default(),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| TypeReference::from_any_js_expression(resolver, &super_class)),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| TypeMember::from_any_js_class_member(resolver, &member))
                .collect(),
        }))
    }

    pub fn from_js_function_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &JsFunctionDeclaration,
    ) -> Self {
        Self::Function(Box::new(Function {
            is_async: decl.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(resolver, decl.type_parameters()),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(resolver, decl.parameters()),
            return_type: return_type_from_annotation(resolver, decl.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(resolver, decl.async_token())),
        }))
    }

    pub fn from_js_function_expression(
        resolver: &mut dyn TypeResolver,
        expr: &JsFunctionExpression,
    ) -> Self {
        Self::Function(Box::new(Function {
            is_async: expr.async_token().is_some(),
            type_parameters: generic_params_from_ts_type_params(resolver, expr.type_parameters()),
            name: expr
                .id()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(resolver, expr.parameters()),
            return_type: return_type_from_annotation(resolver, expr.return_type_annotation())
                .unwrap_or_else(|| return_type_from_async_token(resolver, expr.async_token())),
        }))
    }

    pub fn from_js_new_expression(
        resolver: &mut dyn TypeResolver,
        expr: &JsNewExpression,
    ) -> Option<Self> {
        Some(Self::TypeofExpression(Box::new(TypeofExpression::New(
            TypeofNewExpression {
                callee: TypeReference::from_any_js_expression(resolver, &expr.callee().ok()?),
                arguments: CallArgumentType::types_from_js_call_arguments(
                    resolver,
                    expr.arguments(),
                ),
            },
        ))))
    }

    pub fn from_js_object_expression(
        resolver: &mut dyn TypeResolver,
        expr: &JsObjectExpression,
    ) -> Self {
        Self::object_with_members(
            expr.members()
                .into_iter()
                .filter_map(|member| TypeMember::from_any_js_object_member(resolver, &member.ok()?))
                .collect(),
        )
    }

    pub fn from_js_reference_identifier(id: &JsReferenceIdentifier) -> Self {
        if id.is_undefined() {
            Self::Undefined
        } else {
            id.name()
                .map(|name| Self::Reference(Box::new(TypeReference::from_name(name))))
                .unwrap_or_default()
        }
    }

    pub fn from_js_variable_declarator(
        resolver: &mut dyn TypeResolver,
        decl: &JsVariableDeclarator,
    ) -> Option<Self> {
        let ty = match decl.variable_annotation() {
            Some(annotation) => {
                let data = Self::from_any_ts_type(
                    resolver,
                    &annotation.type_annotation().ok()??.ty().ok()?,
                );
                match data {
                    Self::InstanceOf(type_instance) => Self::InstanceOf(type_instance),
                    _ => Self::instance_of(resolver.reference_to_registered_data(data)),
                }
            }
            None => Self::from_any_js_expression(resolver, &decl.initializer()?.expression().ok()?),
        };

        Some(ty)
    }

    pub fn from_ts_reference_type(resolver: &mut dyn TypeResolver, ty: &TsReferenceType) -> Self {
        ty.name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                Self::instance_of(TypeReference::Qualifier(qualifier.with_type_parameters(
                    TypeReference::types_from_ts_type_arguments(resolver, ty.type_arguments()),
                )))
            })
            .unwrap_or_default()
    }

    pub fn from_ts_type_alias_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &TsTypeAliasDeclaration,
    ) -> Option<Self> {
        Some(match decl.type_parameters() {
            Some(params) => Self::instance_of(TypeInstance {
                ty: TypeReference::from_any_ts_type(resolver, &decl.ty().ok()?),
                type_parameters: GenericTypeParameter::params_from_ts_type_parameters(
                    resolver, &params,
                ),
            }),
            None => Self::from_any_ts_type(resolver, &decl.ty().ok()?),
        })
    }

    pub fn from_ts_typeof_type(resolver: &mut dyn TypeResolver, ty: &TsTypeofType) -> Self {
        ty.expression_name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(&name))
            .map(|qualifier| {
                Self::TypeofType(Box::new(TypeReference::Qualifier(
                    qualifier.with_type_parameters(TypeReference::types_from_ts_type_arguments(
                        resolver,
                        ty.type_arguments(),
                    )),
                )))
            })
            .unwrap_or_default()
    }

    pub fn instance_of(instance: impl Into<TypeInstance>) -> Self {
        Self::InstanceOf(Box::new(instance.into()))
    }

    pub fn object_with_members(members: Box<[TypeMember]>) -> Self {
        Self::Object(Box::new(Object {
            prototype: None,
            members,
        }))
    }

    pub fn promise_of(ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::Qualifier(TypeReferenceQualifier {
            path: Box::new([Text::Static("Promise")]),
            type_parameters: Box::new([ty]),
        }))
    }

    pub fn typed_bindings_from_js_variable_declaration(
        resolver: &mut dyn TypeResolver,
        decl: &JsVariableDeclaration,
    ) -> Box<[(Text, Self)]> {
        decl.declarators()
            .into_iter()
            .filter_map(|decl| decl.ok())
            .filter_map(|decl| Self::typed_bindings_from_js_variable_declarator(resolver, &decl))
            .flatten()
            .collect()
    }

    pub fn typed_bindings_from_js_variable_declarator(
        resolver: &mut dyn TypeResolver,
        decl: &JsVariableDeclarator,
    ) -> Option<Box<[(Text, Self)]>> {
        match decl.id().ok()? {
            AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                let binding = binding.as_js_identifier_binding()?;
                let name_token = binding.name_token().ok()?;
                Box::new([(
                    name_token.token_text_trimmed().into(),
                    Self::from_js_variable_declarator(resolver, decl)?,
                )])
            }),
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                let pattern_ty = Self::from_js_variable_declarator(resolver, decl)?;
                pattern_ty.apply_array_binding_pattern(resolver, &pattern)
            }),
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                let pattern_ty = Self::from_js_variable_declarator(resolver, decl)?;
                pattern_ty.apply_object_binding_pattern(resolver, &pattern)
            }),
        }
    }
}

impl CallArgumentType {
    pub fn types_from_js_call_arguments(
        resolver: &mut dyn TypeResolver,
        arguments: Option<JsCallArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.args()
                    .into_iter()
                    .filter_map(|arg| Some(Self::from_any_js_call_argument(resolver, &arg.ok()?)))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn from_any_js_call_argument(
        resolver: &mut dyn TypeResolver,
        arg: &AnyJsCallArgument,
    ) -> Self {
        match arg {
            AnyJsCallArgument::AnyJsExpression(expr) => {
                Self::Argument(TypeReference::from_any_js_expression(resolver, expr))
            }
            AnyJsCallArgument::JsSpread(spread) => Self::Spread(
                spread
                    .argument()
                    .map(|arg| TypeReference::from_any_js_expression(resolver, &arg))
                    .unwrap_or_default(),
            ),
        }
    }
}

impl FunctionParameter {
    pub fn from_any_js_parameter(resolver: &mut dyn TypeResolver, param: &AnyJsParameter) -> Self {
        match param {
            AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                param,
            )) => Self::from_js_formal_parameter(resolver, param),
            AnyJsParameter::AnyJsFormalParameter(_) => Self {
                name: None,
                ty: TypeReference::Unknown,
                bindings: [].into(),
                is_optional: false,
                is_rest: false,
            },
            AnyJsParameter::JsRestParameter(param) => {
                let ty = param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| TypeData::from_any_ts_type(resolver, &ty))
                    .unwrap_or_default();
                let bindings = param
                    .binding()
                    .ok()
                    .and_then(|binding| {
                        FunctionParameterBinding::bindings_from_any_js_binding_pattern_of_type(
                            resolver, &binding, &ty,
                        )
                    })
                    .unwrap_or_default();
                Self {
                    name: None,
                    ty: resolver.reference_to_registered_data(ty),
                    bindings,
                    is_optional: false,
                    is_rest: true,
                }
            }
            AnyJsParameter::TsThisParameter(param) => Self {
                name: Some(Text::Static("this")),
                ty: param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .unwrap_or_default(),
                bindings: [].into(),
                is_optional: false,
                is_rest: false,
            },
        }
    }

    pub fn from_js_formal_parameter(
        resolver: &mut dyn TypeResolver,
        param: &JsFormalParameter,
    ) -> Self {
        let name = param
            .binding()
            .ok()
            .as_ref()
            .and_then(|binding| binding.as_any_js_binding())
            .and_then(|binding| binding.as_js_identifier_binding())
            .and_then(|identifier| identifier.name_token().ok())
            .map(|token| token.token_text_trimmed().into());
        let ty = param
            .type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .map(|ty| TypeData::from_any_ts_type(resolver, &ty))
            .unwrap_or_default();
        let bindings = param
            .binding()
            .ok()
            .and_then(|binding| {
                FunctionParameterBinding::bindings_from_any_js_binding_pattern_of_type(
                    resolver, &binding, &ty,
                )
            })
            .unwrap_or_default();
        Self {
            name,
            ty: resolver.reference_to_registered_data(ty),
            bindings,
            is_optional: param.question_mark_token().is_some(),
            is_rest: false,
        }
    }

    pub fn params_from_js_parameters(
        resolver: &mut dyn TypeResolver,
        params: &JsParameters,
    ) -> Box<[Self]> {
        params
            .as_fields()
            .items
            .into_iter()
            .filter_map(|param| {
                param
                    .ok()
                    .map(|param| Self::from_any_js_parameter(resolver, &param))
            })
            .collect()
    }
}

impl From<(Text, TypeData)> for FunctionParameterBinding {
    fn from((name, ty): (Text, TypeData)) -> Self {
        Self { name, ty }
    }
}

impl FunctionParameterBinding {
    pub fn bindings_from_any_js_binding_pattern_of_type(
        resolver: &mut dyn TypeResolver,
        pattern: &AnyJsBindingPattern,
        ty: &TypeData,
    ) -> Option<Box<[Self]>> {
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                let binding = binding.as_js_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                Some(Box::new([Self {
                    name,
                    ty: ty.clone(),
                }]))
            }
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some(
                ty.apply_array_binding_pattern(resolver, pattern)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some(
                ty.apply_object_binding_pattern(resolver, pattern)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
        }
    }
}

impl GenericTypeParameter {
    pub fn from_ts_type_parameter(
        resolver: &mut dyn TypeResolver,
        param: &TsTypeParameter,
    ) -> Option<Self> {
        param
            .name()
            .and_then(|name| name.ident_token())
            .map(|name| Self {
                name: name.token_text_trimmed().into(),
                ty: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map(|default_ty| TypeReference::from_any_ts_type(resolver, &default_ty))
                    .unwrap_or_default(),
            })
            .ok()
    }

    pub fn params_from_ts_type_parameters(
        resolver: &mut dyn TypeResolver,
        params: &TsTypeParameters,
    ) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .filter_map(|param| param.ok())
            .filter_map(|param| Self::from_ts_type_parameter(resolver, &param))
            .collect()
    }
}

impl ReturnType {
    pub fn from_any_ts_return_type(
        resolver: &mut dyn TypeResolver,
        ty: &AnyTsReturnType,
    ) -> Option<Self> {
        match ty {
            AnyTsReturnType::AnyTsType(ty) => {
                Some(Self::Type(TypeReference::from_any_ts_type(resolver, ty)))
            }
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
                            .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
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
                            .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                            .unwrap_or_default(),
                    }))
                })
            }
        }
    }
}

impl TupleElementType {
    pub fn from_any_ts_tuple_type_element(
        resolver: &mut dyn TypeResolver,
        el: &AnyTsTupleTypeElement,
    ) -> Self {
        match el {
            AnyTsTupleTypeElement::AnyTsType(ty) => Self {
                ty: TypeReference::from_any_ts_type(resolver, ty),
                name: None,
                is_optional: false,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsNamedTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
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
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: true,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsRestTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: false,
                is_rest: true,
            },
        }
    }
}

impl TypeMember {
    pub fn from_any_js_class_member(
        resolver: &mut dyn TypeResolver,
        member: &AnyJsClassMember,
    ) -> Option<Self> {
        match member {
            AnyJsClassMember::JsMethodClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Method(MethodTypeMember {
                        is_async: member.async_token().is_some(),
                        type_parameters: generic_params_from_ts_type_params(
                            resolver,
                            member.type_parameters(),
                        ),
                        name: text_from_class_member_name(name),
                        parameters: function_params_from_js_params(resolver, member.parameters()),
                        return_type: return_type_from_annotation(
                            resolver,
                            member.return_type_annotation(),
                        )
                        .unwrap_or_else(|| {
                            return_type_from_async_token(resolver, member.async_token())
                        }),
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
                        ty: match member
                            .property_annotation()
                            .and_then(|annotation| annotation.type_annotation().ok())
                            .flatten()
                            .and_then(|annotation| annotation.ty().ok())
                        {
                            Some(ty) => TypeReference::from_any_ts_type(resolver, &ty),
                            None => member
                                .value()
                                .and_then(|initializer| initializer.expression().ok())
                                .map(|expr| TypeReference::from_any_js_expression(resolver, &expr))
                                .unwrap_or_default(),
                        },
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
                        ty: TypeReference::from_any_js_expression(
                            resolver,
                            &member.value().ok()?.expression().ok()?,
                        ),
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
                            .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
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

    pub fn from_any_js_object_member(
        resolver: &mut dyn TypeResolver,
        member: &AnyJsObjectMember,
    ) -> Option<Self> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(_) => {
                // TODO: Handle getters
                None
            }
            AnyJsObjectMember::JsMethodObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Method(MethodTypeMember {
                        is_async: member.async_token().is_some(),
                        type_parameters: generic_params_from_ts_type_params(
                            resolver,
                            member.type_parameters(),
                        ),
                        name: name.into(),
                        parameters: function_params_from_js_params(resolver, member.parameters()),
                        return_type: return_type_from_annotation(
                            resolver,
                            member.return_type_annotation(),
                        )
                        .unwrap_or_else(|| {
                            return_type_from_async_token(resolver, member.async_token())
                        }),
                        is_optional: false,
                        is_static: false,
                    })
                })
            }
            AnyJsObjectMember::JsPropertyObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    Self::Property(PropertyTypeMember {
                        name: name.into(),
                        ty: member
                            .value()
                            .map(|value| TypeReference::from_any_js_expression(resolver, &value))
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
                    Self::Property(PropertyTypeMember {
                        name: name.clone(),
                        ty: resolver.reference_to_registered_data(TypeData::TypeofValue(Box::new(
                            TypeofValue {
                                identifier: name,
                                ty: TypeReference::Unknown,
                            },
                        ))),
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

    pub fn from_any_ts_type_member(
        resolver: &mut dyn TypeResolver,
        member: &AnyTsTypeMember,
    ) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_) => None,
            AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                Some(Self::CallSignature(CallSignatureTypeMember {
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        member.type_parameters(),
                    ),
                    parameters: function_params_from_js_params(resolver, member.parameters()),
                    return_type: return_type_from_annotation(
                        resolver,
                        member.return_type_annotation(),
                    )
                    .unwrap_or_default(),
                }))
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                Some(Self::Constructor(ConstructorTypeMember {
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        member.type_parameters(),
                    ),
                    parameters: function_params_from_js_params(resolver, member.parameters()),
                    return_type: type_from_annotation(resolver, member.type_annotation()),
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
                            resolver,
                            member.type_parameters(),
                        ),
                        name: name.into(),
                        parameters: function_params_from_js_params(resolver, member.parameters()),
                        return_type: return_type_from_annotation(
                            resolver,
                            member.return_type_annotation(),
                        )
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
                        ty: type_from_annotation(resolver, member.type_annotation())
                            .unwrap_or_default(),
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
    pub fn from_any_js_expression(resolver: &mut dyn TypeResolver, expr: &AnyJsExpression) -> Self {
        let data = TypeData::from_any_js_expression(resolver, expr);
        resolver.reference_to_registered_data(data)
    }

    pub fn from_any_ts_type(resolver: &mut dyn TypeResolver, ty: &AnyTsType) -> Self {
        let data = TypeData::from_any_ts_type(resolver, ty);
        resolver.reference_to_registered_data(data)
    }

    pub fn from_name(name: TokenText) -> Self {
        Self::Qualifier(TypeReferenceQualifier::from_name(name.into()))
    }

    pub fn types_from_ts_type_arguments(
        resolver: &mut dyn TypeResolver,
        arguments: Option<TsTypeArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.ts_type_argument_list()
                    .into_iter()
                    .filter_map(|arg| arg.ok().map(|ty| Self::from_any_ts_type(resolver, &ty)))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl TypeReferenceQualifier {
    pub fn from_any_ts_name(name: &AnyTsName) -> Option<Self> {
        match name {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                text_from_token(identifier.value_token()).map(Self::from_name)
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
                Some(Self {
                    path: identifiers.into(),
                    type_parameters: [].into(),
                })
            }
        }
    }

    pub fn from_name(name: Text) -> Self {
        Self {
            path: Box::new([name]),
            type_parameters: [].into(),
        }
    }

    pub fn with_type_parameters(mut self, params: Box<[TypeReference]>) -> Self {
        self.type_parameters = params;
        self
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
                Some(TypeReferenceQualifier::from_name(name).into())
            })
            .unwrap_or_default();

        Self { parent }
    }
}

fn function_params_from_js_params(
    resolver: &mut dyn TypeResolver,
    params: SyntaxResult<JsParameters>,
) -> Box<[FunctionParameter]> {
    params
        .ok()
        .map(|params| FunctionParameter::params_from_js_parameters(resolver, &params))
        .unwrap_or_default()
}

fn generic_params_from_ts_type_params(
    resolver: &mut dyn TypeResolver,
    params: Option<TsTypeParameters>,
) -> Box<[GenericTypeParameter]> {
    params
        .map(|params| GenericTypeParameter::params_from_ts_type_parameters(resolver, &params))
        .unwrap_or_default()
}

fn return_type_from_annotation(
    resolver: &mut dyn TypeResolver,
    annotation: Option<TsReturnTypeAnnotation>,
) -> Option<ReturnType> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .and_then(|ty| ReturnType::from_any_ts_return_type(resolver, &ty))
}

fn return_type_from_async_token(
    resolver: &mut dyn TypeResolver,
    async_token: Option<JsSyntaxToken>,
) -> ReturnType {
    ReturnType::Type(match async_token {
        Some(_) => {
            resolver.reference_to_registered_data(TypeData::promise_of(TypeReference::Unknown))
        }
        None => TypeReference::Unknown,
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

fn type_from_annotation(
    resolver: &mut dyn TypeResolver,
    annotation: Option<TsTypeAnnotation>,
) -> Option<TypeReference> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .map(|ty| TypeReference::from_any_ts_type(resolver, &ty))
}

fn unescaped_text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(unescape_js_string(inner_string_text(&token.ok()?)))
}
