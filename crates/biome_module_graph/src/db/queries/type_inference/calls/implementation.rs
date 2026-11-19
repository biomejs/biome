//! Call matching, argument expansion, and generic return substitution.
//!
//! Query entry points delegate to this module after Salsa has established their
//! incremental boundary. The algorithms here share bounded traversal state and
//! resolved argument representations with expression inference.

use super::resolve_callable_function;
use crate::db::queries::{CallableFunctionInput, resolve_callable_type};
use crate::db::type_inference::{
    apply_substitutions, apply_substitutions_to_root_body, collected_type_result,
    find_member_type_on_demand, receiver_substitutions, resolve_local_type_on_demand,
    substitutions_for_instance,
};
use crate::module_graph::ModuleInfoKind;
use crate::{ModuleDb, module_for_key};
use biome_js_type_info::{
    Class, PrimitiveLiteralKind, RawTypeData, TypeReference, TypeResolverLevel, global_types,
    interned_types::{
        CallArgumentType as InferredCallArgumentType, ClassParents,
        FunctionParameter as InferredFunctionParameter, InternedConstructor as InferredConstructor,
        InternedFunction as InferredFunction,
        InternedGenericTypeParameter as InferredGenericTypeParameter,
        InternedMergedReference as InferredMergedReference, InternedTuple as InferredTuple,
        InternedTypeInstance as InferredTypeInstance, Literal as InferredLiteral,
        LocalTypeHandle as InferredLocalTypeHandle, LocalTypeId as InferredLocalTypeId,
        ModuleKey as InferredModuleKey, ReturnType, TupleElementType as InferredTupleElementType,
        TypeData as InferredTypeData, TypeMember as InferredTypeMember,
        TypeSubstitution as InferredTypeSubstitution, TypeTransformResult,
    },
    is_assignable_to, is_subtype_of,
};
use biome_rowan::Text;
use rustc_hash::FxHashSet;

const MAX_ARGUMENT_MATCH_STEPS: usize = 1024;
const MAX_ARGUMENT_SEQUENCE_STEPS: usize = 4096;
const MAX_ARGUMENT_TYPE_STEPS: usize = 1024;
const MAX_LOCAL_EXTENDS_STEPS: usize = 1024;
const MAX_STRUCTURAL_MEMBER_STEPS: usize = 1024;

// #region CALL INFERENCE IMPLEMENTATION

/// Infers a call result from definite positional argument types.
pub(in crate::db) fn infer_call_expression_return_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[InferredTypeData<'db>],
) -> InferredTypeData<'db> {
    let args = args
        .iter()
        .copied()
        .map(ResolvedCallArgument::Argument)
        .collect::<Vec<_>>();
    infer_call_expression_return_type_from_args(db, callee, &args, &mut || None)
}

/// Infers a call result after optional and spread arguments are resolved.
///
/// A union of callable types produces a union of their return types. A `null`
/// or `undefined` member contributes `undefined`. Other non-callable members
/// are omitted. The result is `Unknown` when no supported callable member
/// remains.
///
/// `receiver` gives the type of the object that the callee is read from, as
/// `xs` in `xs.filter(isString)`, or `None` for another callee (see
/// [`CallReceiver`]). Its type arguments let a type parameter of the method,
/// such as `S` in `filter<S extends T>`, be checked against a constraint
/// that names a type parameter of the class or interface, such as `T`.
///
/// In this example, the result is `string | number`.
///
/// ```ts
/// declare const read: (() => string) | (() => number);
/// const result = read();
/// ```
pub(in crate::db) fn infer_call_expression_return_type_from_args<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    receiver: &mut CallReceiver<'db, '_>,
) -> InferredTypeData<'db> {
    match callee {
        InferredTypeData::Union(union) => collected_type_result(
            db,
            union
                .types(db)
                .iter()
                .filter_map(|callee| {
                    if matches!(callee, InferredTypeData::Null | InferredTypeData::Undefined) {
                        Some(InferredTypeData::Undefined)
                    } else {
                        infer_function_call_type(db, *callee, args, receiver)
                    }
                })
                .collect(),
        )
        .unwrap_or(InferredTypeData::Unknown),
        callee => infer_function_call_type(db, callee, args, receiver)
            .unwrap_or(InferredTypeData::Unknown),
    }
}

fn infer_function_call_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    receiver: &mut CallReceiver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Function(function) => {
            infer_function_return_type(db, function, args, receiver)
        }
        InferredTypeData::InstanceOf(instance) => {
            let target = instance.ty(db);
            // A class's call signatures belong to its constructor value.
            if matches!(
                target.expand_canonical_global(db),
                InferredTypeData::Class(_)
            ) {
                return None;
            }
            let substitutions =
                substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
            let target = apply_substitutions_to_root_body(db, target, &substitutions);
            infer_function_call_type(db, target, args, receiver)
        }
        InferredTypeData::Interface(interface) => {
            select_call_signature(db, interface.members(db), args)
                .and_then(|function| infer_function_return_type(db, function, args, receiver))
        }
        InferredTypeData::Class(class) => select_call_signature(db, class.members(db), args)
            .and_then(|function| infer_function_return_type(db, function, args, receiver)),
        InferredTypeData::Object(object) => select_call_signature(db, object.members(db), args)
            .and_then(|function| infer_function_return_type(db, function, args, receiver)),
        InferredTypeData::Union(union) => collected_type_result(
            db,
            union
                .types(db)
                .iter()
                .filter_map(|callee| {
                    if matches!(callee, InferredTypeData::Null | InferredTypeData::Undefined) {
                        Some(InferredTypeData::Undefined)
                    } else {
                        infer_function_call_type(db, *callee, args, receiver)
                    }
                })
                .collect(),
        ),
        InferredTypeData::TypeofType(typeof_type) => {
            infer_function_call_type(db, typeof_type.ty(db), args, receiver)
        }
        InferredTypeData::TypeofValue(typeof_value) => {
            infer_function_call_type(db, typeof_value.ty(db), args, receiver)
        }
        _ => None,
    }
}

fn select_call_signature<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredFunction<'db>> {
    let mut signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature())
        .filter_map(|member| member.ty.expand_canonical_global(db).callable_function(db));
    let first = signatures.next()?;
    let Some(second) = signatures.next() else {
        return Some(first);
    };

    [first, second]
        .into_iter()
        .chain(signatures)
        .find(|function| signature_may_accept_arguments(db, *function, args))
}

/// Expands tuple spreads and maps a source argument index to the expanded list.
///
/// Required tuple elements become positional arguments. Optional elements may
/// be absent, and tuple rest elements remain spreads. A non-tuple spread also
/// remains a spread. Cyclic or over-budget tuple branches become unknown
/// spreads.
///
/// In this example, `enabled` has source index 1 and expanded index 2 because
/// `args` contributes two entries.
///
/// ```ts
/// declare function run(
///     name: string,
///     countOrEnabled: number | boolean | undefined,
///     enabled?: boolean,
/// ): void;
/// const args: [string, number?] = ["task"];
/// const enabled = true;
/// run(...args, enabled);
/// ```
pub(in crate::db) fn resolved_call_arguments<'db>(
    db: &'db dyn ModuleDb,
    args: &[InferredCallArgumentType<'db>],
    argument_index: usize,
) -> (Vec<ResolvedCallArgument<'db>>, usize) {
    let mut resolved = Vec::new();
    let mut resolved_argument_index = argument_index;

    for (index, argument) in args.iter().enumerate() {
        if index == argument_index {
            resolved_argument_index = resolved.len();
        }
        match argument {
            InferredCallArgumentType::Argument(ty) => {
                resolved.push(ResolvedCallArgument::Argument(*ty));
            }
            InferredCallArgumentType::Spread(ty) => {
                push_resolved_spread_type(db, *ty, &mut resolved);
            }
        }
    }

    (resolved, resolved_argument_index)
}

fn resolve_tuple_instance<'db>(
    db: &'db dyn ModuleDb,
    instance: InferredTypeInstance<'db>,
    remaining_steps: &mut usize,
) -> TupleInstanceResolution<'db> {
    let mut ty = InferredTypeData::InstanceOf(instance);
    let mut seen = FxHashSet::default();

    loop {
        let InferredTypeData::InstanceOf(instance) = ty else {
            return match ty {
                InferredTypeData::Tuple(tuple) => TupleInstanceResolution::Tuple(tuple),
                _ => TupleInstanceResolution::NotTuple,
            };
        };
        if *remaining_steps == 0 || !seen.insert(instance) {
            return TupleInstanceResolution::Indeterminate;
        }
        *remaining_steps -= 1;

        let target = resolve_local_type_on_demand(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        ty = apply_substitutions_to_root_body(db, target, &substitutions);
    }
}

fn tuple_expansion_item<'db>(
    element: &InferredTupleElementType<'db>,
    unallocated_steps: &mut usize,
    unallocated_rest_elements: &mut usize,
) -> TupleExpansionItem<'db> {
    if element.is_rest {
        let child_steps = unallocated_steps.div_ceil(*unallocated_rest_elements);
        *unallocated_steps -= child_steps;
        *unallocated_rest_elements -= 1;
        TupleExpansionItem::Type {
            ty: element.ty,
            remaining_steps: child_steps,
        }
    } else if element.is_optional {
        TupleExpansionItem::Element(ExpandedTupleElement::Optional(element.ty))
    } else {
        TupleExpansionItem::Element(ExpandedTupleElement::Required(element.ty))
    }
}

/// Expands nested tuple and tuple-alias elements into argument-order entries.
///
/// Required and optional tuple elements retain their order. Tuple rest
/// elements and non-tuple inputs remain variable-length spreads. A branch that
/// is cyclic or exceeds its expansion budget becomes an `Unknown` spread.
fn expanded_tuple_sequence<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Vec<ExpandedTupleElement<'db>> {
    let mut expanded = Vec::new();
    let mut pending = Vec::from([TupleExpansionItem::Type {
        ty,
        remaining_steps: MAX_ARGUMENT_TYPE_STEPS,
    }]);
    let mut active = FxHashSet::default();

    while let Some(item) = pending.pop() {
        match item {
            TupleExpansionItem::Type {
                ty,
                mut remaining_steps,
            } => {
                if remaining_steps == 0 {
                    expanded.push(ExpandedTupleElement::Spread(InferredTypeData::Unknown));
                    continue;
                }
                remaining_steps -= 1;

                let tuple = match ty {
                    InferredTypeData::Tuple(tuple) => Some(tuple),
                    InferredTypeData::InstanceOf(instance) => {
                        match resolve_tuple_instance(db, instance, &mut remaining_steps) {
                            TupleInstanceResolution::Tuple(tuple) => Some(tuple),
                            TupleInstanceResolution::NotTuple => None,
                            TupleInstanceResolution::Indeterminate => {
                                expanded
                                    .push(ExpandedTupleElement::Spread(InferredTypeData::Unknown));
                                continue;
                            }
                        }
                    }
                    _ => None,
                };
                let Some(tuple) = tuple else {
                    expanded.push(ExpandedTupleElement::Spread(ty));
                    continue;
                };
                if !active.insert(ty) {
                    expanded.push(ExpandedTupleElement::Spread(InferredTypeData::Unknown));
                    continue;
                }

                let elements = tuple.elements(db);
                let scheduled_count = elements.len().min(remaining_steps);
                remaining_steps -= scheduled_count;
                let truncated = scheduled_count < elements.len();
                let prefix_count = if truncated {
                    scheduled_count / 2
                } else {
                    scheduled_count
                };
                let suffix_count = scheduled_count - prefix_count;
                let suffix_start = elements.len() - suffix_count;
                let mut unallocated_rest_elements = elements[..prefix_count]
                    .iter()
                    .chain(&elements[suffix_start..])
                    .filter(|element| element.is_rest)
                    .count();
                let mut scheduled = Vec::with_capacity(scheduled_count + usize::from(truncated));

                for element in &elements[..prefix_count] {
                    scheduled.push(tuple_expansion_item(
                        element,
                        &mut remaining_steps,
                        &mut unallocated_rest_elements,
                    ));
                }
                if truncated {
                    // Keeping both ends preserves fixed suffix parameters after
                    // the unknown middle of an over-budget tuple.
                    scheduled.push(TupleExpansionItem::Element(ExpandedTupleElement::Spread(
                        InferredTypeData::Unknown,
                    )));
                }
                for element in &elements[suffix_start..] {
                    scheduled.push(tuple_expansion_item(
                        element,
                        &mut remaining_steps,
                        &mut unallocated_rest_elements,
                    ));
                }

                pending.push(TupleExpansionItem::Exit(ty));
                pending.extend(scheduled.into_iter().rev());
            }
            TupleExpansionItem::Exit(ty) => {
                active.remove(&ty);
            }
            TupleExpansionItem::Element(element) => expanded.push(element),
        }
    }

    expanded
}

fn push_resolved_spread_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    resolved: &mut Vec<ResolvedCallArgument<'db>>,
) {
    let expanded = expanded_tuple_sequence(db, ty);
    resolved.extend(expanded.into_iter().map(|element| match element {
        ExpandedTupleElement::Required(ty) => ResolvedCallArgument::Argument(ty),
        ExpandedTupleElement::Optional(ty) => {
            let ty = InferredTypeData::union_from_types(
                db,
                Vec::from([ty, InferredTypeData::Undefined]),
            );
            ResolvedCallArgument::Optional(ty)
        }
        ExpandedTupleElement::Spread(ty) => ResolvedCallArgument::Spread(ty),
    }));
}

/// Infers the expected type of one argument in a function call.
///
/// When the function has overloads, the argument being examined is not used to
/// choose one. Its expected type is the result being inferred. The other
/// arguments must match an overload. Returns `None` when the called value is not
/// supported, a union branch cannot provide a type, no overload matches, or
/// inference reaches its work limit.
///
/// In the following example, the first argument selects the `sync` overload.
/// The expected type of the second argument is `() => void`.
///
/// ```ts
/// declare function schedule(kind: "sync", task: () => void): void;
/// declare function schedule(kind: "async", task: () => Promise<void>): void;
/// schedule("sync", async () => {});
/// ```
pub(in crate::db) fn infer_function_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    infer_argument_type(
        db,
        callee,
        args,
        argument_index,
        infer_callable_argument_type,
    )
}

fn infer_callable_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Function(function) => {
            infer_single_signature_parameter_type(db, function, args, argument_index)
        }
        InferredTypeData::Interface(interface) => {
            infer_call_signature_argument_type(db, interface.members(db), args, argument_index)
        }
        InferredTypeData::Object(object) => {
            infer_call_signature_argument_type(db, object.members(db), args, argument_index)
        }
        _ => None,
    }
}

fn infer_call_signature_argument_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let mut signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature())
        .filter_map(|member| member.ty.expand_global_local(db).callable_function(db));
    let first = signatures.next()?;
    let Some(second) = signatures.next() else {
        return infer_single_signature_parameter_type(db, first, args, argument_index);
    };

    [first, second]
        .into_iter()
        .chain(signatures)
        .find_map(|function| infer_parameter_type_for_argument(db, function, args, argument_index))
}

/// Infers the expected type of one argument in a constructor call.
///
/// The value being constructed may be a class or another type that defines a
/// constructor. When it has overloads, the argument being examined is not used
/// to choose one. The other arguments must match an overload. Returns `None`
/// when the value cannot be constructed, a union branch cannot provide a type,
/// no overload matches, or inference reaches its work limit.
///
/// In the following example, the first argument selects the `sync` constructor.
/// The expected type of the second argument is `() => void`.
///
/// ```ts
/// declare const Job: {
///     new (kind: "sync", task: () => void): object;
///     new (kind: "async", task: () => Promise<void>): object;
/// };
/// new Job("sync", async () => {});
/// ```
pub(in crate::db) fn infer_constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    infer_argument_type(
        db,
        callee,
        args,
        argument_index,
        infer_constructable_argument_type,
    )
}

fn infer_constructable_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Class(class) => {
            select_constructor_argument_type(db, class.members(db), args, argument_index)
        }
        InferredTypeData::Interface(interface) => {
            select_constructor_argument_type(db, interface.members(db), args, argument_index)
        }
        InferredTypeData::Object(object) => {
            select_constructor_argument_type(db, object.members(db), args, argument_index)
        }
        InferredTypeData::Constructor(constructor) => {
            let parameters = ResolvedParameters::from_constructor(db, constructor);
            infer_single_signature_resolved_parameter_type(
                db,
                parameters.as_slice(),
                args,
                argument_index,
            )
        }
        _ => None,
    }
}

fn infer_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
    infer_direct: ArgumentTypeInference<'db>,
) -> Option<InferredTypeData<'db>> {
    let mut pending = Vec::from([ArgumentTypeItem::Type(callee)]);
    let mut results = Vec::new();

    for _ in 0..MAX_ARGUMENT_TYPE_STEPS {
        let Some(item) = pending.pop() else {
            let result = results.pop()?;
            return results.is_empty().then_some(result);
        };

        match item {
            ArgumentTypeItem::Type(callee) => match callee {
                InferredTypeData::GlobalType(id) => {
                    pending.push(ArgumentTypeItem::Type(global_types(db).get(id)));
                }
                InferredTypeData::GlobalLocal(local) => {
                    pending.push(ArgumentTypeItem::Type(local.expand(db)));
                }
                InferredTypeData::InstanceOf(instance) => {
                    let target = resolve_local_type_on_demand(db, instance.ty(db));
                    let substitutions =
                        substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
                    pending.push(ArgumentTypeItem::Type(apply_substitutions_to_root_body(
                        db,
                        target,
                        &substitutions,
                    )));
                }
                InferredTypeData::Local(_) => {
                    let resolved = resolve_local_type_on_demand(db, callee);
                    if resolved == callee {
                        return None;
                    }
                    pending.push(ArgumentTypeItem::Type(resolved));
                }
                InferredTypeData::Union(union) => {
                    let types = union.types(db);
                    pending.push(ArgumentTypeItem::CollectUnion(types.len()));
                    pending.extend(types.iter().rev().copied().map(ArgumentTypeItem::Type));
                }
                InferredTypeData::TypeofType(typeof_type) => {
                    pending.push(ArgumentTypeItem::Type(typeof_type.ty(db)));
                }
                InferredTypeData::TypeofValue(typeof_value) => {
                    pending.push(ArgumentTypeItem::Type(typeof_value.ty(db)));
                }
                callee => {
                    results.push(infer_direct(db, callee, args, argument_index)?);
                }
            },
            ArgumentTypeItem::CollectUnion(type_count) => {
                let start = results.len().checked_sub(type_count)?;
                let types = results.split_off(start);
                results.push(collected_type_result(db, types)?);
            }
        }
    }

    None
}

/// Selects a constructor and returns the requested parameter type.
///
/// A type with one constructor uses that constructor directly. Overloaded
/// constructors are checked in declaration order. The argument being examined
/// is ignored during that check, while every other argument must match. Returns
/// the first matching parameter type, or `None` if no constructor matches.
///
/// In the following example, `"named"` selects the second constructor. The
/// expected type of the second argument is `string`.
///
/// ```ts
/// declare const Entry: {
///     new (kind: "indexed", value: number): object;
///     new (kind: "named", value: string): object;
/// };
/// const value: string = "item";
/// new Entry("named", value);
/// ```
fn select_constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let mut signatures = members
        .iter()
        .filter(|member| member.kind.is_constructor())
        .filter_map(|member| {
            ResolvedParameters::from_constructor_signature(db, member.ty.expand_global_local(db))
        });
    let first = signatures.next()?;
    let Some(second) = signatures.next() else {
        return infer_single_signature_resolved_parameter_type(
            db,
            first.as_slice(),
            args,
            argument_index,
        );
    };

    [first, second]
        .into_iter()
        .chain(signatures)
        .find_map(|parameters| {
            infer_parameter_type_for_resolved_argument(
                db,
                parameters.as_slice(),
                args,
                argument_index,
            )
        })
}

/// Returns whether `function` may accept `args` during overload selection
/// for a call's return type: false only when the arguments provably don't
/// fit its parameters, and true for unsupported relations and when the
/// matching runs out of work. [`signature_accepts_arguments`] tells a
/// conclusive acceptance apart from a possible one.
fn signature_may_accept_arguments<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    let parameters = ResolvedParameters::from_function(db, function);
    let parameters = parameters.as_slice();
    if parameters
        .iter()
        .all(|parameter| matches!(parameter, ResolvedParameter::Required(_)))
        && args.iter().all(|argument| {
            matches!(
                argument,
                ResolvedCallArgument::Argument(_)
                    | ResolvedCallArgument::ConstArgument { .. }
                    | ResolvedCallArgument::WidenedArgument { .. }
            )
        })
    {
        return parameters.len() == args.len()
            && parameters
                .iter()
                .zip(args)
                .all(|(parameter, argument)| argument.matches_parameter(db, parameter.ty()));
    }

    matched_parameter_types(db, parameters, args, None).is_some()
}

/// Gives the type of the object that a call's callee is read from, as `xs`
/// in `xs.filter(isString)`, or `None` for a callee that reads no member.
/// Call inference asks for it only to check a type parameter's constraint
/// that names a type parameter of the method's class or interface, so the
/// object, such as a namespace import, is resolved only then.
pub(in crate::db) type CallReceiver<'db, 'a> = dyn FnMut() -> Option<InferredTypeData<'db>> + 'a;

/// Returns whether `function` conclusively accepts `args` under TypeScript's
/// subtype relation ([`is_subtype_of`]), the relation that TypeScript's
/// overload resolution tries first, under which `any` is a subtype only of
/// `any` and `unknown`, and `{}` is not a subtype of `{ a?: number }`.
///
/// Each argument is checked against the parameter at its position. An
/// optional parameter also accepts `undefined`, and a rest parameter of an
/// array type takes the remaining arguments. An argument that may be absent
/// or a spread argument, and a tuple rest parameter, give `false`, and so
/// does a relation that Biome can't decide.
///
/// An acceptance is conclusive only when the matching that selects an
/// overload for a call's return type, which rejects only what it can prove
/// incompatible and accepts the rest, agrees, so a signature that this
/// accepts is one that return-type inference may select.
///
/// In this example, the second signature accepts `1`:
///
/// ```ts
/// declare function fail(code: string): never;
/// declare function fail(code: number): void;
/// fail(1);
/// ```
pub(in crate::db) fn signature_accepts_arguments<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    positional_subtypes(db, function, args) == Some(true)
        && signature_may_accept_arguments(db, function, args)
}

/// Returns whether `function` rejects `args` whatever narrowing gives them:
/// the call passes too few or too many arguments, or an argument shares no
/// value with the parameter that it fills, as `1 | 2` shares none with
/// `string`.
///
/// TypeScript checks the arguments with their types at the call, which a
/// test that Biome doesn't decompose can make narrower than Biome's. A
/// signature that rejects Biome's types can accept TypeScript's, as
/// `(v: string)` accepts `x: string | number` once such a test narrows it to
/// `string`, and `(v: "a")` accepts `x: string` narrowed to `"a"`. Only a
/// rejection that holds for every narrowing shows that TypeScript rejects
/// the signature too, under either relation that it tries.
///
/// Narrowing only removes values, so an argument shares no value with its
/// parameter when every member of the argument's type excludes every member
/// of the parameter's type: two primitive types of different kinds, two
/// literal types of one kind with different values, or `null` or
/// `undefined` and an object type. A primitive type and an object type can
/// share values through an intersection, as `string & { brand: 1 }`, so
/// they count as sharing them, and so does any type that Biome can't
/// classify, such as `any`, `unknown`, or a type parameter.
pub(in crate::db) fn rejects_every_narrowing<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    let Some(parameters) = PositionalParameters::new(db, function) else {
        return false;
    };
    if !parameters.fits(args.len()) {
        return true;
    }
    args.iter().enumerate().any(|(index, argument)| {
        let argument = match argument {
            ResolvedCallArgument::Argument(ty) | ResolvedCallArgument::ConstArgument { ty, .. } => {
                *ty
            }
            ResolvedCallArgument::WidenedArgument { literal_ty, .. } => *literal_ty,
            ResolvedCallArgument::Optional(_) | ResolvedCallArgument::Spread(_) => return false,
        };
        parameters
            .parameter_type(db, index)
            .is_some_and(|parameter| shares_no_value(db, argument, parameter))
    })
}

/// The parameters of a signature that positional arguments fill: the fixed
/// ones, which may be optional, and the type of a final rest parameter.
struct PositionalParameters<'db> {
    fixed: Vec<&'db InferredFunctionParameter<'db>>,
    rest: Option<InferredTypeData<'db>>,
}

impl<'db> PositionalParameters<'db> {
    /// Returns the parameters of `function` other than a `this` parameter,
    /// or `None` when a rest parameter comes before the last one.
    fn new(db: &'db dyn ModuleDb, function: InferredFunction<'db>) -> Option<Self> {
        let mut fixed: Vec<_> = function
            .parameters(db)
            .iter()
            .filter(|parameter| !parameter.is_this())
            .collect();
        let rest = match fixed.last() {
            Some(last) if last.is_rest() => fixed.pop().map(|rest| rest.ty()),
            _ => None,
        };
        (!fixed.iter().any(|parameter| parameter.is_rest())).then_some(Self { fixed, rest })
    }

    /// Returns whether a call with `count` arguments passes neither too few
    /// nor too many.
    fn fits(&self, count: usize) -> bool {
        let required = self
            .fixed
            .iter()
            .rposition(|parameter| !parameter.is_optional())
            .map_or(0, |index| index + 1);
        count >= required && (self.rest.is_some() || count <= self.fixed.len())
    }

    /// Returns the type that the argument at `index` must have: that of its
    /// parameter, with `undefined` for an optional one, or the element type
    /// of an array rest parameter. Returns `None` for a rest parameter of
    /// another type, and past the parameters without one.
    fn parameter_type(&self, db: &'db dyn ModuleDb, index: usize) -> Option<InferredTypeData<'db>> {
        match self.fixed.get(index) {
            Some(parameter) if parameter.is_optional() => Some(InferredTypeData::union_from_types(
                db,
                Vec::from([parameter.ty(), InferredTypeData::Undefined]),
            )),
            Some(parameter) => Some(parameter.ty()),
            None => array_element_type(db, self.rest?),
        }
    }
}

/// Returns whether each of `args` is a subtype of the parameter of
/// `function` at its position, and the call passes neither too few nor too
/// many arguments, or `None` when Biome can't tell. See
/// [`signature_accepts_arguments`].
fn positional_subtypes<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> Option<bool> {
    let parameters = PositionalParameters::new(db, function)?;
    if !parameters.fits(args.len()) {
        return Some(false);
    }
    let mut result = Some(true);
    for (index, argument) in args.iter().enumerate() {
        let argument = match argument {
            ResolvedCallArgument::Argument(ty) | ResolvedCallArgument::ConstArgument { ty, .. } => {
                *ty
            }
            ResolvedCallArgument::WidenedArgument { literal_ty, .. } => *literal_ty,
            ResolvedCallArgument::Optional(_) | ResolvedCallArgument::Spread(_) => return None,
        };
        let parameter = parameters.parameter_type(db, index)?;
        let resolve = |ty| resolve_local_type_on_demand(db, ty).expand_structural_global(db);
        match is_subtype_of(db, argument, parameter, resolve) {
            Some(true) => {}
            Some(false) => return Some(false),
            None => result = None,
        }
    }
    result
}

/// What a member of a union can be after any narrowing, as far as
/// [`shares_no_value`] tells members apart.
#[derive(Clone, Copy, Eq, PartialEq)]
enum ValueKind<'db> {
    /// `bigint`, `boolean`, `number`, or `string`, with the literal type
    /// when the member is one, as `"a"`.
    Primitive(PrimitiveLiteralKind, Option<InferredTypeData<'db>>),
    Symbol,
    Null,
    Undefined,
    /// A type whose values are objects, which are never `null` or
    /// `undefined`.
    Object,
}

impl<'db> ValueKind<'db> {
    /// Returns whether no value of a member of this kind can be a value of a
    /// member of `other`'s kind, whatever narrowing gives either.
    fn excludes(self, other: Self, db: &'db dyn ModuleDb) -> bool {
        match (self, other) {
            (Self::Primitive(kind, literal), Self::Primitive(other_kind, other_literal)) => {
                kind != other_kind
                    || matches!(
                        (literal, other_literal),
                        (Some(literal), Some(other)) if literals_differ(db, literal, other)
                    )
            }
            (Self::Object, Self::Null | Self::Undefined)
            | (Self::Null | Self::Undefined, Self::Object) => true,
            (Self::Object, _) | (_, Self::Object) => false,
            (kind, other) => kind != other,
        }
    }
}

/// Returns whether the literal types `a` and `b` have different values.
///
/// Biome keeps the spelling of a literal, so this compares numbers by value
/// and takes a string with an escape sequence, as `"a"`, or a bigint,
/// which has several spellings, as equal to any other literal of its kind.
fn literals_differ<'db>(
    db: &'db dyn ModuleDb,
    a: InferredTypeData<'db>,
    b: InferredTypeData<'db>,
) -> bool {
    let (InferredTypeData::Literal(a), InferredTypeData::Literal(b)) = (a, b) else {
        return false;
    };
    match (a.literal(db), b.literal(db)) {
        (InferredLiteral::Boolean(a), InferredLiteral::Boolean(b)) => a.as_bool() != b.as_bool(),
        (InferredLiteral::Number(a), InferredLiteral::Number(b)) => {
            matches!((a.to_f64(), b.to_f64()), (Some(a), Some(b)) if a != b)
        }
        (InferredLiteral::String(a), InferredLiteral::String(b)) => {
            !a.as_str().contains('\\') && !b.as_str().contains('\\') && a.as_str() != b.as_str()
        }
        _ => false,
    }
}

/// Returns whether no value of `argument` is a value of `parameter`, after
/// any narrowing of either (see [`rejects_every_narrowing`]).
fn shares_no_value<'db>(
    db: &'db dyn ModuleDb,
    argument: InferredTypeData<'db>,
    parameter: InferredTypeData<'db>,
) -> bool {
    let (Some(arguments), Some(parameters)) =
        (value_kinds(db, argument), value_kinds(db, parameter))
    else {
        return false;
    };
    arguments.iter().all(|argument| {
        parameters
            .iter()
            .all(|parameter| argument.excludes(*parameter, db))
    })
}

/// Returns the kinds of the members of `ty`, a union or a single type, or
/// `None` when a member has no [`ValueKind`] or the members exceed
/// [`MAX_ARGUMENT_TYPE_STEPS`].
fn value_kinds<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<Vec<ValueKind<'db>>> {
    let mut pending = Vec::from([ty]);
    let mut kinds = Vec::new();
    for _ in 0..MAX_ARGUMENT_TYPE_STEPS {
        let Some(ty) = pending.pop() else {
            return Some(kinds);
        };
        let ty = resolve_local_type_on_demand(db, ty).expand_structural_global(db);
        let kind = match ty {
            InferredTypeData::Union(union) => {
                pending.extend(union.types(db).iter().copied());
                continue;
            }
            InferredTypeData::BigInt => ValueKind::Primitive(PrimitiveLiteralKind::BigInt, None),
            InferredTypeData::Boolean => ValueKind::Primitive(PrimitiveLiteralKind::Boolean, None),
            InferredTypeData::Number => ValueKind::Primitive(PrimitiveLiteralKind::Number, None),
            InferredTypeData::String => ValueKind::Primitive(PrimitiveLiteralKind::String, None),
            InferredTypeData::Symbol => ValueKind::Symbol,
            InferredTypeData::Null => ValueKind::Null,
            InferredTypeData::Undefined => ValueKind::Undefined,
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                InferredLiteral::BigInt(_) => {
                    ValueKind::Primitive(PrimitiveLiteralKind::BigInt, Some(ty))
                }
                InferredLiteral::Boolean(_) => {
                    ValueKind::Primitive(PrimitiveLiteralKind::Boolean, Some(ty))
                }
                InferredLiteral::Number(_) => {
                    ValueKind::Primitive(PrimitiveLiteralKind::Number, Some(ty))
                }
                InferredLiteral::String(_) => {
                    ValueKind::Primitive(PrimitiveLiteralKind::String, Some(ty))
                }
                // A template literal type stands for the strings that match
                // its pattern, which Biome doesn't compare with literals.
                InferredLiteral::Template(_) => {
                    ValueKind::Primitive(PrimitiveLiteralKind::String, None)
                }
                InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => ValueKind::Object,
            },
            InferredTypeData::Object(_)
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::Function(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Class(_)
            | InferredTypeData::Interface(_) => ValueKind::Object,
            InferredTypeData::InstanceOf(instance)
                if matches!(
                    resolve_local_type_on_demand(db, instance.ty(db)).expand_canonical_global(db),
                    InferredTypeData::Class(_) | InferredTypeData::Interface(_)
                ) =>
            {
                ValueKind::Object
            }
            _ => return None,
        };
        kinds.push(kind);
    }
    None
}

/// Returns the element type of `ty` when it is an array, as `number` for
/// `number[]`, and `None` otherwise.
fn array_element_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    let InferredTypeData::InstanceOf(instance) = resolve_local_type_on_demand(db, ty) else {
        return None;
    };
    if !resolve_local_type_on_demand(db, instance.ty(db))
        .expand_canonical_global(db)
        .is_array_class(db)
    {
        return None;
    }
    instance.type_parameters(db).first().copied()
}

/// Returns the call signatures of a value of type `callee`, in the order
/// that TypeScript tries them for a call: the signature of a function, or
/// the call signatures of an interface, an object type, or an object
/// literal, which collection and member lookup put in that order for
/// overloads, including those that declarations merge. An instance of a
/// generic type gives its signatures with its type arguments substituted.
///
/// Returns `None` for a type without call signatures, for a class, whose
/// call signatures belong to its constructor value, for a union, and when
/// the walk exceeds its work limit.
pub(in crate::db) fn call_signatures<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
) -> Option<Vec<InferredFunction<'db>>> {
    let mut ty = callee;
    for _ in 0..MAX_ARGUMENT_TYPE_STEPS {
        ty = resolve_local_type_on_demand(db, ty).expand_canonical_global(db);
        let members = match ty {
            InferredTypeData::Function(function) => return Some(Vec::from([function])),
            InferredTypeData::InstanceOf(instance) => {
                let target = resolve_local_type_on_demand(db, instance.ty(db));
                if matches!(
                    target.expand_canonical_global(db),
                    InferredTypeData::Class(_)
                ) {
                    return None;
                }
                let substitutions =
                    substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
                ty = apply_substitutions_to_root_body(db, target, &substitutions);
                continue;
            }
            InferredTypeData::TypeofType(typeof_type) => {
                ty = typeof_type.ty(db);
                continue;
            }
            InferredTypeData::TypeofValue(typeof_value) => {
                ty = typeof_value.ty(db);
                continue;
            }
            InferredTypeData::Interface(interface) => interface.members(db),
            InferredTypeData::Object(object) => object.members(db),
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                InferredLiteral::Object(members) => members,
                _ => return None,
            },
            _ => return None,
        };
        let mut signatures = Vec::new();
        for member in members
            .iter()
            .filter(|member| member.kind.is_call_signature())
        {
            signatures.push(
                member
                    .ty
                    .expand_canonical_global(db)
                    .callable_function(db)?,
            );
        }
        return (!signatures.is_empty()).then_some(signatures);
    }
    None
}

/// Returns the type parameter of `function` that a callback argument binds
/// through the type predicate of the parameter that it fills, and the type
/// that it binds, as TypeScript infers `S` as `string` for
/// `every<S extends T>(predicate: (value: T) => value is S)` with the
/// argument `isString`, a `(v: unknown) => v is string`.
///
/// `parameter_ty` is the parameter's type and `argument_ty` the argument's.
/// Each must have a single signature without type parameters that returns a
/// type predicate about the same parameter, or about `this`, and the
/// parameter's predicate must target a type parameter of `function` itself,
/// as `S`. Returns `None` otherwise, as for a callback that returns a
/// `boolean`.
pub(in crate::db) fn callback_predicate_binding<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    parameter_ty: InferredTypeData<'db>,
    argument_ty: InferredTypeData<'db>,
) -> Option<(InferredGenericTypeParameter<'db>, InferredTypeData<'db>)> {
    let parameter = resolve_callable_function(db, CallableFunctionInput::new(db, parameter_ty))?;
    let ReturnType::Predicate(parameter_predicate) = parameter.return_type(db) else {
        return None;
    };
    let argument = resolve_callable_function(db, CallableFunctionInput::new(db, argument_ty))?;
    let ReturnType::Predicate(argument_predicate) = argument.return_type(db) else {
        return None;
    };
    if !parameter.type_parameters(db).is_empty() || !argument.type_parameters(db).is_empty() {
        return None;
    }
    if predicate_position(db, parameter, &parameter_predicate.parameter_name)?
        != predicate_position(db, argument, &argument_predicate.parameter_name)?
    {
        return None;
    }
    let generic = parameter_predicate.ty.as_generic_reference(db)?;
    function
        .type_parameters(db)
        .iter()
        .any(|ty| ty.as_generic_reference(db) == Some(generic))
        .then_some((generic, argument_predicate.ty))
}

/// Returns which parameter of `function` a type predicate or an assertion
/// that names `name` tests: `Some(None)` for `this`, and the position of the
/// parameter, not counting a `this` parameter, otherwise. Returns `None` when
/// `name` names no parameter or more than one.
fn predicate_position<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    name: &Text,
) -> Option<Option<usize>> {
    let name = name.text();
    if name == "this" {
        return Some(None);
    }
    let mut matching = function
        .parameters(db)
        .iter()
        .filter(|parameter| !parameter.is_this())
        .enumerate()
        .filter(|(_, parameter)| {
            matches!(parameter, InferredFunctionParameter::Named(named) if named.name.text() == name)
        });
    let (index, _) = matching.next()?;
    matching.next().is_none().then_some(Some(index))
}

/// Returns whether `replacement` satisfies the constraint of `generic`, a
/// type parameter of a called signature, as TypeScript requires before it
/// infers `replacement` for it. The constraint is read with `bound`, the
/// replacements already inferred for the signature's other type
/// parameters, and with the type arguments of the object that a method is
/// read from, which `receiver` gives (see [`receiver_substitutions`]), so
/// `S extends T` in a method of `string[]` stands for `S extends string`.
///
/// Returns `Some(true)` for a type parameter without a constraint, and
/// `None` when the constraint still names a type parameter that nothing
/// replaces, or when Biome can't compare the two types.
pub(in crate::db) fn satisfies_constraint<'db>(
    db: &'db dyn ModuleDb,
    generic: InferredGenericTypeParameter<'db>,
    replacement: InferredTypeData<'db>,
    receiver: &mut CallReceiver<'db, '_>,
    bound: &[InferredTypeSubstitution<'db>],
) -> Option<bool> {
    let Some(constraint) = generic.constraint(db) else {
        return Some(true);
    };
    let mut substitutions =
        receiver().map_or_else(Vec::new, |receiver| receiver_substitutions(db, receiver));
    substitutions.extend_from_slice(bound);
    let constraint = apply_substitutions(db, constraint, &substitutions);
    if constraint.may_refer_to_free_generic(db) {
        return None;
    }
    is_assignable_to(db, replacement, constraint, |ty| {
        resolve_local_type_on_demand(db, ty).expand_structural_global(db)
    })
}

/// Returns the substitutions that replace `generic` with `replacement`,
/// both where a type names an instance of the type parameter and where it
/// names the type parameter itself, the two forms that references to a type
/// parameter take. The instance comes first, so that it is replaced as a
/// whole rather than around a replaced type parameter.
pub(in crate::db) fn generic_substitutions<'db>(
    db: &'db dyn ModuleDb,
    generic: InferredGenericTypeParameter<'db>,
    replacement: InferredTypeData<'db>,
) -> [InferredTypeSubstitution<'db>; 2] {
    let generic = InferredTypeData::Generic(generic);
    [
        InferredTypeSubstitution {
            generic: InferredTypeData::instance_of(db, generic, Box::default()),
            replacement,
        },
        InferredTypeSubstitution {
            generic,
            replacement,
        },
    ]
}

/// Finds an expected argument type without requiring a valid complete call.
///
/// A single signature does not need overload selection, so a missing later
/// argument must not suppress the expected type of an earlier argument:
///
/// ```ts
/// declare function consume(callback: () => void, marker: number): void;
/// consume(async () => {});
/// ```
///
/// Fixed-position arguments map directly to parameters. A preceding optional
/// or spread argument can shift that mapping, so those cases still use the
/// sequence matcher to find every viable parameter position.
fn infer_single_signature_parameter_type<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let parameters = ResolvedParameters::from_function(db, function);
    infer_single_signature_resolved_parameter_type(db, parameters.as_slice(), args, argument_index)
}

fn infer_single_signature_resolved_parameter_type<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[ResolvedParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    args.get(argument_index)?;
    if args[..argument_index].iter().all(|argument| {
        matches!(
            argument,
            ResolvedCallArgument::Argument(_)
                | ResolvedCallArgument::ConstArgument { .. }
                | ResolvedCallArgument::WidenedArgument { .. }
        )
    }) && let Some(parameter) = parameters.get(argument_index)
        && parameters
            .iter()
            .take(argument_index + 1)
            .all(|parameter| matches!(parameter, ResolvedParameter::Required(_)))
    {
        return Some(parameter.ty());
    }

    infer_parameter_type_for_resolved_argument(db, parameters, args, argument_index)
}

fn infer_parameter_type_for_argument<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let parameters = ResolvedParameters::from_function(db, function);
    infer_parameter_type_for_resolved_argument(db, parameters.as_slice(), args, argument_index)
}

/// Finds the parameter that receives one call argument and returns its type.
///
/// Arguments and parameters usually match by position. Optional parameters and
/// spread values can shift that position, so every possible match is checked.
/// If the argument can reach several parameters, their types are combined. The
/// argument being examined is ignored while the remaining arguments are
/// checked.
///
/// In the following example, the second and third arguments both map to the
/// `values` rest parameter. Their expected type is `number`.
///
/// ```ts
/// declare function record(label: string, ...values: number[]): void;
/// record("total", 1, 2);
/// ```
fn infer_parameter_type_for_resolved_argument<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[ResolvedParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    if parameters
        .iter()
        .all(|parameter| matches!(parameter, ResolvedParameter::Required(_)))
        && args.iter().all(|argument| {
            matches!(
                argument,
                ResolvedCallArgument::Argument(_)
                    | ResolvedCallArgument::ConstArgument { .. }
                    | ResolvedCallArgument::WidenedArgument { .. }
            )
        })
    {
        let parameter = parameters.get(argument_index)?;
        args.get(argument_index)?;
        return (parameters.len() == args.len()
            && parameters
                .iter()
                .zip(args)
                .enumerate()
                .all(|(index, (parameter, argument))| {
                    index == argument_index || argument.matches_parameter(db, parameter.ty())
                }))
        .then(|| parameter.ty());
    }

    collected_type_result(
        db,
        matched_parameter_types(db, parameters, args, Some(argument_index))?,
    )
}

/// Matches optional and spread argument sequences against a parameter list.
///
/// With `ignored_argument_index`, the result contains every parameter type
/// that the source argument can occupy in a complete call. Without it, an
/// empty result means the full argument sequence can be accepted. Returns
/// `None` if the sequence cannot be matched within the work budget.
fn matched_parameter_types<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[ResolvedParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    ignored_argument_index: Option<usize>,
) -> Option<Vec<InferredTypeData<'db>>> {
    let mut pending = Vec::from([ArgumentSequenceState::default()]);
    let mut seen = FxHashSet::default();
    let mut matched_parameter_types = Vec::new();
    let mut suffix_accepts_zero = vec![true; parameters.len() + 1];
    for index in (0..parameters.len()).rev() {
        suffix_accepts_zero[index] =
            parameters[index].accepts_zero() && suffix_accepts_zero[index + 1];
    }

    for _ in 0..MAX_ARGUMENT_SEQUENCE_STEPS {
        let Some(state) = pending.pop() else {
            return (!matched_parameter_types.is_empty()).then_some(matched_parameter_types);
        };
        if !seen.insert(state) {
            continue;
        }

        let Some(argument) = args.get(state.argument_index).copied() else {
            if suffix_accepts_zero
                .get(state.parameter_index)
                .copied()
                .unwrap_or(true)
            {
                let Some(ignored_argument_index) = ignored_argument_index else {
                    return Some(Vec::new());
                };
                if state.argument_index > ignored_argument_index
                    && let Some(parameter_index) = state.matched_parameter_index
                    && let Some(parameter) = parameters.get(parameter_index)
                    && let ty = parameter.ty()
                    && !matched_parameter_types.contains(&ty)
                {
                    matched_parameter_types.push(ty);
                }
            }
            continue;
        };

        let Some(parameter) = parameters.get(state.parameter_index).copied() else {
            if argument.accepts_zero() {
                pending.push(state.next_argument());
            }
            continue;
        };

        if parameter.accepts_zero() {
            pending.push(state.next_parameter());
        }
        if argument.accepts_zero() {
            pending.push(state.next_argument());
        }

        if ignored_argument_index == Some(state.argument_index) {
            push_consumed_sequence_states(
                state.new_matched_parameter(),
                parameter.is_spread(),
                argument.is_spread(),
                &mut pending,
            );
            continue;
        }

        let matches = if argument.is_spread() {
            ArgumentTypeCompatibility::new(parameter.ty(), spread_element_type(db, argument.ty()))
                .is_satisfied(db)
        } else {
            argument.matches_parameter(db, parameter.ty())
        };
        if matches {
            push_consumed_sequence_states(
                state,
                parameter.is_spread(),
                argument.is_spread(),
                &mut pending,
            );
        }
    }

    None
}

fn push_consumed_sequence_states(
    state: ArgumentSequenceState,
    parameter_is_spread: bool,
    argument_is_spread: bool,
    pending: &mut Vec<ArgumentSequenceState>,
) {
    match (parameter_is_spread, argument_is_spread) {
        (false, false) => pending.push(state.next_parameter().next_argument()),
        (true, false) => pending.push(state.next_argument()),
        (false, true) => pending.push(state.next_parameter()),
        (true, true) => {
            pending.push(state.next_argument());
            pending.push(state.next_parameter());
        }
    }
}

fn spread_element_type<'db>(
    db: &'db dyn ModuleDb,
    arg_ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    match arg_ty {
        InferredTypeData::InstanceOf(instance)
            if instance.ty(db).is_array_class(db) && !instance.type_parameters(db).is_empty() =>
        {
            instance.type_parameters(db)[0]
        }
        InferredTypeData::Tuple(tuple) => collected_type_result(
            db,
            tuple
                .elements(db)
                .iter()
                .map(|element| element.ty)
                .collect(),
        )
        .unwrap_or(InferredTypeData::Unknown),
        InferredTypeData::AnyKeyword
        | InferredTypeData::Unknown
        | InferredTypeData::UnknownKeyword => InferredTypeData::Unknown,
        ty => ty,
    }
}

fn merged_reference_targets<'db>(
    db: &'db dyn ModuleDb,
    reference: InferredMergedReference<'db>,
) -> Vec<InferredTypeData<'db>> {
    [
        reference.namespace_ty(db),
        reference.value_ty(db),
        reference.ty(db),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Computes the result of [`super::resolve_callable_function`].
pub(super) fn resolve_callable_function_impl<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<InferredFunction<'db>> {
    if let InferredTypeData::InstanceOf(instance) = resolve_local_type_on_demand(db, ty) {
        let target = resolve_local_type_on_demand(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        let substituted = apply_substitutions_to_root_body(db, target, &substitutions);
        return resolve_callable_type(db, substituted)?.callable_function(db);
    }

    resolve_callable_type(db, ty)?.callable_function(db)
}

/// Returns whether the function declares that its result is discarded.
///
/// A `void` return position accepts a function returning anything, so an
/// `async` callback satisfies a parameter of type `() => void`.
fn returns_void<'db>(db: &'db dyn ModuleDb, function: InferredFunction<'db>) -> bool {
    match function.return_type(db) {
        ReturnType::Type(ty) => matches!(
            resolve_local_type_on_demand(db, *ty),
            InferredTypeData::VoidKeyword
        ),
        ReturnType::Predicate(_) | ReturnType::Asserts(_) => false,
    }
}

/// A directional relation between an expected parameter type and an actual argument type.
///
/// Union and intersection decomposition depend on which side contains the
/// compound type, so callers must preserve this ordering.
#[derive(Clone, Copy)]
struct ArgumentTypeCompatibility<'db> {
    const_inference: bool,
    parameter_ty: InferredTypeData<'db>,
    argument_ty: InferredTypeData<'db>,
}

impl<'db> ArgumentTypeCompatibility<'db> {
    fn new(parameter_ty: InferredTypeData<'db>, argument_ty: InferredTypeData<'db>) -> Self {
        Self {
            const_inference: false,
            parameter_ty,
            argument_ty,
        }
    }

    /// Returns whether `argument_ty` remains viable for `parameter_ty` during
    /// overload selection.
    ///
    /// Unsupported relations and work-budget exhaustion remain viable.
    /// Callable types must also agree on whether they return a Promise, unless
    /// the parameter discards its return value by declaring `void`.
    fn is_satisfied(self, db: &'db dyn ModuleDb) -> bool {
        if !self.may_match(db) {
            return false;
        }

        match (
            resolve_callable_function(db, CallableFunctionInput::new(db, self.parameter_ty)),
            resolve_callable_function(db, CallableFunctionInput::new(db, self.argument_ty)),
        ) {
            (Some(parameter_function), Some(argument_function)) => {
                returns_void(db, parameter_function)
                    || parameter_function.returns_promise(db)
                        == argument_function.returns_promise(db)
            }
            _ => true,
        }
    }

    fn with_types(
        self,
        parameter_ty: InferredTypeData<'db>,
        argument_ty: InferredTypeData<'db>,
    ) -> Self {
        Self {
            const_inference: self.const_inference,
            parameter_ty,
            argument_ty,
        }
    }

    /// Returns whether an argument may satisfy a parameter during overload search.
    ///
    /// The check rejects only type relationships that this inference layer can
    /// prove incompatible. Unknown and unsupported shapes remain viable so a
    /// partial type model does not discard an otherwise usable overload. Budget
    /// exhaustion also keeps the overload viable.
    fn may_match(self, db: &'db dyn ModuleDb) -> bool {
        let (mut requirements, root) = ArgumentMatchRequirements::new(self);
        let mut pending = Vec::from([Some(root)]);
        let mut remaining_steps = MAX_ARGUMENT_MATCH_STEPS;

        while let Some(requirement_id) = pending.pop() {
            let Some(requirement_id) = requirement_id else {
                return true;
            };
            if remaining_steps == 0 {
                return true;
            }
            remaining_steps -= 1;

            let requirement = requirements.get(requirement_id);
            let compatibility = requirement.compatibility;
            let continuation = requirement.next;

            if compatibility.parameter_ty == compatibility.argument_ty {
                pending.push(continuation);
                continue;
            }

            // Resolving aliases here keeps them off the work budget. Spending
            // one iteration per alias would let a long chain exhaust the loop,
            // and exhaustion leaves every overload viable.
            let compatibility = compatibility.with_types(
                resolve_local_type_on_demand(db, compatibility.parameter_ty),
                resolve_local_type_on_demand(db, compatibility.argument_ty),
            );
            if compatibility.parameter_ty == compatibility.argument_ty {
                pending.push(continuation);
                continue;
            }

            if let (InferredTypeData::Literal(parameter), InferredTypeData::Literal(argument)) =
                (compatibility.parameter_ty, compatibility.argument_ty)
                && !matches!(parameter.literal(db), InferredLiteral::Object(_))
                && !matches!(argument.literal(db), InferredLiteral::Object(_))
            {
                continue;
            }

            let compatibility = compatibility.with_types(
                literal_base_type(db, compatibility.parameter_ty)
                    .unwrap_or(compatibility.parameter_ty),
                literal_base_type(db, compatibility.argument_ty)
                    .unwrap_or(compatibility.argument_ty),
            );

            if compatibility.parameter_ty == compatibility.argument_ty {
                pending.push(continuation);
                continue;
            }

            compatibility
                .action(db)
                .schedule(continuation, &mut requirements, &mut pending);
        }

        false
    }

    /// Decomposes a nontrivial relation into conjunctive or alternative requirements.
    fn action(self, db: &'db dyn ModuleDb) -> ArgumentMatchAction<'db> {
        match (self.parameter_ty, self.argument_ty) {
            (InferredTypeData::Generic(generic), arg_ty) => {
                generic
                    .constraint(db)
                    .map_or(ArgumentMatchAction::Match, |constraint| {
                        ArgumentMatchAction::All(Vec::from([self.with_types(constraint, arg_ty)]))
                    })
            }
            (
                InferredTypeData::AnyKeyword
                | InferredTypeData::Unknown
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::ThisKeyword,
                _,
            )
            | (
                _,
                InferredTypeData::AnyKeyword
                | InferredTypeData::Unknown
                | InferredTypeData::UnknownKeyword
                | InferredTypeData::Generic(_)
                | InferredTypeData::ThisKeyword
                | InferredTypeData::NeverKeyword,
            ) => ArgumentMatchAction::Match,
            (InferredTypeData::NeverKeyword, _) => ArgumentMatchAction::Mismatch,
            (InferredTypeData::Local(parameter), _) => {
                match self.argument_extends_local_parameter(db, parameter) {
                    Some(true) => ArgumentMatchAction::Match,
                    Some(false) => ArgumentMatchAction::Mismatch,
                    None => ArgumentMatchAction::Match,
                }
            }
            (_, InferredTypeData::Local(argument)) => {
                match self.local_argument_extends_parameter(db, argument) {
                    Some(true) => ArgumentMatchAction::Match,
                    Some(false) => ArgumentMatchAction::Mismatch,
                    None => ArgumentMatchAction::Match,
                }
            }
            (InferredTypeData::Class(_), InferredTypeData::Class(_)) => {
                if self.argument_extends_parameter(db) {
                    ArgumentMatchAction::Match
                } else {
                    ArgumentMatchAction::Mismatch
                }
            }
            (InferredTypeData::Union(union), arg_ty) => ArgumentMatchAction::Any(
                union
                    .types(db)
                    .iter()
                    .map(|parameter_ty| self.with_types(*parameter_ty, arg_ty))
                    .collect(),
            ),
            (parameter_ty, InferredTypeData::Union(union)) => ArgumentMatchAction::All(
                union
                    .types(db)
                    .iter()
                    .map(|arg_ty| self.with_types(parameter_ty, *arg_ty))
                    .collect(),
            ),
            (InferredTypeData::Intersection(intersection), arg_ty) => ArgumentMatchAction::All(
                intersection
                    .types(db)
                    .iter()
                    .map(|parameter_ty| self.with_types(*parameter_ty, arg_ty))
                    .collect(),
            ),
            (parameter_ty, InferredTypeData::Intersection(intersection)) => {
                ArgumentMatchAction::Any(
                    intersection
                        .types(db)
                        .iter()
                        .map(|arg_ty| self.with_types(parameter_ty, *arg_ty))
                        .collect(),
                )
            }
            (InferredTypeData::TypeOperator(parameter), arg_ty)
                if self.const_inference
                    && parameter.operator(db) == biome_js_type_info::TypeOperator::Readonly =>
            {
                let arg_ty = match arg_ty {
                    InferredTypeData::TypeOperator(argument)
                        if argument.operator(db) == biome_js_type_info::TypeOperator::Readonly =>
                    {
                        argument.ty(db)
                    }
                    ty => ty,
                };
                ArgumentMatchAction::All(Vec::from([self.with_types(parameter.ty(db), arg_ty)]))
            }
            (parameter_ty, InferredTypeData::TypeOperator(argument))
                if self.const_inference
                    && argument.operator(db) == biome_js_type_info::TypeOperator::Readonly =>
            {
                // Const candidates must not satisfy mutable array requirements,
                // including requirements reached through unions or object members.
                let mutable_array = matches!(parameter_ty, InferredTypeData::Tuple(_))
                    || parameter_ty.is_array_class(db)
                    || matches!(parameter_ty, InferredTypeData::InstanceOf(instance) if instance.ty(db).is_array_class(db));
                if mutable_array {
                    ArgumentMatchAction::Mismatch
                } else {
                    ArgumentMatchAction::All(Vec::from([
                        self.with_types(parameter_ty, argument.ty(db))
                    ]))
                }
            }
            (InferredTypeData::InstanceOf(parameter), InferredTypeData::InstanceOf(argument)) => {
                let mut pairs = Vec::from([self.with_types(parameter.ty(db), argument.ty(db))]);
                pairs.extend(
                    parameter
                        .type_parameters(db)
                        .iter()
                        .zip(argument.type_parameters(db))
                        .map(|(parameter_ty, arg_ty)| self.with_types(*parameter_ty, *arg_ty)),
                );
                ArgumentMatchAction::All(pairs)
            }
            (InferredTypeData::InstanceOf(parameter), arg_ty) => {
                ArgumentMatchAction::All(Vec::from([self.with_types(parameter.ty(db), arg_ty)]))
            }
            (parameter_ty, InferredTypeData::InstanceOf(argument)) => ArgumentMatchAction::All(
                Vec::from([self.with_types(parameter_ty, argument.ty(db))]),
            ),
            (InferredTypeData::MergedReference(reference), arg_ty) => ArgumentMatchAction::Any(
                merged_reference_targets(db, reference)
                    .into_iter()
                    .map(|parameter_ty| self.with_types(parameter_ty, arg_ty))
                    .collect(),
            ),
            (parameter_ty, InferredTypeData::MergedReference(reference)) => {
                ArgumentMatchAction::Any(
                    merged_reference_targets(db, reference)
                        .into_iter()
                        .map(|arg_ty| self.with_types(parameter_ty, arg_ty))
                        .collect(),
                )
            }
            (InferredTypeData::TypeofType(parameter), arg_ty) => {
                ArgumentMatchAction::All(Vec::from([self.with_types(parameter.ty(db), arg_ty)]))
            }
            (parameter_ty, InferredTypeData::TypeofType(argument)) => ArgumentMatchAction::All(
                Vec::from([self.with_types(parameter_ty, argument.ty(db))]),
            ),
            (InferredTypeData::TypeofValue(parameter), arg_ty) => {
                ArgumentMatchAction::All(Vec::from([self.with_types(parameter.ty(db), arg_ty)]))
            }
            (parameter_ty, InferredTypeData::TypeofValue(argument)) => ArgumentMatchAction::All(
                Vec::from([self.with_types(parameter_ty, argument.ty(db))]),
            ),
            (
                InferredTypeData::Interface(_) | InferredTypeData::Object(_),
                InferredTypeData::Interface(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Literal(_),
            ) => self.structural_object_action(db),
            (
                InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword,
                InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword,
            ) => ArgumentMatchAction::Mismatch,
            (
                InferredTypeData::Conditional
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::GlobalLocal(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::IndexedAccess(_)
                | InferredTypeData::MappedType(_)
                | InferredTypeData::Extends(_)
                | InferredTypeData::TypeofExpression(_),
                _,
            )
            | (
                _,
                InferredTypeData::Conditional
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::GlobalLocal(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::IndexedAccess(_)
                | InferredTypeData::MappedType(_)
                | InferredTypeData::Extends(_)
                | InferredTypeData::TypeofExpression(_),
            )
            | (
                InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_),
                InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_),
            ) => ArgumentMatchAction::Match,
            (
                InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_),
                InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword,
            )
            | (
                InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword,
                InferredTypeData::Class(_)
                | InferredTypeData::Constructor(_)
                | InferredTypeData::Function(_)
                | InferredTypeData::Interface(_)
                | InferredTypeData::Module(_)
                | InferredTypeData::Namespace(_)
                | InferredTypeData::Object(_)
                | InferredTypeData::Tuple(_),
            ) => ArgumentMatchAction::Mismatch,
        }
    }

    /// Turns an object relation into one requirement per member the parameter
    /// requires.
    ///
    /// A parameter whose required members cannot be listed matches, because
    /// nothing about the argument can be disproved. An empty list of
    /// requirements also matches: the parameter asks for nothing.
    ///
    /// A required member missing from the argument rejects the relation only
    /// when the argument lists every member it has. An object literal whose
    /// spread inference could not expand, or a type carrying an index
    /// signature, may hold the name without saying so, so its absence proves
    /// nothing and the relation stays viable.
    ///
    /// A member the parameter declares but that cannot be looked up is skipped
    /// instead of becoming a requirement no argument could meet.
    ///
    /// Here the first overload is rejected because `{}` lists every member it
    /// has and `initial` is not among them, so the call returns `boolean`:
    ///
    /// ```ts
    /// declare function query(options: { initial: string }): { isPending: false };
    /// declare function query(options: { initial?: string }): { isPending: boolean };
    ///
    /// const { isPending } = query({});
    /// ```
    fn structural_object_action(self, db: &'db dyn ModuleDb) -> ArgumentMatchAction<'db> {
        let Some(required_names) = required_structural_member_names(db, self.parameter_ty) else {
            return ArgumentMatchAction::Match;
        };
        let argument_lists_every_member = lists_every_member(db, self.argument_ty);
        let mut requirements = Vec::with_capacity(required_names.len());
        for name in required_names {
            let Some(parameter_member) =
                find_member_type_on_demand(db, self.parameter_ty, name.text())
            else {
                continue;
            };
            match find_member_type_on_demand(db, self.argument_ty, name.text()) {
                Some(argument_member) => {
                    requirements.push(self.with_types(parameter_member, argument_member));
                }
                None if argument_lists_every_member => return ArgumentMatchAction::Mismatch,
                None => {}
            }
        }
        ArgumentMatchAction::All(requirements)
    }

    fn argument_extends_local_parameter(
        self,
        db: &'db dyn ModuleDb,
        parameter: InferredLocalTypeHandle<'db>,
    ) -> Option<bool> {
        let argument_ty = self.argument_ty;
        let parameter_ty = InferredTypeData::Local(parameter);
        if argument_ty == parameter_ty {
            return Some(true);
        }
        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([argument_ty]);
        let mut resolved_known_type = false;

        for _ in 0..MAX_LOCAL_EXTENDS_STEPS {
            let Some(ty) = pending.pop() else {
                return resolved_known_type.then_some(false);
            };
            if ty == parameter_ty {
                return Some(true);
            }
            if !seen.insert(ty) {
                continue;
            }

            match ty {
                InferredTypeData::Class(class) => {
                    resolved_known_type = true;
                    if class.name(db).as_ref().is_some_and(|argument_name| {
                        raw_local_class_name_matches(db, parameter, argument_name.text())
                    }) {
                        return Some(true);
                    }
                    let ClassParents {
                        extends,
                        interface_bases,
                    } = class.parents(db);
                    pending.extend(extends.iter().chain(interface_bases.iter()).copied());
                }
                InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
                InferredTypeData::Local(local) => {
                    if local == parameter {
                        return Some(true);
                    }
                    resolved_known_type = true;
                    pending.extend(
                        raw_local_class_parents(db, local)
                            .into_iter()
                            .map(InferredTypeData::Local),
                    );
                }
                InferredTypeData::BigInt
                | InferredTypeData::Boolean
                | InferredTypeData::Null
                | InferredTypeData::Number
                | InferredTypeData::String
                | InferredTypeData::Symbol
                | InferredTypeData::Undefined
                | InferredTypeData::VoidKeyword => return Some(false),
                _ => {}
            }
        }

        None
    }

    fn local_argument_extends_parameter(
        self,
        db: &'db dyn ModuleDb,
        argument: InferredLocalTypeHandle<'db>,
    ) -> Option<bool> {
        let parameter_ty = self.parameter_ty;
        match parameter_ty {
            InferredTypeData::Class(parameter) => {
                let parameter_name = parameter.name(db).as_ref()?.text();
                Some(raw_local_extends_class_name(db, argument, parameter_name))
            }
            InferredTypeData::BigInt
            | InferredTypeData::Boolean
            | InferredTypeData::Null
            | InferredTypeData::Number
            | InferredTypeData::String
            | InferredTypeData::Symbol
            | InferredTypeData::Undefined
            | InferredTypeData::VoidKeyword => Some(false),
            _ => None,
        }
    }

    fn argument_extends_parameter(self, db: &'db dyn ModuleDb) -> bool {
        let argument_ty = self.argument_ty;
        let parameter_ty = self.parameter_ty;
        if argument_ty == parameter_ty {
            return true;
        }

        let mut seen = FxHashSet::default();
        let mut pending = Vec::from([argument_ty]);

        for _ in 0..MAX_LOCAL_EXTENDS_STEPS {
            let Some(ty) = pending.pop() else {
                return false;
            };
            if ty == parameter_ty {
                return true;
            }
            if !seen.insert(ty) {
                continue;
            }

            match ty {
                InferredTypeData::Class(class) => {
                    let ClassParents {
                        extends,
                        interface_bases,
                    } = class.parents(db);
                    pending.extend(extends.iter().chain(interface_bases.iter()).copied());
                }
                InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
                InferredTypeData::Local(local) => {
                    if let InferredTypeData::Class(parameter) = parameter_ty
                        && let Some(parameter_name) = parameter.name(db).as_ref()
                        && raw_local_extends_class_name(db, local, parameter_name.text())
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }
}

/// Lists the names a value must carry to be usable as `ty`, in declaration
/// order.
///
/// Optional and static members are left out because neither has to be present
/// on a value. Inherited names are included: an interface's `extends` clauses
/// and an object's prototype are traversed as well.
///
/// The result is `None` when `ty` is not a shape whose members can be listed,
/// or when the traversal exceeds its work limit. Both mean the requirements are
/// unknown, not that there are none.
fn required_structural_member_names<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<Vec<Text>> {
    let mut names = Vec::new();
    let mut collected_names = FxHashSet::default();
    let mut seen_types = FxHashSet::default();
    let mut pending = Vec::from([ty]);

    for _ in 0..MAX_STRUCTURAL_MEMBER_STEPS {
        let Some(ty) = pending.pop() else {
            return Some(names);
        };
        let ty = resolve_local_type_on_demand(db, ty);
        if !seen_types.insert(ty) {
            continue;
        }

        let members = match ty {
            InferredTypeData::Interface(interface) => {
                pending.extend(interface.extends(db).iter().copied());
                interface.members(db)
            }
            InferredTypeData::InstanceOf(instance) => {
                pending.push(instance.ty(db));
                continue;
            }
            InferredTypeData::Object(object) => {
                if let Some(prototype) = object.prototype(db) {
                    pending.push(prototype);
                }
                object.members(db)
            }
            _ => return None,
        };

        for member in members {
            if member.kind.is_optional() || member.kind.is_static() {
                continue;
            }
            if let Some(name) = member.kind.name()
                && collected_names.insert(name.clone())
            {
                names.push(name);
            }
        }
    }

    None
}

/// Returns whether the members of `ty` can be enumerated in full.
///
/// A missing name proves that a value of this type lacks the name only when
/// this returns `true`. Three things make the answer `false`: an object literal
/// that inference could only model in part, such as one built with a spread; a
/// member keyed by anything other than a fixed name, which can answer a lookup
/// for any name; and a shape this traversal does not support, including a type
/// alias that could not be resolved. Exhausting the work limit also answers
/// `false`, since the unvisited part may hold either.
fn lists_every_member<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> bool {
    let mut seen_types = FxHashSet::default();
    let mut pending = Vec::from([ty]);

    for _ in 0..MAX_STRUCTURAL_MEMBER_STEPS {
        let Some(ty) = pending.pop() else {
            return true;
        };
        let ty = resolve_local_type_on_demand(db, ty);
        if !seen_types.insert(ty) {
            continue;
        }

        let members = match ty {
            InferredTypeData::Interface(interface) => {
                pending.extend(interface.extends(db).iter().copied());
                interface.members(db)
            }
            InferredTypeData::InstanceOf(instance) => {
                pending.push(instance.ty(db));
                continue;
            }
            InferredTypeData::Object(object) => {
                if object.has_unknown_members(db) {
                    return false;
                }
                if let Some(prototype) = object.prototype(db) {
                    pending.push(prototype);
                }
                object.members(db)
            }
            InferredTypeData::Literal(literal) => match literal.literal(db) {
                InferredLiteral::Object(members) => members,
                _ => return false,
            },
            _ => return false,
        };

        // A call signature carries no name to look up, but every other nameless
        // member is keyed by a computed value or an index signature and can
        // stand in for any name.
        if members
            .iter()
            .any(|member| member.kind.name().is_none() && !member.kind.is_call_signature())
        {
            return false;
        }
    }

    false
}

fn raw_local_extends_class_name<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
    parameter_name: &str,
) -> bool {
    let mut seen = FxHashSet::default();
    let mut pending = Vec::from([local]);

    for _ in 0..MAX_LOCAL_EXTENDS_STEPS {
        let Some(local) = pending.pop() else {
            return false;
        };
        if !seen.insert(local) {
            continue;
        }

        if raw_local_class_name_matches(db, local, parameter_name) {
            return true;
        }
        pending.extend(raw_local_class_parents(db, local));
    }

    false
}

fn raw_local_class_name_matches<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
    expected_name: &str,
) -> bool {
    let module_key = local.module(db);
    let Some(current) = module_for_key(db, module_key) else {
        return false;
    };
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return false;
    };
    let Some(RawTypeData::Class(class)) = js_info.raw_types.get(local.type_id(db).index()) else {
        return false;
    };
    class
        .name
        .as_ref()
        .is_some_and(|name| name.text() == expected_name)
}

/// Returns local handles for the parent class and then the interface bases of
/// the raw class behind `local`. A parent that doesn't resolve to a type of
/// the same module is left out, and the result is empty when no raw class is
/// behind `local`.
fn raw_local_class_parents<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
) -> Vec<InferredLocalTypeHandle<'db>> {
    let module_key = local.module(db);
    let Some(current) = module_for_key(db, module_key) else {
        return Vec::new();
    };
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return Vec::new();
    };
    let Some(RawTypeData::Class(class)) = js_info.raw_types.get(local.type_id(db).index()) else {
        return Vec::new();
    };
    let Class {
        name: _,
        type_parameters: _,
        extends,
        implements: _,
        interface_bases,
        members: _,
        scope_id: _,
    } = class.as_ref();
    extends
        .iter()
        .chain(interface_bases.iter())
        .filter_map(|reference| local_handle_from_reference(db, module_key, reference))
        .collect()
}

fn local_handle_from_reference<'db>(
    db: &'db dyn ModuleDb,
    module_key: InferredModuleKey,
    reference: &TypeReference,
) -> Option<InferredLocalTypeHandle<'db>> {
    let TypeReference::Resolved(resolved) = reference else {
        return None;
    };
    if resolved.level() != TypeResolverLevel::Thin {
        return None;
    }

    Some(InferredLocalTypeHandle::new(
        db,
        module_key,
        InferredLocalTypeId::new(resolved.index()),
    ))
}

fn literal_base_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    let InferredTypeData::Literal(literal) = ty else {
        return None;
    };

    match literal.literal(db) {
        InferredLiteral::BigInt(_) => Some(InferredTypeData::BigInt),
        InferredLiteral::Boolean(_) => Some(InferredTypeData::Boolean),
        InferredLiteral::Number(_) => Some(InferredTypeData::Number),
        InferredLiteral::String(_) | InferredLiteral::Template(_) => Some(InferredTypeData::String),
        InferredLiteral::Object(_) | InferredLiteral::RegExp(_) => None,
    }
}

fn infer_function_return_type<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
    receiver: &mut CallReceiver<'db, '_>,
) -> Option<InferredTypeData<'db>> {
    match function.return_type(db) {
        ReturnType::Type(ty) => Some(
            infer_generic_return_type(db, function, *ty, args, receiver)
                .expand_structural_global(db),
        ),
        ReturnType::Predicate(_) | ReturnType::Asserts(_) => Some(InferredTypeData::Boolean),
    }
}

/// Substitutes generic references in a function's return type.
///
/// A direct generic parameter is inferred from its argument. A generic in a
/// callback return type is inferred from the callback's return type. A
/// generic that a callback parameter's type predicate targets is inferred
/// from the type predicate of the callback argument, when that type
/// satisfies the generic's constraint (see [`callback_predicate_binding`]
/// and [`satisfies_constraint`], which reads `receiver`). An unbound
/// parameter uses its declared default. Defaults are processed in
/// declaration order, so a default may refer to an earlier parameter.
/// Parameters with no inferred value or default remain generic. A failed type
/// transformation returns `Unknown`.
///
/// In this example, `strings` is `string[]` because `S` is inferred from the
/// type predicate of `isString`.
///
/// ```ts
/// declare function isString(value: unknown): value is string;
/// declare const values: (string | number)[];
/// const strings = values.filter(isString);
/// ```
///
/// In this example, `value` is `number` because `T` is inferred from the
/// callback's return type.
///
/// ```ts
/// declare function produce<T>(callback: () => T): T;
/// const value = produce(() => 1);
/// ```
///
/// In this example, both tuple elements are `string`. `T` uses its default,
/// then the default for `U` resolves through `T`.
///
/// ```ts
/// declare function defaults<T = string, U = T>(): [T, U];
/// const value = defaults();
/// ```
fn infer_generic_return_type<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    mut return_ty: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    receiver: &mut CallReceiver<'db, '_>,
) -> InferredTypeData<'db> {
    let mut substitutions: Vec<InferredTypeSubstitution<'db>> = Vec::new();
    for (index, parameter) in function.parameters(db).iter().enumerate() {
        let parameter_ty = parameter.ty();
        let const_parameter = matches!(parameter_ty, InferredTypeData::Generic(generic) if generic.is_const(db))
            || matches!(parameter_ty, InferredTypeData::InstanceOf(instance) if matches!(instance.ty(db), InferredTypeData::Generic(generic) if generic.is_const(db)));
        let arg = if parameter.is_rest() && const_parameter {
            let remaining = args.get(index..).unwrap_or_default();
            let tuple = |const_inference| {
                InferredTypeData::Tuple(InferredTuple::new(
                    db,
                    remaining
                        .iter()
                        .map(|arg| InferredTupleElementType {
                            name: None,
                            ty: if const_inference {
                                match arg {
                                    ResolvedCallArgument::ConstArgument { const_ty, .. } => {
                                        *const_ty
                                    }
                                    _ => arg.ty(),
                                }
                            } else {
                                arg.ty()
                            },
                            is_optional: matches!(arg, ResolvedCallArgument::Optional(_)),
                            is_rest: matches!(arg, ResolvedCallArgument::Spread(_)),
                        })
                        .collect::<Box<[_]>>(),
                    false,
                ))
            };
            ResolvedCallArgument::ConstArgument {
                ty: tuple(false),
                const_ty: InferredTypeData::TypeOperator(
                    biome_js_type_info::interned_types::InternedTypeOperatorType::new(
                        db,
                        tuple(true),
                        biome_js_type_info::TypeOperator::Readonly,
                    ),
                ),
            }
            .for_parameter(db, parameter_ty)
        } else if let Some(arg) = args.get(index) {
            arg.for_parameter(db, parameter_ty)
        } else {
            continue;
        };
        if parameter_ty.is_generic_reference(db) {
            let substitution = InferredTypeSubstitution {
                generic: parameter_ty,
                replacement: arg,
            };
            let TypeTransformResult::Transformed(substituted) =
                return_ty.substitute_type(db, substitution)
            else {
                return InferredTypeData::Unknown;
            };
            return_ty = substituted;
            substitutions.push(substitution);
            continue;
        }

        let Some(parameter_function) =
            resolve_callable_function(db, CallableFunctionInput::new(db, parameter_ty))
        else {
            continue;
        };
        if let Some((generic, replacement)) =
            callback_predicate_binding(db, function, parameter_ty, arg)
        {
            if satisfies_constraint(db, generic, replacement, receiver, &substitutions)
                != Some(true)
            {
                continue;
            }
            for substitution in generic_substitutions(db, generic, replacement) {
                let TypeTransformResult::Transformed(substituted) =
                    return_ty.substitute_type(db, substitution)
                else {
                    return InferredTypeData::Unknown;
                };
                return_ty = substituted;
                substitutions.push(substitution);
            }
            continue;
        }
        let ReturnType::Type(parameter_return_ty) = parameter_function.return_type(db) else {
            continue;
        };
        let Some(argument_function) =
            resolve_callable_function(db, CallableFunctionInput::new(db, arg))
        else {
            continue;
        };
        let ReturnType::Type(argument_return_ty) = argument_function.return_type(db) else {
            continue;
        };

        let Some(callback_substitutions) =
            collect_callback_return_replacements(db, *parameter_return_ty, *argument_return_ty)
        else {
            return InferredTypeData::Unknown;
        };
        for substitution in callback_substitutions {
            let TypeTransformResult::Transformed(substituted) =
                return_ty.substitute_type(db, substitution)
            else {
                return InferredTypeData::Unknown;
            };
            return_ty = substituted;
            substitutions.push(substitution);
        }
    }

    for type_parameter in function.type_parameters(db) {
        let Some(type_parameter) = substitutions
            .iter()
            .try_fold(*type_parameter, |ty, substitution| {
                ty.substitute_type(db, *substitution).map_or(None, Some)
            })
        else {
            return InferredTypeData::Unknown;
        };
        let (generic, alternate) = match type_parameter {
            InferredTypeData::Generic(generic) => (
                Some(generic),
                Some(InferredTypeData::instance_of(
                    db,
                    type_parameter,
                    Box::default(),
                )),
            ),
            InferredTypeData::InstanceOf(instance) => match instance.ty(db) {
                ty @ InferredTypeData::Generic(generic) => (Some(generic), Some(ty)),
                _ => (None, None),
            },
            _ => (None, None),
        };
        if let Some(default) = generic.and_then(|generic| generic.default(db)) {
            let Some(replacement) = substitutions.iter().try_fold(default, |ty, substitution| {
                ty.substitute_type(db, *substitution).map_or(None, Some)
            }) else {
                return InferredTypeData::Unknown;
            };
            let substitution = InferredTypeSubstitution {
                generic: type_parameter,
                replacement,
            };
            let TypeTransformResult::Transformed(substituted) =
                return_ty.substitute_type(db, substitution)
            else {
                return InferredTypeData::Unknown;
            };
            return_ty = substituted;
            substitutions.push(substitution);
            if let Some(alternate) = alternate
                && alternate != type_parameter
            {
                let substitution = InferredTypeSubstitution {
                    generic: alternate,
                    replacement,
                };
                let TypeTransformResult::Transformed(substituted) =
                    return_ty.substitute_type(db, substitution)
                else {
                    return InferredTypeData::Unknown;
                };
                return_ty = substituted;
                substitutions.push(substitution);
            }
        }
    }

    return_ty
}

/// Determines the value produced by a callback with a generic return type.
///
/// When a callback may return either `T` or `Promise<T>`, a Promise result uses
/// its inner value. In this example, `value` is `number` rather than
/// `Promise<number>`.
///
/// ```ts
/// declare function unwrap<T>(callback: () => T | Promise<T>): T;
/// const value = unwrap(async () => 1);
/// ```
fn collect_callback_return_replacements<'db>(
    db: &'db dyn ModuleDb,
    parameter_return_ty: InferredTypeData<'db>,
    argument_return_ty: InferredTypeData<'db>,
) -> Option<Vec<InferredTypeSubstitution<'db>>> {
    if let InferredTypeData::Union(union) = parameter_return_ty {
        if let InferredTypeData::InstanceOf(argument) = argument_return_ty {
            for parameter in union.types(db) {
                let InferredTypeData::InstanceOf(parameter_instance) = parameter else {
                    continue;
                };
                if parameter_instance.ty(db) == argument.ty(db)
                    || parameter_instance.ty(db).is_promise_class(db)
                        && argument.ty(db).is_promise_class(db)
                {
                    let replacements =
                        parameter.collect_generic_replacements(db, argument_return_ty)?;
                    if !replacements.is_empty() {
                        return Some(replacements);
                    }
                }
            }
        }

        if let Some(generic) = union
            .types(db)
            .iter()
            .find(|ty| ty.is_generic_reference(db))
            && union.types(db).iter().any(|ty| {
                matches!(
                    ty,
                    InferredTypeData::InstanceOf(instance)
                        if instance.ty(db).is_promise_class(db)
                            && instance.type_parameters(db).contains(generic)
                )
            })
        {
            return Some(Vec::from([InferredTypeSubstitution {
                generic: *generic,
                replacement: argument_return_ty,
            }]));
        }
    }

    parameter_return_ty.collect_generic_replacements(db, argument_return_ty)
}

enum TupleInstanceResolution<'db> {
    Tuple(InferredTuple<'db>),
    NotTuple,
    Indeterminate,
}

#[derive(Clone, Copy, Debug)]
enum ExpandedTupleElement<'db> {
    Required(InferredTypeData<'db>),
    Optional(InferredTypeData<'db>),
    Spread(InferredTypeData<'db>),
}

enum TupleExpansionItem<'db> {
    Type {
        ty: InferredTypeData<'db>,
        remaining_steps: usize,
    },
    Exit(InferredTypeData<'db>),
    Element(ExpandedTupleElement<'db>),
}

/// One entry in an argument sequence after tuple spreads are expanded.
#[derive(Clone, Copy, Debug)]
pub(in crate::db) enum ResolvedCallArgument<'db> {
    /// A required argument that consumes one parameter position.
    Argument(InferredTypeData<'db>),
    /// An inline literal with a separate candidate for const generic inference.
    ConstArgument {
        ty: InferredTypeData<'db>,
        const_ty: InferredTypeData<'db>,
    },
    /// An object or array expression whose properties have fresh literal
    /// types. `ty` widens those literal types, as TypeScript does when no
    /// parameter gives them a literal type, and `literal_ty` keeps them.
    ///
    /// A parameter is matched against `literal_ty`, because TypeScript keeps
    /// the argument's literal types where the parameter has literal types,
    /// so `{ kind: "b" }` matches `{ kind: "b" }` and not `{ kind: "a" }`
    /// when an overload is selected. A generic parameter is inferred from
    /// `ty`, so `T` is `{ a: number }` for `id({ a: 1 })`.
    WidenedArgument {
        ty: InferredTypeData<'db>,
        literal_ty: InferredTypeData<'db>,
    },
    /// An argument that may be absent and consumes at most one position.
    Optional(InferredTypeData<'db>),
    /// An argument that may consume zero or more parameter positions.
    Spread(InferredTypeData<'db>),
}

impl<'db> ResolvedCallArgument<'db> {
    fn accepts_zero(self) -> bool {
        !matches!(
            self,
            Self::Argument(_) | Self::ConstArgument { .. } | Self::WidenedArgument { .. }
        )
    }

    fn is_spread(self) -> bool {
        matches!(self, Self::Spread(_))
    }

    pub(in crate::db) fn ty(self) -> InferredTypeData<'db> {
        match self {
            Self::Argument(ty) | Self::Optional(ty) | Self::Spread(ty) => ty,
            Self::ConstArgument { ty, .. } | Self::WidenedArgument { ty, .. } => ty,
        }
    }

    fn matches_parameter(self, db: &'db dyn ModuleDb, parameter: InferredTypeData<'db>) -> bool {
        if let Self::WidenedArgument { literal_ty, .. } = self {
            return ArgumentTypeCompatibility::new(parameter, literal_ty).is_satisfied(db);
        }
        if ArgumentTypeCompatibility::new(parameter, self.ty()).is_satisfied(db) {
            return true;
        }
        let is_const = matches!(parameter, InferredTypeData::Generic(generic) if generic.is_const(db))
            || matches!(parameter, InferredTypeData::InstanceOf(instance) if matches!(instance.ty(db), InferredTypeData::Generic(generic) if generic.is_const(db)));
        if is_const && let Self::ConstArgument { const_ty, .. } = self {
            return ArgumentTypeCompatibility {
                const_inference: true,
                parameter_ty: parameter,
                argument_ty: const_ty,
            }
            .is_satisfied(db);
        }
        false
    }

    pub(in crate::db) fn for_parameter(
        self,
        db: &'db dyn ModuleDb,
        parameter: InferredTypeData<'db>,
    ) -> InferredTypeData<'db> {
        let generic = match parameter {
            InferredTypeData::Generic(generic) => Some(generic),
            InferredTypeData::InstanceOf(instance) => match instance.ty(db) {
                InferredTypeData::Generic(generic) => Some(generic),
                _ => None,
            },
            _ => None,
        };
        if let Some(generic) = generic
            && generic.is_const(db)
            && let Self::ConstArgument { const_ty, .. } = self
        {
            if let Some(constraint) = generic.constraint(db) {
                let constraint = resolve_local_type_on_demand(db, constraint);
                if !(ArgumentTypeCompatibility {
                    const_inference: true,
                    parameter_ty: constraint,
                    argument_ty: const_ty,
                })
                .is_satisfied(db)
                {
                    return constraint;
                }
            }
            return const_ty;
        }
        self.ty()
    }
}

#[derive(Clone, Copy, Debug)]
enum ResolvedParameter<'db> {
    Required(InferredTypeData<'db>),
    Optional(InferredTypeData<'db>),
    Spread(InferredTypeData<'db>),
}

impl<'db> ResolvedParameter<'db> {
    fn accepts_zero(self) -> bool {
        !matches!(self, Self::Required(_))
    }

    fn is_spread(self) -> bool {
        matches!(self, Self::Spread(_))
    }

    fn ty(self) -> InferredTypeData<'db> {
        match self {
            Self::Required(ty) | Self::Optional(ty) | Self::Spread(ty) => ty,
        }
    }
}

/// Positional parameters after formal tuple rests are expanded.
///
/// Entries retain whether they consume exactly one, at most one, or any number
/// of argument positions.
#[derive(Debug)]
struct ResolvedParameters<'db>(Vec<ResolvedParameter<'db>>);

impl<'db> ResolvedParameters<'db> {
    /// Returns parameters for a constructor or function signature.
    fn from_constructor_signature(
        db: &'db dyn ModuleDb,
        ty: InferredTypeData<'db>,
    ) -> Option<Self> {
        match ty.expand_canonical_global(db) {
            InferredTypeData::Constructor(constructor) => {
                Some(Self::from_constructor(db, constructor))
            }
            InferredTypeData::Function(function) => Some(Self::from_function(db, function)),
            _ => None,
        }
    }

    fn from_function(db: &'db dyn ModuleDb, function: InferredFunction<'db>) -> Self {
        let parameters = function.parameters(db);
        let mut resolved = Self::with_capacity(parameters.len());
        for parameter in parameters {
            resolved.push_parameter(db, parameter);
        }
        resolved
    }

    fn from_constructor(db: &'db dyn ModuleDb, constructor: InferredConstructor<'db>) -> Self {
        let parameters = constructor.parameters(db);
        let mut resolved = Self::with_capacity(parameters.len());
        for parameter in parameters {
            resolved.push_parameter(db, &parameter.parameter);
        }
        resolved
    }

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn push_parameter(
        &mut self,
        db: &'db dyn ModuleDb,
        parameter: &InferredFunctionParameter<'db>,
    ) {
        if parameter.is_rest() {
            self.push_rest_parameter(db, parameter.ty());
        } else if parameter.is_optional() {
            self.0.push(ResolvedParameter::Optional(parameter.ty()));
        } else {
            self.0.push(ResolvedParameter::Required(parameter.ty()));
        }
    }

    fn push_rest_parameter(&mut self, db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) {
        let expanded = expanded_tuple_sequence(db, ty);
        self.0
            .extend(expanded.into_iter().map(|element| match element {
                ExpandedTupleElement::Required(ty) => ResolvedParameter::Required(ty),
                ExpandedTupleElement::Optional(ty) => ResolvedParameter::Optional(ty),
                ExpandedTupleElement::Spread(ty) => {
                    ResolvedParameter::Spread(spread_element_type(db, ty))
                }
            }));
    }

    fn as_slice(&self) -> &[ResolvedParameter<'db>] {
        &self.0
    }
}

type ArgumentTypeInference<'db> = fn(
    &'db dyn ModuleDb,
    InferredTypeData<'db>,
    &[ResolvedCallArgument<'db>],
    usize,
) -> Option<InferredTypeData<'db>>;

enum ArgumentTypeItem<'db> {
    Type(InferredTypeData<'db>),
    CollectUnion(usize),
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct ArgumentSequenceState {
    parameter_index: usize,
    argument_index: usize,
    matched_parameter_index: Option<usize>,
}

impl ArgumentSequenceState {
    fn new_matched_parameter(mut self) -> Self {
        self.matched_parameter_index = Some(self.parameter_index);
        self
    }

    fn next_parameter(mut self) -> Self {
        self.parameter_index += 1;
        self
    }

    fn next_argument(mut self) -> Self {
        self.argument_index += 1;
        self
    }
}

#[derive(Clone, Copy)]
struct ArgumentMatchRequirementId(usize);

#[derive(Clone, Copy)]
struct ArgumentMatchRequirement<'db> {
    compatibility: ArgumentTypeCompatibility<'db>,
    next: Option<ArgumentMatchRequirementId>,
}

/// Arena of immutable compatibility-requirement nodes.
///
/// Node IDs remain valid for the lifetime of a search because nodes are never
/// removed.
struct ArgumentMatchRequirements<'db>(Vec<ArgumentMatchRequirement<'db>>);

impl<'db> ArgumentMatchRequirements<'db> {
    fn new(compatibility: ArgumentTypeCompatibility<'db>) -> (Self, ArgumentMatchRequirementId) {
        let mut requirements = Self(Vec::new());
        let root = requirements.prepend(None, compatibility);
        (requirements, root)
    }

    fn get(&self, id: ArgumentMatchRequirementId) -> ArgumentMatchRequirement<'db> {
        self.0[id.0]
    }

    fn prepend(
        &mut self,
        next: Option<ArgumentMatchRequirementId>,
        compatibility: ArgumentTypeCompatibility<'db>,
    ) -> ArgumentMatchRequirementId {
        let id = ArgumentMatchRequirementId(self.0.len());
        self.0.push(ArgumentMatchRequirement {
            compatibility,
            next,
        });
        id
    }

    fn prepend_all(
        &mut self,
        mut next: Option<ArgumentMatchRequirementId>,
        compatibilities: impl IntoIterator<Item = ArgumentTypeCompatibility<'db>>,
    ) -> Option<ArgumentMatchRequirementId> {
        for compatibility in compatibilities {
            next = Some(self.prepend(next, compatibility));
        }
        next
    }
}

/// Boolean requirements produced by decomposing one compatibility relation.
///
/// `All` is a conjunction and `Any` is a disjunction. `Match` resumes the
/// current conjunction, while `Mismatch` abandons its branch.
enum ArgumentMatchAction<'db> {
    Match,
    Mismatch,
    All(Vec<ArgumentTypeCompatibility<'db>>),
    Any(Vec<ArgumentTypeCompatibility<'db>>),
}

impl<'db> ArgumentMatchAction<'db> {
    fn schedule(
        self,
        continuation: Option<ArgumentMatchRequirementId>,
        requirements: &mut ArgumentMatchRequirements<'db>,
        pending: &mut Vec<Option<ArgumentMatchRequirementId>>,
    ) {
        match self {
            Self::Match => pending.push(continuation),
            Self::Mismatch => {}
            Self::All(compatibilities) => {
                pending.push(requirements.prepend_all(continuation, compatibilities));
            }
            Self::Any(compatibilities) => {
                // Requirement nodes are immutable, so every alternative can
                // share the same continuation.
                for compatibility in compatibilities.into_iter().rev() {
                    pending.push(Some(requirements.prepend(continuation, compatibility)));
                }
            }
        }
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_type_info::interned_types::{
        InternedLiteral, NamedFunctionParameter, PredicateReturnType,
    };
    use biome_js_type_info::literal::{NumberLiteral, StringLiteral};

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        storage: salsa::Storage<Self>,
        fs: biome_fs::MemoryFileSystem,
        resolver_paths: biome_resolver::ResolverPaths,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, _path: &camino::Utf8Path) -> Option<biome_db::Source> {
            None
        }
    }

    #[salsa::db]
    impl biome_parser::db::ParseDb for TestDb {
        fn parse_source(&self, _source: biome_db::Source) -> biome_parser::db::SourceOutput {
            unreachable!("The database has no sources")
        }
    }

    #[salsa::db]
    impl biome_resolver::ResolverDb for TestDb {
        fn resolver_fs(&self) -> &dyn biome_resolver::ResolverFsProxy {
            &self.fs
        }

        fn resolver_paths(&self) -> &biome_resolver::ResolverPaths {
            &self.resolver_paths
        }
    }

    #[salsa::db]
    impl biome_js_type_info::TypeDb for TestDb {}

    #[salsa::db]
    impl biome_languages::LanguageDb for TestDb {
        fn source_from_index(&self, _index: usize) -> Option<biome_languages::DocumentFileSource> {
            None
        }
    }

    #[salsa::db]
    impl ModuleDb for TestDb {
        fn module_for_path(&self, _path: &camino::Utf8Path) -> Option<crate::ModuleInfo> {
            None
        }

        fn for_each_module(&self, _f: &mut dyn FnMut(crate::ModuleInfo)) {}
    }

    /// Returns a signature `(v: parameters[0], ...): v is string`, where each
    /// parameter is `(type, optional)`.
    fn signature<'db>(
        db: &'db TestDb,
        parameters: &[(InferredTypeData<'db>, bool)],
    ) -> InferredFunction<'db> {
        let parameters: Box<[_]> = parameters
            .iter()
            .enumerate()
            .map(|(index, (ty, is_optional))| {
                InferredFunctionParameter::Named(NamedFunctionParameter {
                    name: Text::new_static(["v", "w", "x"][index]),
                    ty: *ty,
                    is_optional: *is_optional,
                    is_rest: false,
                })
            })
            .collect();
        InferredFunction::new(
            db,
            Box::default(),
            parameters,
            ReturnType::Predicate(PredicateReturnType {
                parameter_name: Text::new_static("v"),
                ty: InferredTypeData::String,
            }),
            false,
            None,
        )
    }

    fn string_literal<'db>(db: &'db TestDb, value: &str) -> InferredTypeData<'db> {
        InferredTypeData::Literal(InternedLiteral::new(
            db,
            InferredLiteral::String(StringLiteral::from(value)),
        ))
    }

    fn number_literal(db: &TestDb) -> InferredTypeData<'_> {
        InferredTypeData::Literal(InternedLiteral::new(
            db,
            InferredLiteral::Number(NumberLiteral::new(Text::new_static("1"))),
        ))
    }

    // `signature_accepts_arguments` accepts only where the matching that
    // overload selection uses for return types finds the signature viable,
    // so it never accepts a signature that that selection skips.
    // `rejects_every_narrowing` rejects only arguments that no narrowing can
    // make acceptable.
    #[test]
    fn conclusive_acceptance_agrees_with_return_type_selection() {
        use InferredTypeData::{AnyKeyword, Null, Number, ObjectKeyword, String, Unknown};
        let db = TestDb::default();
        let a = string_literal(&db, "a");
        let one = number_literal(&db);
        let string_or_null = InferredTypeData::union_from_types(&db, Vec::from([String, Null]));
        let string_or_number = InferredTypeData::union_from_types(&db, Vec::from([String, Number]));
        // Each case gives whether the signature accepts the arguments, and
        // whether it rejects them whatever narrowing gives them.
        let cases = [
            // A literal is a subtype of its primitive.
            (
                "literal",
                &[(String, false)][..],
                Vec::from([a]),
                true,
                false,
            ),
            (
                "union",
                &[(string_or_number, false)],
                Vec::from([String]),
                true,
                false,
            ),
            (
                "missing optional",
                &[(String, false), (Number, true)],
                Vec::from([a]),
                true,
                false,
            ),
            (
                "other primitive",
                &[(String, false)],
                Vec::from([one]),
                false,
                true,
            ),
            // Narrowing can remove `null`.
            (
                "nullable",
                &[(String, false)],
                Vec::from([string_or_null]),
                false,
                false,
            ),
            (
                "missing argument",
                &[(String, false)],
                Vec::new(),
                false,
                true,
            ),
            (
                "extra argument",
                &[(String, false)],
                Vec::from([a, a]),
                false,
                true,
            ),
            // Return-type selection treats an unresolved argument as viable,
            // and so does this, without accepting it.
            (
                "unresolved",
                &[(String, false)],
                Vec::from([Unknown]),
                false,
                false,
            ),
            // Narrowing can give `string` the literal type `"a"`.
            (
                "wider than a literal",
                &[(a, false)],
                Vec::from([String]),
                false,
                false,
            ),
            // `any` is assignable to every type other than `never`, but a
            // subtype only of `any` and `unknown`, and narrowing can give it
            // any type.
            (
                "any",
                &[(String, false)],
                Vec::from([AnyKeyword]),
                false,
                false,
            ),
            // No object is `null`.
            (
                "null object",
                &[(ObjectKeyword, false)],
                Vec::from([Null]),
                false,
                true,
            ),
            // A primitive can share values with an object type through an
            // intersection.
            (
                "primitive object",
                &[(ObjectKeyword, false)],
                Vec::from([one]),
                false,
                false,
            ),
        ];
        for (name, parameters, arguments, accepts, rejects) in cases {
            let function = signature(&db, parameters);
            let arguments: Vec<_> = arguments
                .into_iter()
                .map(ResolvedCallArgument::Argument)
                .collect();
            let acceptance = signature_accepts_arguments(&db, function, &arguments);
            assert_eq!(acceptance, accepts, "{name}: acceptance");
            assert_eq!(
                rejects_every_narrowing(&db, function, &arguments),
                rejects,
                "{name}: rejection"
            );
            if acceptance {
                assert!(
                    signature_may_accept_arguments(&db, function, &arguments),
                    "{name}: accepted but not viable"
                );
            }
        }
        // A spread argument can fill any number of parameters.
        let function = signature(&db, &[(String, false)]);
        let spread = [ResolvedCallArgument::Spread(Unknown)];
        assert!(!signature_accepts_arguments(&db, function, &spread));
        assert!(!rejects_every_narrowing(&db, function, &spread));
    }
}
