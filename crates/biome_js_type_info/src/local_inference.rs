//! Collects raw type information from one JavaScript or TypeScript syntax tree.
//!
//! This pass records local declarations, expressions, and references. Database
//! queries resolve imports and turn these raw entries into inferred types later.

use std::borrow::Cow;
use std::str::FromStr;

use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsArrayElement, AnyJsArrowFunctionParameters,
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsClass,
    AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsDeclaration,
    AnyJsDeclarationClause, AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsFormalParameter,
    AnyJsFunction, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsParameter,
    AnyJsStatement, AnyJsSwitchClause, AnyJsTemplateElement, AnyJsTryStatement,
    AnyTsEnumMemberName, AnyTsModuleName, AnyTsName, AnyTsReturnType, AnyTsTupleTypeElement,
    AnyTsType, AnyTsTypeMember, AnyTsTypePredicateParameterName, ClassMemberName,
    JsArrayBindingPattern, JsArrayExpression, JsArrowFunctionExpression, JsAssignmentExpression,
    JsBinaryExpression, JsBinaryOperator, JsBreakStatement, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression,
    JsClassMemberList, JsConstructorParameters, JsContinueStatement, JsDoWhileStatement, JsExport,
    JsExtendsClause, JsForInStatement, JsForOfStatement, JsForStatement, JsForVariableDeclaration,
    JsFormalParameter, JsFunctionBody, JsFunctionDeclaration, JsFunctionExpression,
    JsGetterObjectMember, JsIdentifierAssignment, JsIdentifierBinding, JsIdentifierExpression,
    JsInitializerClause, JsLanguage, JsLogicalExpression, JsLogicalOperator, JsMethodClassMember,
    JsMethodObjectMember, JsModuleItemList, JsNewExpression, JsObjectBindingPattern,
    JsObjectExpression, JsParameterList, JsParameters, JsParenthesizedExpression,
    JsPropertyClassMember, JsPropertyObjectMember, JsReferenceIdentifier, JsRestParameter,
    JsReturnStatement, JsSetterObjectMember, JsStatementList, JsSwitchStatement, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsTemplateExpression, JsUnaryExpression, JsUnaryOperator,
    JsVariableDeclaration, JsVariableDeclarator, JsWhileStatement, TsConditionalType,
    TsDeclarationModule, TsDeclareFunctionDeclaration, TsDeclareStatement, TsEnumDeclaration,
    TsExportDeclareClause, TsExternalModuleDeclaration, TsGlobalDeclaration, TsInferType,
    TsInstantiationExpression, TsInterfaceDeclaration, TsMappedType, TsMethodSignatureTypeMember,
    TsModuleBlock, TsModuleDeclaration, TsPropertyParameterModifierList, TsReferenceType,
    TsReturnTypeAnnotation, TsSatisfiesExpression, TsTypeAliasDeclaration, TsTypeAnnotation,
    TsTypeArguments, TsTypeList, TsTypeMemberList, TsTypeParameter, TsTypeParameters, TsTypeofType,
    inner_string_text, unescape_js_string,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, SyntaxResult, Text, TextRange,
    TokenText, WalkEvent, declare_node_union,
};
use rustc_hash::FxHashMap;

use crate::globals::{
    GLOBAL_GLOBAL_ID, GLOBAL_INSTANCEOF_PROMISE_ID, GLOBAL_NUMBER_KEYWORD_ID,
    GLOBAL_STRING_KEYWORD_ID, GLOBAL_UNDEFINED_ID,
};
use crate::literal::{BooleanLiteral, NumberLiteral, RegexpLiteral, StringLiteral};
use crate::{
    AssertsReturnType, CallArgumentType, Class, Constructor, ConstructorParameter,
    DestructureField, ExtendsType, Function, FunctionParameter, FunctionParameterBinding,
    GenericTypeParameter, IndexedAccessType, InferredPredicateReturnType, Interface, Intersection,
    Literal, MappedType, MappedTypeKeys, MappedTypeModifier, Module, NamedFunctionParameter,
    Namespace, Object, ObjectKind, Path, PatternFunctionParameter, PredicateReturnType,
    RawTypeCollector, RawTypeId, ReturnType, ScopeId, Tuple, TupleElementType, TypeData,
    TypeInstance, TypeMember, TypeMemberAccessibility, TypeMemberKind, TypeOperator,
    TypeOperatorType, TypeReference, TypeReferenceQualifier, TypeofAdditionExpression,
    TypeofAwaitExpression, TypeofBitwiseNotExpression, TypeofCallArgumentExpression,
    TypeofCallExpression, TypeofComputedMemberExpression, TypeofConditionalExpression,
    TypeofDestructureExpression, TypeofExpression, TypeofImplicitReturnExpression,
    TypeofIndexExpression, TypeofInferredPredicateExpression, TypeofIterableValueOfExpression,
    TypeofLogicalAndExpression, TypeofLogicalOrExpression, TypeofNewExpression,
    TypeofNullishCoalescingExpression, TypeofParameterExpression, TypeofStaticMemberExpression,
    TypeofThisOrSuperExpression, TypeofTypeofExpression, TypeofUnaryMinusExpression, TypeofValue,
    TypeofWidenedLiteralExpression, Union,
};

const MAX_CONST_ASSERTION_DEPTH: usize = 50;

/// Bounds the raw references and union members that
/// [`may_have_fresh_literal`] follows.
const MAX_FRESH_LITERAL_SHAPE_DEPTH: usize = 8;

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
                let init = elem.init().and_then(|init| init.expression().ok());
                let field = DestructureField::Index {
                    index: i,
                    has_initializer: init.is_some(),
                };
                match elem.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(name, {
                            let data = Self::defaulted_destructuring_of(
                                collector, scope_id, reference, field, init,
                            );
                            collector.reference_to_owned_data(data)
                        })])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::defaulted_destructuring_of(
                            collector, scope_id, reference, field, init,
                        );
                        data.apply_array_binding_pattern(collector, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = Self::defaulted_destructuring_of(
                            collector, scope_id, reference, field, init,
                        );
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
                let init = prop.init().and_then(|init| init.expression().ok());
                let field = DestructureField::Name {
                    name: member_name?,
                    has_initializer: init.is_some(),
                };
                match prop.pattern().ok()? {
                    AnyJsBindingPattern::AnyJsBinding(binding) => Some({
                        let binding = binding.as_js_identifier_binding()?;
                        let name = text_from_token(binding.name_token())?;
                        Box::new([(name, {
                            let data = Self::defaulted_destructuring_of(
                                collector, scope_id, reference, field, init,
                            );
                            collector.reference_to_owned_data(data)
                        })])
                    }),
                    AnyJsBindingPattern::JsArrayBindingPattern(pattern) => Some({
                        let data = Self::defaulted_destructuring_of(
                            collector, scope_id, reference, field, init,
                        );
                        data.apply_array_binding_pattern(collector, scope_id, &pattern)
                    }),
                    AnyJsBindingPattern::JsObjectBindingPattern(pattern) => Some({
                        let data = Self::defaulted_destructuring_of(
                            collector, scope_id, reference, field, init,
                        );
                        data.apply_object_binding_pattern(collector, scope_id, &pattern)
                    }),
                }
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(prop) => {
                Some({
                    let member_name = member_name?;
                    let init = prop.init().and_then(|init| init.expression().ok());
                    let field = DestructureField::Name {
                        name: member_name.clone(),
                        has_initializer: init.is_some(),
                    };
                    let data = Self::defaulted_destructuring_of(
                        collector, scope_id, reference, field, init,
                    );
                    Box::new([(member_name, collector.reference_to_owned_data(data))])
                })
            }
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

    /// Types a destructured binding. With a default, the binding is either the
    /// destructured value without `undefined` or the default's type.
    fn defaulted_destructuring_of(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: TypeReference,
        destructure_field: DestructureField,
        default: Option<AnyJsExpression>,
    ) -> Self {
        let destructured = Self::destructuring_of(ty, destructure_field);
        let Some(default) = default else {
            return destructured;
        };
        let destructured = collector.reference_to_owned_data(destructured);
        let default = collector.reference_to_resolved_expression(scope_id, &default);
        Self::Union(Box::new(Union(Box::new([destructured, default]))))
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
            AnyJsDeclaration::TsEnumDeclaration(decl) => Self::reference(
                EnumTypes::from_ts_enum_declaration(collector, scope_id, decl).declaration,
            ),
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
                Self::from_ts_module_declaration(collector, decl).unwrap_or_default()
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
            .map_or_default(|decl| Self::from_any_js_declaration(collector, scope_id, &decl))
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
                    type_parameters: decl.type_parameters().map_or_default(|params| {
                        TypeReference::types_from_ts_type_parameters(collector, scope_id, &params)
                    }),
                    extends: decl.extends_clause().and_then(|extends| {
                        reference_to_extends_clause(collector, scope_id, extends)
                    }),
                    implements: decl.implements_clause().map_or_default(|implements| {
                        TypeReference::types_from_ts_type_list(
                            collector,
                            scope_id,
                            implements.types(),
                        )
                    }),
                    interface_bases: Box::default(),
                    members: TypeMember::collect_class_members(collector, scope_id, decl.members())
                        .into_boxed_slice(),
                    scope_id: collector.scope_for_node(decl.syntax()),
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
                    return_type: {
                        let return_type = function_return_type(
                            collector,
                            scope_id,
                            is_async,
                            decl.return_type_annotation(),
                            decl.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                        );
                        with_inferred_predicate(
                            collector,
                            &AnyJsFunction::from(decl.clone()).into(),
                            return_type,
                        )
                    },
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
            AnyJsExpression::JsArrayExpression(expr) => {
                let context = ArrayExpressionContext::of(expr);
                Self::Tuple(Box::new(Tuple {
                    elements: expr
                        .elements()
                        .into_iter()
                        .filter_map(|el| match el {
                            Ok(AnyJsArrayElement::AnyJsExpression(expr)) => {
                                let ty = value_reference(collector, scope_id, &expr);
                                Some(TupleElementType {
                                    // TypeScript widens the elements of the value
                                    // that a destructuring pattern takes apart,
                                    // because the pattern gives them no literal
                                    // type, so `const [a] = [1]` declares `a` as
                                    // `number`.
                                    ty: if context == ArrayExpressionContext::Destructured {
                                        widened_literal_reference(collector, ty, false)
                                    } else {
                                        ty
                                    },
                                    name: None,
                                    is_optional: false,
                                    is_rest: false,
                                })
                            }
                            Ok(AnyJsArrayElement::JsSpread(spread)) => spread
                                .argument()
                                .ok()
                                .map(|expr| {
                                    collector.reference_to_resolved_expression(scope_id, &expr)
                                })
                                .map(|ty| TupleElementType {
                                    ty,
                                    name: None,
                                    is_optional: false,
                                    is_rest: true,
                                }),
                            Ok(AnyJsArrayElement::JsArrayHole(_)) | Err(_) => {
                                Some(TupleElementType {
                                    ty: TypeReference::unknown(),
                                    name: None,
                                    is_optional: false,
                                    is_rest: false,
                                })
                            }
                        })
                        .collect(),
                    is_inferred_array: context == ArrayExpressionContext::Array,
                }))
            }
            AnyJsExpression::JsArrowFunctionExpression(expr) => {
                Self::from_js_arrow_function_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsAwaitExpression(expr) => {
                Self::from(TypeofExpression::Await(TypeofAwaitExpression {
                    argument: expr.argument().map_or_default(|arg| {
                        collector.reference_to_resolved_expression(scope_id, &arg)
                    }),
                }))
            }
            AnyJsExpression::JsBinaryExpression(expr) => {
                Self::from_js_binary_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsCallExpression(expr) => match expr.callee() {
                Ok(callee) => Self::from(TypeofExpression::Call(TypeofCallExpression {
                    callee: callee_reference(collector, scope_id, &callee, expr.type_arguments()),
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
                    ) => unescaped_text_from_token(member.value_token()).map_or_default(|member| {
                        let expression = TypeofStaticMemberExpression {
                            object: collector.reference_to_resolved_expression(scope_id, &object),
                            member,
                        };
                        Self::from(if expr.is_optional_chain() {
                            TypeofExpression::OptionalChainStaticMember(expression)
                        } else {
                            TypeofExpression::StaticMember(expression)
                        })
                    }),
                    (
                        Ok(object),
                        Ok(AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsNumberLiteralExpression(member),
                        )),
                    ) => unescaped_text_from_token(member.value_token()).map_or_default(|member| {
                        match member.parse() {
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
                        }
                    }),
                    (Ok(object), Ok(member)) => Self::from(TypeofExpression::ComputedMember(
                        TypeofComputedMemberExpression {
                            object: collector.reference_to_resolved_expression(scope_id, &object),
                            member: collector.reference_to_resolved_expression(scope_id, &member),
                            is_optional_chain: expr.is_optional_chain(),
                        },
                    )),
                    _ => Self::unknown(),
                }
            }
            AnyJsExpression::JsConditionalExpression(expr) => {
                Self::from(TypeofExpression::Conditional(TypeofConditionalExpression {
                    test: expr.test().map_or_default(|sub| {
                        collector.reference_to_resolved_expression(scope_id, &sub)
                    }),
                    consequent: expr.consequent().map_or_default(|sub| {
                        collector.reference_to_resolved_expression(scope_id, &sub)
                    }),
                    alternate: expr.alternate().map_or_default(|sub| {
                        collector.reference_to_resolved_expression(scope_id, &sub)
                    }),
                }))
            }
            AnyJsExpression::JsFunctionExpression(expr) => {
                Self::from_js_function_expression(collector, scope_id, expr)
            }
            AnyJsExpression::JsIdentifierExpression(expr) => expr
                .name()
                .map_or_default(|name| Self::from_js_reference_identifier(scope_id, &name)),
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
                .map_or_default(|expr| collector.resolve_expression(scope_id, &expr).into_owned()),
            AnyJsExpression::JsPostUpdateExpression(_)
            | AnyJsExpression::JsPreUpdateExpression(_) => Self::number(),
            AnyJsExpression::JsSequenceExpression(expr) => expr
                .right()
                .map_or_default(|expr| collector.resolve_expression(scope_id, &expr).into_owned()),
            AnyJsExpression::JsStaticMemberExpression(expr) => match (expr.object(), expr.member())
            {
                (Ok(object), Ok(member)) => {
                    text_from_any_js_name(member).map_or_default(|member| {
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
                }
                _ => Self::unknown(),
            },
            AnyJsExpression::JsSuperExpression(_) => Self::from(TypeofExpression::Super(
                TypeofThisOrSuperExpression::from_any_js_expression(scope_id, expr),
            )),
            AnyJsExpression::JsTemplateExpression(expr) => Self::from_js_template_expression(expr),
            AnyJsExpression::JsThisExpression(_) => match this_parameter_type(expr) {
                Some(annotation) => Self::from_any_ts_type(collector, scope_id, &annotation),
                None => Self::from(TypeofExpression::This(
                    TypeofThisOrSuperExpression::from_any_js_expression(scope_id, expr),
                )),
            },
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
                    let ty = Self::from_any_ts_type(collector, scope_id, &annotation);
                    asserted_type(collector, ty)
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
                    let ty = Self::from_any_ts_type(collector, scope_id, &annotation);
                    asserted_type(collector, ty)
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
                    .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
            ),
            AnyTsType::TsBigintLiteralType(ty) => match (ty.minus_token(), ty.literal_token()) {
                (Some(_), Ok(literal_token)) => Self::Literal(Box::new(Literal::BigInt(
                    format!("-{}", literal_token.text_trimmed()).into(),
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
                Self::from_ts_conditional_type(collector, scope_id, ty)
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
            AnyTsType::TsIndexedAccessType(ty) => match (ty.object_type(), ty.index_type()) {
                (Ok(object), Ok(index)) => Self::IndexedAccess(Box::new(IndexedAccessType {
                    object: TypeReference::from_any_ts_type(collector, scope_id, &object),
                    index: TypeReference::from_any_ts_type(collector, scope_id, &index),
                })),
                _ => Self::unknown(),
            },
            AnyTsType::TsInferType(ty) => {
                GenericTypeParameter::from_ts_infer_type(collector, scope_id, ty)
                    .map_or_else(Self::unknown, Self::from)
            }
            AnyTsType::TsIntersectionType(ty) => Self::Intersection(Box::new(Intersection(
                ty.types()
                    .into_iter()
                    .flatten()
                    .map(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty))
                    .collect(),
            ))),
            AnyTsType::TsMappedType(ty) => Self::from_ts_mapped_type(collector, scope_id, ty),
            AnyTsType::TsNeverType(_) => Self::NeverKeyword,
            AnyTsType::TsNonPrimitiveType(_) => Self::ObjectKeyword,
            AnyTsType::TsNullLiteralType(_) => Self::Null,
            AnyTsType::TsNumberLiteralType(ty) => match ty.literal_token() {
                Ok(token) if ty.minus_token().is_some() => Literal::Number(NumberLiteral::new(
                    format!("-{}", token.text_trimmed()).into(),
                ))
                .into(),
                Ok(token) => {
                    Literal::Number(NumberLiteral::new(token.token_text_trimmed().into())).into()
                }
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsNumberType(_) => Self::reference(GLOBAL_NUMBER_KEYWORD_ID),
            AnyTsType::TsObjectType(ty) => {
                let mut has_unknown_members = false;
                let members = TypeMember::collect_type_members(collector, scope_id, ty.members())
                    .into_iter()
                    .filter_map(|member| {
                        has_unknown_members |= member.is_none();
                        member
                    })
                    .collect();
                Self::Object(Box::new(Object {
                    prototype: None,
                    members,
                    has_unknown_members,
                    kind: ObjectKind::Shape,
                }))
            }
            AnyTsType::TsParenthesizedType(ty) => ty
                .ty()
                .map_or_default(|ty| Self::from_any_ts_type(collector, scope_id, &ty)),
            AnyTsType::TsReferenceType(ty) => Self::from_ts_reference_type(collector, scope_id, ty),
            AnyTsType::TsStringLiteralType(ty) => match ty.inner_string_text() {
                Ok(token) => Literal::String(token.text().into()).into(),
                Err(_) => Self::unknown(),
            },
            AnyTsType::TsStringType(_) => Self::reference(GLOBAL_STRING_KEYWORD_ID),
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
                    Ok(elements) => Self::Tuple(Box::new(Tuple {
                        elements,
                        is_inferred_array: false,
                    })),
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
        ty.map_or_default(|ty| Self::from_any_ts_type(collector, scope_id, &ty))
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
            return_type: {
                let return_type = function_return_type(
                    collector,
                    scope_id,
                    is_async,
                    expr.return_type_annotation(),
                    expr.body().ok(),
                );
                with_inferred_predicate(
                    collector,
                    &AnyJsFunction::from(expr.clone()).into(),
                    return_type,
                )
            },
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
            implements: decl.implements_clause().map_or_default(|implements| {
                TypeReference::types_from_ts_type_list(collector, scope_id, implements.types())
            }),
            interface_bases: Box::default(),
            members: TypeMember::members_from_class_member_list(
                collector,
                scope_id,
                decl.members(),
            ),
            scope_id: collector.scope_for_node(decl.syntax()),
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
            implements: decl.implements_clause().map_or_default(|implements| {
                TypeReference::types_from_ts_type_list(collector, scope_id, implements.types())
            }),
            interface_bases: Box::default(),
            members: TypeMember::members_from_class_member_list(
                collector,
                scope_id,
                decl.members(),
            ),
            scope_id: collector.scope_for_node(decl.syntax()),
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
            return_type: {
                let return_type = function_return_type(
                    collector,
                    scope_id,
                    is_async,
                    decl.return_type_annotation(),
                    decl.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                );
                with_inferred_predicate(
                    collector,
                    &AnyJsFunction::from(decl.clone()).into(),
                    return_type,
                )
            },
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
            return_type: {
                let return_type = function_return_type(
                    collector,
                    scope_id,
                    is_async,
                    expr.return_type_annotation(),
                    expr.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                );
                with_inferred_predicate(
                    collector,
                    &AnyJsFunction::from(expr.clone()).into(),
                    return_type,
                )
            },
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
            callee: callee_reference(
                collector,
                scope_id,
                &expr.callee().ok()?,
                expr.type_arguments(),
            ),
            arguments: CallArgumentType::types_from_js_call_arguments(
                collector,
                scope_id,
                expr.arguments(),
            ),
        })))
    }

    /// Types parameter `parameter_index` (not counting `this`) of the
    /// callback `function` as the corresponding parameter of the type
    /// expected for its argument position. Returns `None` when the callback
    /// is not a direct argument of a call or `new` expression.
    pub fn from_contextual_callback_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        function: &JsSyntaxNode,
        parameter_index: usize,
        has_initializer: bool,
    ) -> Option<Self> {
        let mut argument = function.clone();
        let mut parent = argument.parent()?;
        while JsParenthesizedExpression::can_cast(parent.kind()) {
            argument = parent;
            parent = argument.parent()?;
        }
        let argument_list = JsCallArgumentList::cast(parent)?;
        let argument_range = argument.text_trimmed_range();
        let argument_index = argument_list.iter().position(|item| {
            item.is_ok_and(|item| item.syntax().text_trimmed_range() == argument_range)
        })?;
        let call_like = argument_list
            .parent::<JsCallArguments>()?
            .syntax()
            .parent()?;
        let (callee, type_arguments, is_constructor) =
            if let Some(call) = JsCallExpression::cast_ref(&call_like) {
                (call.callee().ok()?, call.type_arguments(), false)
            } else {
                let new = JsNewExpression::cast_ref(&call_like)?;
                (new.callee().ok()?, new.type_arguments(), true)
            };

        let callee = callee_reference(collector, scope_id, &callee, type_arguments);
        let arguments = argument_list
            .iter()
            .enumerate()
            .map(|(index, item)| match item {
                Ok(item) if index != argument_index => {
                    CallArgumentType::from_any_js_call_argument(collector, scope_id, &item)
                }
                _ => CallArgumentType::Argument(TypeReference::unknown()),
            })
            .collect();

        let expected = collector.reference_to_owned_data(Self::from(
            TypeofExpression::CallArgument(TypeofCallArgumentExpression {
                callee,
                arguments,
                index: argument_index.try_into().ok()?,
                is_constructor,
            }),
        ));
        Some(Self::from(TypeofExpression::Parameter(
            TypeofParameterExpression {
                function: expected,
                index: parameter_index.try_into().ok()?,
                has_initializer,
            },
        )))
    }

    /// Contextual type of an unannotated parameter of a function expression
    /// or arrow function; see [`Self::from_contextual_callback_parameter`].
    pub fn from_contextual_js_formal_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &JsFormalParameter,
    ) -> Option<Self> {
        if param.type_annotation().is_some() {
            return None;
        }
        let param_range = param.syntax().text_trimmed_range();
        let parameter_list = param.parent::<JsParameterList>()?;
        let parameter_index = parameter_list
            .iter()
            .filter_map(Result::ok)
            .filter(|item| !matches!(item, AnyJsParameter::TsThisParameter(_)))
            .position(|item| item.syntax().text_trimmed_range() == param_range)?;
        let function = parameter_list.parent::<JsParameters>()?.syntax().parent()?;
        if !matches!(
            function.kind(),
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION | JsSyntaxKind::JS_FUNCTION_EXPRESSION
        ) {
            return None;
        }
        Self::from_contextual_callback_parameter(
            collector,
            scope_id,
            &function,
            parameter_index,
            param.initializer().is_some(),
        )
    }

    /// Contextual type of the single unparenthesised arrow parameter, as in
    /// `run(value => value)`; see [`Self::from_contextual_callback_parameter`].
    pub fn from_contextual_js_arrow_function_binding(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsArrowFunctionExpression,
    ) -> Option<Self> {
        match expr.parameters().ok()? {
            AnyJsArrowFunctionParameters::AnyJsBinding(_) => {
                Self::from_contextual_callback_parameter(
                    collector,
                    scope_id,
                    expr.syntax(),
                    0,
                    false,
                )
            }
            AnyJsArrowFunctionParameters::JsParameters(_) => None,
        }
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
            kind: ObjectKind::Expression,
        }))
    }

    pub fn from_js_reference_identifier(scope_id: ScopeId, id: &JsReferenceIdentifier) -> Self {
        id.name().map_or(Self::unknown(), |name| match name.text() {
            "globalThis" => Self::reference(GLOBAL_GLOBAL_ID),
            "undefined" => Self::Undefined,
            _ => Self::reference(TypeReference::from_name(scope_id, name)),
        })
    }

    pub fn from_js_template_expression(expr: &JsTemplateExpression) -> Self {
        // A tag function may return any type.
        if expr.tag().is_some() {
            return Self::unknown();
        }

        // Only templates without substitutions have a literal value.
        let mut elements = expr.elements().into_iter();
        let text = match (elements.next(), elements.next()) {
            (None, _) => Some(Text::default()),
            (Some(AnyJsTemplateElement::JsTemplateChunkElement(chunk)), None) => {
                text_from_token(chunk.template_chunk_token())
            }
            _ => None,
        };

        match text {
            Some(text) => Self::Literal(Box::new(Literal::String(StringLiteral::from(text)))),
            None => Self::string(),
        }
    }

    pub fn from_js_unary_expression(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        expr: &JsUnaryExpression,
    ) -> Self {
        if let Ok(operator @ (JsUnaryOperator::Minus | JsUnaryOperator::Plus)) = expr.operator()
            && let Ok(argument) = expr.argument()
            && let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) = argument.omit_parentheses()
            && let Some(text) = text_from_token(literal.value_token())
        {
            let text = if operator == JsUnaryOperator::Minus {
                format!("-{text}").into()
            } else {
                text
            };
            return Literal::Number(NumberLiteral::new(text)).into();
        }
        expr.operator().map_or_default(|operator| match operator {
            JsUnaryOperator::BitwiseNot => {
                Self::from(TypeofExpression::BitwiseNot(TypeofBitwiseNotExpression {
                    argument: expr.argument().map_or_default(|arg| {
                        collector.reference_to_resolved_expression(scope_id, &arg)
                    }),
                }))
            }
            JsUnaryOperator::Delete => Self::Boolean,
            JsUnaryOperator::Minus => {
                Self::from(TypeofExpression::UnaryMinus(TypeofUnaryMinusExpression {
                    is_literal_argument: expr.argument().is_ok_and(|argument| {
                        matches!(
                            argument,
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                                    | AnyJsLiteralExpression::JsBigintLiteralExpression(_)
                            )
                        )
                    }),
                    argument: expr.argument().map_or_default(|arg| {
                        collector.reference_to_resolved_expression(scope_id, &arg)
                    }),
                }))
            }
            JsUnaryOperator::LogicalNot => Self::Boolean,
            JsUnaryOperator::Plus => Self::Number,
            JsUnaryOperator::Typeof => {
                Self::from(TypeofExpression::Typeof(TypeofTypeofExpression {
                    argument: expr.argument().map_or_default(|arg| {
                        collector.reference_to_resolved_expression(scope_id, &arg)
                    }),
                }))
            }
            JsUnaryOperator::Void => Self::VoidKeyword,
        })
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
            extends: decl.extends_clause().map_or_default(|extends| {
                TypeReference::types_from_ts_type_list(collector, scope_id, extends.types())
            }),
            members: TypeMember::collect_type_members(collector, scope_id, decl.members())
                .into_iter()
                .map(|member| member.unwrap_or_else(TypeMember::uncollected))
                .collect(),
            scope_id: collector.scope_for_node(decl.syntax()),
        }))
    }

    pub fn from_ts_module_declaration(
        collector: &dyn RawTypeCollector,
        decl: &TsModuleDeclaration,
    ) -> Option<Self> {
        let mut enclosing = Vec::new();
        let mut outermost = decl.clone();
        while let Some(outer) = enclosing_namespace(&outermost) {
            push_reversed_module_name_parts(outer.name().ok()?, &mut enclosing)?;
            outermost = outer;
        }
        enclosing.reverse();
        let namespace = Namespace {
            path: path_from_any_ts_module_name(decl.name().ok()?)?,
            // We don't initialise members of namespaces during local inference.
            // This is because our semantic model will pick them up during
            // module-level inference, so we add references for them at that
            // time.
            members: Box::new([]),
            scope_id: collector.enclosing_scope_for_node(outermost.syntax()),
            enclosing: enclosing.into_boxed_slice(),
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
            .map_or_default(|qualifier| {
                Self::instance_of(TypeReference::from(qualifier.with_type_parameters(
                    TypeReference::types_from_ts_type_arguments(
                        collector,
                        scope_id,
                        ty.type_arguments(),
                    ),
                )))
            })
    }

    pub fn from_ts_conditional_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsConditionalType,
    ) -> Self {
        let check_type = ty.check_type();
        let distributive = check_type.as_ref().is_ok_and(is_naked_type_parameter);
        let check_type = check_type
            .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty));
        let extends_type = ty
            .extends_type()
            .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty));
        // Names declared by `infer` are scoped to the true branch.
        let true_type = ty.true_type().map_or_default(|true_type| {
            let scope_id = collector
                .scope_for_node(true_type.syntax())
                .unwrap_or(scope_id);
            TypeReference::from_any_ts_type(collector, scope_id, &true_type)
        });
        let false_type = ty
            .false_type()
            .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty));
        let infer_types = ty
            .extends_type()
            .map_or_default(|extends_type| infer_types_declared_by(&extends_type))
            .into_iter()
            .map(|infer| {
                TypeReference::from_any_ts_type(collector, scope_id, &AnyTsType::from(infer))
            })
            .collect();

        Self::Extends(Box::new(ExtendsType {
            check_type,
            extends_type,
            true_type,
            false_type,
            infer_types,
            distributive,
        }))
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

    /// Collects a mapped type such as `{ [K in keyof T]?: T[K] }`.
    ///
    /// The mapped type declares `K` in its own scope, so its keys and property
    /// type are collected from that scope. Key remapping with an `as` clause
    /// is not supported and results in an unknown type.
    pub fn from_ts_mapped_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsMappedType,
    ) -> Self {
        if ty.as_clause().is_some() {
            return Self::unknown();
        }
        let Some(type_parameter) = GenericTypeParameter::from_ts_mapped_type(ty) else {
            return Self::unknown();
        };

        let scope_id = collector.scope_for_node(ty.syntax()).unwrap_or(scope_id);
        let type_parameter = collector
            .register_and_resolve(Self::from(type_parameter))
            .into();
        // `[K in (keyof T)]` iterates the same keys as `[K in keyof T]`.
        let mut keys = ty.keys_type();
        while let Ok(AnyTsType::TsParenthesizedType(parenthesized)) = keys {
            keys = parenthesized.ty();
        }
        let keys =
            match keys {
                Ok(AnyTsType::TsTypeOperatorType(operator))
                    if operator
                        .operator_token()
                        .is_ok_and(|token| token.text_trimmed() == "keyof") =>
                {
                    MappedTypeKeys::Keyof(operator.ty().map_or_default(|ty| {
                        TypeReference::from_any_ts_type(collector, scope_id, &ty)
                    }))
                }
                keys => MappedTypeKeys::Type(keys.map_or_default(|keys| {
                    TypeReference::from_any_ts_type(collector, scope_id, &keys)
                })),
            };
        let property_ty = ty
            .mapped_type()
            .and_then(|annotation| annotation.ty().ok())
            .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty));

        Self::MappedType(Box::new(MappedType {
            type_parameter,
            keys,
            ty: property_ty,
            readonly_modifier: ty
                .readonly_modifier()
                .map(|modifier| mapped_type_modifier(modifier.operator_token())),
            optional_modifier: ty
                .optional_modifier()
                .map(|modifier| mapped_type_modifier(modifier.operator_token())),
        }))
    }

    pub fn from_ts_typeof_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsTypeofType,
    ) -> Self {
        ty.expression_name()
            .ok()
            .and_then(|name| TypeReferenceQualifier::from_any_ts_name(scope_id, &name))
            .map_or_default(|qualifier| {
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
    }

    pub fn object_with_members(members: Box<[TypeMember]>) -> Self {
        Self::Object(Box::new(Object {
            prototype: None,
            members,
            has_unknown_members: false,
            kind: ObjectKind::Shape,
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
        } else {
            let for_of = JsForOfStatement::cast(parent)?;
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

    /// Returns the variables that `decl` declares, with their types.
    ///
    /// A `let` or `var` variable without a type annotation takes the type of
    /// its initializer with fresh literal types widened, so `let n = 1`
    /// declares `n` as `number`, while `const n = 1` declares it as `1`. See
    /// [`TypeofWidenedLiteralExpression`] for which literal types are fresh.
    ///
    /// A destructuring pattern without a type annotation whose initializer is
    /// a non-null assertion, as `s!` in `const { kind } = s!`, takes apart the
    /// type of `s`, because local inference gives the assertion itself no
    /// type. The pattern's variables get nothing from the `null` and
    /// `undefined` members of that type, which the assertion removes.
    pub fn typed_bindings_from_js_variable_declarator(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        decl: &JsVariableDeclarator,
    ) -> Option<Box<[(Text, TypeReference)]>> {
        let pattern = decl.id().ok()?;
        let asserted = decl
            .variable_annotation()
            .is_none()
            .then(|| decl.initializer()?.expression().ok())
            .flatten()
            .filter(|_| !matches!(pattern, AnyJsBindingPattern::AnyJsBinding(_)))
            .and_then(|initializer| match initializer.omit_parentheses() {
                AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                    assertion.expression().ok()
                }
                _ => None,
            });
        let mut ty = match asserted {
            Some(expression) => collector
                .resolve_expression(scope_id, &expression)
                .into_owned(),
            None => Self::from_js_variable_declarator(collector, scope_id, decl)?.into_owned(),
        };
        if decl.variable_annotation().is_none()
            && matches!(pattern, AnyJsBindingPattern::AnyJsBinding(_))
            && decl
                .declaration()
                .is_some_and(|declaration| declaration.is_let() || declaration.is_var())
            && let Some(initializer) = decl
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
        {
            let value = value_reference(collector, scope_id, &initializer);
            if may_have_fresh_literal(collector, &value, false, MAX_FRESH_LITERAL_SHAPE_DEPTH) {
                ty = Self::Reference(widened_literal_reference(collector, value, false));
            }
        }
        Self::typed_bindings_from_js_binding_pattern(collector, scope_id, ty, &pattern, false)
    }
}

impl CallArgumentType {
    pub fn types_from_js_call_arguments(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        arguments: Option<JsCallArguments>,
    ) -> Box<[Self]> {
        arguments.map_or_default(|args| {
            args.args()
                .into_iter()
                .flatten()
                .map(|arg| Self::from_any_js_call_argument(collector, scope_id, &arg))
                .collect()
        })
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
            AnyJsCallArgument::JsSpread(spread) => {
                Self::Spread(spread.argument().map_or_default(|arg| {
                    collector.reference_to_resolved_expression(scope_id, &arg)
                }))
            }
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
                .map_or_default(|formal_param| Self {
                    parameter: FunctionParameter::from_any_js_formal_parameter(
                        collector,
                        scope_id,
                        &formal_param,
                    ),
                    accessibility: Some(TypeMemberAccessibility::from_modifier_list(
                        param.modifiers(),
                    )),
                }),
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
                    .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
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
            param.initializer(),
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
            None,
            false,
            true,
        )
    }

    /// Builds the parameter that `binding` declares, typed by `annotation`.
    ///
    /// A parameter named by an identifier without an annotation takes the
    /// type of its default value, `initializer`, with fresh literal types
    /// widened, so `x` is `number` in `function f(x = 1) {}`. Without either,
    /// and for a destructuring pattern without an annotation, the type is
    /// unknown.
    fn from_binding_with_annotation(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        binding: SyntaxResult<AnyJsBindingPattern>,
        annotation: Option<TsTypeAnnotation>,
        initializer: Option<JsInitializerClause>,
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
        let ty_data = match (annotation, initializer) {
            (Some(annotation), _) => annotation
                .ty()
                .ok()
                .map_or_default(|ty| TypeData::from_any_ts_type(collector, scope_id, &ty)),
            (None, Some(initializer)) if name.is_some() => {
                initializer.expression().ok().map_or_default(|expression| {
                    let value = value_reference(collector, scope_id, &expression);
                    TypeData::Reference(widened_literal_reference(collector, value, false))
                })
            }
            (None, _) => TypeData::unknown(),
        };
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
    /// Creates the generic declared by `infer U` in a conditional type.
    pub fn from_ts_infer_type(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        ty: &TsInferType,
    ) -> Option<Self> {
        let name = ty.name().ok()?.ident_token().ok()?;
        Some(Self {
            is_const: false,
            name: name.token_text_trimmed().into(),
            constraint: ty
                .constraint()
                .and_then(|constraint| constraint.ty().ok())
                .map_or_default(|constraint| {
                    TypeReference::from_any_ts_type(collector, scope_id, &constraint)
                }),
            default: TypeReference::unknown(),
        })
    }

    pub fn from_ts_type_parameter(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        param: &TsTypeParameter,
    ) -> Option<Self> {
        param
            .name()
            .and_then(|name| name.ident_token())
            .map(|name| Self {
                is_const: param
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_ts_const_modifier().is_some()),
                name: name.token_text_trimmed().into(),
                constraint: param
                    .constraint()
                    .and_then(|constraint| constraint.ty().ok())
                    .map_or_default(|constraint_ty| {
                        TypeReference::from_any_ts_type(collector, scope_id, &constraint_ty)
                    }),
                default: param
                    .default()
                    .and_then(|default| default.ty().ok())
                    .map_or_default(|default_ty| {
                        TypeReference::from_any_ts_type(collector, scope_id, &default_ty)
                    }),
            })
            .ok()
    }

    /// Builds the type parameter a mapped type declares, such as `K` in
    /// `{ [K in keyof T]: T[K] }`.
    ///
    /// The parameter carries no constraint. The mapped type records the keys
    /// it iterates separately, and the parameter must be identical wherever
    /// it is built so references to `K` can be substituted.
    pub fn from_ts_mapped_type(ty: &TsMappedType) -> Option<Self> {
        let name = ty.property_name().ok()?.ident_token().ok()?;
        Some(Self {
            name: name.token_text_trimmed().into(),
            constraint: TypeReference::unknown(),
            default: TypeReference::unknown(),
            is_const: false,
        })
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
                        ty: match ty.predicate() {
                            Some(predicate) => predicate.ty().ok().map_or_default(|ty| {
                                TypeReference::from_any_ts_type(collector, scope_id, &ty)
                            }),
                            // Bare assertions require truthiness, rather than a specific type.
                            // `Conditional` marks a value whose truthiness decides a condition,
                            // and no type annotation resolves to it, so it cannot be confused
                            // with an `asserts value is T` predicate.
                            None => collector.reference_to_owned_data(TypeData::Conditional),
                        },
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
                        ty: ty.ty().map_or_default(|ty| {
                            TypeReference::from_any_ts_type(collector, scope_id, &ty)
                        }),
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
                    .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
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
                    .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
                name: None,
                is_optional: true,
                is_rest: false,
            },
            AnyTsTupleTypeElement::TsRestTupleTypeElement(el) => Self {
                ty: el
                    .ty()
                    .map_or_default(|ty| TypeReference::from_any_ts_type(collector, scope_id, &ty)),
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
                    return_type: {
                        let return_type = function_return_type(
                            collector,
                            scope_id,
                            is_async,
                            member.return_type_annotation(),
                            member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                        );
                        with_inferred_predicate(collector, &member.clone().into(), return_type)
                    },
                };
                let ty = collector.register_and_resolve(function.into());
                let is_static = member
                    .modifiers()
                    .into_iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some());
                Self::from_class_member_info(
                    collector,
                    scope_id,
                    name,
                    ty.into(),
                    is_static,
                    false,
                    false,
                )
            }),
            AnyJsClassMember::TsMethodSignatureClassMember(member) => {
                member.name().ok().and_then(|name| {
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
                            None,
                        ),
                    };
                    let ty = collector.register_and_resolve(function.into());
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member.question_mark_token().is_some();
                    Self::from_class_member_info(
                        collector,
                        scope_id,
                        name,
                        ty.into(),
                        is_static,
                        is_optional,
                        false,
                    )
                })
            }
            AnyJsClassMember::JsPropertyClassMember(member) => {
                member.name().ok().and_then(|name| {
                    let is_readonly = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_ts_readonly_modifier().is_some());
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
                            .map_or_default(|expr| {
                                let value = value_reference(collector, scope_id, &expr);
                                field_initializer_reference(collector, value, is_readonly)
                            }),
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
                        is_readonly,
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
                    let is_readonly = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_ts_readonly_modifier().is_some());
                    let value = value_reference(
                        collector,
                        scope_id,
                        &member.value().ok()?.expression().ok()?,
                    );
                    let ty = field_initializer_reference(collector, value, is_readonly);
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
                        is_readonly,
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
                        .map_or_default(|ty| {
                            TypeReference::from_any_ts_type(collector, scope_id, &ty)
                        });
                    let is_static = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some());
                    let is_optional = member
                        .property_annotation()
                        .as_ref()
                        .and_then(|annotation| annotation.as_ts_optional_property_annotation())
                        .is_some();
                    let is_readonly = member
                        .modifiers()
                        .into_iter()
                        .any(|modifier| modifier.as_ts_readonly_modifier().is_some());
                    Self::from_class_member_info(
                        collector,
                        scope_id,
                        name,
                        ty,
                        is_static,
                        is_optional,
                        is_readonly,
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
                        return_type: {
                            let return_type = function_return_type(
                                collector,
                                scope_id,
                                is_async,
                                member.return_type_annotation(),
                                member.body().ok().map(AnyJsFunctionBody::JsFunctionBody),
                            );
                            with_inferred_predicate(collector, &member.clone().into(), return_type)
                        },
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
                        ty: value.map_or_default(|value| {
                            collector.reference_to_resolved_expression(scope_id, &value)
                        }),
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
            AnyTsTypeMember::JsMetavariable(_) => None,
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
                method_signature_type(collector, scope_id, member).map(|(name, ty)| {
                    let is_optional = member.optional_token().is_some();
                    Self::from_name_and_optional_type(collector, name, ty, is_optional)
                })
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(member) => {
                member.name().ok().and_then(|name| name.name()).map(|name| {
                    let ty = type_from_annotation(collector, scope_id, member.type_annotation())
                        .unwrap_or_default();
                    let is_optional = member.optional_token().is_some();
                    let property =
                        Self::from_name_and_optional_type(collector, name, ty, is_optional);
                    if member.readonly_token().is_some() {
                        Self {
                            kind: property.kind.with_readonly(),
                            ..property
                        }
                    } else {
                        property
                    }
                })
            }
            AnyTsTypeMember::TsSetterSignatureTypeMember(_member) => {
                // TODO: Handle setters
                None
            }
        }
    }

    /// Builds the member for a class method or field named `name`.
    ///
    /// `is_readonly` marks a named instance field as declared `readonly`.
    /// Static and computed fields ignore it.
    #[inline]
    fn from_class_member_info(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        name: AnyJsClassMemberName,
        ty: TypeReference,
        is_static: bool,
        is_optional: bool,
        is_readonly: bool,
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
        let kind = if is_readonly {
            kind.with_readonly()
        } else {
            kind
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

    /// Stands in for an interface member that inference cannot turn into a
    /// [`TypeMember`], such as one with a computed key.
    ///
    /// An interface has no equivalent of [`Object::has_unknown_members`], so
    /// leaving the member out would make the member list look complete. For
    /// example, `keyof Shape` would be `"a"` here, without the key `"b"`:
    ///
    /// ```ts
    /// declare const key: "b";
    /// interface Shape { a: number; [key]: number }
    /// ```
    fn uncollected() -> Self {
        Self {
            kind: TypeMemberKind::ComputedValue(TypeReference::unknown()),
            ty: TypeReference::unknown(),
        }
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
        let mut members = Self::collect_class_members(collector, scope_id, member_list.clone());

        // Extend members with those from constructor definitions:
        let num_members = members.len();
        let mut readonly_properties = None;
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
                        let kind = if named_param.is_optional {
                            TypeMemberKind::NamedOptional(named_param.name.clone())
                        } else {
                            TypeMemberKind::Named(named_param.name.clone())
                        };
                        let readonly_properties = readonly_properties
                            .get_or_insert_with(|| readonly_parameter_properties(&member_list));
                        let kind = if readonly_properties
                            .iter()
                            .any(|property| property.text() == named_param.name.text())
                        {
                            kind.with_readonly()
                        } else {
                            kind
                        };
                        members.push(Self {
                            kind,
                            ty: param.parameter.ty().clone(),
                        });
                    }
                }
            }
        }

        members.into()
    }

    fn collect_class_members(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        member_list: JsClassMemberList,
    ) -> Vec<Self> {
        enum CollectedMember {
            Direct(TypeMember),
            MethodOverload {
                member: TypeMember,
                signatures: Vec<TypeReference>,
            },
        }

        let mut collected = Vec::new();
        let mut overloads_by_name = FxHashMap::default();
        for syntax_member in member_list {
            let is_method_signature = matches!(
                &syntax_member,
                AnyJsClassMember::TsMethodSignatureClassMember(_)
            );
            let is_method_implementation =
                matches!(&syntax_member, AnyJsClassMember::JsMethodClassMember(_));
            let Some(member) = Self::from_any_js_class_member(collector, scope_id, &syntax_member)
            else {
                continue;
            };
            if !is_method_signature && (!is_method_implementation || overloads_by_name.is_empty()) {
                collected.push(CollectedMember::Direct(member));
                continue;
            }

            let Some(name) = member.kind.name() else {
                collected.push(CollectedMember::Direct(member));
                continue;
            };
            let key = (name, member.is_static());

            if is_method_signature {
                if let Some(index) = overloads_by_name.get(&key).copied()
                    && let CollectedMember::MethodOverload { signatures, .. } =
                        &mut collected[index]
                {
                    signatures.push(member.ty);
                } else {
                    overloads_by_name.insert(key, collected.len());
                    collected.push(CollectedMember::MethodOverload {
                        signatures: vec![member.ty.clone()],
                        member,
                    });
                }
            } else if is_method_implementation
                && let Some(index) = overloads_by_name.get(&key).copied()
                && let CollectedMember::MethodOverload {
                    member: representative,
                    ..
                } = &mut collected[index]
            {
                *representative = member;
            } else {
                collected.push(CollectedMember::Direct(member));
            }
        }

        collected
            .into_iter()
            .map(|collected| match collected {
                CollectedMember::Direct(member) => member,
                CollectedMember::MethodOverload {
                    mut member,
                    signatures,
                } => {
                    member.ty = match signatures.as_slice() {
                        [signature] => signature.clone(),
                        _ => Self::overload_set_type(collector, signatures),
                    };
                    member
                }
            })
            .collect()
    }

    /// Collects the members of an interface or an object type.
    ///
    /// Method signatures with the same name form one member, so that a call
    /// selects the overload that accepts its arguments. The type of that
    /// member has one call signature per overload, in the order that
    /// TypeScript tries them (see [`specialized_first`]). The call signatures
    /// of the type itself are in that order too. An optional method is
    /// optional as a whole. Members that can't be collected are `None`.
    ///
    /// In this example, `listen` is one member with two call signatures. A
    /// call with a single argument, such as `listen({})`, selects the second
    /// signature and returns `Promise<string>`.
    ///
    /// ```ts
    /// interface Server {
    ///     listen(options: object, callback: () => void): void;
    ///     listen(options?: object): Promise<string>;
    /// }
    /// ```
    fn collect_type_members(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        member_list: TsTypeMemberList,
    ) -> Vec<Option<Self>> {
        enum CollectedMember {
            Direct(Option<TypeMember>),
            Method {
                member: TypeMember,
                is_optional: bool,
                signatures: Vec<TypeReference>,
            },
        }

        let mut collected = Vec::new();
        let mut methods_by_name = FxHashMap::default();
        for syntax_member in member_list {
            let AnyTsTypeMember::TsMethodSignatureTypeMember(method) = &syntax_member else {
                collected.push(CollectedMember::Direct(Self::from_any_ts_type_member(
                    collector,
                    scope_id,
                    &syntax_member,
                )));
                continue;
            };
            let Some((name, signature)) = method_signature_type(collector, scope_id, method) else {
                collected.push(CollectedMember::Direct(None));
                continue;
            };

            let key = Text::from(name.clone());
            // SAFETY: `index` is in bounds. The `else` branch stores
            // `collected.len()` and then pushes a `CollectedMember::Method`,
            // so every index in `methods_by_name` points at that member.
            // Nothing removes items from `collected`.
            if let Some(index) = methods_by_name.get(&key).copied()
                && let CollectedMember::Method { signatures, .. } = &mut collected[index]
            {
                signatures.push(signature);
            } else {
                let is_optional = method.optional_token().is_some();
                methods_by_name.insert(key, collected.len());
                collected.push(CollectedMember::Method {
                    member: Self::from_name_and_optional_type(
                        collector,
                        name,
                        signature.clone(),
                        is_optional,
                    ),
                    is_optional,
                    signatures: vec![signature],
                });
            }
        }

        let mut members: Vec<_> = collected
            .into_iter()
            .map(|collected| match collected {
                CollectedMember::Direct(member) => member,
                CollectedMember::Method {
                    mut member,
                    is_optional,
                    signatures,
                } => {
                    if signatures.len() > 1 {
                        let ty = Self::overload_set_type(collector, signatures);
                        member.ty = match is_optional {
                            true => RawTypeId::Local(collector.optional(ty)).into(),
                            false => ty,
                        };
                    }
                    Some(member)
                }
            })
            .collect();
        // The call signatures of the type are its overloads, which keep their
        // positions among the other members.
        let positions: Vec<_> = (0..members.len())
            .filter(|index| {
                members[*index]
                    .as_ref()
                    .is_some_and(|member| member.kind == TypeMemberKind::CallSignature)
            })
            .collect();
        if positions.len() > 1 {
            let signatures: Vec<_> = positions
                .iter()
                .filter_map(|index| members[*index].take())
                .collect();
            let signatures = specialized_first(collector, signatures, |member| &member.ty);
            for (index, signature) in positions.into_iter().zip(signatures) {
                members[index] = Some(signature);
            }
        }
        members
    }

    /// Returns an object type with one call signature per overload, in the
    /// order that TypeScript tries them for a call (see
    /// [`specialized_first`]).
    fn overload_set_type(
        collector: &mut dyn RawTypeCollector,
        signatures: Vec<TypeReference>,
    ) -> TypeReference {
        let signatures = specialized_first(collector, signatures, |ty| ty);
        collector.reference_to_owned_data(TypeData::object_with_members(
            signatures
                .into_iter()
                .map(|ty| Self {
                    kind: TypeMemberKind::CallSignature,
                    ty,
                })
                .collect(),
        ))
    }
}

/// Returns the overloads of one declaration in the order that TypeScript
/// tries them for a call: those with a parameter of a literal type first
/// (see [`Function::is_specialized`]), then the others, each group in
/// declaration order. `signature` returns the type of an overload.
///
/// For example, with `f(x: string): 2; f(x: "a"): 1`, `f("a")` is `1`.
pub fn specialized_first<T>(
    collector: &dyn RawTypeCollector,
    overloads: Vec<T>,
    signature: impl Fn(&T) -> &TypeReference,
) -> Vec<T> {
    let (mut specialized, others): (Vec<_>, Vec<_>) = overloads.into_iter().partition(|overload| {
        matches!(
            collector.get_by_reference(signature(overload)),
            Some(TypeData::Function(function))
                if function.is_specialized(|ty| collector.get_by_reference(ty))
        )
    });
    specialized.extend(others);
    specialized
}

/// Registers the function type of a method signature and returns it with the
/// method name.
///
/// Returns `None` when the name isn't known statically, such as `[key](): void`.
fn method_signature_type(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    member: &TsMethodSignatureTypeMember,
) -> Option<(TokenText, TypeReference)> {
    let type_parameters = member.type_parameters();
    // Reusing the enclosing scope lets nongeneric signatures share raw types.
    let scope_id = if type_parameters.is_some() {
        collector
            .scope_for_node(member.syntax())
            .unwrap_or(scope_id)
    } else {
        scope_id
    };
    let name = member.name().ok()?.name()?;
    let function = Function {
        is_async: false,
        type_parameters: generic_params_from_ts_type_params(collector, scope_id, type_parameters),
        name: Some(name.clone().into()),
        parameters: function_params_from_js_params(collector, scope_id, member.parameters()),
        return_type: return_type_from_annotation(
            collector,
            scope_id,
            member.return_type_annotation(),
        )
        .unwrap_or_default(),
    };
    let ty = collector.register_and_resolve(function.into()).into();
    Some((name, ty))
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
        arguments.map_or_default(|args| {
            args.ts_type_argument_list()
                .into_iter()
                .filter_map(Result::ok)
                .map(|ty| Self::from_any_ts_type(collector, scope_id, &ty))
                .collect()
        })
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
                        .map_or_default(Self::from)
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

/// The nodes that give the code inside them a `this` that no `this`
/// parameter declares: class members other than methods, the getters and
/// setters of an object literal, and classes, which a walk reaches from their
/// decorators or `extends` clause.
const THIS_WITHOUT_PARAMETER_KINDS: SyntaxKindSet<JsLanguage> = AnyJsClassMember::KIND_SET
    .union(AnyJsClass::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Returns the type annotation of the `this` parameter that declares the type
/// of `expr`, a `this` expression, as `T` in `function f(this: T) { this }`.
///
/// The parameter belongs to the nearest function or method around `expr`
/// that isn't an arrow function, because an arrow function has the `this` of
/// the code around it. Returns `None` when that function or method declares
/// no `this` parameter with a type annotation, when `expr` is in the
/// decorators or computed key of a method, which run with the `this` around
/// the class or object, and when a class member, a getter, or a setter gives
/// `expr` its `this` first.
fn this_parameter_type(expr: &AnyJsExpression) -> Option<AnyTsType> {
    let mut child = expr.syntax().clone();
    for node in expr.syntax().ancestors().skip(1) {
        let parameters = if JsArrowFunctionExpression::can_cast(node.kind()) {
            child = node;
            continue;
        } else if let Some(function) = AnyJsFunction::cast_ref(&node) {
            match function.parameters().ok()? {
                AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters,
                AnyJsArrowFunctionParameters::AnyJsBinding(_) => return None,
            }
        } else if let Some(method) = JsMethodClassMember::cast_ref(&node) {
            method.parameters().ok()?
        } else if let Some(method) = JsMethodObjectMember::cast_ref(&node) {
            method.parameters().ok()?
        } else if THIS_WITHOUT_PARAMETER_KINDS.matches(node.kind()) {
            return None;
        } else {
            child = node;
            continue;
        };
        if !JsFunctionBody::can_cast(child.kind()) && !JsParameters::can_cast(child.kind()) {
            return None;
        }
        let AnyJsParameter::TsThisParameter(parameter) = parameters.items().first()?.ok()? else {
            return None;
        };
        return parameter.type_annotation()?.ty().ok();
    }
    None
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

/// Returns the names of the parameter properties that a constructor
/// implementation in `member_list` declares `readonly`, as
/// `constructor(readonly name: string)` does.
///
/// The constructor's type doesn't record the modifiers of its parameter
/// properties, so this reads them from the syntax.
fn readonly_parameter_properties(member_list: &JsClassMemberList) -> Vec<TokenText> {
    member_list
        .into_iter()
        .filter_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(constructor) => {
                constructor.parameters().ok()
            }
            _ => None,
        })
        .flat_map(|parameters| parameters.parameters())
        .flatten()
        .filter_map(|parameter| {
            let AnyJsConstructorParameter::TsPropertyParameter(parameter) = parameter else {
                return None;
            };
            let is_readonly = parameter
                .modifiers()
                .into_iter()
                .any(|modifier| modifier.as_ts_readonly_modifier().is_some());
            if !is_readonly {
                return None;
            }
            parameter
                .formal_parameter()
                .ok()
                .as_ref()
                .and_then(AnyJsFormalParameter::as_js_formal_parameter)
                .and_then(|formal_parameter| formal_parameter.binding().ok())
                .as_ref()
                .and_then(AnyJsBindingPattern::as_any_js_binding)
                .and_then(AnyJsBinding::as_js_identifier_binding)
                .and_then(|identifier| identifier.name_token().ok())
                .map(|token| token.token_text_trimmed())
        })
        .collect()
}

#[inline]
fn constructor_params_from_js_constructor_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsConstructorParameters>,
) -> Box<[ConstructorParameter]> {
    params.map_or_default(|params| {
        ConstructorParameter::params_from_js_constructor_parameters(collector, scope_id, &params)
    })
}

#[inline]
fn constructor_params_from_js_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsParameters>,
) -> Box<[ConstructorParameter]> {
    params.map_or_default(|params| {
        params
            .as_fields()
            .items
            .into_iter()
            .flatten()
            .map(|param| ConstructorParameter {
                parameter: FunctionParameter::from_any_js_parameter(collector, scope_id, &param),
                accessibility: None,
            })
            .collect()
    })
}

/// Callee reference of a call or `new` expression, instantiated with its
/// explicit type arguments when present.
fn callee_reference(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    callee: &AnyJsExpression,
    type_arguments: Option<TsTypeArguments>,
) -> TypeReference {
    let callee = collector.reference_to_resolved_expression(scope_id, callee);
    let type_parameters =
        TypeReference::types_from_ts_type_arguments(collector, scope_id, type_arguments);
    if type_parameters.is_empty() {
        return callee;
    }
    collector.reference_to_owned_data(TypeData::instance_of(TypeInstance {
        ty: callee,
        type_parameters,
    }))
}

#[inline]
fn function_params_from_js_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: SyntaxResult<JsParameters>,
) -> Box<[FunctionParameter]> {
    params.map_or_default(|params| {
        FunctionParameter::params_from_js_parameters(collector, scope_id, &params)
    })
}

/// Returns the return type of a function: its annotation, or the type of
/// the values that its body returns, wrapped in a `Promise` for an `async`
/// function.
///
/// An inferred return type widens only when it is a single fresh literal
/// type, so `() => 1` returns `number`, while `() => flag ? "a" : "b"`
/// returns `"a" | "b"`.
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

    let return_ty = match body {
        Some(AnyJsFunctionBody::AnyJsExpression(return_expr)) => {
            TypeData::Reference(value_reference(collector, scope_id, &return_expr))
        }
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
    let return_ty = collector.reference_to_owned_data(return_ty);
    let return_ty = widened_literal_reference(collector, return_ty, true);

    ReturnType::Type(if is_async {
        collector.reference_to_owned_data(TypeData::promise_of(scope_id, return_ty))
    } else {
        return_ty
    })
}

/// Returns the type of a getter: its annotation, or the type of the values
/// that its body returns, widened when it is a single fresh literal type.
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

    let return_ty = collector.reference_to_owned_data(return_ty);
    widened_literal_reference(collector, return_ty, true)
}

/// A modifier without an operator, such as `?`, adds like `+?` does.
fn mapped_type_modifier(operator_token: Option<JsSyntaxToken>) -> MappedTypeModifier {
    match operator_token {
        Some(token) if token.kind() == JsSyntaxKind::MINUS => MappedTypeModifier::Remove,
        _ => MappedTypeModifier::Add,
    }
}

#[inline]
fn generic_params_from_ts_type_params(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    params: Option<TsTypeParameters>,
) -> Box<[TypeReference]> {
    params.map_or_default(|params| {
        TypeReference::types_from_ts_type_parameters(collector, scope_id, &params)
    })
}

#[inline]
fn path_from_any_ts_module_name(module_name: AnyTsModuleName) -> Option<Path> {
    let mut reversed_path = Vec::new();
    push_reversed_module_name_parts(module_name, &mut reversed_path)?;
    Some(Path::from_reversed_parts(reversed_path))
}

/// Pushes the names that `module_name` is made of to `parts`, last first, so
/// `A.B` pushes `B`, then `A`.
fn push_reversed_module_name_parts(
    mut module_name: AnyTsModuleName,
    parts: &mut Vec<Text>,
) -> Option<()> {
    loop {
        match module_name {
            AnyTsModuleName::AnyTsIdentifierBinding(binding) => {
                let binding = binding.as_ts_identifier_binding()?;
                parts.push(text_from_token(binding.name_token())?);
                return Some(());
            }
            AnyTsModuleName::TsQualifiedModuleName(qualified) => {
                let right = qualified.right().ok()?;
                parts.push(text_from_token(right.value_token())?);

                module_name = qualified.left().ok()?;
            }
        }
    }
}

/// The nodes that can stand between a namespace declaration and the body of
/// the namespace that encloses it, as in `namespace A { export namespace N {} }`.
const NAMESPACE_ITEM_KINDS: SyntaxKindSet<JsLanguage> = JsModuleItemList::KIND_SET
    .union(JsExport::KIND_SET)
    .union(TsDeclareStatement::KIND_SET);

/// Returns the namespace whose body declares `decl`, or `None` when `decl` is
/// not declared directly in the body of a namespace.
fn enclosing_namespace(decl: &TsModuleDeclaration) -> Option<TsModuleDeclaration> {
    let mut node = decl.syntax().parent()?;
    while NAMESPACE_ITEM_KINDS.matches(node.kind()) {
        node = node.parent()?;
    }
    let block = TsModuleBlock::cast(node)?;
    block.syntax().parent().and_then(TsModuleDeclaration::cast)
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

/// The types that an enum declaration gives its name and its members.
///
/// TypeScript gives each member of an enum the literal type of its value, and
/// the enum, used as a type, the union of those types. Used as a value, the
/// enum is an object with a readonly property per member. Biome uses plain
/// literal types, so `E.A` with the value `0` has the type `0`, the same
/// values as TypeScript's `E.A`.
#[derive(Clone, Debug, Default)]
pub struct EnumTypes {
    /// The type of the enum's name: a [`TypeData::MergedReference`] whose type
    /// is the union of the member types and whose value is the enum object.
    /// The object has a readonly property for each member with a name, and,
    /// when a member is a number, the reverse mapping from values to names as
    /// a `[n: number]: string` index signature.
    pub declaration: TypeReference,

    /// The type of each member, by the range of its name, in declaration
    /// order.
    pub members: Box<[(TextRange, TypeReference)]>,
}

impl EnumTypes {
    /// Computes the types of `declaration` and its members from syntax.
    ///
    /// A member's type is the literal type of its value. Collection computes
    /// the value as TypeScript does from number and string literals, template
    /// literals without substitutions, `+`, `-`, and `~` before a value,
    /// arithmetic and bitwise operators between values, `+` between strings,
    /// the earlier members of the declaration, named as `A`, `E.A`, or
    /// `E["A"]`, and a `const` without a type annotation declared before the
    /// enum and initialized with such values, as `K` in `const K = "k"`. A
    /// member without an initializer has the value of the member before it
    /// plus one, or `0` for the first member. In an ambient enum that isn't
    /// `const`, as one that `declare` declares, such a member has a value
    /// that the code that defines the enum chooses, so its type is `number`.
    ///
    /// When Biome can't compute a value, the member's type is `number` when
    /// the value is a number in code that TypeScript accepts, such as a call
    /// or `"c".length`, which TypeScript requires to be a number, `string`
    /// for a string, such as a template literal with substitutions, and
    /// Unknown otherwise, such as for a reference to a member of another enum
    /// or to a `const` with a type annotation, because TypeScript computes
    /// some of those and they can be strings. A computed number without a
    /// decimal spelling that Biome can borrow, one other than an integer from
    /// 0 to 255, also has the type `number`. Each of these types holds the
    /// member's value, so none is narrower than TypeScript's.
    pub fn from_ts_enum_declaration(
        collector: &mut dyn RawTypeCollector,
        scope_id: ScopeId,
        declaration: &TsEnumDeclaration,
    ) -> Self {
        let is_ambient = declaration.const_token().is_none()
            && declaration
                .syntax()
                .ancestors()
                .any(|node| AMBIENT_DECLARATION_KINDS.matches(node.kind()));
        let mut evaluator = EnumEvaluator {
            collector,
            scope_id,
            name: declaration
                .id()
                .ok()
                .and_then(|id| id.as_js_identifier_binding()?.name_token().ok())
                .map(|token| token.token_text_trimmed()),
            members: Vec::new(),
            outside: false,
        };
        let mut members = Vec::new();
        for member in declaration.members() {
            let Ok(member) = member else {
                continue;
            };
            let Ok(name) = member.name() else {
                continue;
            };
            let value = match member
                .initializer()
                .map(|initializer| initializer.expression())
            {
                Some(Ok(initializer)) => evaluator.value_of(&initializer, MAX_ENUM_VALUE_DEPTH),
                Some(Err(_)) => EnumValue::Unknown,
                None if is_ambient => EnumValue::SomeNumber,
                None => match evaluator.members.last() {
                    None => EnumValue::Number(0.0, None),
                    Some((_, EnumValue::Number(value, _))) => EnumValue::number(value + 1.0, None),
                    Some((_, EnumValue::SomeNumber)) => EnumValue::SomeNumber,
                    // TypeScript requires an initializer after a string.
                    Some((_, EnumValue::String(_) | EnumValue::Unknown)) => EnumValue::Unknown,
                },
            };
            let ty = evaluator.member_type(value.clone());
            let value = match value {
                EnumValue::Number(value, _) => EnumValue::Number(value, Some(ty.clone())),
                EnumValue::String(_) => EnumValue::String(Some(ty.clone())),
                value @ (EnumValue::SomeNumber | EnumValue::Unknown) => value,
            };
            evaluator.members.push((enum_member_name(&name), value));
            members.push((name.range(), ty));
        }

        let collector = evaluator.collector;
        let has_number = evaluator
            .members
            .iter()
            .any(|(_, value)| matches!(value, EnumValue::Number(..) | EnumValue::SomeNumber));
        let union = match members.as_slice() {
            [] => TypeReference::unknown(),
            [(_, ty)] => ty.clone(),
            members => collector.reference_to_owned_data(TypeData::Union(Box::new(Union(
                members.iter().map(|(_, ty)| ty.clone()).collect(),
            )))),
        };
        let object_members = evaluator
            .members
            .iter()
            .zip(&members)
            .filter_map(|((name, _), (_, ty))| {
                Some(TypeMember {
                    kind: TypeMemberKind::ReadonlyNamed(name.clone()?.into()),
                    ty: ty.clone(),
                })
            })
            .chain(has_number.then(|| TypeMember {
                kind: TypeMemberKind::IndexSignature(GLOBAL_NUMBER_KEYWORD_ID.into()),
                ty: GLOBAL_STRING_KEYWORD_ID.into(),
            }))
            .collect();
        // A shape, because the literal types of the members aren't fresh, as
        // those of the properties of an object expression are, so they don't
        // widen.
        let object = collector.reference_to_owned_data(TypeData::from(Object {
            prototype: None,
            members: object_members,
            has_unknown_members: false,
            kind: ObjectKind::Shape,
        }));
        let declaration = collector.reference_to_owned_data(TypeData::merged_reference(
            Some(union),
            Some(object),
            None::<TypeReference>,
        ));
        Self {
            declaration,
            members: members.into_boxed_slice(),
        }
    }
}

/// Declarations whose contents are ambient: they describe values that code
/// elsewhere defines.
const AMBIENT_DECLARATION_KINDS: SyntaxKindSet<JsLanguage> = TsDeclareStatement::KIND_SET
    .union(TsExportDeclareClause::KIND_SET)
    .union(TsDeclarationModule::KIND_SET)
    .union(TsGlobalDeclaration::KIND_SET)
    .union(TsExternalModuleDeclaration::KIND_SET);

/// Limits the nesting of the operators in the value of an enum member that
/// [`EnumTypes::from_ts_enum_declaration`] computes.
const MAX_ENUM_VALUE_DEPTH: usize = 32;

/// Decimal spellings of the integers that a computed enum value can borrow
/// for its literal type.
const INTEGER_SPELLINGS: [&str; 256] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99", "100", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110",
    "111", "112", "113", "114", "115", "116", "117", "118", "119", "120", "121", "122", "123",
    "124", "125", "126", "127", "128", "129", "130", "131", "132", "133", "134", "135", "136",
    "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147", "148", "149",
    "150", "151", "152", "153", "154", "155", "156", "157", "158", "159", "160", "161", "162",
    "163", "164", "165", "166", "167", "168", "169", "170", "171", "172", "173", "174", "175",
    "176", "177", "178", "179", "180", "181", "182", "183", "184", "185", "186", "187", "188",
    "189", "190", "191", "192", "193", "194", "195", "196", "197", "198", "199", "200", "201",
    "202", "203", "204", "205", "206", "207", "208", "209", "210", "211", "212", "213", "214",
    "215", "216", "217", "218", "219", "220", "221", "222", "223", "224", "225", "226", "227",
    "228", "229", "230", "231", "232", "233", "234", "235", "236", "237", "238", "239", "240",
    "241", "242", "243", "244", "245", "246", "247", "248", "249", "250", "251", "252", "253",
    "254", "255",
];

/// The value of an enum member, as far as collection computes it.
#[derive(Clone)]
enum EnumValue {
    /// A finite number that Biome knows, with the reference to its literal
    /// type when an expression spells it, as a literal or a member does.
    Number(f64, Option<TypeReference>),
    /// A number that Biome doesn't know.
    SomeNumber,
    /// A string, with the reference to its literal type when an expression
    /// spells it.
    String(Option<TypeReference>),
    /// A number or a string, or a value that TypeScript rejects for a member.
    Unknown,
}

impl EnumValue {
    /// Returns the number `value`, or [`Self::SomeNumber`] when `value` is
    /// `NaN` or infinite, which have no literal type.
    fn number(value: f64, ty: Option<TypeReference>) -> Self {
        if value.is_finite() {
            Self::Number(value, ty)
        } else {
            Self::SomeNumber
        }
    }
}

/// Computes the values of the members of one enum declaration, in order.
struct EnumEvaluator<'a> {
    collector: &'a mut dyn RawTypeCollector,
    scope_id: ScopeId,
    /// The name of the enum, which `E.A` names it by.
    name: Option<TokenText>,
    /// The name and the value of each member before the current one.
    members: Vec<(Option<TokenText>, EnumValue)>,
    /// Whether the evaluator is computing the initializer of a `const` outside
    /// the enum, where a name reads no member and TypeScript computes only
    /// the expressions that [`is_constant_outside_enum`] accepts.
    outside: bool,
}

impl EnumEvaluator<'_> {
    /// Returns the value of `expression`, the initializer of a member or an
    /// operand in one, which nests at most `depth` operators.
    fn value_of(&mut self, expression: &AnyJsExpression, depth: usize) -> EnumValue {
        let Some(depth) = depth.checked_sub(1) else {
            return EnumValue::Unknown;
        };
        if self.outside && !is_constant_outside_enum(expression) {
            return EnumValue::Unknown;
        }
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            ) => {
                match literal.value_token().ok().and_then(|token| {
                    NumberLiteral::new(token.token_text_trimmed().into()).to_f64()
                }) {
                    Some(value) => EnumValue::number(value, self.literal_type(expression)),
                    None => EnumValue::SomeNumber,
                }
            }
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(_),
            ) => EnumValue::String(self.literal_type(expression)),
            AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
                EnumValue::String(self.literal_type(expression))
            }
            AnyJsExpression::JsParenthesizedExpression(expression) => expression
                .expression()
                .map_or(EnumValue::Unknown, |inner| self.value_of(&inner, depth)),
            AnyJsExpression::JsUnaryExpression(unary) => {
                let (Ok(operator), Ok(argument)) = (unary.operator(), unary.argument()) else {
                    return EnumValue::Unknown;
                };
                let operation: fn(f64) -> f64 = match operator {
                    JsUnaryOperator::Minus => |value| -value,
                    JsUnaryOperator::Plus => |value| value,
                    JsUnaryOperator::BitwiseNot => |value| f64::from(!to_int32(value)),
                    JsUnaryOperator::Delete
                    | JsUnaryOperator::Void
                    | JsUnaryOperator::Typeof
                    | JsUnaryOperator::LogicalNot => return EnumValue::Unknown,
                };
                match self.value_of(&argument, depth) {
                    // Collection types `-1` and `+1` as literals.
                    EnumValue::Number(value, _) => {
                        let ty = self.literal_type(expression);
                        EnumValue::number(operation(value), ty)
                    }
                    _ => EnumValue::SomeNumber,
                }
            }
            AnyJsExpression::JsBinaryExpression(binary) => {
                let (Ok(operator), Ok(left), Ok(right)) =
                    (binary.operator(), binary.left(), binary.right())
                else {
                    return EnumValue::Unknown;
                };
                let left = self.value_of(&left, depth);
                let right = self.value_of(&right, depth);
                binary_value(operator, &left, &right)
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let member = if self.outside {
                    None
                } else {
                    identifier
                        .name()
                        .ok()
                        .and_then(|name| name.value_token().ok())
                        .and_then(|token| self.member_value(token.text_trimmed()))
                };
                member.unwrap_or_else(|| self.const_value(identifier, depth))
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let Ok(object) = member.object() else {
                    return EnumValue::Unknown;
                };
                let name = member
                    .member()
                    .ok()
                    .and_then(|member| member.as_js_name()?.value_token().ok());
                match name {
                    Some(name) if !member.is_optional_chain() && self.names_enum(&object) => self
                        .member_value(name.text_trimmed())
                        .unwrap_or(EnumValue::Unknown),
                    // A dotted name can name a constant that TypeScript
                    // computes, such as a member of another enum.
                    _ if is_entity_name(&object) => EnumValue::Unknown,
                    // TypeScript computes no other member access, and
                    // requires the value of such an initializer to be a
                    // number, as `"c".length` is.
                    _ => EnumValue::SomeNumber,
                }
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                let (Ok(object), Ok(key)) = (member.object(), member.member()) else {
                    return EnumValue::Unknown;
                };
                let key = match key {
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(key),
                    ) => key.inner_string_text().ok(),
                    _ => None,
                };
                match key {
                    Some(key) if !member.is_optional_chain() && self.names_enum(&object) => {
                        self.member_value(key.text()).unwrap_or(EnumValue::Unknown)
                    }
                    _ => EnumValue::Unknown,
                }
            }
            // TypeScript computes none of these, and requires the value of
            // such an initializer to be a number.
            AnyJsExpression::JsCallExpression(_)
            | AnyJsExpression::JsConditionalExpression(_)
            | AnyJsExpression::JsLogicalExpression(_)
            | AnyJsExpression::JsAwaitExpression(_)
            | AnyJsExpression::JsTemplateExpression(_)
            | AnyJsExpression::TsAsExpression(_)
            | AnyJsExpression::TsSatisfiesExpression(_)
            | AnyJsExpression::TsNonNullAssertionExpression(_)
            | AnyJsExpression::TsTypeAssertionExpression(_) => EnumValue::SomeNumber,
            _ => EnumValue::Unknown,
        }
    }

    /// Returns the value of the member named `name` before the current one,
    /// or `None` when no member before it has that name, because the name
    /// then refers to a later member, which TypeScript rejects, or to a
    /// variable outside the enum.
    fn member_value(&self, name: &str) -> Option<EnumValue> {
        self.members
            .iter()
            .rev()
            .find(|(member, _)| member.as_ref().is_some_and(|member| member.text() == name))
            .map(|(_, value)| value.clone())
    }

    /// Returns the value of the `const` outside the enum that `identifier`
    /// reads, as `K` in `const K = "k"; enum F { X = K }`, which nests at most
    /// `depth` operators.
    ///
    /// As TypeScript does, the evaluator computes the value only when the
    /// `const` has no type annotation, is declared before `identifier`, and
    /// is initialized with number or string literals, template literals,
    /// operators, and other such `const`s. The value of any other `const`,
    /// such as one initialized with `as const` or a call, which TypeScript
    /// doesn't compute there, and of any other variable, is Unknown.
    fn const_value(&mut self, identifier: &JsIdentifierExpression, depth: usize) -> EnumValue {
        let Some(initializer) = self.collector.const_initializer(identifier) else {
            return EnumValue::Unknown;
        };
        let outside = std::mem::replace(&mut self.outside, true);
        let value = self.value_of(&initializer, depth);
        self.outside = outside;
        value
    }

    /// Returns whether `object` names the enum, as `E` in `E.A` does. When a
    /// member has the enum's name, `E` names that member instead.
    fn names_enum(&self, object: &AnyJsExpression) -> bool {
        let Some(name) = &self.name else {
            return false;
        };
        let AnyJsExpression::JsIdentifierExpression(identifier) = object else {
            return false;
        };
        identifier
            .name()
            .ok()
            .and_then(|reference| reference.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == name.text())
            && !self
                .members
                .iter()
                .any(|(member, _)| member.as_ref() == Some(name))
    }

    /// Returns the reference to the type of `expression` when collection
    /// types it as a number or string literal, as it types `1`, `-1`, `"a"`,
    /// and `` `a` ``.
    fn literal_type(&mut self, expression: &AnyJsExpression) -> Option<TypeReference> {
        let is_literal = matches!(
            self.collector.resolve_expression(self.scope_id, expression).as_ref(),
            TypeData::Literal(literal)
                if matches!(literal.as_ref(), Literal::Number(_) | Literal::String(_))
        );
        is_literal.then(|| {
            self.collector
                .reference_to_resolved_expression(self.scope_id, expression)
        })
    }

    /// Returns the type of a member with the value `value`.
    fn member_type(&mut self, value: EnumValue) -> TypeReference {
        match value {
            EnumValue::Number(_, Some(ty)) | EnumValue::String(Some(ty)) => ty,
            EnumValue::Number(value, None) => {
                let spelling = (value.fract() == 0.0 && (0.0..256.0).contains(&value))
                    .then(|| INTEGER_SPELLINGS[value as usize]);
                match spelling {
                    Some(spelling) => {
                        self.collector
                            .reference_to_owned_data(TypeData::from(Literal::Number(
                                NumberLiteral::new(Text::new_static(spelling)),
                            )))
                    }
                    None => GLOBAL_NUMBER_KEYWORD_ID.into(),
                }
            }
            EnumValue::SomeNumber => GLOBAL_NUMBER_KEYWORD_ID.into(),
            EnumValue::String(None) => GLOBAL_STRING_KEYWORD_ID.into(),
            EnumValue::Unknown => TypeReference::unknown(),
        }
    }
}

/// Returns whether TypeScript can compute `expression` in the initializer of
/// a `const` that an enum member reads: a number or string literal, a template
/// literal without a tag, parentheses, a unary or binary operator, or a name.
fn is_constant_outside_enum(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        _ => matches!(
            expression,
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                    | AnyJsLiteralExpression::JsStringLiteralExpression(_)
            ) | AnyJsExpression::JsParenthesizedExpression(_)
                | AnyJsExpression::JsUnaryExpression(_)
                | AnyJsExpression::JsBinaryExpression(_)
                | AnyJsExpression::JsIdentifierExpression(_)
        ),
    }
}

/// Returns the value of `left` and `right` combined by `operator`, following
/// JavaScript's arithmetic. Comparisons and `in` give booleans, which
/// TypeScript rejects as values of members.
fn binary_value(operator: JsBinaryOperator, left: &EnumValue, right: &EnumValue) -> EnumValue {
    let numbers = match (left, right) {
        (EnumValue::Number(left, _), EnumValue::Number(right, _)) => Some((*left, *right)),
        _ => None,
    };
    let value = match operator {
        JsBinaryOperator::Plus => {
            return match (left, right) {
                (EnumValue::Number(left, _), EnumValue::Number(right, _)) => {
                    EnumValue::number(left + right, None)
                }
                (EnumValue::String(_), _) | (_, EnumValue::String(_)) => EnumValue::String(None),
                (
                    EnumValue::Number(..) | EnumValue::SomeNumber,
                    EnumValue::Number(..) | EnumValue::SomeNumber,
                ) => EnumValue::SomeNumber,
                _ => EnumValue::Unknown,
            };
        }
        JsBinaryOperator::Minus => numbers.map(|(left, right)| left - right),
        JsBinaryOperator::Times => numbers.map(|(left, right)| left * right),
        JsBinaryOperator::Divide => numbers.map(|(left, right)| left / right),
        JsBinaryOperator::Remainder => numbers.map(|(left, right)| left % right),
        JsBinaryOperator::Exponent => numbers.and_then(|(left, right)| integer_power(left, right)),
        JsBinaryOperator::LeftShift => numbers
            .map(|(left, right)| f64::from(to_int32(left).wrapping_shl(to_uint32(right) & 31))),
        JsBinaryOperator::RightShift => {
            numbers.map(|(left, right)| f64::from(to_int32(left) >> (to_uint32(right) & 31)))
        }
        JsBinaryOperator::UnsignedRightShift => {
            numbers.map(|(left, right)| f64::from(to_uint32(left) >> (to_uint32(right) & 31)))
        }
        JsBinaryOperator::BitwiseAnd => {
            numbers.map(|(left, right)| f64::from(to_int32(left) & to_int32(right)))
        }
        JsBinaryOperator::BitwiseOr => {
            numbers.map(|(left, right)| f64::from(to_int32(left) | to_int32(right)))
        }
        JsBinaryOperator::BitwiseXor => {
            numbers.map(|(left, right)| f64::from(to_int32(left) ^ to_int32(right)))
        }
        JsBinaryOperator::LessThan
        | JsBinaryOperator::GreaterThan
        | JsBinaryOperator::LessThanOrEqual
        | JsBinaryOperator::GreaterThanOrEqual
        | JsBinaryOperator::Equality
        | JsBinaryOperator::StrictEquality
        | JsBinaryOperator::Inequality
        | JsBinaryOperator::StrictInequality => return EnumValue::Unknown,
    };
    value.map_or(EnumValue::SomeNumber, |value| {
        EnumValue::number(value, None)
    })
}

/// Returns `base ** exponent` when both are integers, the exponent isn't
/// negative, and the result is an integer that a double holds exactly, so
/// that it is the value that JavaScript computes.
fn integer_power(base: f64, exponent: f64) -> Option<f64> {
    if base.fract() != 0.0 || exponent.fract() != 0.0 || !(0.0..=64.0).contains(&exponent) {
        return None;
    }
    let value = base.powi(exponent as i32);
    (value.abs() <= MAX_SAFE_INTEGER).then_some(value)
}

/// The largest integer that a double holds exactly, with every integer below
/// it, `2 ** 53 - 1`.
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// Converts the finite `value` to a 32-bit integer, as JavaScript's bitwise
/// operators do.
fn to_int32(value: f64) -> i32 {
    to_uint32(value) as i32
}

/// Converts the finite `value` to an unsigned 32-bit integer, as JavaScript's
/// `>>>` and shift counts do.
fn to_uint32(value: f64) -> u32 {
    value.trunc().rem_euclid(4_294_967_296.0) as u32
}

/// Returns whether `expression` is an identifier, or a chain of `.name`
/// accesses on one, as `a.b.c`.
fn is_entity_name(expression: &AnyJsExpression) -> bool {
    let mut expression = expression.clone();
    loop {
        match expression {
            AnyJsExpression::JsIdentifierExpression(_) => return true,
            AnyJsExpression::JsStaticMemberExpression(member)
                if !member.is_optional_chain()
                    && member
                        .member()
                        .is_ok_and(|member| member.as_js_name().is_some()) =>
            {
                let Ok(object) = member.object() else {
                    return false;
                };
                expression = object;
            }
            _ => return false,
        }
    }
}

/// Returns the name of an enum member: an identifier, or the text of a string
/// literal, written as the name or as a computed name, as `["a"]`.
fn enum_member_name(name: &AnyTsEnumMemberName) -> Option<TokenText> {
    match name {
        AnyTsEnumMemberName::TsLiteralEnumMemberName(name) => {
            let token = name.value().ok()?;
            Some(if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                inner_string_text(&token)
            } else {
                token.token_text_trimmed()
            })
        }
        AnyTsEnumMemberName::JsComputedMemberName(name) => match name.expression().ok()? {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(literal),
            ) => literal.inner_string_text().ok(),
            _ => None,
        },
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

/// The nodes whose `return` statements return from a function of their own,
/// so [`own_returns`] skips them.
const NESTED_FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(AnyJsClass::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// The nodes that [`own_returns`] visits.
const RETURN_SCAN_KINDS: SyntaxKindSet<JsLanguage> =
    JsReturnStatement::KIND_SET.union(NESTED_FUNCTION_KINDS);

/// Returns the `return` statements of `body`, the body of a function, that
/// return from that function, in source order, including those in code that
/// no path reaches. The `return` statements of nested functions, methods,
/// accessors, and classes return from those instead, so they are skipped.
pub fn own_returns(body: &JsFunctionBody) -> impl Iterator<Item = JsReturnStatement> + use<'_> {
    let mut events = body.syntax().preorder_of_kinds(RETURN_SCAN_KINDS);
    std::iter::from_fn(move || {
        while let Some(event) = events.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if NESTED_FUNCTION_KINDS.matches(node.kind()) {
                events.skip_subtree();
                continue;
            }
            if let Some(statement) = JsReturnStatement::cast(node) {
                return Some(statement);
            }
        }
        None
    })
}

/// Returns the type of the values that `body`, the body of a function
/// without a return type annotation, returns, as TypeScript infers it.
///
/// Only the `return` statements of the function's own code count (see
/// [`own_returns`]). When none of them has a value, the type is `void`.
/// Otherwise it is the union of the types of their values, including those
/// in code that no path reaches, with `undefined` when a `return` has no
/// value. When the body may complete, as far as its syntax shows (see
/// [`ends_by_exit`]), the union also holds a
/// [`TypeofImplicitReturnExpression`], which resolves to `undefined` when a
/// path reaches the end of the body.
fn type_from_function_body(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    body: JsFunctionBody,
) -> TypeData {
    let mut return_types = Vec::new();
    let mut has_bare_return = false;
    for return_statement in own_returns(&body) {
        match return_statement.argument() {
            Some(argument) => return_types.push(value_reference(collector, scope_id, &argument)),
            None => has_bare_return = true,
        }
    }

    if return_types.is_empty() {
        return TypeData::VoidKeyword;
    }
    if has_bare_return {
        return_types.push(GLOBAL_UNDEFINED_ID.into());
    }
    let is_closed = last_running_statement(body.statements()).is_some_and(ends_by_exit);
    if !is_closed && let Some(function) = body.syntax().parent() {
        return_types.push(collector.reference_to_owned_data(TypeData::from(
            TypeofExpression::ImplicitReturn(TypeofImplicitReturnExpression {
                function: function.text_trimmed_range(),
                has_bare_return,
            }),
        )));
    }

    match return_types.len() {
        1 => TypeData::Reference(return_types.remove(0)),
        _ => TypeData::Union(Box::new(Union(return_types.into_boxed_slice()))),
    }
}

/// Returns whether `statement` never completes, as far as its syntax shows.
/// Each path through such a statement ends at a `return` or a `throw`.
///
/// A statement never completes when it is:
/// - a `return` or a `throw`;
/// - a block, and its last statement never completes;
/// - an `if` with an `else`, and both branches never complete;
/// - a `switch` with a `default` clause, the last statement of its last
///   clause never completes, and no jump leaves it (see [`may_leave_switch`]);
/// - a `try` with a `catch` clause, and the last statements of the `try`
///   block and of the `catch` block never complete.
///
/// The last statement of a list is the last one that runs code (see
/// [`last_running_statement`]). Any other statement may complete.
fn ends_by_exit(statement: AnyJsStatement) -> bool {
    let mut pending = vec![statement];
    while let Some(statement) = pending.pop() {
        match statement {
            AnyJsStatement::JsReturnStatement(_) | AnyJsStatement::JsThrowStatement(_) => {}
            AnyJsStatement::JsBlockStatement(block) => {
                let Some(last) = last_running_statement(block.statements()) else {
                    return false;
                };
                pending.push(last);
            }
            AnyJsStatement::JsIfStatement(statement) => {
                let (Ok(consequent), Some(alternate)) = (
                    statement.consequent(),
                    statement
                        .else_clause()
                        .and_then(|clause| clause.alternate().ok()),
                ) else {
                    return false;
                };
                pending.push(consequent);
                pending.push(alternate);
            }
            // Each clause falls through to the next one, so a path that stays
            // in the statement reaches the end of the last clause. Without
            // `default`, the path on which no case matches skips every clause.
            AnyJsStatement::JsSwitchStatement(statement) => {
                let clauses = statement.cases();
                let has_default = clauses
                    .iter()
                    .any(|clause| matches!(clause, AnyJsSwitchClause::JsDefaultClause(_)));
                if !has_default || may_leave_switch(&statement) {
                    return false;
                }
                let Some(last) = clauses
                    .last()
                    .and_then(|clause| last_running_statement(clause.consequent()))
                else {
                    return false;
                };
                pending.push(last);
            }
            // The statement completes only when its `try` block or its `catch`
            // clause completes. A `finally` block can't make it complete.
            AnyJsStatement::JsTryStatement(_) | AnyJsStatement::JsTryFinallyStatement(_) => {
                let Some(statement) = AnyJsTryStatement::cast_ref(statement.syntax()) else {
                    return false;
                };
                let (Ok(block), Some(Ok(handler))) = (
                    statement.body(),
                    statement.catch_clause().map(|clause| clause.body()),
                ) else {
                    return false;
                };
                pending.push(AnyJsStatement::JsBlockStatement(block));
                pending.push(AnyJsStatement::JsBlockStatement(handler));
            }
            _ => return false,
        }
    }
    true
}

/// Returns the last statement of `statements` that runs code. Function
/// declarations and TypeScript declarations of types or ambient values run
/// no code where they appear, so trailing ones are skipped.
fn last_running_statement(statements: JsStatementList) -> Option<AnyJsStatement> {
    statements.iter().rev().find(|statement| {
        !matches!(
            statement,
            AnyJsStatement::JsFunctionDeclaration(_)
                | AnyJsStatement::TsDeclareFunctionDeclaration(_)
                | AnyJsStatement::TsTypeAliasDeclaration(_)
                | AnyJsStatement::TsInterfaceDeclaration(_)
                | AnyJsStatement::TsDeclareStatement(_)
                | AnyJsStatement::TsExternalModuleDeclaration(_)
                | AnyJsStatement::TsGlobalDeclaration(_)
        )
    })
}

/// The loop statements. A `break` without a label targets the innermost loop
/// or `switch`, and a `continue` without a label the innermost loop.
const LOOP_KINDS: SyntaxKindSet<JsLanguage> = JsWhileStatement::KIND_SET
    .union(JsDoWhileStatement::KIND_SET)
    .union(JsForStatement::KIND_SET)
    .union(JsForInStatement::KIND_SET)
    .union(JsForOfStatement::KIND_SET);

/// The nodes that [`may_leave_switch`] visits.
const JUMP_SCAN_KINDS: SyntaxKindSet<JsLanguage> = JsBreakStatement::KIND_SET
    .union(JsContinueStatement::KIND_SET)
    .union(LOOP_KINDS)
    .union(JsSwitchStatement::KIND_SET)
    .union(NESTED_FUNCTION_KINDS);

/// Returns whether a `break` or a `continue` in the clauses of `statement`
/// may jump out of it.
///
/// A `break` without a label targets `statement` unless a loop or a `switch`
/// nested in it encloses the `break`. A `continue` without a label needs a
/// nested loop. A jump with a label always counts as leaving, so the walk
/// never resolves labels. Jumps in nested functions, methods, accessors, and
/// classes stay in their own code, so they are skipped.
fn may_leave_switch(statement: &JsSwitchStatement) -> bool {
    let mut loops = 0usize;
    let mut switches = 0usize;
    let clauses = statement.cases();
    let mut events = clauses.syntax().preorder_of_kinds(JUMP_SCAN_KINDS);
    while let Some(event) = events.next() {
        match event {
            WalkEvent::Enter(node) => {
                let kind = node.kind();
                if NESTED_FUNCTION_KINDS.matches(kind) {
                    events.skip_subtree();
                } else if LOOP_KINDS.matches(kind) {
                    loops += 1;
                } else if JsSwitchStatement::can_cast(kind) {
                    switches += 1;
                } else if let Some(jump) = JsBreakStatement::cast_ref(&node)
                    && (jump.label().is_some() || loops + switches == 0)
                {
                    return true;
                } else if let Some(jump) = JsContinueStatement::cast_ref(&node)
                    && (jump.label().is_some() || loops == 0)
                {
                    return true;
                }
            }
            WalkEvent::Leave(node) => {
                let kind = node.kind();
                if LOOP_KINDS.matches(kind) {
                    loops -= 1;
                } else if JsSwitchStatement::can_cast(kind) {
                    switches -= 1;
                }
            }
        }
    }
    false
}

declare_node_union! {
    /// A function, method, or arrow function whose body TypeScript can infer
    /// a type predicate from.
    pub AnyPredicateFunction = AnyJsFunction | JsMethodClassMember | JsMethodObjectMember
}

impl AnyPredicateFunction {
    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            Self::AnyJsFunction(function) => function.return_type_annotation(),
            Self::JsMethodClassMember(method) => method.return_type_annotation(),
            Self::JsMethodObjectMember(method) => method.return_type_annotation(),
        }
    }

    fn is_async(&self) -> bool {
        match self {
            Self::AnyJsFunction(function) => function.is_async(),
            Self::JsMethodClassMember(method) => method.async_token().is_some(),
            Self::JsMethodObjectMember(method) => method.async_token().is_some(),
        }
    }

    fn is_generator(&self) -> bool {
        match self {
            Self::AnyJsFunction(function) => function.is_generator(),
            Self::JsMethodClassMember(method) => method.star_token().is_some(),
            Self::JsMethodObjectMember(method) => method.star_token().is_some(),
        }
    }

    fn body(&self) -> Option<AnyJsFunctionBody> {
        match self {
            Self::AnyJsFunction(function) => function.body().ok(),
            Self::JsMethodClassMember(method) => {
                method.body().ok().map(AnyJsFunctionBody::JsFunctionBody)
            }
            Self::JsMethodObjectMember(method) => {
                method.body().ok().map(AnyJsFunctionBody::JsFunctionBody)
            }
        }
    }

    fn parameters(&self) -> Option<AnyJsArrowFunctionParameters> {
        match self {
            Self::AnyJsFunction(function) => function.parameters().ok(),
            Self::JsMethodClassMember(method) => method
                .parameters()
                .ok()
                .map(AnyJsArrowFunctionParameters::JsParameters),
            Self::JsMethodObjectMember(method) => method
                .parameters()
                .ok()
                .map(AnyJsArrowFunctionParameters::JsParameters),
        }
    }

    /// Returns whether the function is a function or arrow function
    /// expression passed directly as an argument of a call or `new`
    /// expression, whose signature gives a type to its parameters without
    /// a type annotation.
    fn is_call_argument(&self) -> bool {
        let Self::AnyJsFunction(
            AnyJsFunction::JsArrowFunctionExpression(_) | AnyJsFunction::JsFunctionExpression(_),
        ) = self
        else {
            return false;
        };
        let mut parent = self.syntax().parent();
        while let Some(node) = parent.as_ref()
            && JsParenthesizedExpression::can_cast(node.kind())
        {
            parent = node.parent();
        }
        parent
            .and_then(JsCallArgumentList::cast)
            .and_then(|list| list.parent::<JsCallArguments>())
            .and_then(|arguments| arguments.syntax().parent())
            .is_some_and(|call| {
                JsCallExpression::can_cast(call.kind()) || JsNewExpression::can_cast(call.kind())
            })
    }
}

/// Limits the expressions that [`gives_boolean`] and [`tests_name`] visit.
const MAX_BOOLEAN_STEPS: usize = 64;

/// Returns `return_type`, the return type collected for `function`, or an
/// [`InferredPredicateReturnType`] around it when TypeScript may infer a type
/// predicate from the function's body, as [`inferred_predicate_parameter`]
/// decides, so that inference decides whether the function has one.
fn with_inferred_predicate(
    collector: &mut dyn RawTypeCollector,
    function: &AnyPredicateFunction,
    return_type: ReturnType,
) -> ReturnType {
    let ReturnType::Type(ty) = return_type else {
        return return_type;
    };
    let Some(parameter_name) = inferred_predicate_parameter(function)
        .and_then(|(_, binding)| binding.name_token().ok())
        .map(|name| Text::from(name.token_text_trimmed()))
    else {
        return ReturnType::Type(ty);
    };
    let target = collector.reference_to_owned_data(TypeData::from(
        TypeofExpression::InferredPredicate(TypeofInferredPredicateExpression {
            function: function.syntax().text_trimmed_range(),
        }),
    ));
    ReturnType::InferredPredicate(Box::new(InferredPredicateReturnType {
        parameter_name,
        target,
        ty,
    }))
}

/// Returns the value that `function` returns and the parameter that a type
/// predicate inferred from its body would test, when TypeScript may infer
/// one for the function, or `None` otherwise.
///
/// TypeScript considers a function without a return type annotation that is
/// neither `async` nor a generator, whose body is an expression or ends with
/// its only `return` statement, which has a value, and whose returned value
/// is a boolean. Biome also requires that `return` statement to be the only
/// statement of the body, because a statement before it can narrow a
/// parameter where the value is returned, as `if (x === null) throw 0;` does.
/// The value gives a boolean when it is a comparison, `instanceof`, `in`,
/// `!`, a call that is not part of an optional chain, or `&&` or `||` of such
/// values, in parentheses or not.
///
/// TypeScript tries the parameters in order and takes the first one that the
/// returned value narrows. Biome narrows less than TypeScript, so it can't
/// tell that TypeScript finds no type predicate about a parameter that the
/// value mentions, and it takes only the first such parameter, skipping
/// `this`, rest, and destructured parameters and those annotated `boolean`,
/// which TypeScript skips too. That parameter must be named without escapes,
/// never be assigned anywhere in the function, and get its type from an
/// annotation or, without one and without a default value, from the call
/// that the function is passed to. The value must also test it directly, as
/// [`tests_name`] describes, because inferring a type predicate costs a
/// resolution of the parameter's type, which is wasted on a value such as
/// `query(builder)` or `builder.done()`. Otherwise the function gets no type
/// predicate.
pub fn inferred_predicate_parameter(
    function: &AnyPredicateFunction,
) -> Option<(AnyJsExpression, JsIdentifierBinding)> {
    if function.return_type_annotation().is_some() || function.is_async() || function.is_generator()
    {
        return None;
    }
    let returned = match function.body()? {
        AnyJsFunctionBody::AnyJsExpression(expression) => expression,
        AnyJsFunctionBody::JsFunctionBody(body) => {
            let statements = body.statements();
            if statements.len() != 1 {
                return None;
            }
            let AnyJsStatement::JsReturnStatement(statement) = statements.first()? else {
                return None;
            };
            statement.argument()?
        }
    };
    let mut remaining = MAX_BOOLEAN_STEPS;
    if !gives_boolean(&returned, &mut remaining) {
        return None;
    }
    let parameters = match function.parameters()? {
        AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
            let binding = binding.as_js_identifier_binding()?.clone();
            let name = binding.name_token().ok()?;
            let text = name.text_trimmed();
            let mut steps = MAX_BOOLEAN_STEPS;
            return (mentions_name(&returned, text)
                && function.is_call_argument()
                && !text.contains('\\')
                && tests_name(&returned, text, &mut steps)
                && !assigns_name(function, text))
            .then_some((returned, binding));
        }
        AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters,
    };
    for parameter in parameters.items() {
        let formal = match parameter.ok()? {
            AnyJsParameter::TsThisParameter(_) | AnyJsParameter::JsRestParameter(_) => continue,
            AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                formal,
            )) => formal,
            AnyJsParameter::AnyJsFormalParameter(
                AnyJsFormalParameter::JsBogusParameter(_) | AnyJsFormalParameter::JsMetavariable(_),
            ) => return None,
        };
        let binding = match formal.binding().ok()? {
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                binding
            }
            AnyJsBindingPattern::AnyJsBinding(_) => return None,
            AnyJsBindingPattern::JsArrayBindingPattern(_)
            | AnyJsBindingPattern::JsObjectBindingPattern(_) => continue,
        };
        let name = binding.name_token().ok()?;
        let text = name.text_trimmed();
        if !mentions_name(&returned, text) {
            continue;
        }
        let annotation = formal.type_annotation();
        let is_boolean = annotation
            .as_ref()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| matches!(ty, AnyTsType::TsBooleanType(_)));
        if is_boolean && formal.question_mark_token().is_none() {
            continue;
        }
        // TypeScript tests the annotated type of a parameter, whatever its
        // default value. Without an annotation, the default value takes part
        // in the parameter's type, so such a parameter gets no type predicate.
        let is_typed =
            annotation.is_some() || (function.is_call_argument() && formal.initializer().is_none());
        let mut steps = MAX_BOOLEAN_STEPS;
        if !is_typed
            || text.contains('\\')
            || !tests_name(&returned, text, &mut steps)
            || assigns_name(function, text)
        {
            return None;
        }
        return Some((returned, binding));
    }
    None
}

/// Returns whether `expression` gives only `true` or `false`, as
/// [`with_inferred_predicate`] lists such expressions. Returns false when the
/// walk runs out of `remaining` steps.
fn gives_boolean(expression: &AnyJsExpression, remaining: &mut usize) -> bool {
    let Some(left) = remaining.checked_sub(1) else {
        return false;
    };
    *remaining = left;
    match expression {
        AnyJsExpression::JsParenthesizedExpression(expression) => expression
            .expression()
            .is_ok_and(|expression| gives_boolean(&expression, remaining)),
        AnyJsExpression::JsBinaryExpression(binary) => binary.operator().is_ok_and(|operator| {
            matches!(
                operator,
                JsBinaryOperator::LessThan
                    | JsBinaryOperator::GreaterThan
                    | JsBinaryOperator::LessThanOrEqual
                    | JsBinaryOperator::GreaterThanOrEqual
                    | JsBinaryOperator::Equality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictInequality
            )
        }),
        AnyJsExpression::JsInstanceofExpression(_) | AnyJsExpression::JsInExpression(_) => true,
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary.operator().ok() == Some(JsUnaryOperator::LogicalNot)
        }
        AnyJsExpression::JsCallExpression(_) => !expression.is_optional_chain(),
        AnyJsExpression::JsLogicalExpression(logical) => {
            matches!(
                logical.operator(),
                Ok(JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr)
            ) && logical
                .left()
                .is_ok_and(|left| gives_boolean(&left, remaining))
                && logical
                    .right()
                    .is_ok_and(|right| gives_boolean(&right, remaining))
        }
        _ => false,
    }
}

/// Returns whether `expression`, a value that gives a boolean (see
/// [`gives_boolean`]), tests the variable named `name` directly: as the
/// operand of `typeof` or `!`, an operand of a comparison with `==`, `!=`,
/// `===`, or `!==`, the left operand of `instanceof`, the right operand of
/// `in`, or an operand of `&&` or `||`, each also through a property of the
/// variable, or as an argument of a call. An identifier written with escapes
/// counts as any name. Returns false when the walk runs out of `remaining`
/// steps.
fn tests_name(expression: &AnyJsExpression, name: &str, remaining: &mut usize) -> bool {
    let Some(left) = remaining.checked_sub(1) else {
        return false;
    };
    *remaining = left;
    let tests = |operand: SyntaxResult<AnyJsExpression>, remaining: &mut usize| {
        operand.is_ok_and(|operand| {
            is_path_of(&operand, name, remaining) || tests_name(&operand, name, remaining)
        })
    };
    match expression {
        AnyJsExpression::JsParenthesizedExpression(expression) => expression
            .expression()
            .is_ok_and(|expression| tests_name(&expression, name, remaining)),
        AnyJsExpression::JsBinaryExpression(binary) => {
            matches!(
                binary.operator(),
                Ok(JsBinaryOperator::Equality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictInequality)
            ) && (tests(binary.left(), remaining) || tests(binary.right(), remaining))
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            matches!(
                unary.operator(),
                Ok(JsUnaryOperator::LogicalNot | JsUnaryOperator::Typeof)
            ) && tests(unary.argument(), remaining)
        }
        AnyJsExpression::JsInstanceofExpression(instanceof) => tests(instanceof.left(), remaining),
        AnyJsExpression::JsInExpression(in_expression) => tests(in_expression.object(), remaining),
        AnyJsExpression::JsLogicalExpression(logical) => {
            tests(logical.left(), remaining) || tests(logical.right(), remaining)
        }
        AnyJsExpression::JsCallExpression(call) => call.arguments().is_ok_and(|arguments| {
            arguments.args().iter().any(|argument| {
                matches!(&argument, Ok(AnyJsCallArgument::AnyJsExpression(argument))
                    if is_path_of(argument, name, &mut 0))
            })
        }),
        _ => false,
    }
}

/// Returns whether `expression` reads the variable named `name`, or a
/// property of it, through property accesses when `remaining` allows them,
/// counting an identifier written with escapes as any name.
fn is_path_of(expression: &AnyJsExpression, name: &str, remaining: &mut usize) -> bool {
    let mut expression = expression.clone();
    loop {
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                match parenthesized.expression() {
                    Ok(inner) => inner,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return identifier
                    .name()
                    .and_then(|name| name.value_token())
                    .is_ok_and(|token| {
                        let text = token.text_trimmed();
                        text == name || text.contains('\\')
                    });
            }
            AnyJsExpression::JsStaticMemberExpression(member) if *remaining > 0 => {
                *remaining -= 1;
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsComputedMemberExpression(member) if *remaining > 0 => {
                *remaining -= 1;
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            _ => return false,
        };
    }
}

/// Returns whether `expression` reads a variable named `name`, counting an
/// identifier written with escapes as a read of any name. A read in a nested
/// function counts too.
fn mentions_name(expression: &AnyJsExpression, name: &str) -> bool {
    expression
        .syntax()
        .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => JsReferenceIdentifier::cast(node),
            WalkEvent::Leave(_) => None,
        })
        .any(|reference| {
            reference.value_token().is_ok_and(|token| {
                let text = token.text_trimmed();
                text == name || text.contains('\\')
            })
        })
}

/// Returns whether `function` assigns a variable named `name` anywhere,
/// including in nested functions, counting an assignment target written
/// with escapes as an assignment of any name.
fn assigns_name(function: &AnyPredicateFunction, name: &str) -> bool {
    function
        .syntax()
        .preorder_of_kinds(JsIdentifierAssignment::KIND_SET)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => JsIdentifierAssignment::cast(node),
            WalkEvent::Leave(_) => None,
        })
        .any(|assignment| {
            assignment.name_token().is_ok_and(|token| {
                let text = token.text_trimmed();
                text == name || text.contains('\\')
            })
        })
}

/// Checks for TypeScript's special `const` assertion target.
fn is_const_reference_type(type_annotation: &AnyTsType) -> bool {
    let Some(reference_type) = type_annotation.as_ts_reference_type() else {
        return false;
    };

    reference_type.type_arguments().is_none()
        && reference_type.name().is_ok_and(|name| {
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

/// How TypeScript infers an array expression, which depends on where it
/// appears.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayExpressionContext {
    /// An array, as `const values = [1, "a"]` gives `(number | string)[]`.
    Array,
    /// A tuple that a destructuring pattern takes apart, as in
    /// `const [first, second] = [1, "a"]` and
    /// `[first, second] = [second, first]`. The pattern gives the elements no
    /// literal type, so their literal types are widened, and `first` is
    /// `number`.
    Destructured,
    /// Another tuple, as in `call(...[1, 2])` and
    /// `[1, "a"] satisfies [number, string]`.
    Tuple,
}

impl ArrayExpressionContext {
    fn of(expr: &JsArrayExpression) -> Self {
        let mut node = expr.syntax().clone();
        let parent = loop {
            let Some(parent) = node.parent() else {
                return Self::Array;
            };
            if parent.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
                break parent;
            }
            node = parent;
        };
        if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
            let Some(owner) = initializer.syntax().parent() else {
                return Self::Array;
            };
            let pattern = JsVariableDeclarator::cast_ref(&owner)
                .and_then(|declarator| declarator.id().ok())
                .or_else(|| {
                    JsFormalParameter::cast_ref(&owner)
                        .and_then(|parameter| parameter.binding().ok())
                });
            return if matches!(pattern, Some(AnyJsBindingPattern::JsArrayBindingPattern(_))) {
                Self::Destructured
            } else {
                Self::Array
            };
        }
        if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
            return if matches!(
                assignment.left(),
                Ok(AnyJsAssignmentPattern::JsArrayAssignmentPattern(_))
            ) {
                Self::Destructured
            } else {
                Self::Array
            };
        }
        if parent.kind() == JsSyntaxKind::JS_SPREAD
            && parent
                .parent()
                .is_some_and(|list| list.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST)
        {
            return Self::Tuple;
        }
        if let Some(satisfies) = TsSatisfiesExpression::cast_ref(&parent)
            && matches!(satisfies.ty(), Ok(AnyTsType::TsTupleType(_)))
        {
            return Self::Tuple;
        }
        Self::Array
    }
}

/// Returns the type that the assertion `expression as T` or `<T>expression`
/// gives, where `ty` is the type of `T`.
///
/// Asserted shapes and primitive literal types stay behind a reference, so
/// const inference and literal widening don't mistake the annotation for the
/// type of an object, array, or literal expression. The raw table stores
/// equal types once, so the `1` of `1 as 1` has the same id as the fresh `1`
/// of the expression `1`, which TypeScript widens in a mutable location.
fn asserted_type(collector: &mut dyn RawTypeCollector, ty: TypeData) -> TypeData {
    let is_shape_or_literal = match &ty {
        TypeData::Object(_) | TypeData::Tuple(_) => true,
        TypeData::Literal(literal) => literal.is_primitive(),
        _ => false,
    };
    if is_shape_or_literal {
        TypeData::Reference(collector.reference_to_owned_data(ty))
    } else {
        ty
    }
}

/// Returns the reference to the type of `expression`, as a value that may
/// initialize a mutable location. A type assertion, such as `1 as 1` or
/// `1 as const`, gets a reference of its own. Its literal type is not fresh,
/// and the reference keeps widening from taking it for the fresh literal
/// type of a literal expression.
fn value_reference(
    collector: &mut dyn RawTypeCollector,
    scope_id: ScopeId,
    expression: &AnyJsExpression,
) -> TypeReference {
    let reference = collector.reference_to_resolved_expression(scope_id, expression);
    if matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsTypeAssertionExpression(_)
    ) {
        let id = collector.register_type(Cow::Owned(TypeData::Reference(reference)));
        collector.reference_to_id(id)
    } else {
        reference
    }
}

/// Returns the type that a mutable location without a type annotation takes
/// from a value of type `value`: a [`TypeofWidenedLiteralExpression`] that
/// widens the fresh literal types of `value`, or `value` itself when its
/// collected type shows that it has no fresh literal type, as for an object,
/// a call, or a function.
///
/// With `is_return_type`, `value` is the type of the values that a
/// function's body returns, which widens only when it is a single literal
/// type. Resolution decides which literal types are fresh.
fn widened_literal_reference(
    collector: &mut dyn RawTypeCollector,
    value: TypeReference,
    is_return_type: bool,
) -> TypeReference {
    if !may_have_fresh_literal(
        collector,
        &value,
        is_return_type,
        MAX_FRESH_LITERAL_SHAPE_DEPTH,
    ) {
        return value;
    }
    collector.reference_to_owned_data(TypeData::from(TypeofExpression::WidenedLiteral(
        TypeofWidenedLiteralExpression {
            value,
            is_return_type,
        },
    )))
}

/// Returns the type that a class field takes from an initializer of type
/// `value`: `value` with its fresh literal types widened, or `value` itself
/// for a `readonly` field, whose literal types TypeScript keeps.
fn field_initializer_reference(
    collector: &mut dyn RawTypeCollector,
    value: TypeReference,
    is_readonly: bool,
) -> TypeReference {
    if is_readonly {
        value
    } else {
        widened_literal_reference(collector, value, false)
    }
}

/// Returns whether the collected type at `reference` may be or include a
/// fresh literal type. It may when the type is a primitive literal type, a
/// conditional or logical expression, a unary minus of a literal, a name,
/// because a name may refer to a `const` declared without a type annotation,
/// or a member access on a name with a name or a string literal as its key,
/// as `E.A` and `E["A"]`, because the name may refer to an enum, whose
/// members have fresh literal types. A reference to another local type is not
/// fresh, because
/// [`asserted_type`] and [`value_reference`] put assertions such as `1 as 1`
/// behind such a reference.
///
/// With `is_return_values`, a union stands for the values that a function
/// returns, and it may have a fresh literal type when one of its members
/// may. Any other union is a type annotation, whose literal types aren't
/// fresh. Beyond `depth` nested references and union members, the type
/// counts as possibly fresh.
fn may_have_fresh_literal(
    collector: &dyn RawTypeCollector,
    reference: &TypeReference,
    is_return_values: bool,
    depth: usize,
) -> bool {
    if depth == 0 {
        return true;
    }
    match reference {
        TypeReference::Qualifier(_) | TypeReference::Import(_) => true,
        TypeReference::Resolved(RawTypeId::Global(_)) => false,
        TypeReference::Resolved(RawTypeId::Local(id)) => match collector.get_by_id(*id) {
            TypeData::Literal(literal) => matches!(
                literal.as_ref(),
                Literal::BigInt(_) | Literal::Boolean(_) | Literal::Number(_) | Literal::String(_)
            ),
            TypeData::TypeofExpression(expression) => match expression.as_ref() {
                TypeofExpression::UnaryMinus(expression) => expression.is_literal_argument,
                TypeofExpression::Conditional(_)
                | TypeofExpression::LogicalAnd(_)
                | TypeofExpression::LogicalOr(_)
                | TypeofExpression::NullishCoalescing(_) => true,
                TypeofExpression::StaticMember(expression)
                | TypeofExpression::OptionalChainStaticMember(expression) => {
                    is_plain_name(collector, &expression.object)
                }
                TypeofExpression::ComputedMember(expression) => {
                    is_plain_name(collector, &expression.object)
                        && matches!(
                            collector.get_by_reference(&expression.member),
                            Some(TypeData::Literal(literal))
                                if matches!(literal.as_ref(), Literal::String(_))
                        )
                }
                _ => false,
            },
            TypeData::Reference(
                next @ (TypeReference::Qualifier(_) | TypeReference::Import(_)),
            ) => may_have_fresh_literal(collector, next, false, depth - 1),
            TypeData::Union(union) if is_return_values => union
                .types()
                .iter()
                .any(|member| may_have_fresh_literal(collector, member, false, depth - 1)),
            _ => false,
        },
    }
}

/// Returns whether the collected type at `reference` is the type of an
/// identifier expression, a name without type arguments, as `E` in `E.A`.
fn is_plain_name(collector: &dyn RawTypeCollector, reference: &TypeReference) -> bool {
    let qualifier = match reference {
        TypeReference::Qualifier(qualifier) => qualifier,
        TypeReference::Resolved(RawTypeId::Local(id)) => match collector.get_by_id(*id) {
            TypeData::Reference(TypeReference::Qualifier(qualifier)) => qualifier,
            _ => return false,
        },
        TypeReference::Resolved(RawTypeId::Global(_)) | TypeReference::Import(_) => return false,
    };
    qualifier.type_parameters.is_empty() && qualifier.path.identifier().is_some()
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
            TypeData::Tuple(Box::new(Tuple {
                elements,
                is_inferred_array: false,
            }))
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
            kind: object.kind,
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

/// Returns the `infer` types declared by the `extends` clause of a conditional.
///
/// An `infer` inside a nested conditional's `extends` clause belongs to the
/// nested conditional instead.
fn infer_types_declared_by(extends_type: &AnyTsType) -> Vec<TsInferType> {
    extends_type
        .syntax()
        .descendants()
        .filter_map(TsInferType::cast)
        .filter(|infer| {
            infer
                .syntax()
                .ancestors()
                .take_while(|ancestor| ancestor != extends_type.syntax())
                .filter_map(TsConditionalType::cast)
                .all(|conditional| {
                    conditional.extends_type().is_ok_and(|nested| {
                        !nested
                            .syntax()
                            .text_trimmed_range()
                            .contains_range(infer.syntax().text_trimmed_range())
                    })
                })
        })
        .collect()
}

/// Returns whether `ty` is a bare reference to a type parameter, which makes
/// a conditional type with this check type distributive.
///
/// For example, `T` is naked in `T extends string ? 1 : 2`, but `[T]` is not.
fn is_naked_type_parameter(ty: &AnyTsType) -> bool {
    let ty = ty.clone().omit_parentheses();
    let Some(reference) = ty.as_ts_reference_type() else {
        return false;
    };
    if reference.type_arguments().is_some() {
        return false;
    }
    let Some(name) = reference
        .name()
        .ok()
        .and_then(|name| name.as_js_reference_identifier().cloned())
        .and_then(|name| name.value_token().ok())
    else {
        return false;
    };
    let name = name.token_text_trimmed();

    reference.syntax().ancestors().any(|ancestor| {
        let declares_parameter = ancestor
            .children()
            .filter_map(TsTypeParameters::cast)
            .flat_map(|params| params.items().into_iter().flatten())
            .filter_map(|param| param.name().ok()?.ident_token().ok())
            .any(|param| param.token_text_trimmed() == name);
        let declares_infer = TsConditionalType::cast_ref(&ancestor)
            .filter(|conditional| {
                conditional.true_type().is_ok_and(|true_type| {
                    true_type
                        .syntax()
                        .text_trimmed_range()
                        .contains_range(reference.syntax().text_trimmed_range())
                })
            })
            .and_then(|conditional| conditional.extends_type().ok())
            .is_some_and(|extends_type| {
                infer_types_declared_by(&extends_type).iter().any(|infer| {
                    infer
                        .name()
                        .ok()
                        .and_then(|name| name.ident_token().ok())
                        .is_some_and(|infer| infer.token_text_trimmed() == name)
                })
            });
        declares_parameter || declares_infer
    })
}
