use std::borrow::Cow;
use std::str::FromStr;

use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsClassMember, AnyJsDeclaration,
    AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsFormalParameter,
    AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsName, AnyJsObjectBindingPatternMember,
    AnyJsObjectMember, AnyJsObjectMemberName, AnyJsParameter, AnyTsModuleName, AnyTsName,
    AnyTsReturnType, AnyTsTupleTypeElement, AnyTsType, AnyTsTypeMember,
    AnyTsTypePredicateParameterName, ClassMemberName, JsArrayBindingPattern,
    JsArrowFunctionExpression, JsBinaryExpression, JsBinaryOperator, JsCallArguments,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression, JsFormalParameter,
    JsFunctionBody, JsFunctionDeclaration, JsFunctionExpression, JsLogicalExpression,
    JsLogicalOperator, JsNewExpression, JsObjectBindingPattern, JsObjectExpression, JsParameters,
    JsReferenceIdentifier, JsReturnStatement, JsSyntaxToken, JsUnaryExpression, JsUnaryOperator,
    JsVariableDeclaration, JsVariableDeclarator, TsDeclareFunctionDeclaration,
    TsExternalModuleDeclaration, TsInterfaceDeclaration, TsModuleDeclaration, TsReferenceType,
    TsReturnTypeAnnotation, TsTypeAliasDeclaration, TsTypeAnnotation, TsTypeArguments, TsTypeList,
    TsTypeParameter, TsTypeParameters, TsTypeofType, inner_string_text, unescape_js_string,
};
use biome_rowan::{AstNode, SyntaxResult, Text, TokenText};

use crate::globals::{
    GLOBAL_GLOBAL_ID, GLOBAL_INSTANCEOF_PROMISE_ID, GLOBAL_NUMBER_ID, GLOBAL_STRING_ID,
    GLOBAL_UNDEFINED_ID,
};
use crate::literal::{BooleanLiteral, NumberLiteral, StringLiteral};
use crate::{
    AssertsReturnType, CallArgumentType, Class, Constructor, DestructureField, Function,
    FunctionParameter, FunctionParameterBinding, GLOBAL_UNKNOWN_ID, GenericTypeParameter,
    Interface, Literal, Module, Namespace, Object, PredicateReturnType, ResolvedTypeId, ReturnType,
    ScopeId, Tuple, TupleElementType, TypeData, TypeInstance, TypeMember, TypeMemberKind,
    TypeOperator, TypeOperatorType, TypeReference, TypeReferenceQualifier, TypeResolver,
    TypeofAdditionExpression, TypeofBitwiseNotExpression, TypeofCallExpression,
    TypeofConditionalExpression, TypeofDestructureExpression, TypeofExpression,
    TypeofLogicalAndExpression, TypeofLogicalOrExpression, TypeofNewExpression,
    TypeofNullishCoalescingExpression, TypeofStaticMemberExpression, TypeofThisOrSuperExpression,
    TypeofTypeofExpression, TypeofUnaryMinusExpression, TypeofValue,
};

impl TypeData {
    /// Applies the given `pattern` and returns the named bindings, and their
    /// associated types.
    pub fn apply_array_binding_pattern(
        &self,
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        pattern: &JsArrayBindingPattern,
    ) -> Box<[(Text, TypeReference)]> {
        pattern
            .elements()
            .into_iter()
            .enumerate()
            .filter_map(|(i, elem)| elem.ok().map(|elem| (i, elem)))
            .filter_map(|(i, elem)| {
                self.apply_array_binding_pattern_element(resolver, scope_id, i, elem)
            })
            .flatten()
            .collect()
    }

    fn apply_array_binding_pattern_element(
        &self,
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        i: usize,
        elem: AnyJsArrayBindingPatternElement,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let reference = resolver.reference_to_registered_data(self);
        match elem {
            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(elem) => {
                match elem.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            resolver.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::Index(i),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::Index(i));
                        data.apply_array_binding_pattern(resolver, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::Index(i));
                        data.apply_object_binding_pattern(resolver, scope_id, &pattern)
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
                            resolver.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::RestFrom(i),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::RestFrom(i));
                        data.apply_array_binding_pattern(resolver, scope_id, &pattern)
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
        scope_id: ScopeId,
        pattern: &JsObjectBindingPattern,
    ) -> Box<[(Text, TypeReference)]> {
        // Accumulate names to exclude from the rest operator.
        let mut names = Vec::new();

        pattern
            .properties()
            .into_iter()
            .flatten()
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

                self.apply_object_binding_pattern_member(resolver, scope_id, &names, name, member)
            })
            .flatten()
            .collect()
    }

    fn apply_object_binding_pattern_member(
        &self,
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        names: &[Text],
        member_name: Option<Text>,
        member: AnyJsObjectBindingPatternMember,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let reference = resolver.reference_to_registered_data(self);
        match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(prop) => {
                let member_name = member_name?;
                match prop.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            resolver.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::Name(member_name),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data =
                            Self::destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_array_binding_pattern(resolver, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data =
                            Self::destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_object_binding_pattern(resolver, scope_id, &pattern)
                    }),
                }
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(_) => Some({
                let member_name = member_name?;
                Box::new([(
                    member_name.clone(),
                    resolver.reference_to_owned_data(Self::destructuring_of(
                        reference,
                        DestructureField::Name(member_name),
                    )),
                )])
            }),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => Some({
                let binding = rest.binding().ok()?;
                let binding = binding.as_js_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                Box::new([(
                    name,
                    resolver.reference_to_owned_data(Self::destructuring_of(
                        reference,
                        DestructureField::RestExcept(names.iter().cloned().collect()),
                    )),
                )])
            }),
            AnyJsObjectBindingPatternMember::JsBogusBinding(_)
            | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
        }
    }

    fn destructuring_of(ty: TypeReference, destructure_field: DestructureField) -> Self {
        Self::TypeofExpression(Box::new(TypeofExpression::Destructure(
            TypeofDestructureExpression {
                ty,
                destructure_field,
            },
        )))
    }

    pub fn from_any_js_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &AnyJsDeclaration,
    ) -> Self {
        match decl {
            AnyJsDeclaration::JsClassDeclaration(decl) => {
                Self::from_js_class_declaration(resolver, scope_id, decl)
            }
            AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                Self::from_js_function_declaration(resolver, scope_id, decl)
            }
            AnyJsDeclaration::JsVariableDeclaration(_) => {
                // Variable declarations don't have a type;
                // only their inner declarators have.
                Self::unknown()
            }
            AnyJsDeclaration::TsDeclareFunctionDeclaration(decl) => {
                Self::from_ts_declare_function_declaration(resolver, scope_id, decl)
            }
            AnyJsDeclaration::TsEnumDeclaration(_decl) => {
                // TODO: Handle enum declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsExternalModuleDeclaration(decl) => {
                Self::from_ts_external_module_declaration(decl).unwrap_or_default()
            }
            AnyJsDeclaration::TsGlobalDeclaration(_decl) => {
                // TODO: Handle global declarations.
                Self::unknown()
            }
            AnyJsDeclaration::TsImportEqualsDeclaration(_decl) => {
                // TODO: Handle `import T = Name` syntax.
                Self::unknown()
            }
            AnyJsDeclaration::TsInterfaceDeclaration(decl) => {
                Self::from_ts_interface_declaration(resolver, scope_id, decl).unwrap_or_default()
            }
            AnyJsDeclaration::TsModuleDeclaration(decl) => {
                Self::from_ts_module_declaration(decl).unwrap_or_default()
            }
            AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                Self::from_ts_type_alias_declaration(resolver, scope_id, decl).unwrap_or_default()
            }
        }
    }

    pub fn from_any_js_declaration_clause(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: AnyJsDeclarationClause,
    ) -> Self {
        decl.into_declaration()
            .map(|decl| Self::from_any_js_declaration(resolver, scope_id, &decl))
            .unwrap_or_default()
    }

    pub fn from_any_js_export_default_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
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
                            TypeReference::types_from_ts_type_parameters(
                                resolver, scope_id, &params,
                            )
                        })
                        .unwrap_or_default(),
                    extends: decl
                        .extends_clause()
                        .and_then(|extends| extends.super_class().ok())
                        .map(|super_class| {
                            resolver.reference_to_resolved_expression(scope_id, &super_class)
                        }),
                    implements: decl
                        .implements_clause()
                        .map(|implements| {
                            TypeReference::types_from_ts_type_list(
                                resolver,
                                scope_id,
                                implements.types(),
                            )
                        })
                        .unwrap_or_default(),
                    members: decl
                        .members()
                        .into_iter()
                        .filter_map(|member| {
                            TypeMember::from_any_js_class_member(resolver, scope_id, &member)
                        })
                        .collect(),
                }))
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(decl) => {
                let is_async = decl.async_token().is_some();
                Self::Function(Box::new(Function {
                    is_async,
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        scope_id,
                        decl.type_parameters(),
                    ),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(
                        resolver,
                        scope_id,
                        decl.parameters(),
                    ),
                    return_type: function_return_type(
                        resolver,
                        scope_id,
                        is_async,
                        decl.return_type_annotation(),
                        decl.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                    ),
                }))
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(decl) => {
                let is_async = decl.async_token().is_some();
                Self::Function(Box::new(Function {
                    is_async,
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        scope_id,
                        decl.type_parameters(),
                    ),
                    name: decl
                        .id()
                        .as_ref()
                        .and_then(|id| id.as_js_identifier_binding())
                        .and_then(|id| id.name_token().ok())
                        .map(|token| token.token_text_trimmed().into()),
                    parameters: function_params_from_js_params(
                        resolver,
                        scope_id,
                        decl.parameters(),
                    ),
                    return_type: function_return_type(
                        resolver,
                        scope_id,
                        is_async,
                        decl.return_type_annotation(),
                        None,
                    ),
                }))
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(_decl) => {
                // TODO: Handle interface declarations.
                Self::unknown()
            }
        }
    }

    pub fn from_any_js_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Self {
        match expr {
            AnyJsExpression::AnyJsLiteralExpression(expr) => {
                Self::from_any_js_literal_expression(expr).unwrap_or_default()
            }
            AnyJsExpression::JsArrayExpression(expr) => Self::Tuple(Box::new(Tuple(
                expr.elements()
                    .into_iter()
                    .filter_map(|el| match el {
                        Ok(AnyJsArrayElement::AnyJsExpression(expr)) => Some(TupleElementType {
                            ty: resolver.reference_to_resolved_expression(scope_id, &expr),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        }),
                        Ok(AnyJsArrayElement::JsSpread(spread)) => spread
                            .argument()
                            .ok()
                            .map(|expr| resolver.reference_to_resolved_expression(scope_id, &expr))
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
                Self::from_js_arrow_function_expression(resolver, scope_id, expr)
            }
            AnyJsExpression::JsBinaryExpression(expr) => {
                Self::from_js_binary_expression(resolver, scope_id, expr)
            }
            AnyJsExpression::JsCallExpression(expr) => match expr.callee() {
                Ok(callee) => Self::from(TypeofExpression::Call(TypeofCallExpression {
                    callee: resolver.reference_to_resolved_expression(scope_id, &callee),
                    arguments: CallArgumentType::types_from_js_call_arguments(
                        resolver,
                        scope_id,
                        expr.arguments().ok(),
                    ),
                })),
                Err(_) => Self::unknown(),
            },
            AnyJsExpression::JsClassExpression(expr) => {
                Self::from_js_class_expression(resolver, scope_id, expr)
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
                            Self::from(TypeofExpression::StaticMember(
                                TypeofStaticMemberExpression {
                                    object: resolver
                                        .reference_to_resolved_expression(scope_id, &object),
                                    member,
                                },
                            ))
                        })
                        .unwrap_or_default(),
                    _ => Self::unknown(),
                }
            }
            AnyJsExpression::JsConditionalExpression(expr) => {
                Self::from(TypeofExpression::Conditional(TypeofConditionalExpression {
                    test: expr
                        .test()
                        .map(|sub| resolver.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                    consequent: expr
                        .consequent()
                        .map(|sub| resolver.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                    alternate: expr
                        .alternate()
                        .map(|sub| resolver.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                }))
            }
            AnyJsExpression::JsFunctionExpression(expr) => {
                Self::from_js_function_expression(resolver, scope_id, expr)
            }
            AnyJsExpression::JsIdentifierExpression(expr) => expr
                .name()
                .map(|name| Self::from_js_reference_identifier(scope_id, &name))
                .unwrap_or_default(),
            AnyJsExpression::JsImportCallExpression(_expr) => {
                Self::reference(GLOBAL_INSTANCEOF_PROMISE_ID)
            }
            AnyJsExpression::JsInstanceofExpression(_expr) => Self::Boolean,
            AnyJsExpression::JsLogicalExpression(expr) => {
                Self::from_js_logical_expression(resolver, scope_id, expr).unwrap_or_default()
            }
            AnyJsExpression::JsNewExpression(expr) => {
                Self::from_js_new_expression(resolver, scope_id, expr).unwrap_or_default()
            }
            AnyJsExpression::JsObjectExpression(expr) => Self::object_with_members(
                expr.members()
                    .into_iter()
                    .flatten()
                    .filter_map(|member| {
                        TypeMember::from_any_js_object_member(resolver, scope_id, &member)
                    })
                    .collect(),
            ),
            AnyJsExpression::JsParenthesizedExpression(expr) => expr
                .expression()
                .map(|expr| resolver.resolve_expression(scope_id, &expr).into_owned())
                .unwrap_or_default(),
            AnyJsExpression::JsPostUpdateExpression(_)
            | AnyJsExpression::JsPreUpdateExpression(_) => Self::number(),
            AnyJsExpression::JsSequenceExpression(expr) => expr
                .right()
                .map(|expr| resolver.resolve_expression(scope_id, &expr).into_owned())
                .unwrap_or_default(),
            AnyJsExpression::JsStaticMemberExpression(expr) => match (expr.object(), expr.member())
            {
                (Ok(object), Ok(member)) => text_from_any_js_name(member)
                    .map(|member| {
                        Self::from(TypeofExpression::StaticMember(
                            TypeofStaticMemberExpression {
                                object: resolver
                                    .reference_to_resolved_expression(scope_id, &object),
                                member,
                            },
                        ))
                    })
                    .unwrap_or_default(),
                _ => Self::unknown(),
            },
            AnyJsExpression::JsSuperExpression(_) => Self::from(TypeofExpression::Super(
                TypeofThisOrSuperExpression::from_any_js_expression(scope_id, expr),
            )),
            AnyJsExpression::JsThisExpression(_) => Self::from(TypeofExpression::This(
                TypeofThisOrSuperExpression::from_any_js_expression(scope_id, expr),
            )),
            AnyJsExpression::JsUnaryExpression(expr) => {
                Self::from_js_unary_expression(resolver, scope_id, expr)
            }
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
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => return Some(Self::Null),
            AnyJsLiteralExpression::JsNumberLiteralExpression(expr) => {
                Literal::Number(NumberLiteral::new(text_from_token(expr.value_token())?))
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(expr) => {
                Literal::RegExp(text_from_token(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(expr) => Literal::String(
                StringLiteral::from(Text::Borrowed(expr.inner_string_text().ok()?)),
            ),
        };

        Some(Self::Literal(Box::new(literal)))
    }

    pub fn from_any_ts_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &AnyTsType,
    ) -> Self {
        match ty {
            AnyTsType::JsMetavariable(_) => Self::unknown(),
            AnyTsType::TsAnyType(_) => Self::AnyKeyword,
            AnyTsType::TsArrayType(ty) => Self::array_of(
                scope_id,
                ty.element_type()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .unwrap_or_default(),
            ),
            AnyTsType::TsBigintLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    Text::Owned(format!("{minus_token}{literal_token}")),
                ))),
                (None, Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => Self::unknown(),
            },
            AnyTsType::TsBigintType(_) => Self::BigInt,
            AnyTsType::TsBogusType(_) => Self::unknown(),
            AnyTsType::TsBooleanLiteralType(ty) => match ty.literal() {
                Ok(token) => Self::Literal(Box::new(Literal::Boolean(
                    BooleanLiteral::parse(token.text_trimmed()).unwrap(),
                ))),
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsBooleanType(_) => Self::Boolean,
            AnyTsType::TsConditionalType(ty) => {
                // We don't attempt to evaluate the condition, so we simply
                // infer a union of both the possibilities.
                let types = Box::new([
                    ty.true_type()
                        .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                        .unwrap_or_default(),
                    ty.false_type()
                        .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                        .unwrap_or_default(),
                ]);

                Self::union_of(resolver, types)
            }
            AnyTsType::TsConstructorType(ty) => Self::Constructor(Box::new(Constructor {
                type_parameters: generic_params_from_ts_type_params(
                    resolver,
                    scope_id,
                    ty.type_parameters(),
                ),
                parameters: function_params_from_js_params(resolver, scope_id, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty)),
            })),
            AnyTsType::TsFunctionType(ty) => Self::Function(Box::new(Function {
                is_async: false,
                type_parameters: generic_params_from_ts_type_params(
                    resolver,
                    scope_id,
                    ty.type_parameters(),
                ),
                name: None,
                parameters: function_params_from_js_params(resolver, scope_id, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .and_then(|ty| ReturnType::from_any_ts_return_type(resolver, scope_id, &ty))
                    .unwrap_or_default(),
            })),
            AnyTsType::TsImportType(_) => {
                // TODO: Handle import types (`import("./module").T`).
                Self::unknown()
            }
            AnyTsType::TsIndexedAccessType(_) => {
                // TODO: Handle type indexing (`T[U]`).
                Self::unknown()
            }
            AnyTsType::TsInferType(_) => {
                // TODO: Handle `infer T` syntax.
                Self::unknown()
            }
            AnyTsType::TsIntersectionType(ty) => Self::intersection_of(
                ty.types()
                    .into_iter()
                    .flatten()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .collect(),
            ),
            AnyTsType::TsMappedType(_) => {
                // TODO: Handle mapped types (`type T<U> = { [K in keyof U]: V }`).
                Self::unknown()
            }
            AnyTsType::TsNeverType(_) => Self::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => Self::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => Self::Null,
            AnyTsType::TsNumberLiteralType(ty) => {
                if ty.literal_token().is_err() {
                    return Self::unknown();
                }

                Literal::Number(NumberLiteral::new(ty.to_trimmed_text())).into()
            }
            AnyTsType::TsNumberType(_) => Self::reference(GLOBAL_NUMBER_ID),
            AnyTsType::TsObjectType(ty) => Self::object_with_members(
                ty.members()
                    .into_iter()
                    .filter_map(|member| {
                        TypeMember::from_any_ts_type_member(resolver, scope_id, &member)
                    })
                    .collect(),
            ),
            AnyTsType::TsParenthesizedType(ty) => ty
                .ty()
                .map(|ty| Self::from_any_ts_type(resolver, scope_id, &ty))
                .unwrap_or_default(),
            AnyTsType::TsReferenceType(ty) => Self::from_ts_reference_type(resolver, scope_id, ty),
            AnyTsType::TsStringLiteralType(ty) => match ty.inner_string_text() {
                Ok(token) => Literal::String(token.text().into()).into(),
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsStringType(_) => Self::reference(GLOBAL_STRING_ID),
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
                        el.map(|el| {
                            TupleElementType::from_any_ts_tuple_type_element(
                                resolver, scope_id, &el,
                            )
                        })
                    })
                    .collect();
                match elements {
                    Ok(elements) => Self::Tuple(Box::new(Tuple(elements))),
                    Err(_) => Self::unknown(),
                }
            }
            AnyTsType::TsTypeOperatorType(ty) => match (ty.operator_token(), ty.ty()) {
                (Ok(operator_token), Ok(ty)) => TypeOperator::from_str(
                    operator_token.text_trimmed(),
                )
                .map_or(Self::unknown(), |operator| {
                    Self::TypeOperator(Box::new(TypeOperatorType {
                        operator,
                        ty: TypeReference::from_any_ts_type(resolver, scope_id, &ty),
                    }))
                }),
                _ => Self::unknown(),
            },
            AnyTsType::TsTypeofType(ty) => Self::from_ts_typeof_type(resolver, scope_id, ty),
            AnyTsType::TsUndefinedType(_) => Self::Undefined,
            AnyTsType::TsUnionType(ty) => {
                let types = ty
                    .types()
                    .into_iter()
                    .flatten()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .collect();

                Self::union_of(resolver, types)
            }
            AnyTsType::TsUnknownType(_) => Self::UnknownKeyword,
            AnyTsType::TsVoidType(_) => Self::VoidKeyword,
        }
    }

    pub fn from_any_ts_type_result(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: SyntaxResult<AnyTsType>,
    ) -> Self {
        ty.map(|ty| Self::from_any_ts_type(resolver, scope_id, &ty))
            .unwrap_or_default()
    }

    pub fn from_js_arrow_function_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsArrowFunctionExpression,
    ) -> Self {
        let is_async = expr.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                expr.type_parameters(),
            ),
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
                                ty: TypeReference::Unknown,
                            })
                            .collect(),
                        name,
                        ty: TypeReference::Unknown,
                        is_optional: false,
                        is_rest: false,
                    }])
                }
                Ok(AnyJsArrowFunctionParameters::JsParameters(params)) => {
                    function_params_from_js_params(resolver, scope_id, Ok(params))
                }
                Err(_) => Box::default(),
            },
            return_type: function_return_type(
                resolver,
                scope_id,
                is_async,
                expr.return_type_annotation(),
                expr.body().ok(),
            ),
        }))
    }

    pub fn from_js_binary_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsBinaryExpression,
    ) -> Self {
        let (Ok(left), Ok(operator), Ok(right)) = (expr.left(), expr.operator(), expr.right())
        else {
            return Self::unknown();
        };

        let left = resolver.resolve_expression(scope_id, &left).into_owned();
        let right = resolver.resolve_expression(scope_id, &right);

        match operator {
            JsBinaryOperator::BitwiseAnd
            | JsBinaryOperator::BitwiseOr
            | JsBinaryOperator::BitwiseXor
            | JsBinaryOperator::Divide
            | JsBinaryOperator::Exponent
            | JsBinaryOperator::LeftShift
            | JsBinaryOperator::Minus
            | JsBinaryOperator::Times
            | JsBinaryOperator::Remainder
            | JsBinaryOperator::RightShift
            | JsBinaryOperator::UnsignedRightShift => Self::number(),
            JsBinaryOperator::Equality => match (left, right.as_ref()) {
                (Self::Literal(left), Self::Literal(right)) if left == *right => {
                    Literal::Boolean(true.into()).into()
                }
                _ => Self::boolean(),
            },
            JsBinaryOperator::GreaterThan
            | JsBinaryOperator::GreaterThanOrEqual
            | JsBinaryOperator::LessThan
            | JsBinaryOperator::LessThanOrEqual => Self::boolean(),
            JsBinaryOperator::Inequality => match (left, right.as_ref()) {
                (Self::Literal(left), Self::Literal(right)) if left == *right => {
                    Literal::Boolean(false.into()).into()
                }
                _ => Self::boolean(),
            },
            JsBinaryOperator::Plus => {
                let right = right.into_owned();
                Self::from(TypeofExpression::Addition(TypeofAdditionExpression {
                    left: resolver.reference_to_owned_data(left),
                    right: resolver.reference_to_owned_data(right),
                }))
            }
            JsBinaryOperator::StrictEquality => match (left, right.as_ref()) {
                (Self::Literal(left), Self::Literal(right)) => {
                    Literal::Boolean((left == *right).into()).into()
                }
                _ => Self::boolean(),
            },
            JsBinaryOperator::StrictInequality => match (left, right.as_ref()) {
                (Self::Literal(left), Self::Literal(right)) => {
                    Literal::Boolean((left != *right).into()).into()
                }
                _ => Self::boolean(),
            },
        }
    }

    pub fn from_js_class_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
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
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| {
                    resolver.reference_to_resolved_expression(scope_id, &super_class)
                }),
            implements: decl
                .implements_clause()
                .map(|implements| {
                    TypeReference::types_from_ts_type_list(resolver, scope_id, implements.types())
                })
                .unwrap_or_default(),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| {
                    TypeMember::from_any_js_class_member(resolver, scope_id, &member)
                })
                .collect(),
        }))
    }

    pub fn from_js_class_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &JsClassExpression,
    ) -> Self {
        Self::Class(Box::new(Class {
            name: decl
                .id()
                .as_ref()
                .and_then(|id| id.as_js_identifier_binding())
                .and_then(|id| id.name_token().ok())
                .map(|token| token.token_text_trimmed().into()),
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .and_then(|extends| extends.super_class().ok())
                .map(|super_class| {
                    resolver.reference_to_resolved_expression(scope_id, &super_class)
                }),
            implements: decl
                .implements_clause()
                .map(|implements| {
                    TypeReference::types_from_ts_type_list(resolver, scope_id, implements.types())
                })
                .unwrap_or_default(),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| {
                    TypeMember::from_any_js_class_member(resolver, scope_id, &member)
                })
                .collect(),
        }))
    }

    pub fn from_js_function_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &JsFunctionDeclaration,
    ) -> Self {
        let is_async = decl.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                decl.type_parameters(),
            ),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(resolver, scope_id, decl.parameters()),
            return_type: function_return_type(
                resolver,
                scope_id,
                is_async,
                decl.return_type_annotation(),
                decl.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
            ),
        }))
    }

    pub fn from_js_function_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsFunctionExpression,
    ) -> Self {
        let is_async = expr.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                expr.type_parameters(),
            ),
            name: expr
                .id()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(resolver, scope_id, expr.parameters()),
            return_type: function_return_type(
                resolver,
                scope_id,
                is_async,
                expr.return_type_annotation(),
                expr.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
            ),
        }))
    }

    pub fn from_js_logical_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsLogicalExpression,
    ) -> Option<Self> {
        let left = expr
            .left()
            .map(|left| Self::from_any_js_expression(resolver, scope_id, &left))
            .map(|left| resolver.reference_to_owned_data(left))
            .ok()?;
        let right = expr
            .right()
            .map(|right| Self::from_any_js_expression(resolver, scope_id, &right))
            .map(|right| resolver.reference_to_owned_data(right))
            .ok()?;

        match expr.operator().ok()? {
            JsLogicalOperator::LogicalAnd => Some(Self::from(TypeofExpression::LogicalAnd(
                TypeofLogicalAndExpression { left, right },
            ))),
            JsLogicalOperator::LogicalOr => Some(Self::from(TypeofExpression::LogicalOr(
                TypeofLogicalOrExpression { left, right },
            ))),
            JsLogicalOperator::NullishCoalescing => {
                Some(Self::from(TypeofExpression::NullishCoalescing(
                    TypeofNullishCoalescingExpression { left, right },
                )))
            }
        }
    }

    pub fn from_js_new_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsNewExpression,
    ) -> Option<Self> {
        Some(Self::from(TypeofExpression::New(TypeofNewExpression {
            callee: resolver.reference_to_resolved_expression(scope_id, &expr.callee().ok()?),
            arguments: CallArgumentType::types_from_js_call_arguments(
                resolver,
                scope_id,
                expr.arguments(),
            ),
        })))
    }

    pub fn from_js_object_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsObjectExpression,
    ) -> Self {
        Self::object_with_members(
            expr.members()
                .into_iter()
                .filter_map(|member| {
                    TypeMember::from_any_js_object_member(resolver, scope_id, &member.ok()?)
                })
                .collect(),
        )
    }

    pub fn from_js_reference_identifier(scope_id: ScopeId, id: &JsReferenceIdentifier) -> Self {
        id.name().map_or(
            Self::Reference(GLOBAL_UNKNOWN_ID.into()),
            |name| match name.text() {
                "globalThis" => Self::reference(GLOBAL_GLOBAL_ID),
                "undefined" => Self::Undefined,
                _ => Self::reference(TypeReference::from_name(scope_id, name)),
            },
        )
    }

    pub fn from_js_unary_expression(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        expr: &JsUnaryExpression,
    ) -> Self {
        expr.operator()
            .map(|operator| match operator {
                JsUnaryOperator::BitwiseNot => {
                    Self::from(TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                        argument: expr
                            .argument()
                            .map(|arg| resolver.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::Delete => Self::Boolean,
                JsUnaryOperator::Minus => {
                    Self::from(TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                        argument: expr
                            .argument()
                            .map(|arg| resolver.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::LogicalNot => Self::Boolean,
                JsUnaryOperator::Plus => Self::Number,
                JsUnaryOperator::Typeof => {
                    Self::from(TypeofExpression::Typeof(TypeofTypeofExpression {
                        argument: expr
                            .argument()
                            .map(|arg| resolver.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::Void => Self::VoidKeyword,
            })
            .unwrap_or_default()
    }

    pub fn from_js_variable_declarator<'a>(
        resolver: &'a mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &JsVariableDeclarator,
    ) -> Option<Cow<'a, Self>> {
        let ty = match decl.variable_annotation() {
            Some(annotation) => {
                let data = Self::from_any_ts_type(
                    resolver,
                    scope_id,
                    &annotation.type_annotation().ok()??.ty().ok()?,
                );
                Cow::Owned(match data {
                    Self::InstanceOf(type_instance) => Self::InstanceOf(type_instance),
                    _ => Self::instance_of(resolver.reference_to_owned_data(data)),
                })
            }
            None => resolver.resolve_expression(scope_id, &decl.initializer()?.expression().ok()?),
        };

        Some(ty)
    }

    pub fn from_ts_declare_function_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &TsDeclareFunctionDeclaration,
    ) -> Self {
        let is_async = decl.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                decl.type_parameters(),
            ),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(resolver, scope_id, decl.parameters()),
            return_type: function_return_type(
                resolver,
                scope_id,
                is_async,
                decl.return_type_annotation(),
                None,
            ),
        }))
    }

    pub fn from_ts_external_module_declaration(decl: &TsExternalModuleDeclaration) -> Option<Self> {
        let module = Module {
            name: text_from_token(decl.source().ok()?.as_js_module_source()?.value_token())?,
            // We don't initialise members of modules during local inference.
            // This is because our semantic model will pick them up during
            // module-level inference, so we add references for them at that
            // time.
            members: Box::new([]),
        };
        Some(module.into())
    }

    pub fn from_ts_interface_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &TsInterfaceDeclaration,
    ) -> Option<Self> {
        Some(Self::from(Interface {
            name: text_from_token(decl.id().ok()?.as_ts_identifier_binding()?.name_token())?,
            type_parameters: generic_params_from_ts_type_params(
                resolver,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .map(|extends| {
                    TypeReference::types_from_ts_type_list(resolver, scope_id, extends.types())
                })
                .unwrap_or_default(),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| {
                    TypeMember::from_any_ts_type_member(resolver, scope_id, &member)
                })
                .collect(),
        }))
    }

    pub fn from_ts_module_declaration(decl: &TsModuleDeclaration) -> Option<Self> {
        let namespace = Namespace {
            path: path_from_any_ts_module_name(decl.name().ok()?)?,
            // We don't initialise members of namespaces during local inference.
            // This is because our semantic model will pick them up during
            // module-level inference, so we add references for them at that
            // time.
            members: Box::new([]),
        };
        Some(namespace.into())
    }

    pub fn from_ts_reference_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &TsReferenceType,
    ) -> Self {
        ty.name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(scope_id, &name))
            .map(|qualifier| {
                Self::instance_of(TypeReference::from(qualifier.with_type_parameters(
                    TypeReference::types_from_ts_type_arguments(
                        resolver,
                        scope_id,
                        ty.type_arguments(),
                    ),
                )))
            })
            .unwrap_or_default()
    }

    pub fn from_ts_type_alias_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &TsTypeAliasDeclaration,
    ) -> Option<Self> {
        Some(match decl.type_parameters() {
            Some(params) => Self::instance_of(TypeInstance {
                ty: TypeReference::from_any_ts_type(resolver, scope_id, &decl.ty().ok()?),
                type_parameters: TypeReference::types_from_ts_type_parameters(
                    resolver, scope_id, &params,
                ),
            }),
            None => Self::from_any_ts_type(resolver, scope_id, &decl.ty().ok()?),
        })
    }

    pub fn from_ts_typeof_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &TsTypeofType,
    ) -> Self {
        ty.expression_name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(scope_id, &name))
            .map(|qualifier| {
                let type_arguments = ty.type_arguments();
                let qualifier = if type_arguments.is_some() {
                    qualifier.without_type_only().with_type_parameters(
                        TypeReference::types_from_ts_type_arguments(
                            resolver,
                            scope_id,
                            type_arguments,
                        ),
                    )
                } else {
                    qualifier.without_type_only()
                };
                Self::TypeofType(Box::new(TypeReference::from(qualifier)))
            })
            .unwrap_or_default()
    }

    pub fn object_with_members(members: Box<[TypeMember]>) -> Self {
        Self::Object(Box::new(Object {
            prototype: None,
            members,
        }))
    }

    pub fn promise_of(scope_id: ScopeId, ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::from(
            TypeReferenceQualifier::from_path(scope_id, [Text::Static("Promise")])
                .with_type_parameters([ty]),
        ))
    }

    pub fn typed_bindings_from_js_variable_declaration(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &JsVariableDeclaration,
    ) -> Box<[(Text, TypeReference)]> {
        decl.declarators()
            .into_iter()
            .flatten()
            .filter_map(|decl| {
                Self::typed_bindings_from_js_variable_declarator(resolver, scope_id, &decl)
            })
            .flatten()
            .collect()
    }

    pub fn typed_bindings_from_js_variable_declarator(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        decl: &JsVariableDeclarator,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        match decl.id().ok()? {
            AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                let binding = binding.as_js_identifier_binding()?;
                let name_token = binding.name_token().ok()?;
                let data =
                    Self::from_js_variable_declarator(resolver, scope_id, decl)?.into_owned();
                Box::new([(
                    name_token.token_text_trimmed().into(),
                    resolver.reference_to_owned_data(data),
                )])
            }),
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                let pattern_ty = Self::from_js_variable_declarator(resolver, scope_id, decl)?;
                pattern_ty
                    .into_owned()
                    .apply_array_binding_pattern(resolver, scope_id, &pattern)
            }),
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                let pattern_ty = Self::from_js_variable_declarator(resolver, scope_id, decl)?;
                pattern_ty
                    .into_owned()
                    .apply_object_binding_pattern(resolver, scope_id, &pattern)
            }),
        }
    }
}

impl CallArgumentType {
    pub fn types_from_js_call_arguments(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        arguments: Option<JsCallArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.args()
                    .into_iter()
                    .flatten()
                    .map(|arg| Self::from_any_js_call_argument(resolver, scope_id, &arg))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn from_any_js_call_argument(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        arg: &AnyJsCallArgument,
    ) -> Self {
        match arg {
            AnyJsCallArgument::AnyJsExpression(expr) => {
                Self::Argument(resolver.reference_to_resolved_expression(scope_id, expr))
            }
            AnyJsCallArgument::JsSpread(spread) => Self::Spread(
                spread
                    .argument()
                    .map(|arg| resolver.reference_to_resolved_expression(scope_id, &arg))
                    .unwrap_or_default(),
            ),
        }
    }
}

impl FunctionParameter {
    pub fn from_any_js_parameter(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        param: &AnyJsParameter,
    ) -> Self {
        match param {
            AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                param,
            )) => Self::from_js_formal_parameter(resolver, scope_id, param),
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
                    .map(|ty| TypeData::from_any_ts_type(resolver, scope_id, &ty))
                    .unwrap_or_default();
                let bindings = param
                    .binding()
                    .ok()
                    .and_then(|binding| {
                        FunctionParameterBinding::bindings_from_any_js_binding_pattern_of_type(
                            resolver, scope_id, &binding, &ty,
                        )
                    })
                    .unwrap_or_default();
                Self {
                    name: None,
                    ty: resolver.reference_to_owned_data(ty),
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
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .unwrap_or_default(),
                bindings: [].into(),
                is_optional: false,
                is_rest: false,
            },
        }
    }

    pub fn from_js_formal_parameter(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
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
            .map(|ty| TypeData::from_any_ts_type(resolver, scope_id, &ty))
            .unwrap_or_default();
        let bindings = param
            .binding()
            .ok()
            .and_then(|binding| {
                FunctionParameterBinding::bindings_from_any_js_binding_pattern_of_type(
                    resolver, scope_id, &binding, &ty,
                )
            })
            .unwrap_or_default();
        Self {
            name,
            ty: resolver.reference_to_owned_data(ty),
            bindings,
            is_optional: param.question_mark_token().is_some(),
            is_rest: false,
        }
    }

    pub fn params_from_js_parameters(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        params: &JsParameters,
    ) -> Box<[Self]> {
        params
            .as_fields()
            .items
            .into_iter()
            .flatten()
            .map(|param| Self::from_any_js_parameter(resolver, scope_id, &param))
            .collect()
    }
}

impl From<(Text, TypeReference)> for FunctionParameterBinding {
    fn from((name, ty): (Text, TypeReference)) -> Self {
        Self { name, ty }
    }
}

impl FunctionParameterBinding {
    pub fn bindings_from_any_js_binding_pattern_of_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        pattern: &AnyJsBindingPattern,
        ty: &TypeData,
    ) -> Option<Box<[Self]>> {
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(binding) => {
                let binding = binding.as_js_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                Some(Box::new([Self {
                    name,
                    ty: resolver.reference_to_registered_data(ty),
                }]))
            }
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some(
                ty.apply_array_binding_pattern(resolver, scope_id, pattern)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some(
                ty.apply_object_binding_pattern(resolver, scope_id, pattern)
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
        scope_id: ScopeId,
        param: &TsTypeParameter,
    ) -> Option<Self> {
        param
            .name()
            .and_then(|name| name.ident_token())
            .map(|name| Self {
                name: name.token_text_trimmed().into(),
                constraint: param
                    .constraint()
                    .and_then(|constraint| constraint.ty().ok())
                    .map(|constraint_ty| {
                        TypeReference::from_any_ts_type(resolver, scope_id, &constraint_ty)
                    })
                    .unwrap_or_default(),
                default: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map(|default_ty| {
                        TypeReference::from_any_ts_type(resolver, scope_id, &default_ty)
                    })
                    .unwrap_or_default(),
            })
            .ok()
    }

    pub fn params_from_ts_type_parameters(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        params: &TsTypeParameters,
    ) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .flatten()
            .filter_map(|param| Self::from_ts_type_parameter(resolver, scope_id, &param))
            .collect()
    }
}

impl ReturnType {
    pub fn from_any_ts_return_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &AnyTsReturnType,
    ) -> Option<Self> {
        match ty {
            AnyTsReturnType::AnyTsType(ty) => Some(Self::Type(TypeReference::from_any_ts_type(
                resolver, scope_id, ty,
            ))),
            AnyTsReturnType::TsAssertsReturnType(ty) => {
                ty.parameter_name().ok().and_then(|parameter_name| {
                    Some(Self::Asserts(Box::new(AssertsReturnType {
                        parameter_name: match parameter_name {
                            AnyTsTypePredicateParameterName::JsReferenceIdentifier(identifier) => {
                                text_from_token(identifier.value_token())?
                            }
                            AnyTsTypePredicateParameterName::TsThisType(_) => Text::Static("text"),
                        },
                        ty: ty
                            .predicate()
                            .and_then(|asserts| asserts.ty().ok())
                            .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                            .unwrap_or_default(),
                    })))
                })
            }
            AnyTsReturnType::TsPredicateReturnType(ty) => {
                ty.parameter_name().ok().and_then(|parameter_name| {
                    Some(Self::Predicate(Box::new(PredicateReturnType {
                        parameter_name: match parameter_name {
                            AnyTsTypePredicateParameterName::JsReferenceIdentifier(identifier) => {
                                text_from_token(identifier.value_token())?
                            }
                            AnyTsTypePredicateParameterName::TsThisType(_) => Text::Static("text"),
                        },
                        ty: ty
                            .ty()
                            .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                            .unwrap_or_default(),
                    })))
                })
            }
        }
    }
}

impl TupleElementType {
    pub fn from_any_ts_tuple_type_element(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        el: &AnyTsTupleTypeElement,
    ) -> Self {
        match el {
            AnyTsTupleTypeElement::AnyTsType(ty) => Self {
                ty: TypeReference::from_any_ts_type(resolver, scope_id, ty),
                name: None,
                is_optional: false,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsNamedTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
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
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: true,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsRestTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
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
        scope_id: ScopeId,
        member: &AnyJsClassMember,
    ) -> Option<Self> {
        match member {
            AnyJsClassMember::JsMethodClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let is_async = member.async_token().is_some();
                    let function = Function {
                        is_async,
                        type_parameters: generic_params_from_ts_type_params(
                            resolver,
                            scope_id,
                            member.type_parameters(),
                        ),
                        name: Some(text_from_class_member_name(name.clone())),
                        parameters: function_params_from_js_params(
                            resolver,
                            scope_id,
                            member.parameters(),
                        ),
                        return_type: function_return_type(
                            resolver,
                            scope_id,
                            is_async,
                            member.return_type_annotation(),
                            member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                        ),
                    };
                    let ty = resolver.register_and_resolve(function.into());
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    Self::from_class_member_info(resolver, name, ty.into(), is_static, false)
                })
            }
            AnyJsClassMember::JsPropertyClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let ty = match member
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok())
                        .flatten()
                        .and_then(|annotation| annotation.ty().ok())
                    {
                        Some(ty) => TypeReference::from_any_ts_type(resolver, scope_id, &ty),
                        None => member
                            .value()
                            .and_then(|initializer| initializer.expression().ok())
                            .map(|expr| resolver.reference_to_resolved_expression(scope_id, &expr))
                            .unwrap_or_default(),
                    };
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member
                        .property_annotation()
                        .as_ref()
                        .and_then(|annotation| annotation.as_ts_optional_property_annotation())
                        .is_some();
                    Self::from_class_member_info(resolver, name, ty, is_static, is_optional)
                })
            }
            AnyJsClassMember::JsGetterClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let name = text_from_class_member_name(name.clone());
                    let function = Function {
                        is_async: false,
                        type_parameters: [].into(),
                        name: Some(name.clone()),
                        parameters: [].into(),
                        return_type: ReturnType::Type(getter_return_type(
                            resolver,
                            scope_id,
                            member.return_type(),
                            member.body().ok(),
                        )),
                    };
                    Self {
                        kind: TypeMemberKind::Getter(name),
                        ty: resolver.reference_to_owned_data(function.into()),
                    }
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(member) => member
                .name()
                .ok()
                .and_then(|name| name.name())
                .and_then(|name| {
                    let ty = resolver.reference_to_resolved_expression(
                        scope_id,
                        &member.value().ok()?.expression().ok()?,
                    );
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member.question_mark_token().is_some();
                    Some(Self::from_class_member_info(
                        resolver,
                        name,
                        ty,
                        is_static,
                        is_optional,
                    ))
                }),
            AnyJsClassMember::TsPropertySignatureClassMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let ty = member
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok())
                        .flatten()
                        .and_then(|annotation| annotation.ty().ok())
                        .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                        .unwrap_or_default();
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member
                        .property_annotation()
                        .as_ref()
                        .and_then(|annotation| annotation.as_ts_optional_property_annotation())
                        .is_some();
                    Self::from_class_member_info(resolver, name, ty, is_static, is_optional)
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
        scope_id: ScopeId,
        member: &AnyJsObjectMember,
    ) -> Option<Self> {
        match member {
            AnyJsObjectMember::JsBogusMember(_) => None,
            AnyJsObjectMember::JsGetterObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let function = Function {
                        is_async: false,
                        type_parameters: [].into(),
                        name: Some(name.clone().into()),
                        parameters: [].into(),
                        return_type: ReturnType::Type(getter_return_type(
                            resolver,
                            scope_id,
                            member.return_type(),
                            member.body().ok(),
                        )),
                    };
                    Self {
                        kind: TypeMemberKind::Getter(name.into()),
                        ty: resolver.register_and_resolve(function.into()).into(),
                    }
                })
            }
            AnyJsObjectMember::JsMethodObjectMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let is_async = member.async_token().is_some();
                    let function = Function {
                        is_async,
                        type_parameters: generic_params_from_ts_type_params(
                            resolver,
                            scope_id,
                            member.type_parameters(),
                        ),
                        name: Some(name.clone().into()),
                        parameters: function_params_from_js_params(
                            resolver,
                            scope_id,
                            member.parameters(),
                        ),
                        return_type: function_return_type(
                            resolver,
                            scope_id,
                            is_async,
                            member.return_type_annotation(),
                            member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                        ),
                    };
                    Self {
                        kind: TypeMemberKind::Named(name.into()),
                        ty: resolver.register_and_resolve(function.into()).into(),
                    }
                })
            }
            AnyJsObjectMember::JsPropertyObjectMember(member) => member
                .name()
                .ok()
                .and_then(|name| name.name())
                .map(|name| Self {
                    kind: TypeMemberKind::Named(name.into()),
                    ty: member
                        .value()
                        .map(|value| resolver.reference_to_resolved_expression(scope_id, &value))
                        .unwrap_or_default(),
                }),
            AnyJsObjectMember::JsSetterObjectMember(_) => {
                // TODO: Handle setters
                None
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => member
                .name()
                .ok()
                .and_then(|name| text_from_token(name.value_token()))
                .map(|name| Self {
                    kind: TypeMemberKind::Named(name.clone()),
                    ty: resolver.reference_to_owned_data(TypeData::from(TypeofValue {
                        identifier: name,
                        ty: TypeReference::Unknown,
                        scope_id: None,
                    })),
                }),
            AnyJsObjectMember::JsSpread(_) => {
                // TODO: Handle spread operator
                None
            }
        }
    }

    pub fn from_any_ts_type_member(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        member: &AnyTsTypeMember,
    ) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_) => None,
            AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                let function = Function {
                    is_async: false,
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        scope_id,
                        member.type_parameters(),
                    ),
                    name: None,
                    parameters: function_params_from_js_params(
                        resolver,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: return_type_from_annotation(
                        resolver,
                        scope_id,
                        member.return_type_annotation(),
                    )
                    .unwrap_or_default(),
                };
                let ty = resolver.register_and_resolve(function.into());
                Some(Self {
                    kind: TypeMemberKind::CallSignature,
                    ty: ty.into(),
                })
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                let constructor = Constructor {
                    type_parameters: generic_params_from_ts_type_params(
                        resolver,
                        scope_id,
                        member.type_parameters(),
                    ),
                    parameters: function_params_from_js_params(
                        resolver,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: type_from_annotation(resolver, scope_id, member.type_annotation()),
                };
                let ty = resolver.register_and_resolve(constructor.into());
                Some(Self {
                    kind: TypeMemberKind::Constructor,
                    ty: ty.into(),
                })
            }
            AnyTsTypeMember::TsGetterSignatureTypeMember(member) => {
                let name = member.name().ok().and_then(|name| name.name())?;
                let function = Function {
                    is_async: false,
                    type_parameters: [].into(),
                    name: Some(name.clone().into()),
                    parameters: [].into(),
                    return_type: ReturnType::Type(getter_return_type(
                        resolver,
                        scope_id,
                        member.type_annotation(),
                        None,
                    )),
                };
                let ty = resolver.register_and_resolve(function.into()).into();
                Some(Self {
                    kind: TypeMemberKind::Getter(name.into()),
                    ty: ResolvedTypeId::new(resolver.level(), resolver.optional(ty)).into(),
                })
            }
            AnyTsTypeMember::TsIndexSignatureTypeMember(member) => {
                let key_ty = member
                    .parameter()
                    .and_then(|parameter| parameter.type_annotation())
                    .and_then(|annotation| annotation.ty())
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .ok()?;
                let value_ty = member
                    .type_annotation()
                    .and_then(|annotation| annotation.ty())
                    .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
                    .ok()?;
                Some(Self {
                    kind: TypeMemberKind::IndexSignature(key_ty),
                    ty: value_ty,
                })
            }
            AnyTsTypeMember::TsMethodSignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let function = Function {
                        is_async: false,
                        type_parameters: generic_params_from_ts_type_params(
                            resolver,
                            scope_id,
                            member.type_parameters(),
                        ),
                        name: Some(name.clone().into()),
                        parameters: function_params_from_js_params(
                            resolver,
                            scope_id,
                            member.parameters(),
                        ),
                        return_type: return_type_from_annotation(
                            resolver,
                            scope_id,
                            member.return_type_annotation(),
                        )
                        .unwrap_or_default(),
                    };
                    let ty = resolver.register_and_resolve(function.into()).into();
                    let is_optional = member.optional_token().is_some();
                    Self::from_name_and_optional_type(resolver, name, ty, is_optional)
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let ty = type_from_annotation(resolver, scope_id, member.type_annotation())
                        .unwrap_or_default();
                    let is_optional = member.optional_token().is_some();
                    Self::from_name_and_optional_type(resolver, name, ty, is_optional)
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_member) => {
                // TODO: Handle setters
                None
            }
        }
    }

    #[inline]
    fn from_class_member_info(
        resolver: &mut dyn TypeResolver,
        name: ClassMemberName,
        ty: TypeReference,
        is_static: bool,
        is_optional: bool,
    ) -> Self {
        let name = text_from_class_member_name(name);
        Self {
            kind: if is_static {
                TypeMemberKind::NamedStatic(name)
            } else {
                TypeMemberKind::Named(name)
            },
            ty: match is_optional {
                true => {
                    let id = resolver.optional(ty);
                    resolver.reference_to_id(id)
                }
                false => ty,
            },
        }
    }

    #[inline]
    fn from_name_and_optional_type(
        resolver: &mut dyn TypeResolver,
        name: TokenText,
        ty: TypeReference,
        is_optional: bool,
    ) -> Self {
        Self {
            kind: TypeMemberKind::Named(name.into()),
            ty: match is_optional {
                true => ResolvedTypeId::new(resolver.level(), resolver.optional(ty)).into(),
                false => ty,
            },
        }
    }
}

impl TypeReference {
    pub fn from_any_ts_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &AnyTsType,
    ) -> Self {
        let data = TypeData::from_any_ts_type(resolver, scope_id, ty);
        resolver.reference_to_owned_data(data)
    }

    pub fn from_name(scope_id: ScopeId, name: TokenText) -> Self {
        Self::from(TypeReferenceQualifier::from_name(scope_id, name.into()))
    }

    pub fn from_ts_reference_type(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        ty: &TsReferenceType,
    ) -> Self {
        let data = TypeData::from_ts_reference_type(resolver, scope_id, ty);
        resolver.reference_to_owned_data(data)
    }

    pub fn types_from_ts_type_arguments(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        arguments: Option<TsTypeArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.ts_type_argument_list()
                    .into_iter()
                    .filter_map(Result::ok)
                    .map(|ty| Self::from_any_ts_type(resolver, scope_id, &ty))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn types_from_ts_type_list(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        types: TsTypeList,
    ) -> Box<[Self]> {
        types
            .into_iter()
            .filter_map(Result::ok)
            .map(|ty| Self::from_ts_reference_type(resolver, scope_id, &ty))
            .collect()
    }

    pub fn types_from_ts_type_parameters(
        resolver: &mut dyn TypeResolver,
        scope_id: ScopeId,
        params: &TsTypeParameters,
    ) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .map(|param| match param {
                Ok(param) => {
                    GenericTypeParameter::from_ts_type_parameter(resolver, scope_id, &param)
                        .map(|generic| resolver.register_and_resolve(TypeData::from(generic)))
                        .map(Self::from)
                        .unwrap_or_default()
                }
                Err(_) => Self::Unknown,
            })
            .collect()
    }
}

impl TypeReferenceQualifier {
    pub fn from_any_ts_name(scope_id: ScopeId, name: &AnyTsName) -> Option<Self> {
        match name {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                text_from_token(identifier.value_token())
                    .map(|name| Self::from_name(scope_id, name).with_type_only())
            }
            AnyTsName::TsQualifiedName(name) => {
                let mut fields = name.as_fields();
                let mut path = Vec::new();
                loop {
                    path.insert(0, text_from_token(fields.right.ok()?.value_token())?);

                    match fields.left.ok()? {
                        AnyTsName::JsReferenceIdentifier(identifier) => {
                            path.insert(0, text_from_token(identifier.value_token())?);
                            break;
                        }
                        AnyTsName::TsQualifiedName(name) => {
                            fields = name.as_fields();
                        }
                    }
                }
                Some(Self::from_path(scope_id, path).with_type_only())
            }
        }
    }

    pub fn from_name(scope_id: ScopeId, name: Text) -> Self {
        Self::from_path(scope_id, [name])
    }

    pub fn from_path(scope_id: ScopeId, path: impl Into<Box<[Text]>>) -> Self {
        Self {
            path: path.into(),
            type_parameters: [].into(),
            scope_id,
            type_only: false,
            excluded_binding_id: None,
        }
    }

    pub fn with_type_only(mut self) -> Self {
        self.type_only = true;
        self
    }

    pub fn with_type_parameters(mut self, params: impl Into<Box<[TypeReference]>>) -> Self {
        self.type_parameters = params.into();
        self
    }

    pub fn without_type_only(mut self) -> Self {
        self.type_only = false;
        self
    }
}

impl TypeofThisOrSuperExpression {
    fn from_any_js_expression(scope_id: ScopeId, expr: &AnyJsExpression) -> Self {
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
                Some(TypeReferenceQualifier::from_name(scope_id, name).into())
            })
            .unwrap_or_default();

        Self { parent }
    }
}

#[inline]
fn function_params_from_js_params(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    params: SyntaxResult<JsParameters>,
) -> Box<[FunctionParameter]> {
    params
        .map(|params| FunctionParameter::params_from_js_parameters(resolver, scope_id, &params))
        .unwrap_or_default()
}

fn function_return_type(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    is_async: bool,
    annotation: Option<TsReturnTypeAnnotation>,
    body: Option<AnyJsFunctionBody>,
) -> ReturnType {
    if let Some(return_ty) = return_type_from_annotation(resolver, scope_id, annotation) {
        return if is_async && return_ty.as_type().is_some_and(|ty| !ty.is_known()) {
            ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into())
        } else {
            return_ty
        };
    }

    let mut return_ty = match body {
        Some(AnyJsFunctionBody::AnyJsExpression(return_expr)) => resolver
            .resolve_expression(scope_id, &return_expr)
            .into_owned(),
        Some(AnyJsFunctionBody::JsFunctionBody(body)) => {
            type_from_function_body(resolver, scope_id, body)
        }
        None => {
            return ReturnType::Type(match is_async {
                true => GLOBAL_INSTANCEOF_PROMISE_ID.into(),
                false => TypeReference::Unknown,
            });
        }
    };

    if is_async {
        return_ty = TypeData::promise_of(scope_id, resolver.reference_to_owned_data(return_ty));
    }

    ReturnType::Type(resolver.reference_to_owned_data(return_ty))
}

fn getter_return_type(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    annotation: Option<TsTypeAnnotation>,
    body: Option<JsFunctionBody>,
) -> TypeReference {
    if let Some(return_ty) = type_from_annotation(resolver, scope_id, annotation) {
        return return_ty;
    }

    let return_ty = match body {
        Some(body) => type_from_function_body(resolver, scope_id, body),
        None => return TypeReference::Unknown,
    };

    resolver.reference_to_owned_data(return_ty)
}

#[inline]
fn generic_params_from_ts_type_params(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    params: Option<TsTypeParameters>,
) -> Box<[TypeReference]> {
    params
        .map(|params| TypeReference::types_from_ts_type_parameters(resolver, scope_id, &params))
        .unwrap_or_default()
}

#[inline]
fn path_from_any_ts_module_name(module_name: AnyTsModuleName) -> Option<Box<[Text]>> {
    let mut path = Vec::new();
    let mut module_name = module_name;
    loop {
        match module_name {
            AnyTsModuleName::AnyTsIdentifierBinding(binding) => {
                let binding = binding.as_ts_identifier_binding()?;
                let name = text_from_token(binding.name_token())?;
                path.insert(0, name);
                break;
            }
            AnyTsModuleName::TsQualifiedModuleName(qualified) => {
                let right = qualified.right().ok()?;
                path.insert(0, text_from_token(right.value_token())?);

                module_name = qualified.left().ok()?;
            }
        }
    }

    Some(path.into())
}

#[inline]
fn return_type_from_annotation(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    annotation: Option<TsReturnTypeAnnotation>,
) -> Option<ReturnType> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .and_then(|ty| ReturnType::from_any_ts_return_type(resolver, scope_id, &ty))
}

#[inline]
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

#[inline]
fn text_from_class_member_name(name: ClassMemberName) -> Text {
    match name {
        ClassMemberName::Private(name) => Text::Owned(format!("#{name}")),
        ClassMemberName::Public(name) => Text::Borrowed(name),
    }
}

#[inline]
fn text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(token.ok()?.token_text_trimmed().into())
}

#[inline]
fn type_from_annotation(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    annotation: Option<TsTypeAnnotation>,
) -> Option<TypeReference> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .map(|ty| TypeReference::from_any_ts_type(resolver, scope_id, &ty))
}

fn type_from_function_body(
    resolver: &mut dyn TypeResolver,
    scope_id: ScopeId,
    body: JsFunctionBody,
) -> TypeData {
    let mut return_types: Vec<_> = body
        .syntax()
        .descendants()
        .filter_map(JsReturnStatement::cast)
        .map(|return_statement| {
            return_statement.argument().map_or(
                TypeData::Reference(GLOBAL_UNDEFINED_ID.into()),
                |argument| TypeData::from_any_js_expression(resolver, scope_id, &argument),
            )
        })
        .collect();

    match return_types.len() {
        0 => TypeData::VoidKeyword,
        1 => return_types.remove(0),
        _ => {
            let return_types = return_types
                .into_iter()
                .map(|ty| resolver.reference_to_owned_data(ty))
                .collect();

            TypeData::union_of(resolver, return_types)
        }
    }
}

#[inline]
fn unescaped_text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(unescape_js_string(inner_string_text(&token.ok()?)))
}
