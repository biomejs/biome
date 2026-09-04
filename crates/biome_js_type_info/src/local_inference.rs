//! Collects raw type information from one JavaScript or TypeScript syntax tree.
//!
//! This pass records local declarations, expressions, and references. Database
//! queries resolve imports and turn these raw entries into inferred types later.

use std::borrow::Cow;
use std::str::FromStr;

use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters,
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsDeclaration,
    AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsFormalParameter,
    AnyJsFunction, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsParameter,
    AnyJsSwitchClause, AnyTsModuleName, AnyTsName, AnyTsReturnType, AnyTsTupleTypeElement,
    AnyTsType, AnyTsTypeMember, AnyTsTypePredicateParameterName, ClassMemberName,
    JsArrayBindingPattern, JsArrowFunctionExpression, JsAssignmentOperator, JsBinaryExpression,
    JsBinaryOperator, JsCallArguments, JsCallExpression, JsCaseClause, JsClassDeclaration,
    JsClassExportDefaultDeclaration, JsClassExpression, JsClassMemberList,
    JsComputedMemberAssignment, JsConstructorParameters, JsExpressionStatement, JsExtendsClause,
    JsForInStatement, JsForOfStatement, JsForVariableDeclaration, JsFormalParameter,
    JsFunctionBody, JsFunctionDeclaration, JsFunctionExpression, JsGetterObjectMember,
    JsIdentifierAssignment, JsIdentifierBinding, JsIfStatement, JsInitializerClause,
    JsInstanceofExpression, JsLogicalExpression, JsLogicalOperator, JsMethodObjectMember,
    JsNewExpression, JsObjectBindingPattern, JsObjectExpression, JsParameters,
    JsPropertyClassMember, JsPropertyObjectMember, JsReferenceIdentifier, JsRestParameter,
    JsReturnStatement, JsSetterObjectMember, JsStaticMemberAssignment, JsSwitchStatement,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsUnaryExpression, JsUnaryOperator,
    JsVariableDeclaration, JsVariableDeclarator, TsDeclareFunctionDeclaration,
    TsExternalModuleDeclaration, TsInstantiationExpression, TsInterfaceDeclaration,
    TsModuleDeclaration, TsPropertyParameterModifierList, TsReferenceType, TsReturnTypeAnnotation,
    TsTypeAliasDeclaration, TsTypeAnnotation, TsTypeArguments, TsTypeList, TsTypeParameter,
    TsTypeParameters, TsTypeofType, inner_string_text, unescape_js_string,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxResult, Text, TextRange, TokenText,
};

use crate::globals::{
    GLOBAL_GLOBAL_ID, GLOBAL_INSTANCEOF_PROMISE_ID, GLOBAL_NUMBER_ID, GLOBAL_STRING_ID,
    GLOBAL_UNDEFINED_ID,
};
use crate::literal::{BooleanLiteral, NumberLiteral, RegexpLiteral, StringLiteral};
use crate::{
    AssertsReturnType, CallArgumentType, Class, Constructor, ConstructorParameter,
    DestructureField, Function, FunctionParameter, FunctionParameterBinding, GenericTypeParameter,
    Interface, Intersection, Literal, MemberEqualsPredicate, Module, NamedFunctionParameter,
    Namespace, NarrowingInvalidationKind, NarrowingPredicate, Object, Path,
    PatternFunctionParameter, PredicateCallPredicate, PredicateReturnType, RawTypeCollector,
    RawTypeId, ReturnType, ScopeId, Tuple, TupleElementType, TypeData, TypeInstance, TypeMember,
    TypeMemberAccessibility, TypeMemberKind, TypeOperator, TypeOperatorType, TypeReference,
    TypeReferenceQualifier, TypeofAdditionExpression, TypeofAwaitExpression,
    TypeofBitwiseNotExpression, TypeofCallExpression, TypeofConditionalExpression,
    TypeofDestructureExpression, TypeofExpression, TypeofIndexExpression,
    TypeofIterableValueOfExpression, TypeofLogicalAndExpression, TypeofLogicalOrExpression,
    TypeofNarrowedExpression, TypeofNewExpression, TypeofNullishCoalescingExpression,
    TypeofStaticMemberExpression, TypeofTag, TypeofThisOrSuperExpression, TypeofTypeofExpression,
    TypeofUnaryMinusExpression, TypeofValue, Union,
};

const MAX_CONST_ASSERTION_DEPTH: usize = 50;

impl TypeData {
    /// Applies the given `pattern` and returns the named bindings, and their
    /// associated types.
    pub fn apply_array_binding_pattern(
        &self,
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        pattern: &JsArrayBindingPattern,
    ) -> Box<[(Text, TypeReference)]> {
        pattern
            .elements()
            .into_iter()
            .enumerate()
            .filter_map(|(i, elem)| elem.ok().map(|elem| (i, elem)))
            .filter_map(|(i, elem)| {
                self.apply_array_binding_pattern_element(collector, scope_id, i, elem)
            })
            .flatten()
            .collect()
    }

    fn apply_array_binding_pattern_element(
        &self,
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        i: usize,
        elem: AnyJsArrayBindingPatternElement,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let reference = collector.reference_to_registered_data(self);
        match elem {
            AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(elem) => {
                match elem.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            collector.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::Index(i),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::Index(i));
                        data.apply_array_binding_pattern(collector, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::Index(i));
                        data.apply_object_binding_pattern(collector, scope_id, &pattern)
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
                            collector.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::RestFrom(i),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::destructuring_of(reference, DestructureField::RestFrom(i));
                        data.apply_array_binding_pattern(collector, scope_id, &pattern)
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
        collector: &mut dyn RawTypeCollector,
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

                self.apply_object_binding_pattern_member(collector, scope_id, &names, name, member)
            })
            .flatten()
            .collect()
    }

    fn apply_object_binding_pattern_member(
        &self,
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        names: &[Text],
        member_name: Option<Text>,
        member: AnyJsObjectBindingPatternMember,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let reference = collector.reference_to_registered_data(self);
        match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(prop) => {
                let member_name = member_name?;
                match prop.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(
                            name,
                            collector.reference_to_owned_data(Self::destructuring_of(
                                reference,
                                DestructureField::Name(member_name),
                            )),
                        )])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data =
                            Self::destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_array_binding_pattern(collector, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data =
                            Self::destructuring_of(reference, DestructureField::Name(member_name));
                        data.apply_object_binding_pattern(collector, scope_id, &pattern)
                    }),
                }
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(_) => Some({
                let member_name = member_name?;
                Box::new([(
                    member_name.clone(),
                    collector.reference_to_owned_data(Self::destructuring_of(
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
                    collector.reference_to_owned_data(Self::destructuring_of(
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &AnyJsDeclaration,
    ) -> Self {
        match decl {
            AnyJsDeclaration::JsClassDeclaration(decl) => {
                Self::from_js_class_declaration(collector, scope_id, decl)
            }
            AnyJsDeclaration::JsFunctionDeclaration(decl) => {
                Self::from_js_function_declaration(collector, scope_id, decl)
            }
            AnyJsDeclaration::JsVariableDeclaration(_) => {
                // Variable declarations don't have a type;
                // only their inner declarators have.
                Self::unknown()
            }
            AnyJsDeclaration::TsDeclareFunctionDeclaration(decl) => {
                Self::from_ts_declare_function_declaration(collector, scope_id, decl)
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
                Self::from_ts_interface_declaration(collector, scope_id, decl).unwrap_or_default()
            }
            AnyJsDeclaration::TsModuleDeclaration(decl) => {
                Self::from_ts_module_declaration(decl).unwrap_or_default()
            }
            AnyJsDeclaration::TsTypeAliasDeclaration(decl) => {
                Self::from_ts_type_alias_declaration(collector, scope_id, decl).unwrap_or_default()
            }
        }
    }

    pub fn from_any_js_declaration_clause(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: AnyJsDeclarationClause,
    ) -> Self {
        decl.into_declaration()
            .map(|decl| Self::from_any_js_declaration(collector, scope_id, &decl))
            .unwrap_or_default()
    }

    pub fn from_any_js_export_default_declaration(
        collector: &mut dyn RawTypeCollector,
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
                                collector, scope_id, &params,
                            )
                        })
                        .unwrap_or_default(),
                    extends: decl.extends_clause().and_then(|extends| {
                        reference_to_extends_clause(collector, scope_id, extends)
                    }),
                    implements: decl
                        .implements_clause()
                        .map(|implements| {
                            TypeReference::types_from_ts_type_list(
                                collector,
                                scope_id,
                                implements.types(),
                            )
                        })
                        .unwrap_or_default(),
                    members: decl
                        .members()
                        .into_iter()
                        .filter_map(|member| {
                            TypeMember::from_any_js_class_member(collector, scope_id, &member)
                        })
                        .collect(),
                }))
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(decl) => {
                let is_async = decl.async_token().is_some();
                Self::Function(Box::new(Function {
                    is_async,
                    type_parameters: generic_params_from_ts_type_params(
                        collector,
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
                        collector,
                        scope_id,
                        decl.parameters(),
                    ),
                    return_type: function_return_type(
                        collector,
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
                        collector,
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
                        collector,
                        scope_id,
                        decl.parameters(),
                    ),
                    return_type: function_return_type(
                        collector,
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
        collector: &mut dyn RawTypeCollector,
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
                            ty: collector.reference_to_resolved_expression(scope_id, &expr),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        }),
                        Ok(AnyJsArrayElement::JsSpread(spread)) => spread
                            .argument()
                            .ok()
                            .map(|expr| collector.reference_to_resolved_expression(scope_id, &expr))
                            .map(|ty| TupleElementType {
                                ty,
                                name: None,
                                is_optional: false,
                                is_rest: true,
                            }),
                        Ok(AnyJsArrayElement::JsArrayHole(_)) | Err(_) => Some(TupleElementType {
                            ty: TypeReference::unknown(),
                            name: None,
                            is_optional: false,
                            is_rest: false,
                        }),
                    })
                    .collect(),
            ))),
            AnyJsExpression::JsArrowFunctionExpression(expr) => {
                Self::from_js_arrow_function_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsAwaitExpression(expr) => {
                Self::from(TypeofExpression::Await(TypeofAwaitExpression {
                    argument: expr
                        .argument()
                        .map(|arg| collector.reference_to_resolved_expression(scope_id, &arg))
                        .unwrap_or_default(),
                }))
            }
            AnyJsExpression::JsBinaryExpression(expr) => {
                Self::from_js_binary_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsCallExpression(expr) => match expr.callee() {
                Ok(callee) => Self::from(TypeofExpression::Call(TypeofCallExpression {
                    callee: collector.reference_to_resolved_expression(scope_id, &callee),
                    arguments: CallArgumentType::types_from_js_call_arguments(
                        collector,
                        scope_id,
                        expr.arguments().ok(),
                    ),
                })),
                Err(_) => Self::unknown(),
            },
            AnyJsExpression::JsClassExpression(expr) => {
                Self::from_js_class_expression(collector, scope_id, expr)
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
                            let expression = TypeofStaticMemberExpression {
                                object: collector
                                    .reference_to_resolved_expression(scope_id, &object),
                                member,
                            };
                            Self::from(if expr.is_optional_chain() {
                                TypeofExpression::OptionalChainStaticMember(expression)
                            } else {
                                TypeofExpression::StaticMember(expression)
                            })
                        })
                        .unwrap_or_default(),
                    (
                        Ok(object),
                        Ok(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsNumberLiteralExpression(member),
                        )),
                    ) => unescaped_text_from_token(member.value_token())
                        .map(|member| match member.parse() {
                            Ok(index) => {
                                let expression = TypeofIndexExpression {
                                    object: collector
                                        .reference_to_resolved_expression(scope_id, &object),
                                    index,
                                };
                                Self::from(if expr.is_optional_chain() {
                                    TypeofExpression::OptionalChainIndex(expression)
                                } else {
                                    TypeofExpression::Index(expression)
                                })
                            }
                            Err(_) => Self::unknown(),
                        })
                        .unwrap_or_default(),
                    _ => Self::unknown(),
                }
            }
            AnyJsExpression::JsConditionalExpression(expr) => {
                Self::from(TypeofExpression::Conditional(TypeofConditionalExpression {
                    test: expr
                        .test()
                        .map(|sub| collector.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                    consequent: expr
                        .consequent()
                        .map(|sub| collector.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                    alternate: expr
                        .alternate()
                        .map(|sub| collector.reference_to_resolved_expression(scope_id, &sub))
                        .unwrap_or_default(),
                }))
            }
            AnyJsExpression::JsFunctionExpression(expr) => {
                Self::from_js_function_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsIdentifierExpression(expr) => expr
                .name()
                .map(|name| Self::from_js_reference_identifier(collector, scope_id, &name))
                .unwrap_or_default(),
            AnyJsExpression::JsImportCallExpression(_expr) => {
                Self::reference(GLOBAL_INSTANCEOF_PROMISE_ID)
            }
            AnyJsExpression::JsInstanceofExpression(_expr) => Self::Boolean,
            AnyJsExpression::JsLogicalExpression(expr) => {
                Self::from_js_logical_expression(collector, scope_id, expr).unwrap_or_default()
            }
            AnyJsExpression::JsNewExpression(expr) => {
                Self::from_js_new_expression(collector, scope_id, expr).unwrap_or_default()
            }
            AnyJsExpression::JsObjectExpression(expr) => {
                Self::from_js_object_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsParenthesizedExpression(expr) => expr
                .expression()
                .map(|expr| collector.resolve_expression(scope_id, &expr).into_owned())
                .unwrap_or_default(),
            AnyJsExpression::JsPostUpdateExpression(_)
            | AnyJsExpression::JsPreUpdateExpression(_) => Self::number(),
            AnyJsExpression::JsSequenceExpression(expr) => expr
                .right()
                .map(|expr| collector.resolve_expression(scope_id, &expr).into_owned())
                .unwrap_or_default(),
            AnyJsExpression::JsStaticMemberExpression(expr) => match (expr.object(), expr.member())
            {
                (Ok(object), Ok(member)) => text_from_any_js_name(member)
                    .map(|member| {
                        let expression = TypeofStaticMemberExpression {
                            object: collector.reference_to_resolved_expression(scope_id, &object),
                            member,
                        };
                        Self::from(if expr.is_optional_chain() {
                            TypeofExpression::OptionalChainStaticMember(expression)
                        } else {
                            TypeofExpression::StaticMember(expression)
                        })
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
            AnyJsExpression::TsAsExpression(expr) => {
                let Ok(annotation) = expr.ty() else {
                    return Self::unknown();
                };
                let Ok(inner) = expr.expression() else {
                    return Self::unknown();
                };
                if is_const_reference_type(&annotation) {
                    type_data_from_const_assertion_expression(collector, scope_id, &inner)
                } else {
                    Self::from_any_ts_type(collector, scope_id, &annotation)
                }
            }
            AnyJsExpression::TsInstantiationExpression(expr) => {
                Self::from_ts_instantiation_expression(collector, scope_id, expr)
                    .unwrap_or_default()
            }
            AnyJsExpression::TsTypeAssertionExpression(expr) => {
                let Ok(annotation) = expr.ty() else {
                    return Self::unknown();
                };
                let Ok(inner) = expr.expression() else {
                    return Self::unknown();
                };
                if is_const_reference_type(&annotation) {
                    type_data_from_const_assertion_expression(collector, scope_id, &inner)
                } else {
                    Self::from_any_ts_type(collector, scope_id, &annotation)
                }
            }
            AnyJsExpression::JsUnaryExpression(expr) => {
                Self::from_js_unary_expression(collector, scope_id, expr)
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
                Literal::RegExp(split_regex_literal(expr.value_token())?)
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(expr) => Literal::String(
                StringLiteral::from(Text::from(expr.inner_string_text().ok()?)),
            ),
        };

        Some(Self::Literal(Box::new(literal)))
    }

    pub fn from_any_ts_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &AnyTsType,
    ) -> Self {
        match ty {
            AnyTsType::JsMetavariable(_) => Self::unknown(),
            AnyTsType::TsAnyType(_) => Self::AnyKeyword,
            AnyTsType::TsArrayType(ty) => Self::array_of(
                scope_id,
                ty.element_type()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .unwrap_or_default(),
            ),
            AnyTsType::TsBigintLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(minus_token), Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    format!("{minus_token}{literal_token}").into(),
                ))),
                (None, Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    literal_token.token_text_trimmed().into(),
                ))),
                (_, Err(_)) => Self::unknown(),
            },
            AnyTsType::TsBigintType(_) => Self::BigInt,
            AnyTsType::TsBogusType(_) => Self::unknown(),
            AnyTsType::TsBooleanLiteralType(ty) => match ty.literal() {
                Ok(token) => match token.kind() {
                    JsSyntaxKind::TRUE_KW => {
                        Self::Literal(Box::new(Literal::Boolean(BooleanLiteral::from(true))))
                    }
                    JsSyntaxKind::FALSE_KW => {
                        Self::Literal(Box::new(Literal::Boolean(BooleanLiteral::from(false))))
                    }
                    _ => Self::unknown(),
                },
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsBooleanType(_) => Self::Boolean,
            AnyTsType::TsConditionalType(ty) => {
                // We don't attempt to evaluate the condition, so we simply
                // infer a union of both the possibilities.
                let types = Box::new([
                    ty.true_type()
                        .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                        .unwrap_or_default(),
                    ty.false_type()
                        .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                        .unwrap_or_default(),
                ]);

                Self::Union(Box::new(Union(types)))
            }
            AnyTsType::TsConstructorType(ty) => Self::Constructor(Box::new(Constructor {
                type_parameters: generic_params_from_ts_type_params(
                    collector,
                    scope_id,
                    ty.type_parameters(),
                ),
                parameters: constructor_params_from_js_params(collector, scope_id, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
            })),
            AnyTsType::TsFunctionType(ty) => Self::Function(Box::new(Function {
                is_async: false,
                type_parameters: generic_params_from_ts_type_params(
                    collector,
                    scope_id,
                    ty.type_parameters(),
                ),
                name: None,
                parameters: function_params_from_js_params(collector, scope_id, ty.parameters()),
                return_type: ty
                    .return_type()
                    .ok()
                    .and_then(|ty| ReturnType::from_any_ts_return_type(collector, scope_id, &ty))
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
            AnyTsType::TsIntersectionType(ty) => Self::Intersection(Box::new(Intersection(
                ty.types()
                    .into_iter()
                    .flatten()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .collect(),
            ))),
            AnyTsType::TsMappedType(_) => {
                // TODO: Handle mapped types (`type T<U> = { [K in keyof U]: V }`).
                Self::unknown()
            }
            AnyTsType::TsNeverType(_) => Self::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => Self::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => Self::Null,
            AnyTsType::TsNumberLiteralType(ty) => match ty.literal_token() {
                Ok(token) => {
                    Literal::Number(NumberLiteral::new(token.token_text_trimmed().into())).into()
                }
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsNumberType(_) => Self::reference(GLOBAL_NUMBER_ID),
            AnyTsType::TsObjectType(ty) => Self::object_with_members(
                ty.members()
                    .into_iter()
                    .filter_map(|member| {
                        TypeMember::from_any_ts_type_member(collector, scope_id, &member)
                    })
                    .collect(),
            ),
            AnyTsType::TsParenthesizedType(ty) => ty
                .ty()
                .map(|ty| Self::from_any_ts_type(collector, scope_id, &ty))
                .unwrap_or_default(),
            AnyTsType::TsReferenceType(ty) => Self::from_ts_reference_type(collector, scope_id, ty),
            AnyTsType::TsStringLiteralType(ty) => match ty.inner_string_text() {
                Ok(token) => Literal::String(token.text().into()).into(),
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsStringType(_) => Self::reference(GLOBAL_STRING_ID),
            AnyTsType::TsSymbolType(_) => Self::Symbol,
            AnyTsType::TsTemplateLiteralType(ty) => {
                Self::Literal(Box::new(Literal::Template(ty.to_string().into())))
            }
            AnyTsType::TsThisType(_) => Self::ThisKeyword,
            AnyTsType::TsTupleType(ty) => {
                let elements: SyntaxResult<Box<_>> = ty
                    .elements()
                    .into_iter()
                    .map(|el| {
                        el.map(|el| {
                            TupleElementType::from_any_ts_tuple_type_element(
                                collector, scope_id, &el,
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
                        ty: TypeReference::from_any_ts_type(collector, scope_id, &ty),
                    }))
                }),
                _ => Self::unknown(),
            },
            AnyTsType::TsTypeofType(ty) => Self::from_ts_typeof_type(collector, scope_id, ty),
            AnyTsType::TsUndefinedType(_) => Self::Undefined,
            AnyTsType::TsUnionType(ty) => {
                let types = ty
                    .types()
                    .into_iter()
                    .flatten()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .collect();

                Self::Union(Box::new(Union(types)))
            }
            AnyTsType::TsUnknownType(_) => Self::UnknownKeyword,
            AnyTsType::TsVoidType(_) => Self::VoidKeyword,
        }
    }

    pub fn from_any_ts_type_result(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: SyntaxResult<AnyTsType>,
    ) -> Self {
        ty.map(|ty| Self::from_any_ts_type(collector, scope_id, &ty))
            .unwrap_or_default()
    }

    pub fn from_js_arrow_function_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsArrowFunctionExpression,
    ) -> Self {
        let is_async = expr.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                collector,
                scope_id,
                expr.type_parameters(),
            ),
            name: None,
            parameters: match expr.parameters() {
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(binding)) => {
                    let name = binding
                        .as_js_identifier_binding()
                        .and_then(|binding| text_from_token(binding.name_token()))
                        .unwrap_or_default();
                    Box::new([FunctionParameter::Named(NamedFunctionParameter {
                        name,
                        ty: TypeReference::unknown(),
                        is_optional: false,
                        is_rest: false,
                    })])
                }
                Ok(AnyJsArrowFunctionParameters::JsParameters(params)) => {
                    function_params_from_js_params(collector, scope_id, Ok(params))
                }
                Err(_) => Box::default(),
            },
            return_type: function_return_type(
                collector,
                scope_id,
                is_async,
                expr.return_type_annotation(),
                expr.body().ok(),
            ),
        }))
    }

    pub fn from_js_binary_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsBinaryExpression,
    ) -> Self {
        let (Ok(left), Ok(operator), Ok(right)) = (expr.left(), expr.operator(), expr.right())
        else {
            return Self::unknown();
        };

        let left = collector.resolve_expression(scope_id, &left).into_owned();
        let right = collector.resolve_expression(scope_id, &right);

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
                    left: collector.reference_to_owned_data(left),
                    right: collector.reference_to_owned_data(right),
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
        collector: &mut dyn RawTypeCollector,
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
                collector,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .and_then(|extends| reference_to_extends_clause(collector, scope_id, extends)),
            implements: decl
                .implements_clause()
                .map(|implements| {
                    TypeReference::types_from_ts_type_list(collector, scope_id, implements.types())
                })
                .unwrap_or_default(),
            members: TypeMember::members_from_class_member_list(
                collector,
                scope_id,
                decl.members(),
            ),
        }))
    }

    pub fn from_js_class_expression(
        collector: &mut dyn RawTypeCollector,
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
                collector,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .and_then(|extends| reference_to_extends_clause(collector, scope_id, extends)),
            implements: decl
                .implements_clause()
                .map(|implements| {
                    TypeReference::types_from_ts_type_list(collector, scope_id, implements.types())
                })
                .unwrap_or_default(),
            members: TypeMember::members_from_class_member_list(
                collector,
                scope_id,
                decl.members(),
            ),
        }))
    }

    pub fn from_js_function_declaration(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsFunctionDeclaration,
    ) -> Self {
        let is_async = decl.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                collector,
                scope_id,
                decl.type_parameters(),
            ),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(collector, scope_id, decl.parameters()),
            return_type: function_return_type(
                collector,
                scope_id,
                is_async,
                decl.return_type_annotation(),
                decl.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
            ),
        }))
    }

    pub fn from_js_function_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsFunctionExpression,
    ) -> Self {
        let is_async = expr.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                collector,
                scope_id,
                expr.type_parameters(),
            ),
            name: expr
                .id()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(collector, scope_id, expr.parameters()),
            return_type: function_return_type(
                collector,
                scope_id,
                is_async,
                expr.return_type_annotation(),
                expr.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
            ),
        }))
    }

    pub fn from_js_logical_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsLogicalExpression,
    ) -> Option<Self> {
        let left = expr
            .left()
            .map(|left| TypeReference::from_any_js_expression(collector, scope_id, &left))
            .ok()?;
        let right = expr
            .right()
            .map(|right| TypeReference::from_any_js_expression(collector, scope_id, &right))
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsNewExpression,
    ) -> Option<Self> {
        Some(Self::from(TypeofExpression::New(TypeofNewExpression {
            callee: collector.reference_to_resolved_expression(scope_id, &expr.callee().ok()?),
            arguments: CallArgumentType::types_from_js_call_arguments(
                collector,
                scope_id,
                expr.arguments(),
            ),
        })))
    }

    pub fn from_ts_instantiation_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &TsInstantiationExpression,
    ) -> Option<Self> {
        let expression = expr.expression().ok()?;
        let arguments = expr.arguments().ok();
        Some(Self::instance_of(TypeInstance {
            ty: collector.reference_to_resolved_expression(scope_id, &expression),
            type_parameters: TypeReference::types_from_ts_type_arguments(
                collector, scope_id, arguments,
            ),
        }))
    }

    /// Infers the type of an object literal from the members it writes.
    ///
    /// Source members that inference cannot turn into a [`TypeMember`], such as
    /// a spread, are left out of the result and recorded through
    /// [`Object::has_unknown_members`] so callers can tell an object that is
    /// missing a name from one that was only partially modelled.
    pub fn from_js_object_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsObjectExpression,
    ) -> Self {
        let mut has_unknown_members = false;
        let members = expr
            .members()
            .into_iter()
            .filter_map(|member| {
                let member = member.ok().and_then(|member| {
                    TypeMember::from_any_js_object_member(collector, scope_id, &member)
                });
                has_unknown_members |= member.is_none();
                member
            })
            .collect();

        Self::Object(Box::new(Object {
            prototype: None,
            members,
            has_unknown_members,
        }))
    }

    pub fn from_js_reference_identifier(
        resolver: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        id: &JsReferenceIdentifier,
    ) -> Self {
        let Ok(name) = id.name() else {
            return Self::unknown();
        };
        match name.text() {
            "globalThis" => Self::reference(GLOBAL_GLOBAL_ID),
            "undefined" => Self::Undefined,
            _ => {
                let predicate = if resolver.narrowing_enabled() {
                    resolver.narrowing_predicate_mut(scope_id, id, name.clone())
                } else {
                    None
                };
                let reference = TypeReference::from_name(scope_id, name);
                match predicate {
                    Some(predicate) => {
                        Self::from(TypeofExpression::Narrowed(TypeofNarrowedExpression {
                            ty: reference,
                            predicate,
                        }))
                    }
                    None => Self::reference(reference),
                }
            }
        }
    }

    pub fn from_js_unary_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsUnaryExpression,
    ) -> Self {
        expr.operator()
            .map(|operator| match operator {
                JsUnaryOperator::BitwiseNot => {
                    Self::from(TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                        argument: expr
                            .argument()
                            .map(|arg| collector.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::Delete => Self::Boolean,
                JsUnaryOperator::Minus => {
                    Self::from(TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                        argument: expr
                            .argument()
                            .map(|arg| collector.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::LogicalNot => Self::Boolean,
                JsUnaryOperator::Plus => Self::Number,
                JsUnaryOperator::Typeof => {
                    Self::from(TypeofExpression::Typeof(TypeofTypeofExpression {
                        argument: expr
                            .argument()
                            .map(|arg| collector.reference_to_resolved_expression(scope_id, &arg))
                            .unwrap_or_default(),
                    }))
                }
                JsUnaryOperator::Void => Self::VoidKeyword,
            })
            .unwrap_or_default()
    }

    pub fn from_js_variable_declarator<'a>(
        collector: &'a mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsVariableDeclarator,
    ) -> Option<Cow<'a, Self>> {
        let ty = match decl.variable_annotation() {
            Some(annotation) => {
                let data = Self::from_any_ts_type(
                    collector,
                    scope_id,
                    &annotation.type_annotation().ok()??.ty().ok()?,
                );
                Cow::Owned(match data {
                    Self::InstanceOf(type_instance) => Self::InstanceOf(type_instance),
                    _ => Self::instance_of(collector.reference_to_owned_data(data)),
                })
            }
            None => collector.resolve_expression(scope_id, &decl.initializer()?.expression().ok()?),
        };

        Some(ty)
    }

    pub fn from_ts_declare_function_declaration(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &TsDeclareFunctionDeclaration,
    ) -> Self {
        let is_async = decl.async_token().is_some();
        Self::Function(Box::new(Function {
            is_async,
            type_parameters: generic_params_from_ts_type_params(
                collector,
                scope_id,
                decl.type_parameters(),
            ),
            name: decl
                .id()
                .ok()
                .as_ref()
                .and_then(|binding| binding.as_js_identifier_binding())
                .and_then(|binding| text_from_token(binding.name_token())),
            parameters: function_params_from_js_params(collector, scope_id, decl.parameters()),
            return_type: function_return_type(
                collector,
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &TsInterfaceDeclaration,
    ) -> Option<Self> {
        Some(Self::from(Interface {
            name: text_from_token(decl.id().ok()?.as_ts_identifier_binding()?.name_token())?,
            type_parameters: generic_params_from_ts_type_params(
                collector,
                scope_id,
                decl.type_parameters(),
            ),
            extends: decl
                .extends_clause()
                .map(|extends| {
                    TypeReference::types_from_ts_type_list(collector, scope_id, extends.types())
                })
                .unwrap_or_default(),
            members: decl
                .members()
                .into_iter()
                .filter_map(|member| {
                    TypeMember::from_any_ts_type_member(collector, scope_id, &member)
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsReferenceType,
    ) -> Self {
        ty.name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(scope_id, &name))
            .map(|qualifier| {
                Self::instance_of(TypeReference::from(qualifier.with_type_parameters(
                    TypeReference::types_from_ts_type_arguments(
                        collector,
                        scope_id,
                        ty.type_arguments(),
                    ),
                )))
            })
            .unwrap_or_default()
    }

    pub fn from_ts_type_alias_declaration(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &TsTypeAliasDeclaration,
    ) -> Option<Self> {
        Some(match decl.type_parameters() {
            Some(params) => Self::instance_of(TypeInstance {
                ty: TypeReference::from_any_ts_type(collector, scope_id, &decl.ty().ok()?),
                type_parameters: TypeReference::types_from_ts_type_parameters(
                    collector, scope_id, &params,
                ),
            }),
            None => Self::from_any_ts_type(collector, scope_id, &decl.ty().ok()?),
        })
    }

    pub fn from_ts_typeof_type(
        collector: &mut dyn RawTypeCollector,
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
                            collector,
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
            has_unknown_members: false,
        }))
    }

    pub fn promise_of(scope_id: ScopeId, ty: TypeReference) -> Self {
        Self::instance_of(TypeReference::from(
            TypeReferenceQualifier::from_path(scope_id, Text::new_static("Promise"))
                .with_type_parameters([ty]),
        ))
    }

    pub fn typed_bindings_from_js_binding_pattern(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: Self,
        pattern: &AnyJsBindingPattern,
        is_awaited: bool,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let ty = if is_awaited {
            Self::from(TypeofExpression::Await(TypeofAwaitExpression {
                argument: collector.reference_to_owned_data(ty),
            }))
        } else {
            ty
        };

        match pattern {
            AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                let binding = binding.as_js_identifier_binding()?;
                let name_token = binding.name_token().ok()?;
                Box::new([(
                    name_token.token_text_trimmed().into(),
                    collector.reference_to_owned_data(ty),
                )])
            }),
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => {
                Some(ty.apply_array_binding_pattern(collector, scope_id, pattern))
            }
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
                Some(ty.apply_object_binding_pattern(collector, scope_id, pattern))
            }
        }
    }

    pub fn typed_bindings_from_js_for_statement(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsForVariableDeclaration,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let parent = decl.syntax().parent()?;
        let (is_awaited, ty) = if JsForInStatement::can_cast(parent.kind()) {
            (false, Self::string())
        } else if let Some(for_of) = JsForOfStatement::cast(parent) {
            let ty = Self::from(TypeofExpression::IterableValueOf(
                TypeofIterableValueOfExpression {
                    ty: TypeReference::from_any_js_expression(
                        collector,
                        scope_id,
                        &for_of.expression().ok()?,
                    ),
                },
            ));
            (for_of.await_token().is_some(), ty)
        } else {
            return None;
        };

        let declarator = decl.declarator().ok()?;
        let binding = declarator.id().ok()?;
        Self::typed_bindings_from_js_binding_pattern(collector, scope_id, ty, &binding, is_awaited)
    }

    pub fn typed_bindings_from_js_variable_declaration(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsVariableDeclaration,
    ) -> Box<[(Text, TypeReference)]> {
        decl.declarators()
            .into_iter()
            .flatten()
            .filter_map(|decl| {
                Self::typed_bindings_from_js_variable_declarator(collector, scope_id, &decl)
            })
            .flatten()
            .collect()
    }

    pub fn typed_bindings_from_js_variable_declarator(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsVariableDeclarator,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let pattern = decl.id().ok()?;
        let ty = Self::from_js_variable_declarator(collector, scope_id, decl)?.into_owned();
        Self::typed_bindings_from_js_binding_pattern(collector, scope_id, ty, &pattern, false)
    }
}

impl CallArgumentType {
    pub fn types_from_js_call_arguments(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        arguments: Option<JsCallArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.args()
                    .into_iter()
                    .flatten()
                    .map(|arg| Self::from_any_js_call_argument(collector, scope_id, &arg))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn from_any_js_call_argument(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        arg: &AnyJsCallArgument,
    ) -> Self {
        match arg {
            AnyJsCallArgument::AnyJsExpression(expr) => {
                Self::Argument(collector.reference_to_resolved_expression(scope_id, expr))
            }
            AnyJsCallArgument::JsSpread(spread) => Self::Spread(
                spread
                    .argument()
                    .map(|arg| collector.reference_to_resolved_expression(scope_id, &arg))
                    .unwrap_or_default(),
            ),
        }
    }
}

impl ConstructorParameter {
    pub fn from_any_js_constructor_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &AnyJsConstructorParameter,
    ) -> Self {
        match param {
            AnyJsConstructorParameter::AnyJsFormalParameter(param) => Self {
                parameter: FunctionParameter::from_any_js_formal_parameter(
                    collector, scope_id, param,
                ),
                accessibility: None,
            },
            AnyJsConstructorParameter::JsRestParameter(param) => Self {
                parameter: FunctionParameter::from_js_rest_parameter(collector, scope_id, param),
                accessibility: None,
            },
            AnyJsConstructorParameter::TsPropertyParameter(param) => param
                .formal_parameter()
                .map(|formal_param| Self {
                    parameter: FunctionParameter::from_any_js_formal_parameter(
                        collector,
                        scope_id,
                        &formal_param,
                    ),
                    accessibility: Some(TypeMemberAccessibility::from_modifier_list(
                        param.modifiers(),
                    )),
                })
                .unwrap_or_default(),
        }
    }

    pub fn params_from_js_constructor_parameters(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        params: &JsConstructorParameters,
    ) -> Box<[Self]> {
        params
            .as_fields()
            .parameters
            .into_iter()
            .flatten()
            .map(|param| Self::from_any_js_constructor_parameter(collector, scope_id, &param))
            .collect()
    }
}

impl FunctionParameter {
    pub fn from_any_js_formal_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &AnyJsFormalParameter,
    ) -> Self {
        match param {
            AnyJsFormalParameter::JsFormalParameter(param) => {
                Self::from_js_formal_parameter(collector, scope_id, param)
            }
            _ => Self::default(),
        }
    }

    pub fn from_any_js_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &AnyJsParameter,
    ) -> Self {
        match param {
            AnyJsParameter::AnyJsFormalParameter(param) => {
                Self::from_any_js_formal_parameter(collector, scope_id, param)
            }
            AnyJsParameter::JsRestParameter(param) => {
                Self::from_js_rest_parameter(collector, scope_id, param)
            }
            AnyJsParameter::TsThisParameter(param) => Self::Named(NamedFunctionParameter {
                name: Text::new_static("this"),
                ty: param
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .unwrap_or_default(),
                is_optional: false,
                is_rest: false,
            }),
        }
    }

    pub fn from_js_formal_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &JsFormalParameter,
    ) -> Self {
        Self::from_binding_with_annotation(
            collector,
            scope_id,
            param.binding(),
            param.type_annotation(),
            param.question_mark_token().is_some(),
            false,
        )
    }

    pub fn from_js_rest_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &JsRestParameter,
    ) -> Self {
        Self::from_binding_with_annotation(
            collector,
            scope_id,
            param.binding(),
            param.type_annotation(),
            false,
            true,
        )
    }

    fn from_binding_with_annotation(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        binding: SyntaxResult<AnyJsBindingPattern>,
        annotation: Option<TsTypeAnnotation>,
        is_optional: bool,
        is_rest: bool,
    ) -> Self {
        let name = binding
            .as_ref()
            .ok()
            .and_then(AnyJsBindingPattern::as_any_js_binding)
            .and_then(AnyJsBinding::as_js_identifier_binding)
            .and_then(|identifier| identifier.name_token().ok())
            .map(|token| token.token_text_trimmed().into());
        let ty_data = annotation
            .and_then(|annotation| annotation.ty().ok())
            .map(|ty| TypeData::from_any_ts_type(collector, scope_id, &ty))
            .unwrap_or_default();
        // Optional parameters can be called without a value, so include `undefined` in the type
        let ty = if is_optional {
            let ty_ref = collector.reference_to_owned_data(ty_data.clone());
            RawTypeId::Local(collector.optional(ty_ref)).into()
        } else {
            collector.reference_to_owned_data(ty_data.clone())
        };
        if let Some(name) = name {
            Self::Named(NamedFunctionParameter {
                name,
                ty,
                is_optional,
                is_rest,
            })
        } else {
            let bindings = binding
                .ok()
                .and_then(|binding| {
                    FunctionParameterBinding::bindings_from_any_js_binding_pattern_of_type(
                        collector, scope_id, &binding, &ty_data,
                    )
                })
                .unwrap_or_default();
            Self::Pattern(PatternFunctionParameter {
                bindings,
                ty,
                is_optional,
                is_rest,
            })
        }
    }

    pub fn params_from_js_parameters(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        params: &JsParameters,
    ) -> Box<[Self]> {
        params
            .as_fields()
            .items
            .into_iter()
            .flatten()
            .map(|param| Self::from_any_js_parameter(collector, scope_id, &param))
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
        collector: &mut dyn RawTypeCollector,
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
                    ty: collector.reference_to_registered_data(ty),
                }]))
            }
            AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some(
                ty.apply_array_binding_pattern(collector, scope_id, pattern)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some(
                ty.apply_object_binding_pattern(collector, scope_id, pattern)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
        }
    }
}

impl GenericTypeParameter {
    pub fn from_ts_type_parameter(
        collector: &mut dyn RawTypeCollector,
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
                        TypeReference::from_any_ts_type(collector, scope_id, &constraint_ty)
                    })
                    .unwrap_or_default(),
                default: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map(|default_ty| {
                        TypeReference::from_any_ts_type(collector, scope_id, &default_ty)
                    })
                    .unwrap_or_default(),
            })
            .ok()
    }

    pub fn params_from_ts_type_parameters(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        params: &TsTypeParameters,
    ) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .flatten()
            .filter_map(|param| Self::from_ts_type_parameter(collector, scope_id, &param))
            .collect()
    }
}

impl ReturnType {
    pub fn from_any_ts_return_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &AnyTsReturnType,
    ) -> Option<Self> {
        match ty {
            AnyTsReturnType::AnyTsType(ty) => Some(Self::Type(TypeReference::from_any_ts_type(
                collector, scope_id, ty,
            ))),
            AnyTsReturnType::TsAssertsReturnType(ty) => {
                ty.parameter_name().ok().and_then(|parameter_name| {
                    Some(Self::Asserts(Box::new(AssertsReturnType {
                        parameter_name: match parameter_name {
                            AnyTsTypePredicateParameterName::JsReferenceIdentifier(identifier) => {
                                text_from_token(identifier.value_token())?
                            }
                            AnyTsTypePredicateParameterName::TsThisType(_) => {
                                Text::new_static("this")
                            }
                        },
                        ty: ty
                            .predicate()
                            .and_then(|asserts| asserts.ty().ok())
                            .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
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
                            AnyTsTypePredicateParameterName::TsThisType(_) => {
                                Text::new_static("this")
                            }
                        },
                        ty: ty
                            .ty()
                            .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                            .unwrap_or_default(),
                    })))
                })
            }
        }
    }
}

impl TupleElementType {
    pub fn from_any_ts_tuple_type_element(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        el: &AnyTsTupleTypeElement,
    ) -> Self {
        match el {
            AnyTsTupleTypeElement::AnyTsType(ty) => Self {
                ty: TypeReference::from_any_ts_type(collector, scope_id, ty),
                name: None,
                is_optional: false,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsNamedTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
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
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .unwrap_or_default(),
                name: None,
                is_optional: true,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsRestTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        member: &AnyJsClassMember,
    ) -> Option<Self> {
        match member {
            AnyJsClassMember::JsConstructorClassMember(member) => {
                let constructor = Constructor {
                    type_parameters: [].into(),
                    parameters: constructor_params_from_js_constructor_params(
                        collector,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: None,
                };
                let ty = collector.register_and_resolve(constructor.into());
                Some(Self {
                    kind: TypeMemberKind::Constructor,
                    ty: ty.into(),
                })
            }
            AnyJsClassMember::TsConstructorSignatureClassMember(member) => {
                let constructor = Constructor {
                    type_parameters: [].into(),
                    parameters: constructor_params_from_js_constructor_params(
                        collector,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: None,
                };
                let ty = collector.register_and_resolve(constructor.into());
                Some(Self {
                    kind: TypeMemberKind::Constructor,
                    ty: ty.into(),
                })
            }
            AnyJsClassMember::JsMethodClassMember(member) => member.name().ok().and_then(|name| {
                let is_async = member.async_token().is_some();
                let function = Function {
                    is_async,
                    type_parameters: generic_params_from_ts_type_params(
                        collector,
                        scope_id,
                        member.type_parameters(),
                    ),
                    name: name.name().map(text_from_class_member_name),
                    parameters: function_params_from_js_params(
                        collector,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: function_return_type(
                        collector,
                        scope_id,
                        is_async,
                        member.return_type_annotation(),
                        member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                    ),
                };
                let ty = collector.register_and_resolve(function.into());
                let is_static = member
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Self::from_class_member_info(collector, scope_id, name, ty.into(), is_static, false)
            }),
            AnyJsClassMember::JsPropertyClassMember(member) => {
                member.name().ok().and_then(|name| {
                    let ty = match member
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok())
                        .flatten()
                        .and_then(|annotation| annotation.ty().ok())
                    {
                        Some(ty) => TypeReference::from_any_ts_type(collector, scope_id, &ty),
                        None => member
                            .value()
                            .and_then(|initializer| initializer.expression().ok())
                            .map(|expr| collector.reference_to_resolved_expression(scope_id, &expr))
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
                    Self::from_class_member_info(
                        collector,
                        scope_id,
                        name,
                        ty,
                        is_static,
                        is_optional,
                    )
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
                            collector,
                            scope_id,
                            member.return_type(),
                            member.body().ok(),
                        )),
                    };
                    Self {
                        kind: TypeMemberKind::Getter(name),
                        ty: collector.reference_to_owned_data(function.into()),
                    }
                })
            }
            AnyJsClassMember::TsInitializedPropertySignatureClassMember(member) => {
                member.name().ok().and_then(|name| {
                    let ty = collector.reference_to_resolved_expression(
                        scope_id,
                        &member.value().ok()?.expression().ok()?,
                    );
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member.question_mark_token().is_some();
                    Self::from_class_member_info(
                        collector,
                        scope_id,
                        name,
                        ty,
                        is_static,
                        is_optional,
                    )
                })
            }
            AnyJsClassMember::TsPropertySignatureClassMember(member) => {
                member.name().ok().and_then(|name| {
                    let ty = member
                        .property_annotation()
                        .and_then(|annotation| annotation.type_annotation().ok())
                        .flatten()
                        .and_then(|annotation| annotation.ty().ok())
                        .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
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
                    Self::from_class_member_info(
                        collector,
                        scope_id,
                        name,
                        ty,
                        is_static,
                        is_optional,
                    )
                })
            }
            _ => {
                // TODO: Handle more variants
                None
            }
        }
    }

    pub fn from_any_js_object_member(
        collector: &mut dyn RawTypeCollector,
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
                            collector,
                            scope_id,
                            member.return_type(),
                            member.body().ok(),
                        )),
                    };
                    Self {
                        kind: TypeMemberKind::Getter(name.into()),
                        ty: collector.register_and_resolve(function.into()).into(),
                    }
                })
            }
            AnyJsObjectMember::JsMethodObjectMember(member) => member
                .name()
                .ok()
                .and_then(|name| match name {
                    AnyJsObjectMemberName::JsComputedMemberName(name) => {
                        name.expression().ok().map(|expr| {
                            TypeMemberKind::ComputedValue(computed_member_reference(
                                collector, scope_id, &expr,
                            ))
                        })
                    }
                    AnyJsObjectMemberName::JsLiteralMemberName(name) => name
                        .name()
                        .ok()
                        .map(|name| TypeMemberKind::Named(name.into())),
                    _ => None,
                })
                .map(|kind| {
                    let is_async = member.async_token().is_some();
                    let function = Function {
                        is_async,
                        type_parameters: generic_params_from_ts_type_params(
                            collector,
                            scope_id,
                            member.type_parameters(),
                        ),
                        name: match &kind {
                            TypeMemberKind::Named(name) => Some(name.clone()),
                            _ => None,
                        },
                        parameters: function_params_from_js_params(
                            collector,
                            scope_id,
                            member.parameters(),
                        ),
                        return_type: function_return_type(
                            collector,
                            scope_id,
                            is_async,
                            member.return_type_annotation(),
                            member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                        ),
                    };
                    Self {
                        kind,
                        ty: collector.register_and_resolve(function.into()).into(),
                    }
                }),
            AnyJsObjectMember::JsPropertyObjectMember(member) => member
                .name()
                .ok()
                .and_then(|name| match name {
                    AnyJsObjectMemberName::JsComputedMemberName(name) => {
                        name.expression().ok().map(|expr| {
                            TypeMemberKind::ComputedValue(computed_member_reference(
                                collector, scope_id, &expr,
                            ))
                        })
                    }
                    AnyJsObjectMemberName::JsLiteralMemberName(name) => name
                        .name()
                        .ok()
                        .map(|name| TypeMemberKind::Named(name.into())),
                    _ => None,
                })
                .map(|kind| {
                    let value = member.value().ok();
                    let kind = if value.as_ref().is_some_and(expression_is_const_assertion) {
                        kind.with_const_asserted()
                    } else {
                        kind
                    };
                    Self {
                        kind,
                        ty: value
                            .map(|value| {
                                collector.reference_to_resolved_expression(scope_id, &value)
                            })
                            .unwrap_or_default(),
                    }
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
                    ty: collector.reference_to_owned_data(TypeData::from(TypeofValue {
                        identifier: name,
                        ty: TypeReference::unknown(),
                        scope_id: Some(scope_id),
                    })),
                }),
            AnyJsObjectMember::JsSpread(_) => {
                // TODO: Handle spread operator
                None
            }
            AnyJsObjectMember::JsMetavariable(_) => {
                // Standalone metavariable object members (e.g. $...) do not contribute type info
                None
            }
        }
    }

    pub fn from_any_ts_type_member(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        member: &AnyTsTypeMember,
    ) -> Option<Self> {
        match member {
            AnyTsTypeMember::JsBogusMember(_) => None,
            AnyTsTypeMember::TsCallSignatureTypeMember(member) => {
                let function = Function {
                    is_async: false,
                    type_parameters: generic_params_from_ts_type_params(
                        collector,
                        scope_id,
                        member.type_parameters(),
                    ),
                    name: None,
                    parameters: function_params_from_js_params(
                        collector,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: return_type_from_annotation(
                        collector,
                        scope_id,
                        member.return_type_annotation(),
                    )
                    .unwrap_or_default(),
                };
                let ty = collector.register_and_resolve(function.into());
                Some(Self {
                    kind: TypeMemberKind::CallSignature,
                    ty: ty.into(),
                })
            }
            AnyTsTypeMember::TsConstructSignatureTypeMember(member) => {
                let constructor = Constructor {
                    type_parameters: generic_params_from_ts_type_params(
                        collector,
                        scope_id,
                        member.type_parameters(),
                    ),
                    parameters: constructor_params_from_js_params(
                        collector,
                        scope_id,
                        member.parameters(),
                    ),
                    return_type: type_from_annotation(
                        collector,
                        scope_id,
                        member.type_annotation(),
                    ),
                };
                let ty = collector.register_and_resolve(constructor.into());
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
                        collector,
                        scope_id,
                        member.type_annotation(),
                        None,
                    )),
                };
                let ty = collector.register_and_resolve(function.into()).into();
                Some(Self {
                    kind: TypeMemberKind::Getter(name.into()),
                    ty: RawTypeId::Local(collector.optional(ty)).into(),
                })
            }
            AnyTsTypeMember::TsIndexSignatureTypeMember(member) => {
                let key_ty = member
                    .parameter()
                    .and_then(|parameter| parameter.type_annotation())
                    .and_then(|annotation| annotation.ty())
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .ok()?;
                let value_ty = member
                    .type_annotation()
                    .and_then(|annotation| annotation.ty())
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
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
                            collector,
                            scope_id,
                            member.type_parameters(),
                        ),
                        name: Some(name.clone().into()),
                        parameters: function_params_from_js_params(
                            collector,
                            scope_id,
                            member.parameters(),
                        ),
                        return_type: return_type_from_annotation(
                            collector,
                            scope_id,
                            member.return_type_annotation(),
                        )
                        .unwrap_or_default(),
                    };
                    let ty = collector.register_and_resolve(function.into()).into();
                    let is_optional = member.optional_token().is_some();
                    Self::from_name_and_optional_type(collector, name, ty, is_optional)
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let ty = type_from_annotation(collector, scope_id, member.type_annotation())
                        .unwrap_or_default();
                    let is_optional = member.optional_token().is_some();
                    Self::from_name_and_optional_type(collector, name, ty, is_optional)
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
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        name: AnyJsClassMemberName,
        ty: TypeReference,
        is_static: bool,
        is_optional: bool,
    ) -> Option<Self> {
        let kind = match name {
            AnyJsClassMemberName::JsComputedMemberName(name) => TypeMemberKind::ComputedValue(
                computed_member_reference(collector, scope_id, &name.expression().ok()?),
            ),
            _ => {
                let name = text_from_class_member_name(name.name()?);
                if is_static {
                    TypeMemberKind::NamedStatic(name)
                } else if is_optional {
                    TypeMemberKind::NamedOptional(name)
                } else {
                    TypeMemberKind::Named(name)
                }
            }
        };

        Some(Self {
            kind,
            ty: match is_optional {
                true => {
                    let id = collector.optional(ty);
                    collector.reference_to_id(id)
                }
                false => ty,
            },
        })
    }

    #[inline]
    fn from_name_and_optional_type(
        collector: &mut dyn RawTypeCollector,
        name: TokenText,
        ty: TypeReference,
        is_optional: bool,
    ) -> Self {
        let name: Text = name.into();
        Self {
            kind: if is_optional {
                TypeMemberKind::NamedOptional(name)
            } else {
                TypeMemberKind::Named(name)
            },
            ty: match is_optional {
                true => RawTypeId::Local(collector.optional(ty)).into(),
                false => ty,
            },
        }
    }

    fn members_from_class_member_list(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        member_list: JsClassMemberList,
    ) -> Box<[Self]> {
        let mut members: Vec<_> = member_list
            .into_iter()
            .filter_map(|member| Self::from_any_js_class_member(collector, scope_id, &member))
            .collect();

        // Extend members with those from constructor definitions:
        let num_members = members.len();
        for i in 0..num_members {
            let member = &members[i];
            if member.is_constructor()
                && let Some(member_ty) = collector.get_by_reference(&member.ty)
                && let TypeData::Constructor(constructor) = member_ty
            {
                for param in &constructor.parameters {
                    if let Some(_accessibility) = param.accessibility
                        && let FunctionParameter::Named(named_param) = &param.parameter
                    {
                        // TODO: Assign accessibility to type members.
                        members.push(Self {
                            kind: if named_param.is_optional {
                                TypeMemberKind::NamedOptional(named_param.name.clone())
                            } else {
                                TypeMemberKind::Named(named_param.name.clone())
                            },
                            ty: param.parameter.ty().clone(),
                        });
                    }
                }
            }
        }

        members.into()
    }
}

fn computed_member_reference(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    expression: &AnyJsExpression,
) -> TypeReference {
    if let Some(member) = expression.as_js_static_member_expression()
        && let Ok(object) = member.object()
        && let Some(identifier) = object.as_js_identifier_expression()
        && let (Some(object_name), Some(member_name)) = (
            identifier
                .name()
                .ok()
                .and_then(|name| text_from_token(name.value_token())),
            member.member().ok().and_then(text_from_any_js_name),
        )
        && object_name.text() == "Symbol"
        && matches!(member_name.text(), "dispose" | "asyncDispose")
    {
        return TypeReferenceQualifier::from_path(
            scope_id,
            Path::Qualified(vec![object_name, member_name].into_boxed_slice()),
        )
        .into();
    }

    TypeReference::from_any_js_expression(collector, scope_id, expression)
}

impl TypeReference {
    pub fn from_any_js_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &AnyJsExpression,
    ) -> Self {
        let data = TypeData::from_any_js_expression(collector, scope_id, expr);
        collector.reference_to_owned_data(data)
    }

    pub fn from_any_ts_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &AnyTsType,
    ) -> Self {
        let data = TypeData::from_any_ts_type(collector, scope_id, ty);
        collector.reference_to_owned_data(data)
    }

    pub fn from_name(scope_id: ScopeId, name: TokenText) -> Self {
        Self::from(TypeReferenceQualifier::from_path(
            scope_id,
            Text::from(name),
        ))
    }

    pub fn from_ts_reference_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsReferenceType,
    ) -> Self {
        let data = TypeData::from_ts_reference_type(collector, scope_id, ty);
        collector.reference_to_owned_data(data)
    }

    pub fn types_from_ts_type_arguments(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        arguments: Option<TsTypeArguments>,
    ) -> Box<[Self]> {
        arguments
            .map(|args| {
                args.ts_type_argument_list()
                    .into_iter()
                    .filter_map(Result::ok)
                    .map(|ty| Self::from_any_ts_type(collector, scope_id, &ty))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn types_from_ts_type_list(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        types: TsTypeList,
    ) -> Box<[Self]> {
        types
            .into_iter()
            .filter_map(Result::ok)
            .map(|ty| Self::from_ts_reference_type(collector, scope_id, &ty))
            .collect()
    }

    pub fn types_from_ts_type_parameters(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        params: &TsTypeParameters,
    ) -> Box<[Self]> {
        params
            .items()
            .into_iter()
            .map(|param| match param {
                Ok(param) => {
                    GenericTypeParameter::from_ts_type_parameter(collector, scope_id, &param)
                        .map(|generic| collector.register_and_resolve(TypeData::from(generic)))
                        .map(Self::from)
                        .unwrap_or_default()
                }
                Err(_) => Self::unknown(),
            })
            .collect()
    }
}

impl TypeReferenceQualifier {
    pub fn from_any_ts_name(scope_id: ScopeId, name: &AnyTsName) -> Option<Self> {
        match name {
            AnyTsName::JsReferenceIdentifier(identifier) => {
                text_from_token(identifier.value_token())
                    .map(|name| Self::from_path(scope_id, name).with_type_only())
            }
            AnyTsName::TsQualifiedName(name) => {
                let mut fields = name.as_fields();
                let mut reversed_path = Vec::new();
                loop {
                    reversed_path.push(text_from_token(fields.right.ok()?.value_token())?);

                    match fields.left.ok()? {
                        AnyTsName::JsReferenceIdentifier(identifier) => {
                            reversed_path.push(text_from_token(identifier.value_token())?);
                            break;
                        }
                        AnyTsName::TsQualifiedName(name) => {
                            fields = name.as_fields();
                        }
                    }
                }
                let path = Path::from_reversed_parts(reversed_path);
                Some(Self::from_path(scope_id, path).with_type_only())
            }
        }
    }

    pub fn from_path(scope_id: ScopeId, path: impl Into<Path>) -> Self {
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

fn is_direct_class_or_object_member(node: &JsSyntaxNode) -> bool {
    node.ancestors()
        .skip(1)
        .find_map(|node| {
            if let Some(node) = AnyJsExpression::cast_ref(&node) {
                let node = node.omit_parentheses();
                if matches!(
                    node,
                    AnyJsExpression::TsAsExpression(_)
                        | AnyJsExpression::TsNonNullAssertionExpression(_)
                        | AnyJsExpression::TsSatisfiesExpression(_)
                        | AnyJsExpression::TsTypeAssertionExpression(_)
                ) {
                    None
                } else {
                    Some(false)
                }
            } else {
                Some(
                    JsInitializerClause::can_cast(node.kind())
                        && node
                            .parent()
                            .is_some_and(|parent| JsPropertyClassMember::can_cast(parent.kind()))
                        || JsPropertyObjectMember::can_cast(node.kind()),
                )
            }
        })
        .unwrap_or_default()
}

impl TypeofThisOrSuperExpression {
    /// Detect a nearest parent that can be used as type of `this`.
    fn from_any_js_expression(scope_id: ScopeId, expr: &AnyJsExpression) -> Self {
        // The rules are as follows:
        //
        // 1. If we reached a class node, that class is `this`.
        // 2. If we reached a function, `this` is unknown, unless that function
        //    is a direct descendant of a class or an object, ignoring non-exprs and
        //    typescript extras (like `as typ`).
        // 3. If we reached an object literal *and* have already traversed past
        //    a function or an object method, this object is `this`.

        let binds_this_to_object = |node: &JsSyntaxNode| {
            JsFunctionExpression::can_cast(node.kind())
                || JsFunctionDeclaration::can_cast(node.kind())
                || JsGetterObjectMember::can_cast(node.kind())
                || JsMethodObjectMember::can_cast(node.kind())
                || JsSetterObjectMember::can_cast(node.kind())
        };
        let mut may_bind_to_object = false;
        let parent = expr
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(|node| {
                if (JsFunctionExpression::can_cast(node.kind())
                    && !is_direct_class_or_object_member(&node))
                    || JsFunctionDeclaration::can_cast(node.kind())
                {
                    return Some(Err(()));
                }

                may_bind_to_object = may_bind_to_object || binds_this_to_object(&node);

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
                } else if !may_bind_to_object {
                    None
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
                Some(Ok(TypeReferenceQualifier::from_path(scope_id, name).into()))
            })
            .unwrap_or(Err(()))
            .unwrap_or_default();

        Self { parent }
    }
}

fn reference_to_extends_clause(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    extends: JsExtendsClause,
) -> Option<TypeReference> {
    let super_class = extends.super_class().ok()?;
    let super_class = collector.reference_to_resolved_expression(scope_id, &super_class);
    let type_parameters =
        TypeReference::types_from_ts_type_arguments(collector, scope_id, extends.type_arguments());

    if type_parameters.is_empty() {
        Some(super_class)
    } else {
        Some(
            collector.reference_to_owned_data(TypeData::instance_of(TypeInstance {
                ty: super_class,
                type_parameters,
            })),
        )
    }
}

impl TypeMemberAccessibility {
    fn from_modifier_list(modifier_list: TsPropertyParameterModifierList) -> Self {
        for modifier in modifier_list {
            if let Some(modifier) = modifier.as_ts_accessibility_modifier() {
                return match modifier.modifier_token() {
                    Ok(token) if token.kind() == JsSyntaxKind::PRIVATE_KW => Self::Private,
                    Ok(token) if token.kind() == JsSyntaxKind::PROTECTED_KW => Self::Protected,
                    _ => Self::Public,
                };
            }
        }

        Self::default()
    }
}

#[inline]
fn constructor_params_from_js_constructor_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsConstructorParameters>,
) -> Box<[ConstructorParameter]> {
    params
        .map(|params| {
            ConstructorParameter::params_from_js_constructor_parameters(
                collector, scope_id, &params,
            )
        })
        .unwrap_or_default()
}

#[inline]
fn constructor_params_from_js_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsParameters>,
) -> Box<[ConstructorParameter]> {
    params
        .map(|params| {
            params
                .as_fields()
                .items
                .into_iter()
                .flatten()
                .map(|param| ConstructorParameter {
                    parameter: FunctionParameter::from_any_js_parameter(
                        collector, scope_id, &param,
                    ),
                    accessibility: None,
                })
                .collect()
        })
        .unwrap_or_default()
}

#[inline]
fn function_params_from_js_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsParameters>,
) -> Box<[FunctionParameter]> {
    params
        .map(|params| FunctionParameter::params_from_js_parameters(collector, scope_id, &params))
        .unwrap_or_default()
}

fn function_return_type(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    is_async: bool,
    annotation: Option<TsReturnTypeAnnotation>,
    body: Option<AnyJsFunctionBody>,
) -> ReturnType {
    if let Some(return_ty) = return_type_from_annotation(collector, scope_id, annotation) {
        return if is_async && return_ty.as_type().is_some_and(|ty| !ty.is_known()) {
            ReturnType::Type(GLOBAL_INSTANCEOF_PROMISE_ID.into())
        } else {
            return_ty
        };
    }

    let mut return_ty = match body {
        Some(AnyJsFunctionBody::AnyJsExpression(return_expr)) => collector
            .resolve_expression(scope_id, &return_expr)
            .into_owned(),
        Some(AnyJsFunctionBody::JsFunctionBody(body)) => {
            type_from_function_body(collector, scope_id, body)
        }
        None => {
            return ReturnType::Type(match is_async {
                true => GLOBAL_INSTANCEOF_PROMISE_ID.into(),
                false => TypeReference::unknown(),
            });
        }
    };

    if is_async {
        return_ty = TypeData::promise_of(scope_id, collector.reference_to_owned_data(return_ty));
    }

    ReturnType::Type(collector.reference_to_owned_data(return_ty))
}

fn getter_return_type(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    annotation: Option<TsTypeAnnotation>,
    body: Option<JsFunctionBody>,
) -> TypeReference {
    if let Some(return_ty) = type_from_annotation(collector, scope_id, annotation) {
        return return_ty;
    }

    let return_ty = match body {
        Some(body) => type_from_function_body(collector, scope_id, body),
        None => return TypeReference::unknown(),
    };

    collector.reference_to_owned_data(return_ty)
}

#[inline]
fn generic_params_from_ts_type_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: Option<TsTypeParameters>,
) -> Box<[TypeReference]> {
    params
        .map(|params| TypeReference::types_from_ts_type_parameters(collector, scope_id, &params))
        .unwrap_or_default()
}

#[inline]
fn path_from_any_ts_module_name(module_name: AnyTsModuleName) -> Option<Path> {
    let mut reversed_path = Vec::new();
    let mut module_name = module_name;
    loop {
        match module_name {
            AnyTsModuleName::AnyTsIdentifierBinding(binding) => {
                let binding = binding.as_ts_identifier_binding()?;
                reversed_path.push(text_from_token(binding.name_token())?);
                break;
            }
            AnyTsModuleName::TsQualifiedModuleName(qualified) => {
                let right = qualified.right().ok()?;
                reversed_path.push(text_from_token(right.value_token())?);

                module_name = qualified.left().ok()?;
            }
        }
    }

    Some(Path::from_reversed_parts(reversed_path))
}

#[inline]
fn return_type_from_annotation(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    annotation: Option<TsReturnTypeAnnotation>,
) -> Option<ReturnType> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .and_then(|ty| ReturnType::from_any_ts_return_type(collector, scope_id, &ty))
}

#[inline]
fn split_regex_literal(token: SyntaxResult<JsSyntaxToken>) -> Option<RegexpLiteral> {
    let literal = token.ok()?.token_text_trimmed();
    let open_index: usize = literal.find('/')? + 1;
    let close_index: usize = literal.rfind('/')?;
    if open_index >= close_index {
        return None;
    }

    let literal_len = usize::from(literal.len());

    Some(RegexpLiteral {
        pattern: literal
            .clone()
            .slice(TextRange::try_from((open_index, close_index)).ok()?)
            .into(),
        flags: literal
            .slice(TextRange::try_from((close_index + 1, literal_len)).ok()?)
            .into(),
    })
}

#[inline]
fn text_from_any_js_name(name: AnyJsName) -> Option<Text> {
    match name {
        AnyJsName::JsMetavariable(_) => None,
        AnyJsName::JsName(name) => text_from_token(name.value_token()),
        AnyJsName::JsPrivateName(name) => name
            .value_token()
            .ok()
            .map(|token| format!("#{}", token.token_text_trimmed()).into()),
    }
}

#[inline]
fn text_from_class_member_name(name: ClassMemberName) -> Text {
    match name {
        ClassMemberName::Private(name) => format!("#{name}").into(),
        ClassMemberName::Public(name) => name.into(),
    }
}

#[inline]
fn text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(token.ok()?.token_text_trimmed().into())
}

#[inline]
fn type_from_annotation(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    annotation: Option<TsTypeAnnotation>,
) -> Option<TypeReference> {
    annotation
        .and_then(|annotation| annotation.ty().ok())
        .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
}

fn type_from_function_body(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    body: JsFunctionBody,
) -> TypeData {
    let mut return_types: Vec<_> = body
        .syntax()
        .pruned_descendents(|node| !AnyJsFunction::can_cast(node.kind()))
        .filter_map(JsReturnStatement::cast)
        .map(|return_statement| {
            return_statement.argument().map_or(
                TypeData::Reference(GLOBAL_UNDEFINED_ID.into()),
                |argument| {
                    collector
                        .resolve_expression(scope_id, &argument)
                        .into_owned()
                },
            )
        })
        .collect();

    match return_types.len() {
        0 => TypeData::VoidKeyword,
        1 => return_types.remove(0),
        _ => {
            let return_types = return_types
                .into_iter()
                .map(|ty| collector.reference_to_owned_data(ty))
                .collect();

            TypeData::Union(Box::new(Union(return_types)))
        }
    }
}

/// Checks for TypeScript's special `const` assertion target.
fn is_const_reference_type(type_annotation: &AnyTsType) -> bool {
    let Some(reference_type) = type_annotation.as_ts_reference_type() else {
        return false;
    };

    reference_type.type_arguments().is_none()
        && reference_type.name().ok().is_some_and(|name| {
            name.as_js_reference_identifier()
                .and_then(|identifier| identifier.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "const")
        })
}

/// Recognizes direct `as const` and `<const>` assertions, allowing parentheses around them.
fn expression_is_const_assertion(expression: &AnyJsExpression) -> bool {
    let mut current = expression.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(expression) => {
                return expression.ty().is_ok_and(|ty| is_const_reference_type(&ty));
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                return expression.ty().is_ok_and(|ty| is_const_reference_type(&ty));
            }
            AnyJsExpression::JsParenthesizedExpression(expression) => match expression.expression()
            {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            _ => return false,
        }
    }
}

/// Builds the type produced by a const assertion expression.
fn type_data_from_const_assertion_expression(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    expression: &AnyJsExpression,
) -> TypeData {
    let expression = expression.clone().omit_parentheses();
    if let AnyJsExpression::JsUnaryExpression(unary) = &expression
        && unary.operator().ok() == Some(JsUnaryOperator::Minus)
        && let Ok(argument) = unary.argument()
    {
        match argument.omit_parentheses() {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsBigintLiteralExpression(literal),
            ) => {
                if let Some(text) = text_from_token(literal.value_token()) {
                    return TypeData::Literal(Box::new(Literal::BigInt(format!("-{text}").into())));
                }
            }
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                if let Some(text) = text_from_token(literal.value_token()) {
                    return TypeData::Literal(Box::new(Literal::Number(NumberLiteral::new(
                        format!("-{text}").into(),
                    ))));
                }
            }
            _ => {}
        }
    }

    let inner_type = collector
        .resolve_expression(scope_id, &expression)
        .into_owned();
    apply_deep_const(collector, inner_type)
}

/// Applies const assertion conversion to inferred tuple and object types.
fn apply_deep_const(collector: &mut dyn RawTypeCollector, inner_type: TypeData) -> TypeData {
    apply_deep_const_inner(collector, inner_type, 0)
}

/// Recursively applies `as const` to tuple elements and object members.
fn apply_deep_const_inner(
    collector: &mut dyn RawTypeCollector,
    inner_type: TypeData,
    depth: usize,
) -> TypeData {
    if depth >= MAX_CONST_ASSERTION_DEPTH {
        return TypeData::unknown();
    }

    match inner_type {
        TypeData::Tuple(tuple) => {
            let elements = tuple
                .elements()
                .iter()
                .map(|element| TupleElementType {
                    ty: apply_deep_const_reference(collector, &element.ty, depth + 1),
                    name: element.name.clone(),
                    is_optional: element.is_optional,
                    is_rest: element.is_rest,
                })
                .collect();
            TypeData::Tuple(Box::new(Tuple(elements)))
        }
        TypeData::Object(object) => TypeData::Object(Box::new(Object {
            prototype: object.prototype.clone(),
            members: object
                .members
                .iter()
                .map(|member| TypeMember {
                    kind: member.kind.clone().with_const_asserted(),
                    ty: apply_deep_const_reference(collector, &member.ty, depth + 1),
                })
                .collect(),
            has_unknown_members: object.has_unknown_members,
        })),
        _ => inner_type,
    }
}

/// Resolves a type reference, applies const assertion conversion, and stores the result.
fn apply_deep_const_reference(
    collector: &mut dyn RawTypeCollector,
    type_reference: &TypeReference,
    depth: usize,
) -> TypeReference {
    if depth >= MAX_CONST_ASSERTION_DEPTH {
        return TypeReference::unknown();
    }

    let Some(inner_type) = collector.get_by_reference(type_reference).cloned() else {
        return type_reference.clone();
    };

    let inner_type = apply_deep_const_inner(collector, inner_type, depth);
    collector.reference_to_owned_data(inner_type)
}

#[inline]
fn unescaped_text_from_token(token: SyntaxResult<JsSyntaxToken>) -> Option<Text> {
    Some(unescape_js_string(inner_string_text(&token.ok()?)))
}

/// Returns the right-hand side of a statement of the form `<name> = <expr>;`.
fn plain_assignment_rhs(node: &JsSyntaxNode, name: &str) -> Option<AnyJsExpression> {
    let stmt = JsExpressionStatement::cast_ref(node)?;
    let expr = stmt.expression().ok()?.omit_parentheses();
    let AnyJsExpression::JsAssignmentExpression(assignment) = expr else {
        return None;
    };
    if !matches!(assignment.operator(), Ok(JsAssignmentOperator::Assign)) {
        return None;
    }
    let AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(target)) =
        assignment.left().ok()?
    else {
        return None;
    };
    if target.name_token().ok()?.text_trimmed() != name {
        return None;
    }
    assignment.right().ok()
}

/// Returns the narrowing predicate that the guards enclosing a reference
/// establish for it, e.g. `Typeof(String)` for `x` inside the consequent of
/// `if (typeof x === "string")`, or `Truthy` inside the consequent of
/// `if (x)`.
///
/// This backs [`RawTypeCollector::narrowing_predicate_mut`]'s default body.
/// It takes `resolver` generically, rather than as `&mut dyn RawTypeCollector`,
/// so that default method can call it on `self` directly: coercing `&mut
/// Self` to a trait object requires `Self: Sized`, which a default method
/// callable through `&mut dyn RawTypeCollector` can't require.
pub(crate) fn narrowing_predicate<C: RawTypeCollector + ?Sized>(
    resolver: &mut C,
    scope_id: ScopeId,
    id: &JsReferenceIdentifier,
    name_token: TokenText,
) -> Option<NarrowingPredicate> {
    let mut analysis = GuardAnalysis::new(resolver, scope_id, name_token);
    analysis
        .assignment_predicate(id)
        .or_else(|| analysis.narrowing_predicate(id))
}

/// Detects the narrowing guards that apply to one binding.
///
/// Every check is decided against a single name, the one of the reference
/// being narrowed, and guards that mention other bindings (the callee of a
/// predicate call, the class of an `instanceof`) resolve them from the scope
/// that reference resolves in. Both are fixed for the whole analysis, as is
/// the resolver whose invalidation cache the scans share, so they live here
/// instead of being threaded through every check.
struct GuardAnalysis<'a, C: RawTypeCollector + ?Sized> {
    resolver: &'a mut C,
    scope_id: ScopeId,
    name_token: TokenText,
}

impl<'a, C: RawTypeCollector + ?Sized> GuardAnalysis<'a, C> {
    fn new(resolver: &'a mut C, scope_id: ScopeId, name_token: TokenText) -> Self {
        Self {
            resolver,
            scope_id,
            name_token,
        }
    }

    /// The name of the binding being narrowed.
    fn name(&self) -> &str {
        self.name_token.text()
    }

    /// Returns the predicate the guards enclosing `id` establish for it.
    ///
    /// This is a purely syntactic check, scoped to the enclosing function. A
    /// guard whose consequent declares or assigns a binding with the same
    /// name is ignored, since it no longer says anything about that binding.
    ///
    /// Guards can nest on the same name; see [`record_guard_predicate`] for
    /// how the predicates of nested guards combine.
    fn narrowing_predicate(&mut self, id: &JsReferenceIdentifier) -> Option<NarrowingPredicate> {
        let mut child = id.syntax().clone();
        let mut found = None;
        for ancestor in id.syntax().ancestors().skip(1) {
            if let Some(if_stmt) = JsIfStatement::cast_ref(&ancestor) {
                if if_stmt
                    .consequent()
                    .is_ok_and(|consequent| consequent.syntax() == &child)
                    && let Some(predicate) = self.guard_predicate(&if_stmt)
                    && !self.narrowing_invalidated_within(&child, self.name_token.clone())
                    && !record_guard_predicate(&mut found, predicate)
                {
                    return None;
                }
            } else if let Some(case_clause) = JsCaseClause::cast_ref(&ancestor) {
                if case_clause.test().is_ok_and(|test| test.syntax() != &child)
                    && let Some(predicate) = self.switch_case_predicate(&case_clause)
                    && !self.narrowing_invalidated_within(&ancestor, self.name_token.clone())
                    && !record_guard_predicate(&mut found, predicate)
                {
                    return None;
                }
            } else if is_narrowing_boundary(&ancestor) {
                break;
            }
            child = ancestor;
        }
        found
    }

    /// Returns the predicate that the given `case` clause establishes for
    /// references with the narrowed name in its statements, if any.
    ///
    /// Narrowing only applies when every preceding clause of the `switch`
    /// statement provably exits, since execution could otherwise fall through
    /// into the clause while the discriminant held a different value. The
    /// tests of preceding clauses still evaluate in order even when their
    /// clauses are not entered, so a write to the name inside one of them
    /// also declines narrowing:
    ///
    /// ```js
    /// switch (x) {
    ///   case (x = 5, "nope"): break; // evaluates before "a" is tested
    ///   case "a":
    ///     x; // not narrowed: `x` no longer holds the matched value
    /// }
    /// ```
    fn switch_case_predicate(&mut self, case_clause: &JsCaseClause) -> Option<NarrowingPredicate> {
        let test = case_clause.test().ok()?.omit_parentheses();
        let value = string_literal_value(&test)?;

        let switch_stmt = case_clause
            .syntax()
            .ancestors()
            .find_map(JsSwitchStatement::cast)?;
        let discriminant = switch_stmt.discriminant().ok()?.omit_parentheses();
        let member = if is_reference_to(&discriminant, self.name()) {
            None
        } else {
            Some(member_of_reference(&discriminant, self.name())?)
        };

        for clause in switch_stmt.cases() {
            if clause.syntax() == case_clause.syntax() {
                break;
            }
            if !clause_provably_exits(&clause) {
                return None;
            }
            if let AnyJsSwitchClause::JsCaseClause(preceding) = &clause
                && let Ok(preceding_test) = preceding.test()
            {
                if self
                    .narrowing_invalidated_within(preceding_test.syntax(), self.name_token.clone())
                {
                    return None;
                }
                // For a member discriminant, a preceding test can also overwrite
                // the compared member itself.
                if member.is_some() && self.member_write_invalidated_within(preceding_test.syntax())
                {
                    return None;
                }
            }
        }

        match member {
            None => Some(NarrowingPredicate::StringEquals(value)),
            Some(member) => {
                // Writing to a member of the narrowed value inside the clause
                // could change the compared member.
                if self.member_write_invalidated_within(case_clause.syntax()) {
                    return None;
                }
                Some(NarrowingPredicate::MemberEquals(Box::new(
                    MemberEqualsPredicate { member, value },
                )))
            }
        }
    }

    /// Returns the predicate that the test of the given `if` statement
    /// establishes for references with the narrowed name in its consequent,
    /// if any.
    fn guard_predicate(&mut self, if_stmt: &JsIfStatement) -> Option<NarrowingPredicate> {
        let test = if_stmt.test().ok()?.omit_parentheses();
        match &test {
            // `if (x)`
            AnyJsExpression::JsIdentifierExpression(_) => {
                is_reference_to(&test, self.name()).then_some(NarrowingPredicate::Truthy)
            }
            // `if (isFoo(x))`
            AnyJsExpression::JsCallExpression(call) => self
                .predicate_call_guard(if_stmt, call)
                .map(|predicate| NarrowingPredicate::PredicateCall(Box::new(predicate))),
            // `if (x instanceof Class)`
            AnyJsExpression::JsInstanceofExpression(instanceof) => self
                .instanceof_guard_class(if_stmt, instanceof)
                .map(NarrowingPredicate::InstanceOf),
            // `if (!x)`
            AnyJsExpression::JsUnaryExpression(unary)
                if matches!(unary.operator(), Ok(JsUnaryOperator::LogicalNot)) =>
            {
                let argument = unary.argument().ok()?.omit_parentheses();
                is_reference_to(&argument, self.name()).then_some(NarrowingPredicate::Falsy)
            }
            // `if (typeof x === "<tag>")`, `if (x.member === "<value>")`, or
            // `if (x === "<value>")`
            AnyJsExpression::JsBinaryExpression(binary) => typeof_guard_tag(binary, self.name())
                .map(NarrowingPredicate::Typeof)
                .or_else(|| {
                    self.member_equals_guard(if_stmt, binary)
                        .map(|predicate| NarrowingPredicate::MemberEquals(Box::new(predicate)))
                })
                .or_else(|| {
                    string_equals_guard(binary, self.name()).map(NarrowingPredicate::StringEquals)
                }),
            _ => None,
        }
    }

    /// Returns the predicate of a `<name>.<member> === "<value>"` comparison,
    /// if the given binary expression is one.
    ///
    /// Handles both operand orders. Loose equality is not accepted: a test like
    /// `x.kind == "1"` also passes when the member holds the number `1`, so
    /// stripping the variants whose member is not the string `"1"` would narrow
    /// away the value actually present at runtime.
    fn member_equals_guard(
        &mut self,
        if_stmt: &JsIfStatement,
        binary: &JsBinaryExpression,
    ) -> Option<MemberEqualsPredicate> {
        if !matches!(binary.operator().ok()?, JsBinaryOperator::StrictEquality) {
            return None;
        }

        // Writing to a member of the narrowed value inside the consequent could
        // change the compared member.
        if let Ok(consequent) = if_stmt.consequent()
            && self.member_write_invalidated_within(consequent.syntax())
        {
            return None;
        }

        let left = binary.left().ok()?.omit_parentheses();
        let right = binary.right().ok()?.omit_parentheses();
        member_of_reference(&left, self.name())
            .zip(string_literal_value(&right))
            .or_else(|| member_of_reference(&right, self.name()).zip(string_literal_value(&left)))
            .map(|(member, value)| MemberEqualsPredicate { member, value })
    }

    /// Returns the predicate of an `isFoo(<name>)`-style call, if the given
    /// call expression passes a reference with the narrowed name as one of its
    /// arguments.
    ///
    /// Whether the callee is an actual type predicate is only decided during
    /// resolution. A spread among the arguments before the reference makes the
    /// mapping from its position to the callee's parameters ambiguous at
    /// runtime, so no predicate is returned then.
    fn predicate_call_guard(
        &mut self,
        if_stmt: &JsIfStatement,
        call: &JsCallExpression,
    ) -> Option<PredicateCallPredicate> {
        let callee = call.callee().ok()?.omit_parentheses();
        let callee_name = callee
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .name()
            .ok()?;

        // The callee reference is resolved from the scope of the narrowed
        // reference. A same-name binding declared in the consequent would
        // shadow the callee the guard actually invoked.
        if let Ok(consequent) = if_stmt.consequent()
            && self.narrowing_invalidated_within(consequent.syntax(), callee_name.clone())
        {
            return None;
        }

        let mut argument_index = None;
        for (index, argument) in call.arguments().ok()?.args().iter().enumerate() {
            let Ok(AnyJsCallArgument::AnyJsExpression(expression)) = argument else {
                return None;
            };
            if is_reference_to(&expression.omit_parentheses(), self.name()) {
                argument_index = Some(index);
                break;
            }
        }

        Some(PredicateCallPredicate {
            callee: TypeReference::from_name(self.scope_id, callee_name),
            argument_index: argument_index?,
        })
    }

    /// Returns a reference to the class an `instanceof` guard over a reference
    /// with the narrowed name checks against, if the given expression is one.
    fn instanceof_guard_class(
        &mut self,
        if_stmt: &JsIfStatement,
        instanceof: &JsInstanceofExpression,
    ) -> Option<TypeReference> {
        let left = instanceof.left().ok()?.omit_parentheses();
        if !is_reference_to(&left, self.name()) {
            return None;
        }

        let right = instanceof.right().ok()?.omit_parentheses();
        let class_name = right
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .name()
            .ok()?;

        // The class reference is resolved from the scope of the narrowed
        // reference. A same-name binding declared in the consequent would shadow
        // the class the guard actually checked against.
        if let Ok(consequent) = if_stmt.consequent()
            && self.narrowing_invalidated_within(consequent.syntax(), class_name.clone())
        {
            return None;
        }

        Some(TypeReference::from_name(self.scope_id, class_name))
    }

    /// Returns the predicate established by the nearest preceding assignment
    /// to the narrowed binding, if there is one.
    ///
    /// The assignment's right-hand side is taken as the collector already
    /// inferred it, so a collector that records no expression types declines
    /// assignment narrowing rather than inferring the value a second time.
    fn assignment_predicate(&mut self, id: &JsReferenceIdentifier) -> Option<NarrowingPredicate> {
        let source = self.assignment_source(id)?;
        let ty = self.resolver.recorded_expression_type(&source)?;
        Some(NarrowingPredicate::Assigned(ty))
    }

    /// Returns the right-hand side of the nearest assignment to the narrowed
    /// binding that precedes `id` in the same statement list, if the
    /// assignment provably determines the reference's value.
    ///
    /// This is a purely syntactic check over the innermost enclosing
    /// statement list: a reference inside a nested block, an `if` consequent,
    /// or a `case` clause is never narrowed by an assignment in the block
    /// around it. It bails out conservatively when any statement between the
    /// assignment and the reference -- or the reference's own statement,
    /// since loops re-evaluate their heads -- could write the value again or
    /// shadow its binding.
    fn assignment_source(&mut self, id: &JsReferenceIdentifier) -> Option<AnyJsExpression> {
        let containing_stmt = id
            .syntax()
            .ancestors()
            .skip(1)
            .take_while(|ancestor| !is_narrowing_boundary(ancestor))
            .find(|ancestor| {
                ancestor
                    .parent()
                    .is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_STATEMENT_LIST)
            })?;

        // Every reference in a statement list reaches this point, so rule out
        // the common case -- a name the list never writes to -- with a single
        // scan that the whole list shares, before walking its statements one
        // by one.
        let statement_list = containing_stmt.parent()?;
        if !self.narrowing_invalidated_within(&statement_list, self.name_token.clone()) {
            return None;
        }

        if self.narrowing_invalidated_within(&containing_stmt, self.name_token.clone()) {
            return None;
        }

        let mut sibling = containing_stmt.prev_sibling();
        while let Some(stmt) = sibling {
            if let Some(source) = plain_assignment_rhs(&stmt, self.name()) {
                // A write within the right-hand side itself, such as a closure
                // that reassigns the value, could occur after the assignment.
                return (!self
                    .narrowing_invalidated_within(source.syntax(), self.name_token.clone()))
                .then_some(source);
            }
            if self.narrowing_invalidated_within(&stmt, self.name_token.clone()) {
                return None;
            }
            sibling = stmt.prev_sibling();
        }
        None
    }

    /// Returns whether `name_token` is invalidated as a narrowing target
    /// somewhere inside `node`: either a `JsIdentifierBinding` with that name
    /// is declared there, or the name is assigned to (written) within `node`.
    ///
    /// The scan is deliberately conservative: a write anywhere in `node` invalidates
    /// every reference in it, even ones that precede the write. It also keys on
    /// syntax kind rather than on scopes, so `enum name {}` counts (its id is a
    /// `JsIdentifierBinding`) while `namespace name {}` does not (its name is a
    /// `TsIdentifierBinding`). Neither outcome is observable: a reference under
    /// such a declaration resolves to it, not to the guarded binding.
    ///
    /// This runs once per reference identifier inside a guarded consequent, so
    /// a branch with many references would otherwise re-scan the same subtree
    /// repeatedly. The result is memoized in the resolver's
    /// [narrowing invalidation cache](RawTypeCollector::narrowing_invalidation_cache).
    fn narrowing_invalidated_within(&mut self, node: &JsSyntaxNode, name_token: TokenText) -> bool {
        let key = (
            node.clone(),
            Text::from(name_token),
            NarrowingInvalidationKind::Binding,
        );

        if let Some(&cached) = self.resolver.narrowing_invalidation_cache().get(&key) {
            return cached;
        }

        let name = key.1.text();
        let invalidated = node.descendants().any(|descendant| {
            let name_token = if let Some(binding) = JsIdentifierBinding::cast_ref(&descendant) {
                binding.name_token()
            } else if let Some(assignment) = JsIdentifierAssignment::cast_ref(&descendant) {
                assignment.name_token()
            } else {
                return false;
            };
            name_token.is_ok_and(|token| token.text_trimmed() == name)
        });

        self.resolver
            .narrowing_invalidation_cache()
            .insert(key, invalidated);

        invalidated
    }

    /// Returns whether a member of the value with the narrowed name is written
    /// to within `node`, e.g. `name.member = 1` or `name[key] = 1`.
    ///
    /// Like [`Self::narrowing_invalidated_within`], the scan is deliberately
    /// conservative: a member write anywhere in `node` counts, even one that cannot
    /// execute before the reference being narrowed. Results are memoized in the
    /// resolver's narrowing invalidation cache, under
    /// [`NarrowingInvalidationKind::MemberWrite`].
    fn member_write_invalidated_within(&mut self, node: &JsSyntaxNode) -> bool {
        let name = self.name_token.text();
        let key = (
            node.clone(),
            Text::from(self.name_token.clone()),
            NarrowingInvalidationKind::MemberWrite,
        );

        if let Some(&cached) = self.resolver.narrowing_invalidation_cache().get(&key) {
            return cached;
        }

        let invalidated = node.descendants().any(|descendant| {
            let object = match descendant.kind() {
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    JsStaticMemberAssignment::cast(descendant)
                        .and_then(|assignment| assignment.object().ok())
                }
                JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    JsComputedMemberAssignment::cast(descendant)
                        .and_then(|assignment| assignment.object().ok())
                }
                _ => return false,
            };
            object
                .map(AnyJsExpression::omit_parentheses)
                .is_some_and(|object| is_reference_to(&object, name))
        });

        self.resolver
            .narrowing_invalidation_cache()
            .insert(key, invalidated);

        invalidated
    }
}

/// Returns whether execution provably exits at the end of the given clause,
/// instead of falling through to the next one.
///
/// Only a `break`, `continue`, `return`, or `throw` as the clause's last
/// statement counts; an exit nested in a block or an `if` is not detected,
/// so such clauses conservatively decline narrowing for their successors.
fn clause_provably_exits(clause: &AnyJsSwitchClause) -> bool {
    let statements = match clause {
        AnyJsSwitchClause::JsCaseClause(clause) => clause.consequent(),
        AnyJsSwitchClause::JsDefaultClause(clause) => clause.consequent(),
    };
    statements.iter().last().is_some_and(|last| {
        matches!(
            last.syntax().kind(),
            JsSyntaxKind::JS_BREAK_STATEMENT
                | JsSyntaxKind::JS_CONTINUE_STATEMENT
                | JsSyntaxKind::JS_RETURN_STATEMENT
                | JsSyntaxKind::JS_THROW_STATEMENT
        )
    })
}

/// Records the predicate of the next enclosing guard, keeping the innermost
/// one.
///
/// The innermost guard is the most specific, so an outer guard of another
/// kind does not replace it:
///
/// ```js
/// if (x) {
///   if (typeof x === "string") {
///     x; // narrowed by the `typeof` guard; the truthiness guard adds nothing
///   }
/// }
/// ```
///
/// The exception is nested guards comparing the same value or member
/// against different literals: `typeof` against different tags,
/// `StringEquals` against different strings, or `MemberEquals` on the same
/// member against different strings. Each of these can only hold for one
/// value at a time, so the tests cannot both have passed for the same
/// value, and we decline to narrow rather than pick one of the literals:
///
/// ```js
/// if (typeof x === "number") {
///   if (typeof x === "string") {
///     x; // not narrowed: `typeof x` cannot be both "number" and "string"
///   }
/// }
/// ```
///
/// Returns `false` for the contradicting cases above, without touching
/// `found`. Otherwise returns `true`: `found` is set to `predicate` if it
/// was empty, and left as-is (keeping the innermost, more specific guard
/// already found) otherwise.
fn record_guard_predicate(
    found: &mut Option<NarrowingPredicate>,
    predicate: NarrowingPredicate,
) -> bool {
    match (&found, &predicate) {
        (Some(NarrowingPredicate::Typeof(existing)), NarrowingPredicate::Typeof(tag))
            if existing != tag =>
        {
            return false;
        }
        (
            Some(NarrowingPredicate::StringEquals(existing)),
            NarrowingPredicate::StringEquals(value),
        ) if existing != value => {
            return false;
        }
        (
            Some(NarrowingPredicate::MemberEquals(existing)),
            NarrowingPredicate::MemberEquals(next),
        ) if existing.member == next.member && existing.value != next.value => {
            return false;
        }
        (Some(_), _) => {}
        (None, _) => *found = Some(predicate),
    }
    true
}

/// Returns the string of a `<name> === "<value>"` comparison, if the given
/// binary expression is one.
///
/// Handles both operand orders. Loose equality is not accepted: a test like
/// `x == "1"` also passes when `x` holds the number `1`, so stripping the
/// variants that are not the string `"1"` would narrow away the value
/// actually present at runtime.
fn string_equals_guard(binary: &JsBinaryExpression, name: &str) -> Option<Text> {
    if !matches!(binary.operator().ok()?, JsBinaryOperator::StrictEquality) {
        return None;
    }

    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();
    if is_reference_to(&left, name) {
        string_literal_value(&right)
    } else if is_reference_to(&right, name) {
        string_literal_value(&left)
    } else {
        None
    }
}

/// Returns the member name of a `<name>.<member>` expression.
fn member_of_reference(expr: &AnyJsExpression, name: &str) -> Option<Text> {
    let AnyJsExpression::JsStaticMemberExpression(member_expr) = expr else {
        return None;
    };
    let object = member_expr.object().ok()?.omit_parentheses();
    if !is_reference_to(&object, name) {
        return None;
    }

    let member = member_expr.member().ok()?;
    Some(
        member
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed()
            .into(),
    )
}

/// Returns the unescaped value of a string literal expression.
fn string_literal_value(expr: &AnyJsExpression) -> Option<Text> {
    let literal = expr
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;
    unescaped_text_from_token(literal.value_token())
}

/// Returns the tag of a `typeof <name> === "<tag>"` comparison, if the given
/// binary expression is one.
///
/// Handles both operand orders, and treats `==` like `===`.
fn typeof_guard_tag(binary: &JsBinaryExpression, name: &str) -> Option<TypeofTag> {
    if !matches!(
        binary.operator().ok()?,
        JsBinaryOperator::StrictEquality | JsBinaryOperator::Equality
    ) {
        return None;
    }

    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();
    if is_typeof_of(&left, name) {
        typeof_tag_from_literal(&right)
    } else if is_typeof_of(&right, name) {
        typeof_tag_from_literal(&left)
    } else {
        None
    }
}

/// Returns whether `expr` is a reference to a value with the given `name`.
fn is_reference_to(expr: &AnyJsExpression, name: &str) -> bool {
    expr.as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok())
        .and_then(|reference| reference.name().ok())
        .is_some_and(|reference_name| reference_name.text() == name)
}

/// Returns whether `expr` is a `typeof` expression over a reference with the
/// given `name`.
fn is_typeof_of(expr: &AnyJsExpression, name: &str) -> bool {
    let AnyJsExpression::JsUnaryExpression(unary) = expr else {
        return false;
    };
    if !matches!(unary.operator(), Ok(JsUnaryOperator::Typeof)) {
        return false;
    }
    unary
        .argument()
        .ok()
        .map(AnyJsExpression::omit_parentheses)
        .is_some_and(|argument| is_reference_to(&argument, name))
}

fn typeof_tag_from_literal(expr: &AnyJsExpression) -> Option<TypeofTag> {
    let literal = expr
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;
    TypeofTag::from_literal(literal.inner_string_text().ok()?.text())
}

/// Returns whether `node` is a boundary that `typeof` narrowing must not
/// reach into. A guard only vouches for the value at the time its test runs,
/// so it says nothing about code whose execution is deferred:
///
/// - function-like scopes ([`biome_js_syntax::is_function_boundary`]):
///   functions, methods, constructors, getters, setters, and static
///   initialization blocks;
/// - class property members, whose initializers run when the class is
///   instantiated:
///
/// ```js
/// if (typeof x === "number") {
///   return class { p = x }; // `p` is initialized later, `x` may have changed
/// }
/// ```
///
/// A `static` field is evaluated with the class expression itself, so
/// narrowing it would be correct. We treat the whole class body as one
/// boundary anyway, rather than deciding per member.
fn is_narrowing_boundary(node: &JsSyntaxNode) -> bool {
    biome_js_syntax::is_function_boundary(node.kind())
        || matches!(
            node.kind(),
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER
                | JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
        )
}
