use crate::ModuleDb;
use biome_js_type_info::{
    ConstructorParameter as RawConstructorParameter, FunctionParameter as RawFunctionParameter,
    FunctionParameterBinding as RawFunctionParameterBinding, GLOBAL_RESOLVER,
    Literal as RawLiteral, Path, RawTypeData, ReturnType as RawReturnType,
    TupleElementType as RawTupleElementType, TypeId, TypeMember as RawTypeMember,
    TypeMemberAccessibility, TypeMemberKind as RawTypeMemberKind, TypeOperator as RawTypeOperator,
    TypeReference, TypeResolver, TypeResolverLevel,
    interned_types::{
        AssertsReturnType as InferredAssertsReturnType,
        ConstructorParameter as InferredConstructorParameter,
        FunctionParameter as InferredFunctionParameter,
        FunctionParameterBinding as InferredFunctionParameterBinding,
        InternedClass as InferredInternedClass, InternedConstructor as InferredInternedConstructor,
        InternedFunction as InferredInternedFunction,
        InternedGenericTypeParameter as InferredInternedGenericTypeParameter,
        InternedInterface as InferredInternedInterface, InternedLiteral as InferredInternedLiteral,
        InternedMergedReference as InferredInternedMergedReference,
        InternedModule as InferredInternedModule, InternedNamespace as InferredNamespace,
        InternedObject as InferredInternedObject, InternedTuple as InferredInternedTuple,
        InternedTypeOperatorType as InferredInternedTypeOperatorType, Literal as InferredLiteral,
        NamedFunctionParameter as InferredNamedFunctionParameter,
        PatternFunctionParameter as InferredPatternFunctionParameter,
        PredicateReturnType as InferredPredicateReturnType, ReturnType as InferredReturnType,
        TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeMemberKind as InferredTypeMemberKind,
    },
};
use biome_rowan::Text;
use rustc_hash::{FxHashMap, FxHashSet};

enum GlobalTypeWork<'a> {
    TypeId(TypeId),
    Raw(&'a RawTypeData),
    Reference(&'a TypeReference),
    RebuildTypeId(TypeId),
    RebuildClass {
        type_parameters: usize,
        has_extends: bool,
        implements: usize,
        members: usize,
        name: Option<Text>,
    },
    RebuildConstructor {
        type_parameters: usize,
        parameters: usize,
        has_return_type: bool,
    },
    RebuildFunction {
        type_parameters: usize,
        parameters: usize,
        is_async: bool,
        name: Option<Text>,
    },
    RebuildInterface {
        type_parameters: usize,
        extends: usize,
        members: usize,
        name: Text,
    },
    RebuildObject {
        has_prototype: bool,
        members: usize,
    },
    RebuildModule {
        members: usize,
        name: Text,
    },
    RebuildNamespace {
        members: usize,
        path: Path,
    },
    RebuildTuple(usize),
    RebuildGeneric {
        has_constraint: bool,
        has_default: bool,
        name: Text,
    },
    RebuildInstance(usize),
    RebuildIntersection(usize),
    RebuildUnion(usize),
    RebuildTypeOperator(RawTypeOperator),
    RebuildLiteralObject(usize),
    RebuildMergedReference {
        has_ty: bool,
        has_value_ty: bool,
        has_namespace_ty: bool,
    },
    RebuildTypeMember(&'a RawTypeMemberKind),
    RebuildConstructorParameter(Option<TypeMemberAccessibility>),
    RebuildFunctionParameter(&'a RawFunctionParameter),
    RebuildFunctionParameterBinding(Text),
    RebuildReturnType(&'a RawReturnType),
    RebuildTupleElement {
        name: Option<Text>,
        is_optional: bool,
        is_rest: bool,
    },
}

enum GlobalTypeValue<'db> {
    Type(InferredTypeData<'db>),
    Member(InferredTypeMember<'db>),
    ConstructorParameter(InferredConstructorParameter<'db>),
    FunctionParameter(InferredFunctionParameter<'db>),
    FunctionParameterBinding(InferredFunctionParameterBinding<'db>),
    ReturnType(InferredReturnType<'db>),
    TupleElement(InferredTupleElementType<'db>),
}

pub(in crate::db::type_inference) fn resolve_global_type_id<'db>(
    db: &'db dyn ModuleDb,
    type_id: TypeId,
    resolved_globals: &mut FxHashMap<TypeId, InferredTypeData<'db>>,
) -> InferredTypeData<'db> {
    resolve_global_type_id_with_resolver(db, GLOBAL_RESOLVER.as_ref(), type_id, resolved_globals)
}

fn resolve_global_type_id_with_resolver<'db>(
    db: &'db dyn ModuleDb,
    resolver: &dyn TypeResolver,
    type_id: TypeId,
    resolved_globals: &mut FxHashMap<TypeId, InferredTypeData<'db>>,
) -> InferredTypeData<'db> {
    if let Some(ty) = resolved_globals.get(&type_id) {
        return *ty;
    }

    let mut stack = vec![GlobalTypeWork::TypeId(type_id)];
    let mut values = Vec::new();
    let mut active = FxHashSet::default();
    let mut active_stack = Vec::new();
    let mut cycle_contaminated = FxHashSet::default();

    // The stack walks finite borrowed raw type trees. `TypeId` references
    // terminate through the memo table or the active-set cycle placeholder.
    while let Some(work) = stack.pop() {
        match work {
            GlobalTypeWork::TypeId(type_id) => {
                if let Some(ty) = resolved_globals.get(&type_id) {
                    values.push(GlobalTypeValue::Type(*ty));
                } else if active.contains(&type_id) {
                    cycle_contaminated.extend(active_stack.iter().copied());
                    values.push(GlobalTypeValue::Type(global_cycle_placeholder(
                        db, resolver, type_id,
                    )));
                } else {
                    let inserted = active.insert(type_id);
                    debug_assert!(
                        inserted,
                        "global type converter activated duplicate TypeId({}); check active set and active_stack updates around TypeId push/rebuild",
                        type_id.index()
                    );
                    active_stack.push(type_id);
                    stack.push(GlobalTypeWork::RebuildTypeId(type_id));
                    stack.push(GlobalTypeWork::Raw(resolver.get_by_id(type_id)));
                }
            }
            GlobalTypeWork::Raw(raw) => push_global_raw_type(db, raw, &mut stack, &mut values),
            GlobalTypeWork::Reference(reference) => {
                push_global_reference(resolver, reference, &mut stack, &mut values)
            }
            GlobalTypeWork::RebuildTypeId(type_id) => {
                let ty = pop_global_type(&mut values);
                debug_assert_eq!(
                    active_stack.pop(),
                    Some(type_id),
                    "global type converter RebuildTypeId was not paired with the active TypeId; check that every TypeId push has one matching RebuildTypeId"
                );
                let removed = active.remove(&type_id);
                debug_assert!(
                    removed,
                    "global type converter rebuilt inactive TypeId({}); check RebuildTypeId ordering and active set removal",
                    type_id.index()
                );
                let was_cycle_contaminated = cycle_contaminated.remove(&type_id);
                if active.is_empty() && !was_cycle_contaminated {
                    resolved_globals.insert(type_id, ty);
                }
                values.push(GlobalTypeValue::Type(ty));
            }
            GlobalTypeWork::RebuildClass {
                type_parameters,
                has_extends,
                implements,
                members,
                name,
            } => {
                let members = pop_global_members(&mut values, members);
                let implements = pop_global_types(&mut values, implements);
                let extends = has_extends.then(|| pop_global_type(&mut values));
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Class(
                    InferredInternedClass::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        extends,
                        implements.into_boxed_slice(),
                        members.into_boxed_slice(),
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildConstructor {
                type_parameters,
                parameters,
                has_return_type,
            } => {
                let return_type = has_return_type.then(|| pop_global_type(&mut values));
                let parameters = pop_global_constructor_parameters(&mut values, parameters);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Constructor(
                    InferredInternedConstructor::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        parameters.into_boxed_slice(),
                        return_type,
                    ),
                )));
            }
            GlobalTypeWork::RebuildFunction {
                type_parameters,
                parameters,
                is_async,
                name,
            } => {
                let return_type = pop_global_return_type(&mut values);
                let parameters = pop_global_function_parameters(&mut values, parameters);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Function(
                    InferredInternedFunction::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        parameters.into_boxed_slice(),
                        return_type,
                        is_async,
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildInterface {
                type_parameters,
                extends,
                members,
                name,
            } => {
                let members = pop_global_members(&mut values, members);
                let extends = pop_global_types(&mut values, extends);
                let type_parameters = pop_global_types(&mut values, type_parameters);
                values.push(GlobalTypeValue::Type(InferredTypeData::Interface(
                    InferredInternedInterface::new(
                        db,
                        type_parameters.into_boxed_slice(),
                        extends.into_boxed_slice(),
                        members.into_boxed_slice(),
                        name,
                    ),
                )));
            }
            GlobalTypeWork::RebuildObject {
                has_prototype,
                members,
            } => {
                let members = pop_global_members(&mut values, members);
                let prototype = has_prototype.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::Object(
                    InferredInternedObject::new(db, prototype, members.into_boxed_slice()),
                )));
            }
            GlobalTypeWork::RebuildModule { members, name } => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Module(
                    InferredInternedModule::new(db, members.into_boxed_slice(), name),
                )));
            }
            GlobalTypeWork::RebuildNamespace { members, path } => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Namespace(
                    InferredNamespace::new(db, members.into_boxed_slice(), path),
                )));
            }
            GlobalTypeWork::RebuildTuple(elements) => {
                let elements = pop_global_tuple_elements(&mut values, elements);
                values.push(GlobalTypeValue::Type(InferredTypeData::Tuple(
                    InferredInternedTuple::new(db, elements.into_boxed_slice()),
                )));
            }
            GlobalTypeWork::RebuildGeneric {
                has_constraint,
                has_default,
                name,
            } => {
                let default = has_default.then(|| pop_global_type(&mut values));
                let constraint = has_constraint.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::Generic(
                    InferredInternedGenericTypeParameter::new(db, constraint, default, name),
                )));
            }
            GlobalTypeWork::RebuildInstance(type_parameters) => {
                let type_parameters = pop_global_types(&mut values, type_parameters);
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::Type(InferredTypeData::instance_of(
                    db,
                    ty,
                    type_parameters.into_boxed_slice(),
                )));
            }
            GlobalTypeWork::RebuildIntersection(types) => {
                let types = pop_global_types(&mut values, types);
                values.push(GlobalTypeValue::Type(
                    InferredTypeData::intersection_from_types(db, types),
                ));
            }
            GlobalTypeWork::RebuildUnion(types) => {
                let types = pop_global_types(&mut values, types);
                values.push(GlobalTypeValue::Type(InferredTypeData::union_from_types(
                    db, types,
                )));
            }
            GlobalTypeWork::RebuildTypeOperator(operator) => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::Type(InferredTypeData::TypeOperator(
                    InferredInternedTypeOperatorType::new(db, ty, operator),
                )));
            }
            GlobalTypeWork::RebuildLiteralObject(members) => {
                let members = pop_global_members(&mut values, members);
                values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                    InferredInternedLiteral::new(
                        db,
                        InferredLiteral::Object(members.into_boxed_slice()),
                    ),
                )));
            }
            GlobalTypeWork::RebuildMergedReference {
                has_ty,
                has_value_ty,
                has_namespace_ty,
            } => {
                let namespace_ty = has_namespace_ty.then(|| pop_global_type(&mut values));
                let value_ty = has_value_ty.then(|| pop_global_type(&mut values));
                let ty = has_ty.then(|| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Type(InferredTypeData::MergedReference(
                    InferredInternedMergedReference::new(db, ty, value_ty, namespace_ty),
                )));
            }
            GlobalTypeWork::RebuildTypeMember(kind) => {
                let member_ty = pop_global_type(&mut values);
                let key_ty = raw_member_kind_reference(kind).map(|_| pop_global_type(&mut values));
                values.push(GlobalTypeValue::Member(InferredTypeMember {
                    kind: global_member_kind_from_raw(kind, key_ty),
                    ty: member_ty,
                }));
            }
            GlobalTypeWork::RebuildConstructorParameter(accessibility) => {
                let parameter = pop_global_function_parameter(&mut values);
                values.push(GlobalTypeValue::ConstructorParameter(
                    InferredConstructorParameter {
                        parameter,
                        accessibility,
                    },
                ));
            }
            GlobalTypeWork::RebuildFunctionParameter(parameter) => match parameter {
                RawFunctionParameter::Named(parameter) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::FunctionParameter(
                        InferredFunctionParameter::Named(InferredNamedFunctionParameter {
                            name: parameter.name.clone(),
                            ty,
                            is_optional: parameter.is_optional,
                            is_rest: parameter.is_rest,
                        }),
                    ));
                }
                RawFunctionParameter::Pattern(parameter) => {
                    let ty = pop_global_type(&mut values);
                    let bindings = pop_global_function_parameter_bindings(
                        &mut values,
                        parameter.bindings.len(),
                    );
                    values.push(GlobalTypeValue::FunctionParameter(
                        InferredFunctionParameter::Pattern(InferredPatternFunctionParameter {
                            bindings: bindings.into_boxed_slice(),
                            ty,
                            is_optional: parameter.is_optional,
                            is_rest: parameter.is_rest,
                        }),
                    ));
                }
            },
            GlobalTypeWork::RebuildFunctionParameterBinding(name) => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::FunctionParameterBinding(
                    InferredFunctionParameterBinding { name, ty },
                ));
            }
            GlobalTypeWork::RebuildReturnType(return_type) => match return_type {
                RawReturnType::Type(_) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Type(ty)));
                }
                RawReturnType::Predicate(predicate) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Predicate(
                        InferredPredicateReturnType {
                            parameter_name: predicate.parameter_name.clone(),
                            ty,
                        },
                    )));
                }
                RawReturnType::Asserts(asserts) => {
                    let ty = pop_global_type(&mut values);
                    values.push(GlobalTypeValue::ReturnType(InferredReturnType::Asserts(
                        InferredAssertsReturnType {
                            parameter_name: asserts.parameter_name.clone(),
                            ty,
                        },
                    )));
                }
            },
            GlobalTypeWork::RebuildTupleElement {
                name,
                is_optional,
                is_rest,
            } => {
                let ty = pop_global_type(&mut values);
                values.push(GlobalTypeValue::TupleElement(InferredTupleElementType {
                    ty,
                    name,
                    is_optional,
                    is_rest,
                }));
            }
        }
    }

    debug_assert!(
        active.is_empty(),
        "global type converter leaked active TypeIds; check that every active TypeId reaches RebuildTypeId"
    );
    debug_assert!(
        active_stack.is_empty(),
        "global type converter leaked active TypeId stack entries; check active_stack push/pop pairing"
    );
    debug_assert_eq!(
        values.len(),
        1,
        "global type converter stack imbalance; check that every GlobalTypeWork rebuild pushes one value"
    );
    pop_global_type(&mut values)
}

fn global_cycle_placeholder<'db>(
    db: &'db dyn ModuleDb,
    resolver: &dyn TypeResolver,
    type_id: TypeId,
) -> InferredTypeData<'db> {
    // Until globals have identity handles like local types, recursive edges are
    // represented by shallow placeholders. This preserves nominal checks but
    // caps structural member traversal through recursive global cycles.
    match resolver.get_by_id(type_id) {
        RawTypeData::Class(class) => InferredTypeData::Class(InferredInternedClass::new(
            db,
            Box::default(),
            None,
            Box::default(),
            Box::default(),
            class.name.clone(),
        )),
        RawTypeData::Interface(interface) => {
            InferredTypeData::Interface(InferredInternedInterface::new(
                db,
                Box::default(),
                Box::default(),
                Box::default(),
                interface.name.clone(),
            ))
        }
        RawTypeData::Generic(generic) => InferredTypeData::Generic(
            InferredInternedGenericTypeParameter::new(db, None, None, generic.name.clone()),
        ),
        RawTypeData::BigInt => InferredTypeData::BigInt,
        RawTypeData::Boolean => InferredTypeData::Boolean,
        RawTypeData::Conditional => InferredTypeData::Conditional,
        RawTypeData::Global => InferredTypeData::Global,
        RawTypeData::Null => InferredTypeData::Null,
        RawTypeData::Number => InferredTypeData::Number,
        RawTypeData::String => InferredTypeData::String,
        RawTypeData::Symbol => InferredTypeData::Symbol,
        RawTypeData::Undefined => InferredTypeData::Undefined,
        RawTypeData::AnyKeyword => InferredTypeData::AnyKeyword,
        RawTypeData::NeverKeyword => InferredTypeData::NeverKeyword,
        RawTypeData::ObjectKeyword => InferredTypeData::ObjectKeyword,
        RawTypeData::ThisKeyword => InferredTypeData::ThisKeyword,
        RawTypeData::UnknownKeyword => InferredTypeData::UnknownKeyword,
        RawTypeData::VoidKeyword => InferredTypeData::VoidKeyword,
        RawTypeData::Unknown
        | RawTypeData::ImportNamespace(_)
        | RawTypeData::Constructor(_)
        | RawTypeData::Function(_)
        | RawTypeData::Module(_)
        | RawTypeData::Namespace(_)
        | RawTypeData::Object(_)
        | RawTypeData::Tuple(_)
        | RawTypeData::Intersection(_)
        | RawTypeData::Union(_)
        | RawTypeData::TypeOperator(_)
        | RawTypeData::Literal(_)
        | RawTypeData::InstanceOf(_)
        | RawTypeData::Reference(_)
        | RawTypeData::MergedReference(_)
        | RawTypeData::TypeofExpression(_)
        | RawTypeData::TypeofType(_)
        | RawTypeData::TypeofValue(_) => InferredTypeData::Unknown,
    }
}

fn push_global_raw_type<'a, 'db>(
    db: &'db dyn ModuleDb,
    raw: &'a RawTypeData,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match raw {
        RawTypeData::Unknown => values.push(GlobalTypeValue::Type(InferredTypeData::Unknown)),
        RawTypeData::Global => values.push(GlobalTypeValue::Type(InferredTypeData::Global)),
        RawTypeData::BigInt => values.push(GlobalTypeValue::Type(InferredTypeData::BigInt)),
        RawTypeData::Boolean => values.push(GlobalTypeValue::Type(InferredTypeData::Boolean)),
        RawTypeData::Null => values.push(GlobalTypeValue::Type(InferredTypeData::Null)),
        RawTypeData::Number => values.push(GlobalTypeValue::Type(InferredTypeData::Number)),
        RawTypeData::String => values.push(GlobalTypeValue::Type(InferredTypeData::String)),
        RawTypeData::Symbol => values.push(GlobalTypeValue::Type(InferredTypeData::Symbol)),
        RawTypeData::Undefined => values.push(GlobalTypeValue::Type(InferredTypeData::Undefined)),
        RawTypeData::Conditional => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Conditional));
        }
        RawTypeData::ImportNamespace(_)
        | RawTypeData::TypeofExpression(_)
        | RawTypeData::TypeofType(_)
        | RawTypeData::TypeofValue(_) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
        }
        RawTypeData::Class(class) => {
            stack.push(GlobalTypeWork::RebuildClass {
                type_parameters: class.type_parameters.len(),
                has_extends: class.extends.is_some(),
                implements: class.implements.len(),
                members: class.members.len(),
                name: class.name.clone(),
            });
            push_raw_members(stack, &class.members);
            push_references(stack, &class.implements);
            if let Some(extends) = &class.extends {
                stack.push(GlobalTypeWork::Reference(extends));
            }
            push_references(stack, &class.type_parameters);
        }
        RawTypeData::Constructor(constructor) => {
            stack.push(GlobalTypeWork::RebuildConstructor {
                type_parameters: constructor.type_parameters.len(),
                parameters: constructor.parameters.len(),
                has_return_type: constructor.return_type.is_some(),
            });
            if let Some(return_type) = &constructor.return_type {
                stack.push(GlobalTypeWork::Reference(return_type));
            }
            push_constructor_parameters(stack, &constructor.parameters);
            push_references(stack, &constructor.type_parameters);
        }
        RawTypeData::Function(function) => {
            stack.push(GlobalTypeWork::RebuildFunction {
                type_parameters: function.type_parameters.len(),
                parameters: function.parameters.len(),
                is_async: function.is_async,
                name: function.name.clone(),
            });
            push_return_type(stack, &function.return_type);
            push_function_parameters(stack, &function.parameters);
            push_references(stack, &function.type_parameters);
        }
        RawTypeData::Interface(interface) => {
            stack.push(GlobalTypeWork::RebuildInterface {
                type_parameters: interface.type_parameters.len(),
                extends: interface.extends.len(),
                members: interface.members.len(),
                name: interface.name.clone(),
            });
            push_raw_members(stack, &interface.members);
            push_references(stack, &interface.extends);
            push_references(stack, &interface.type_parameters);
        }
        RawTypeData::Module(module) => {
            stack.push(GlobalTypeWork::RebuildModule {
                members: module.members.len(),
                name: module.name.clone(),
            });
            push_raw_members(stack, &module.members);
        }
        RawTypeData::Namespace(namespace) => {
            stack.push(GlobalTypeWork::RebuildNamespace {
                members: namespace.members.len(),
                path: namespace.path.clone(),
            });
            push_raw_members(stack, &namespace.members);
        }
        RawTypeData::Object(object) => {
            stack.push(GlobalTypeWork::RebuildObject {
                has_prototype: object.prototype.is_some(),
                members: object.members.len(),
            });
            push_raw_members(stack, &object.members);
            if let Some(prototype) = &object.prototype {
                stack.push(GlobalTypeWork::Reference(prototype));
            }
        }
        RawTypeData::Tuple(tuple) => {
            stack.push(GlobalTypeWork::RebuildTuple(tuple.elements().len()));
            push_tuple_elements(stack, tuple.elements());
        }
        RawTypeData::Generic(generic) => {
            let has_constraint = generic.constraint.is_known();
            let has_default = generic.default.is_known();
            stack.push(GlobalTypeWork::RebuildGeneric {
                has_constraint,
                has_default,
                name: generic.name.clone(),
            });
            if has_default {
                stack.push(GlobalTypeWork::Reference(&generic.default));
            }
            if has_constraint {
                stack.push(GlobalTypeWork::Reference(&generic.constraint));
            }
        }
        RawTypeData::Intersection(intersection) => {
            stack.push(GlobalTypeWork::RebuildIntersection(
                intersection.types().len(),
            ));
            push_references(stack, intersection.types());
        }
        RawTypeData::Union(union) => {
            stack.push(GlobalTypeWork::RebuildUnion(union.types().len()));
            push_references(stack, union.types());
        }
        RawTypeData::TypeOperator(type_operator) => {
            stack.push(GlobalTypeWork::RebuildTypeOperator(type_operator.operator));
            stack.push(GlobalTypeWork::Reference(&type_operator.ty));
        }
        RawTypeData::Literal(literal) => push_global_literal(db, literal.as_ref(), stack, values),
        RawTypeData::InstanceOf(instance) => {
            stack.push(GlobalTypeWork::RebuildInstance(
                instance.type_parameters.len(),
            ));
            push_references(stack, &instance.type_parameters);
            stack.push(GlobalTypeWork::Reference(&instance.ty));
        }
        RawTypeData::Reference(reference) => stack.push(GlobalTypeWork::Reference(reference)),
        RawTypeData::MergedReference(reference) => {
            stack.push(GlobalTypeWork::RebuildMergedReference {
                has_ty: reference.ty.is_some(),
                has_value_ty: reference.value_ty.is_some(),
                has_namespace_ty: reference.namespace_ty.is_some(),
            });
            if let Some(namespace_ty) = &reference.namespace_ty {
                stack.push(GlobalTypeWork::Reference(namespace_ty));
            }
            if let Some(value_ty) = &reference.value_ty {
                stack.push(GlobalTypeWork::Reference(value_ty));
            }
            if let Some(ty) = &reference.ty {
                stack.push(GlobalTypeWork::Reference(ty));
            }
        }
        RawTypeData::AnyKeyword => values.push(GlobalTypeValue::Type(InferredTypeData::AnyKeyword)),
        RawTypeData::NeverKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::NeverKeyword));
        }
        RawTypeData::ObjectKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::ObjectKeyword));
        }
        RawTypeData::ThisKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::ThisKeyword));
        }
        RawTypeData::UnknownKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::UnknownKeyword));
        }
        RawTypeData::VoidKeyword => {
            values.push(GlobalTypeValue::Type(InferredTypeData::VoidKeyword))
        }
    }
}

fn push_global_reference<'a, 'db>(
    resolver: &'a dyn TypeResolver,
    reference: &'a TypeReference,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match reference {
        TypeReference::Resolved(resolved_id)
            if resolved_id.level() == TypeResolverLevel::Global =>
        {
            stack.push(GlobalTypeWork::TypeId(resolved_id.id()));
        }
        TypeReference::Qualifier(qualifier) => {
            if let Some(resolved_id) = resolver.resolve_qualifier(qualifier) {
                stack.push(GlobalTypeWork::TypeId(resolved_id.id()));
            } else {
                values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
            }
        }
        TypeReference::Resolved(_) | TypeReference::Import(_) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Unknown));
        }
    }
}

fn push_global_literal<'a, 'db>(
    db: &'db dyn ModuleDb,
    literal: &'a RawLiteral,
    stack: &mut Vec<GlobalTypeWork<'a>>,
    values: &mut Vec<GlobalTypeValue<'db>>,
) {
    match literal {
        RawLiteral::BigInt(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::BigInt(value.clone())),
        ))),
        RawLiteral::Boolean(value) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                InferredInternedLiteral::new(db, InferredLiteral::Boolean(value.clone())),
            )))
        }
        RawLiteral::Number(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::Number(value.clone())),
        ))),
        RawLiteral::Object(object) => {
            stack.push(GlobalTypeWork::RebuildLiteralObject(object.members().len()));
            push_raw_members(stack, object.members());
        }
        RawLiteral::RegExp(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::RegExp(value.clone())),
        ))),
        RawLiteral::String(value) => values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
            InferredInternedLiteral::new(db, InferredLiteral::String(value.clone())),
        ))),
        RawLiteral::Template(value) => {
            values.push(GlobalTypeValue::Type(InferredTypeData::Literal(
                InferredInternedLiteral::new(db, InferredLiteral::Template(value.clone())),
            )))
        }
    }
}

fn push_references<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, references: &'a [TypeReference]) {
    for reference in references.iter().rev() {
        stack.push(GlobalTypeWork::Reference(reference));
    }
}

fn push_raw_members<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, members: &'a [RawTypeMember]) {
    for member in members.iter().rev() {
        stack.push(GlobalTypeWork::RebuildTypeMember(&member.kind));
        stack.push(GlobalTypeWork::Reference(&member.ty));
        if let Some(key_ty) = raw_member_kind_reference(&member.kind) {
            stack.push(GlobalTypeWork::Reference(key_ty));
        }
    }
}

fn push_constructor_parameters<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameters: &'a [RawConstructorParameter],
) {
    for parameter in parameters.iter().rev() {
        stack.push(GlobalTypeWork::RebuildConstructorParameter(
            parameter.accessibility,
        ));
        push_function_parameter(stack, &parameter.parameter);
    }
}

fn push_function_parameters<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameters: &'a [RawFunctionParameter],
) {
    for parameter in parameters.iter().rev() {
        push_function_parameter(stack, parameter);
    }
}

fn push_function_parameter<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    parameter: &'a RawFunctionParameter,
) {
    stack.push(GlobalTypeWork::RebuildFunctionParameter(parameter));
    match parameter {
        RawFunctionParameter::Named(parameter) => {
            stack.push(GlobalTypeWork::Reference(&parameter.ty));
        }
        RawFunctionParameter::Pattern(parameter) => {
            stack.push(GlobalTypeWork::Reference(&parameter.ty));
            push_function_parameter_bindings(stack, &parameter.bindings);
        }
    }
}

fn push_function_parameter_bindings<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    bindings: &'a [RawFunctionParameterBinding],
) {
    for binding in bindings.iter().rev() {
        stack.push(GlobalTypeWork::RebuildFunctionParameterBinding(
            binding.name.clone(),
        ));
        stack.push(GlobalTypeWork::Reference(&binding.ty));
    }
}

fn push_return_type<'a>(stack: &mut Vec<GlobalTypeWork<'a>>, return_type: &'a RawReturnType) {
    stack.push(GlobalTypeWork::RebuildReturnType(return_type));
    match return_type {
        RawReturnType::Type(ty) => stack.push(GlobalTypeWork::Reference(ty)),
        RawReturnType::Predicate(predicate) => stack.push(GlobalTypeWork::Reference(&predicate.ty)),
        RawReturnType::Asserts(asserts) => stack.push(GlobalTypeWork::Reference(&asserts.ty)),
    }
}

fn push_tuple_elements<'a>(
    stack: &mut Vec<GlobalTypeWork<'a>>,
    elements: &'a [RawTupleElementType],
) {
    for element in elements.iter().rev() {
        stack.push(GlobalTypeWork::RebuildTupleElement {
            name: element.name.clone(),
            is_optional: element.is_optional,
            is_rest: element.is_rest,
        });
        stack.push(GlobalTypeWork::Reference(&element.ty));
    }
}

fn raw_member_kind_reference(kind: &RawTypeMemberKind) -> Option<&TypeReference> {
    match kind {
        RawTypeMemberKind::ComputedValue(ty)
        | RawTypeMemberKind::ConstAssertedComputedValue(ty)
        | RawTypeMemberKind::ConstAssertedIndexSignature(ty)
        | RawTypeMemberKind::IndexSignature(ty) => Some(ty),
        RawTypeMemberKind::CallSignature
        | RawTypeMemberKind::ConstAssertedCallSignature
        | RawTypeMemberKind::ConstAssertedConstructor
        | RawTypeMemberKind::ConstAssertedGetter(_)
        | RawTypeMemberKind::ConstAssertedNamed(_)
        | RawTypeMemberKind::ConstAssertedNamedOptional(_)
        | RawTypeMemberKind::ConstAssertedNamedStatic(_)
        | RawTypeMemberKind::Constructor
        | RawTypeMemberKind::Getter(_)
        | RawTypeMemberKind::Named(_)
        | RawTypeMemberKind::NamedOptional(_)
        | RawTypeMemberKind::NamedStatic(_) => None,
    }
}

fn global_member_kind_from_raw<'db>(
    kind: &RawTypeMemberKind,
    key_ty: Option<InferredTypeData<'db>>,
) -> InferredTypeMemberKind<'db> {
    match kind {
        RawTypeMemberKind::CallSignature => InferredTypeMemberKind::CallSignature,
        RawTypeMemberKind::ComputedValue(_) => {
            InferredTypeMemberKind::ComputedValue(key_ty.unwrap_or(InferredTypeData::Unknown))
        }
        RawTypeMemberKind::ConstAssertedCallSignature => {
            InferredTypeMemberKind::ConstAssertedCallSignature
        }
        RawTypeMemberKind::ConstAssertedComputedValue(_) => {
            InferredTypeMemberKind::ConstAssertedComputedValue(
                key_ty.unwrap_or(InferredTypeData::Unknown),
            )
        }
        RawTypeMemberKind::ConstAssertedConstructor => {
            InferredTypeMemberKind::ConstAssertedConstructor
        }
        RawTypeMemberKind::ConstAssertedGetter(name) => {
            InferredTypeMemberKind::ConstAssertedGetter(name.clone())
        }
        RawTypeMemberKind::ConstAssertedIndexSignature(_) => {
            InferredTypeMemberKind::ConstAssertedIndexSignature(
                key_ty.unwrap_or(InferredTypeData::Unknown),
            )
        }
        RawTypeMemberKind::ConstAssertedNamed(name) => {
            InferredTypeMemberKind::ConstAssertedNamed(name.clone())
        }
        RawTypeMemberKind::ConstAssertedNamedOptional(name) => {
            InferredTypeMemberKind::ConstAssertedNamedOptional(name.clone())
        }
        RawTypeMemberKind::ConstAssertedNamedStatic(name) => {
            InferredTypeMemberKind::ConstAssertedNamedStatic(name.clone())
        }
        RawTypeMemberKind::Constructor => InferredTypeMemberKind::Constructor,
        RawTypeMemberKind::Getter(name) => InferredTypeMemberKind::Getter(name.clone()),
        RawTypeMemberKind::IndexSignature(_) => {
            InferredTypeMemberKind::IndexSignature(key_ty.unwrap_or(InferredTypeData::Unknown))
        }
        RawTypeMemberKind::Named(name) => InferredTypeMemberKind::Named(name.clone()),
        RawTypeMemberKind::NamedOptional(name) => {
            InferredTypeMemberKind::NamedOptional(name.clone())
        }
        RawTypeMemberKind::NamedStatic(name) => InferredTypeMemberKind::NamedStatic(name.clone()),
    }
}

fn pop_global_type<'db>(values: &mut Vec<GlobalTypeValue<'db>>) -> InferredTypeData<'db> {
    match values.pop() {
        Some(GlobalTypeValue::Type(ty)) => ty,
        _ => {
            unexpected_global_value("type");
            InferredTypeData::Unknown
        }
    }
}

fn pop_global_types<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTypeData<'db>> {
    let mut types = Vec::with_capacity(count);
    for _ in 0..count {
        types.push(pop_global_type(values));
    }
    types.reverse();
    types
}

fn pop_global_members<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTypeMember<'db>> {
    let mut members = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::Member(member)) => members.push(member),
            _ => {
                unexpected_global_value("member");
                members.push(InferredTypeMember {
                    kind: InferredTypeMemberKind::Named(Text::new_static("unknown")),
                    ty: InferredTypeData::Unknown,
                });
            }
        }
    }
    members.reverse();
    members
}

fn pop_global_constructor_parameters<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredConstructorParameter<'db>> {
    let mut parameters = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::ConstructorParameter(parameter)) => parameters.push(parameter),
            _ => {
                unexpected_global_value("constructor parameter");
                parameters.push(InferredConstructorParameter {
                    parameter: InferredFunctionParameter::Pattern(
                        InferredPatternFunctionParameter {
                            bindings: Box::default(),
                            ty: InferredTypeData::Unknown,
                            is_optional: false,
                            is_rest: false,
                        },
                    ),
                    accessibility: None,
                });
            }
        }
    }
    parameters.reverse();
    parameters
}

fn pop_global_function_parameters<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredFunctionParameter<'db>> {
    let mut parameters = Vec::with_capacity(count);
    for _ in 0..count {
        parameters.push(pop_global_function_parameter(values));
    }
    parameters.reverse();
    parameters
}

fn pop_global_function_parameter<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
) -> InferredFunctionParameter<'db> {
    match values.pop() {
        Some(GlobalTypeValue::FunctionParameter(parameter)) => parameter,
        _ => {
            unexpected_global_value("function parameter");
            InferredFunctionParameter::Pattern(InferredPatternFunctionParameter {
                bindings: Box::default(),
                ty: InferredTypeData::Unknown,
                is_optional: false,
                is_rest: false,
            })
        }
    }
}

fn pop_global_function_parameter_bindings<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredFunctionParameterBinding<'db>> {
    let mut bindings = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::FunctionParameterBinding(binding)) => bindings.push(binding),
            _ => {
                unexpected_global_value("function parameter binding");
                bindings.push(InferredFunctionParameterBinding {
                    name: Text::new_static("unknown"),
                    ty: InferredTypeData::Unknown,
                });
            }
        }
    }
    bindings.reverse();
    bindings
}

fn pop_global_return_type<'db>(values: &mut Vec<GlobalTypeValue<'db>>) -> InferredReturnType<'db> {
    match values.pop() {
        Some(GlobalTypeValue::ReturnType(return_type)) => return_type,
        _ => {
            unexpected_global_value("return type");
            InferredReturnType::Type(InferredTypeData::Unknown)
        }
    }
}

fn pop_global_tuple_elements<'db>(
    values: &mut Vec<GlobalTypeValue<'db>>,
    count: usize,
) -> Vec<InferredTupleElementType<'db>> {
    let mut elements = Vec::with_capacity(count);
    for _ in 0..count {
        match values.pop() {
            Some(GlobalTypeValue::TupleElement(element)) => elements.push(element),
            _ => {
                unexpected_global_value("tuple element");
                elements.push(InferredTupleElementType {
                    ty: InferredTypeData::Unknown,
                    name: None,
                    is_optional: false,
                    is_rest: false,
                });
            }
        }
    }
    elements.reverse();
    elements
}

fn unexpected_global_value(expected: &'static str) {
    debug_assert!(
        false,
        "global type converter expected {expected} on the value stack; check GlobalTypeWork rebuild order and pop helper calls"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::module_graph::{ModuleInfo, ModuleInfoKind};
    use std::borrow::Cow;

    use biome_db::ParsedSource;
    use biome_js_semantic::ScopeId;
    use biome_js_syntax::AnyJsExpression;
    use biome_js_type_info::{
        GlobalsResolver, ResolvedTypeData, ResolvedTypeId, TypeDb as JsTypeDb,
        TypeReferenceQualifier,
    };
    use camino::Utf8Path;
    use salsa::Storage;

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &Utf8Path) -> Option<ParsedSource> {
            None
        }
    }

    #[salsa::db]
    impl JsTypeDb for TestDb {}

    #[salsa::db]
    impl ModuleDb for TestDb {
        fn module_for_path(&self, _path: &Utf8Path) -> Option<ModuleInfo> {
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {}
    }

    struct NoopTypeResolver;

    impl TypeResolver for NoopTypeResolver {
        fn level(&self) -> TypeResolverLevel {
            TypeResolverLevel::Global
        }

        fn find_type(&self, _type_data: &RawTypeData) -> Option<TypeId> {
            None
        }

        fn get_by_id(&self, _id: TypeId) -> &RawTypeData {
            panic!("noop resolver must not resolve type IDs");
        }

        fn get_by_resolved_id(&self, _id: ResolvedTypeId) -> Option<ResolvedTypeData<'_>> {
            None
        }

        fn register_type(&mut self, _type_data: Cow<RawTypeData>) -> TypeId {
            panic!("noop resolver must not register types");
        }

        fn resolve_reference(&self, _ty: &TypeReference) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_qualifier(&self, _qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_type_of(
            &self,
            _identifier: &Text,
            _scope_id: ScopeId,
        ) -> Option<ResolvedTypeId> {
            None
        }

        fn resolve_expression(
            &mut self,
            _scope_id: ScopeId,
            _expr: &AnyJsExpression,
        ) -> Cow<'_, RawTypeData> {
            Cow::Owned(RawTypeData::Unknown)
        }

        fn registered_types(&self) -> Vec<&RawTypeData> {
            Vec::new()
        }
    }

    fn global_type_id(resolver: &GlobalsResolver, name: &'static str) -> TypeId {
        resolver
            .resolve_qualifier(&TypeReferenceQualifier::from_path(
                ScopeId::GLOBAL,
                Text::new_static(name),
            ))
            .unwrap_or_else(|| panic!("{name} must resolve as a global type"))
            .id()
    }

    fn global_type_ref(type_id: TypeId) -> TypeReference {
        ResolvedTypeId::new(TypeResolverLevel::Global, type_id).into()
    }

    fn is_class_named(db: &dyn ModuleDb, ty: InferredTypeData, name: &str) -> bool {
        matches!(ty, InferredTypeData::Class(class) if class.name(db).as_ref().is_some_and(|class_name| class_name.text() == name))
    }

    #[test]
    fn global_converter_normalizes_compound_rebuilds() {
        let db = TestDb::default();
        let mut resolver = GlobalsResolver::default();
        let unresolved = NoopTypeResolver;
        let promise_id = global_type_id(&resolver, "Promise");
        let array_id = global_type_id(&resolver, "Array");

        let inner_union = RawTypeData::union_of(
            &unresolved,
            Box::new([global_type_ref(promise_id), global_type_ref(array_id)]),
        );
        let inner_union_id = resolver.register_type(Cow::Owned(inner_union));
        let outer_union = RawTypeData::union_of(
            &unresolved,
            Box::new([global_type_ref(inner_union_id), global_type_ref(promise_id)]),
        );
        let outer_union_id = resolver.register_type(Cow::Owned(outer_union));
        let union_ty = resolve_global_type_id_with_resolver(
            &db,
            &resolver,
            outer_union_id,
            &mut FxHashMap::default(),
        );
        let InferredTypeData::Union(union) = union_ty else {
            panic!("outer union must stay a union, got {union_ty:?}");
        };
        assert_eq!(union.types(&db).len(), 2);
        assert!(
            union
                .types(&db)
                .iter()
                .all(|ty| !matches!(ty, InferredTypeData::Union(_)))
        );
        assert!(
            union
                .types(&db)
                .iter()
                .any(|ty| is_class_named(&db, *ty, "Promise"))
        );
        assert!(
            union
                .types(&db)
                .iter()
                .any(|ty| is_class_named(&db, *ty, "Array"))
        );

        let inner_intersection = RawTypeData::intersection_of(Vec::from([
            global_type_ref(promise_id),
            global_type_ref(array_id),
        ]));
        let inner_intersection_id = resolver.register_type(Cow::Owned(inner_intersection));
        let outer_intersection = RawTypeData::intersection_of(Vec::from([
            global_type_ref(inner_intersection_id),
            global_type_ref(promise_id),
        ]));
        let outer_intersection_id = resolver.register_type(Cow::Owned(outer_intersection));
        let intersection_ty = resolve_global_type_id_with_resolver(
            &db,
            &resolver,
            outer_intersection_id,
            &mut FxHashMap::default(),
        );
        match intersection_ty {
            InferredTypeData::Intersection(intersection) => {
                assert_eq!(intersection.types(&db).len(), 2);
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .all(|ty| !matches!(ty, InferredTypeData::Intersection(_)))
                );
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .any(|ty| is_class_named(&db, *ty, "Promise"))
                );
                assert!(
                    intersection
                        .types(&db)
                        .iter()
                        .any(|ty| is_class_named(&db, *ty, "Array"))
                );
            }
            InferredTypeData::Object(_) => {}
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
            | InferredTypeData::Global
            | InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::Conditional
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => panic!(
                "outer intersection must become a normalized intersection or object, got {intersection_ty:?}"
            ),
        }
    }

    #[test]
    fn global_converter_does_not_memoize_cycle_contaminated_types() {
        let db = TestDb::default();
        let resolver = GlobalsResolver::default();
        let promise_id = global_type_id(&resolver, "Promise");
        let mut resolved_globals = FxHashMap::default();

        let promise_ty =
            resolve_global_type_id_with_resolver(&db, &resolver, promise_id, &mut resolved_globals);

        assert!(is_class_named(&db, promise_ty, "Promise"));
        assert!(resolved_globals.is_empty());

        let promise_ty =
            resolve_global_type_id_with_resolver(&db, &resolver, promise_id, &mut resolved_globals);
        assert!(is_class_named(&db, promise_ty, "Promise"));
        assert!(resolved_globals.is_empty());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "global type converter expected type on the value stack")]
    fn global_converter_value_mismatch_panics_in_debug() {
        let mut values = vec![GlobalTypeValue::ReturnType(InferredReturnType::Type(
            InferredTypeData::Unknown,
        ))];

        let _ = pop_global_type(&mut values);
    }
}
