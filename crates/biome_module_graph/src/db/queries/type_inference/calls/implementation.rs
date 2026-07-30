//! Call matching, argument expansion, and generic return substitution.
//!
//! Query entry points delegate to this module after Salsa has established their
//! incremental boundary. The algorithms here share bounded traversal state and
//! resolved argument representations with expression inference.

use crate::db::queries::resolve_callable_type;
use crate::db::type_inference::{
    apply_substitutions_to_root_body, collected_type_result, find_member_type_on_demand,
    resolve_local_type_on_demand, substitutions_for_instance,
};
use crate::module_graph::ModuleInfoKind;
use crate::{ModuleDb, module_for_key};
use biome_js_type_info::{
    RawTypeData, TypeReference, TypeResolverLevel, global_types,
    interned_types::{
        CallArgumentType as InferredCallArgumentType,
        FunctionParameter as InferredFunctionParameter, InternedConstructor as InferredConstructor,
        InternedFunction as InferredFunction, InternedMergedReference as InferredMergedReference,
        InternedTuple as InferredTuple, InternedTypeInstance as InferredTypeInstance,
        Literal as InferredLiteral, LocalTypeHandle as InferredLocalTypeHandle,
        LocalTypeId as InferredLocalTypeId, ModuleKey as InferredModuleKey, ReturnType,
        TupleElementType as InferredTupleElementType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeSubstitution as InferredTypeSubstitution,
        TypeTransformResult,
    },
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
    infer_call_expression_return_type_from_args(db, callee, &args)
}

/// Infers a call result after optional and spread arguments are resolved.
///
/// A union of callable types produces a union of their return types. A `null`
/// or `undefined` member contributes `undefined`. Other non-callable members
/// are omitted. The result is `Unknown` when no supported callable member
/// remains.
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
                        infer_function_call_type(db, *callee, args)
                    }
                })
                .collect(),
        )
        .unwrap_or(InferredTypeData::Unknown),
        callee => infer_function_call_type(db, callee, args).unwrap_or(InferredTypeData::Unknown),
    }
}

fn infer_function_call_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Function(function) => infer_function_return_type(db, function, args),
        InferredTypeData::InstanceOf(instance) => {
            let target = instance.ty(db);
            let substitutions =
                substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
            let target = apply_substitutions_to_root_body(db, target, &substitutions);
            infer_function_call_type(db, target, args)
        }
        InferredTypeData::Interface(interface) => {
            select_call_signature(db, interface.members(db), args)
                .and_then(|function| infer_function_return_type(db, function, args))
        }
        InferredTypeData::Object(object) => select_call_signature(db, object.members(db), args)
            .and_then(|function| infer_function_return_type(db, function, args)),
        InferredTypeData::Union(union) => collected_type_result(
            db,
            union
                .types(db)
                .iter()
                .filter_map(|callee| {
                    if matches!(callee, InferredTypeData::Null | InferredTypeData::Undefined) {
                        Some(InferredTypeData::Undefined)
                    } else {
                        infer_function_call_type(db, *callee, args)
                    }
                })
                .collect(),
        ),
        InferredTypeData::TypeofType(typeof_type) => {
            infer_function_call_type(db, typeof_type.ty(db), args)
        }
        InferredTypeData::TypeofValue(typeof_value) => {
            infer_function_call_type(db, typeof_value.ty(db), args)
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
        .filter_map(|member| member.ty.callable_function(db));
    let first = signatures.next()?;
    let Some(second) = signatures.next() else {
        return Some(first);
    };

    [first, second]
        .into_iter()
        .chain(signatures)
        .find(|function| signature_accepts_arguments(db, *function, args))
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
        .filter_map(|member| member.ty.callable_function(db));
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
        .filter_map(|member| ResolvedParameters::from_constructor_signature(db, member.ty));
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

fn signature_accepts_arguments<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    let parameters = ResolvedParameters::from_function(db, function);
    let parameters = parameters.as_slice();
    if parameters
        .iter()
        .all(|parameter| matches!(parameter, ResolvedParameter::Required(_)))
        && args
            .iter()
            .all(|argument| matches!(argument, ResolvedCallArgument::Argument(_)))
    {
        return parameters.len() == args.len()
            && parameters.iter().zip(args).all(|(parameter, argument)| {
                ArgumentTypeCompatibility::new(parameter.ty(), argument.ty()).is_satisfied(db)
            });
    }

    matched_parameter_types(db, parameters, args, None).is_some()
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
    if args[..argument_index]
        .iter()
        .all(|argument| matches!(argument, ResolvedCallArgument::Argument(_)))
        && let Some(parameter) = parameters.get(argument_index)
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
        && args
            .iter()
            .all(|argument| matches!(argument, ResolvedCallArgument::Argument(_)))
    {
        let parameter = parameters.get(argument_index)?;
        args.get(argument_index)?;
        return (parameters.len() == args.len()
            && parameters
                .iter()
                .zip(args)
                .enumerate()
                .all(|(index, (parameter, argument))| {
                    index == argument_index
                        || ArgumentTypeCompatibility::new(parameter.ty(), argument.ty())
                            .is_satisfied(db)
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

        let arg_ty = if argument.is_spread() {
            spread_element_type(db, argument.ty())
        } else {
            argument.ty()
        };
        if ArgumentTypeCompatibility::new(parameter.ty(), arg_ty).is_satisfied(db) {
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

/// Finds the function a value of type `ty` can be called as.
///
/// A class instance contributes its type arguments, so a callable reached
/// through `InstanceOf` describes its parameters and return type in terms of
/// the arguments the instance was built with. Every other wrapper, and the
/// shapes that are too ambiguous to resolve, are described on
/// [`resolve_callable_type`].
pub(in crate::db) fn resolve_callable_function<'db>(
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
    parameter_ty: InferredTypeData<'db>,
    argument_ty: InferredTypeData<'db>,
}

impl<'db> ArgumentTypeCompatibility<'db> {
    fn new(parameter_ty: InferredTypeData<'db>, argument_ty: InferredTypeData<'db>) -> Self {
        Self {
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
            resolve_callable_function(db, self.parameter_ty),
            resolve_callable_function(db, self.argument_ty),
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
                | InferredTypeData::Literal(_)
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::TypeOperator(_)
                | InferredTypeData::TypeofExpression(_),
                _,
            )
            | (
                _,
                InferredTypeData::Conditional
                | InferredTypeData::Global
                | InferredTypeData::GlobalType(_)
                | InferredTypeData::Literal(_)
                | InferredTypeData::ObjectKeyword
                | InferredTypeData::TypeOperator(_)
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
                    if let Some(extends) = class.extends(db) {
                        pending.push(extends);
                    }
                }
                InferredTypeData::InstanceOf(instance) => pending.push(instance.ty(db)),
                InferredTypeData::Local(local) => {
                    if local == parameter {
                        return Some(true);
                    }
                    resolved_known_type = true;
                    if let Some(extends) = raw_local_class_extends(db, local) {
                        pending.push(InferredTypeData::Local(extends));
                    }
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
                    if let Some(extends) = class.extends(db) {
                        pending.push(extends);
                    }
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
        if let Some(extends) = raw_local_class_extends(db, local) {
            pending.push(extends);
        }
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

fn raw_local_class_extends<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
) -> Option<InferredLocalTypeHandle<'db>> {
    let module_key = local.module(db);
    let current = module_for_key(db, module_key)?;
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return None;
    };
    let RawTypeData::Class(class) = js_info.raw_types.get(local.type_id(db).index())? else {
        return None;
    };
    local_handle_from_reference(db, module_key, class.extends.as_ref()?)
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
) -> Option<InferredTypeData<'db>> {
    match function.return_type(db) {
        ReturnType::Type(ty) => {
            Some(infer_generic_return_type(db, function, *ty, args).expand_structural_global(db))
        }
        ReturnType::Predicate(_) | ReturnType::Asserts(_) => Some(InferredTypeData::Boolean),
    }
}

/// Substitutes generic references in a function's return type.
///
/// A direct generic parameter is inferred from its argument. A generic in a
/// callback return type is inferred from the callback's return type. An
/// unbound parameter uses its declared default. Defaults are processed in
/// declaration order, so a default may refer to an earlier parameter.
/// Parameters with no inferred value or default remain generic. A failed type
/// transformation returns `Unknown`.
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
) -> InferredTypeData<'db> {
    let mut substitutions: Vec<InferredTypeSubstitution<'db>> = Vec::new();
    for (parameter, arg) in function
        .parameters(db)
        .iter()
        .zip(args.iter().copied().map(ResolvedCallArgument::ty))
    {
        let parameter_ty = parameter.ty();
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

        let Some(parameter_function) = resolve_callable_function(db, parameter_ty) else {
            continue;
        };
        let ReturnType::Type(parameter_return_ty) = parameter_function.return_type(db) else {
            continue;
        };
        let Some(argument_function) = resolve_callable_function(db, arg) else {
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
    /// An argument that may be absent and consumes at most one position.
    Optional(InferredTypeData<'db>),
    /// An argument that may consume zero or more parameter positions.
    Spread(InferredTypeData<'db>),
}

impl<'db> ResolvedCallArgument<'db> {
    fn accepts_zero(self) -> bool {
        !matches!(self, Self::Argument(_))
    }

    fn is_spread(self) -> bool {
        matches!(self, Self::Spread(_))
    }

    pub(in crate::db) fn ty(self) -> InferredTypeData<'db> {
        match self {
            Self::Argument(ty) | Self::Optional(ty) | Self::Spread(ty) => ty,
        }
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
