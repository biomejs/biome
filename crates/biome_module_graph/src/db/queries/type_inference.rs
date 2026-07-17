//! This module represents the database queries used by the module graph.
//!
//! The queries are defined in terms of `ModuleInfo` inputs.
//!
//! The queries are tracked so that Salsa can invalidate them when the inputs
//! change.
//!
//! The queries are also interned, so that Salsa can reuse the same computation
//! when the inputs are the same.
//!
//! This module should contain only tracked functions, exposed to the consumers. Middle
//! functions that aren't queries should be moved somewhere else, unless they are used
//! directly by the tracked functions e.g. cycle detection

use crate::db::type_inference::{
    apply_substitutions_to_root_body, collected_type_result, infer_module_types_cycle_result,
    normalize_type_cycle_result, resolve_raw_types, substitutions_for_instance,
};
use crate::module_for_key;
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{JsExport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_js_type_info::{
    RawTypeData, TypeReference, TypeResolverLevel,
    interned_types::{
        CallArgumentType as InferredCallArgumentType,
        FunctionParameter as InferredFunctionParameter, InternedFunction as InferredFunction,
        InternedMergedReference as InferredMergedReference, InternedTuple as InferredTuple,
        InternedTypeInstance as InferredTypeInstance, Literal as InferredLiteral,
        LocalTypeHandle as InferredLocalTypeHandle, LocalTypeId as InferredLocalTypeId,
        ModuleKey as InferredModuleKey, ReturnType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeSubstitution as InferredTypeSubstitution,
    },
};
use rustc_hash::{FxHashMap, FxHashSet};

pub use crate::db::type_inference::InferredModuleTypes;

const MAX_TYPE_NORMALIZATION_STEPS: usize = 1024;
const MAX_ARGUMENT_MATCH_STEPS: usize = 1024;
const MAX_ARGUMENT_SEQUENCE_STEPS: usize = 1024;
const MAX_ARGUMENT_TYPE_STEPS: usize = 1024;
const MAX_LOCAL_EXTENDS_STEPS: usize = 1024;

// #region EXPORTED TRACKED QUERIES

#[salsa::tracked(cycle_result=infer_module_types_cycle_result)]
pub fn infer_module_types<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let mut import_types = FxHashMap::default();
    for import_path in js_info.static_import_paths.values() {
        if let Some(path) = import_path.as_path()
            && let Some(target) = db.module_for_path(path)
            && let Some(target_types) = infer_module_types(db, target)
        {
            import_types.insert(import_path.resolved_path.clone(), target_types);
        }
    }

    Some(resolve_raw_types(db, module, &js_info, &import_types))
}

// NOTE: this is the only exception to the rule.
/// Infers the types of a module, preparing the modules it imports first.
///
/// This is the entry point to use when answering an outside request, such as
/// a lint rule asking for type information after a file was opened or
/// changed. At that moment the imported modules may not have been inferred
/// yet, and this function works through them one at a time, innermost imports
/// first, so that even very long import chains are handled safely.
///
/// From within other database queries, call [`infer_module_types`] directly
/// instead: there, the imported modules are already taken care of.
pub fn infer_module_types_bottom_up<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
) -> Option<InferredModuleTypes<'db>> {
    let mut visited = FxHashSet::default();
    let mut stack = vec![(module, false)];

    while let Some((current, imports_visited)) = stack.pop() {
        if imports_visited {
            infer_module_types(db, current);
            continue;
        }
        if !visited.insert(current) {
            continue;
        }

        // Revisit this module to infer it once its imports below are done.
        stack.push((current, true));

        let ModuleInfoKind::Js(js_info) = current.kind(db) else {
            continue;
        };
        for import_path in js_info.static_import_paths.values() {
            push_inference_dependency(db, &visited, &mut stack, &import_path.resolved_path);
        }
        for reexport in js_info.blanket_reexports.iter() {
            push_inference_dependency(db, &visited, &mut stack, &reexport.import.resolved_path);
        }
        for export in js_info.exports.values() {
            match export {
                JsExport::Reexport(reexport) | JsExport::ReexportType(reexport) => {
                    push_inference_dependency(
                        db,
                        &visited,
                        &mut stack,
                        &reexport.import.resolved_path,
                    );
                }
                JsExport::Own(JsOwnExport::Namespace(reexport))
                | JsExport::OwnType(JsOwnExport::Namespace(reexport)) => {
                    push_inference_dependency(
                        db,
                        &visited,
                        &mut stack,
                        &reexport.import.resolved_path,
                    );
                }
                JsExport::Own(_) | JsExport::OwnType(_) => {}
            }
        }
    }

    infer_module_types(db, module)
}

// #endregion

// #region QUERY HELPER FUNCTIONS

fn push_inference_dependency(
    db: &dyn ModuleDb,
    visited: &FxHashSet<ModuleInfo>,
    stack: &mut Vec<(ModuleInfo, bool)>,
    resolved_path: &ResolvedPath,
) {
    if let Some(path) = resolved_path.as_path()
        && let Some(target) = db.module_for_path(path)
        && !visited.contains(&target)
    {
        stack.push((target, false));
    }
}

// #endregion

// #region EXPORTED TRACKED QUERIES

#[salsa::tracked]

pub fn infer_call_expression_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallExpressionTypeInput<'db>,
) -> InferredTypeData<'db> {
    let module = input.module(db);
    let callee = normalize_type(db, NormalizeTypeInput::new(db, module, input.callee(db)));
    let args = input.args(db);
    let ty = infer_call_expression_return_type(db, callee, args);

    normalize_type(db, NormalizeTypeInput::new(db, module, ty))
}

// #endregion

// #region QUERY HELPER FUNCTIONS

pub(crate) fn infer_call_expression_return_type<'db>(
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

pub(crate) fn infer_call_expression_return_type_from_args<'db>(
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
            infer_call_signature_type(db, interface.members(db), args)
        }
        InferredTypeData::Object(object) => infer_call_signature_type(db, object.members(db), args),
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

fn infer_call_signature_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredTypeData<'db>> {
    select_call_signature(db, members, args)
        .and_then(|function| infer_function_return_type(db, function, args))
}

fn select_call_signature<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredFunction<'db>> {
    let signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature())
        .filter_map(|member| member.ty.callable_function(db))
        .collect::<Vec<_>>();

    if signatures.len() < 2 {
        return signatures.first().copied();
    }

    signatures
        .into_iter()
        .find(|function| signature_accepts_arguments(db, *function, args))
}

// #endregion

// #region EXPORTED TRACKED QUERIES

/// Returns the parameter type selected for a call argument.
///
/// The requested argument is ignored while selecting an overload because this
/// query is finding the expected type for that argument.
#[salsa::tracked]
pub fn infer_call_argument_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallArgumentTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let (args, argument_index) =
        resolved_call_arguments(db, input.args(db), input.argument_index(db));
    infer_function_argument_type(db, input.callee(db), &args, argument_index)
}

/// Returns the constructor parameter type selected for a call argument.
#[salsa::tracked]
pub fn infer_constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallArgumentTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let (args, argument_index) =
        resolved_call_arguments(db, input.args(db), input.argument_index(db));
    infer_constructor_argument_type_inner(db, input.callee(db), &args, argument_index)
}

// #endregion

// #region QUERY HELPER FUNCTIONS

fn resolved_call_arguments<'db>(
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
            InferredCallArgumentType::Spread(InferredTypeData::InstanceOf(instance)) => {
                if let Some(tuple) = resolve_tuple_instance(db, *instance) {
                    push_resolved_tuple_spread_arguments(db, &tuple, &mut resolved);
                } else {
                    resolved.push(ResolvedCallArgument::Spread(InferredTypeData::InstanceOf(
                        *instance,
                    )));
                }
            }
            InferredCallArgumentType::Spread(InferredTypeData::Tuple(tuple)) => {
                push_resolved_tuple_spread_arguments(db, tuple, &mut resolved);
            }
            InferredCallArgumentType::Spread(ty) => {
                resolved.push(ResolvedCallArgument::Spread(*ty));
            }
        }
    }

    (resolved, resolved_argument_index)
}

fn resolve_tuple_instance<'db>(
    db: &'db dyn ModuleDb,
    instance: InferredTypeInstance<'db>,
) -> Option<InferredTuple<'db>> {
    let mut ty = InferredTypeData::InstanceOf(instance);

    for _ in 0..MAX_ARGUMENT_TYPE_STEPS {
        let InferredTypeData::InstanceOf(instance) = ty else {
            return match ty {
                InferredTypeData::Tuple(tuple) => Some(tuple),
                _ => None,
            };
        };
        let target = resolve_local_type(db, instance.ty(db));
        let substitutions =
            substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
        ty = apply_substitutions_to_root_body(db, target, &substitutions);
    }

    None
}

fn push_resolved_tuple_spread_arguments<'db>(
    db: &'db dyn ModuleDb,
    tuple: &InferredTuple<'db>,
    resolved: &mut Vec<ResolvedCallArgument<'db>>,
) {
    for element in tuple.elements(db) {
        let ty = if element.is_optional || element.is_rest {
            InferredTypeData::union_from_types(
                db,
                Vec::from([element.ty, InferredTypeData::Undefined]),
            )
        } else {
            element.ty
        };
        if element.is_optional || element.is_rest {
            resolved.push(ResolvedCallArgument::Optional(ty));
        } else {
            resolved.push(ResolvedCallArgument::Argument(ty));
        }
    }
}

fn infer_function_argument_type<'db>(
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
        infer_function_argument_type_leaf,
    )
}

fn infer_function_argument_type_leaf<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Function(function) => {
            infer_single_signature_parameter_type(db, function.parameters(db), args, argument_index)
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
    let signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature())
        .filter_map(|member| member.ty.callable_function(db))
        .collect::<Vec<_>>();
    let single_signature = signatures.len() == 1;

    signatures.into_iter().find_map(|function| {
        if single_signature {
            infer_single_signature_parameter_type(db, function.parameters(db), args, argument_index)
        } else {
            infer_parameter_type_for_argument(db, function.parameters(db), args, argument_index)
        }
    })
}

fn infer_constructor_argument_type_inner<'db>(
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
        infer_constructor_argument_type_leaf,
    )
}

fn infer_constructor_argument_type_leaf<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Class(class) => {
            select_constructor_argument_type(db, class.members(db), args, argument_index)
        }
        InferredTypeData::Constructor(constructor) => {
            let parameters = constructor
                .parameters(db)
                .iter()
                .map(|parameter| parameter.parameter.clone())
                .collect::<Vec<_>>();
            infer_single_signature_parameter_type(db, &parameters, args, argument_index)
        }
        _ => None,
    }
}

fn infer_argument_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
    infer_leaf: ArgumentTypeInference<'db>,
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
                InferredTypeData::InstanceOf(instance) => {
                    let target = resolve_local_type(db, instance.ty(db));
                    let substitutions =
                        substitutions_for_instance(db, target, instance.type_parameters(db), &[]);
                    pending.push(ArgumentTypeItem::Type(apply_substitutions_to_root_body(
                        db,
                        target,
                        &substitutions,
                    )));
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
                    results.push(infer_leaf(db, callee, args, argument_index)?);
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

fn resolve_local_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> InferredTypeData<'db> {
    let InferredTypeData::Local(local) = ty else {
        return ty;
    };
    let Some(module) = module_for_key(db, local.module(db)) else {
        return ty;
    };
    let Some(inferred) = infer_module_types(db, module) else {
        return ty;
    };
    inferred.resolve_type(db, ty)
}

fn select_constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let signatures = members
        .iter()
        .filter(|member| member.kind.is_constructor())
        .filter_map(|member| constructor_signature_parameters(db, member.ty))
        .collect::<Vec<_>>();
    let single_signature = signatures.len() == 1;

    signatures.into_iter().find_map(|parameters| {
        if single_signature {
            infer_single_signature_parameter_type(db, &parameters, args, argument_index)
        } else {
            infer_parameter_type_for_argument(db, &parameters, args, argument_index)
        }
    })
}

fn constructor_signature_parameters<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<Box<[InferredFunctionParameter<'db>]>> {
    match ty {
        InferredTypeData::Constructor(constructor) => Some(
            constructor
                .parameters(db)
                .iter()
                .map(|parameter| parameter.parameter.clone())
                .collect(),
        ),
        InferredTypeData::Function(function) => Some(function.parameters(db).clone()),
        _ => None,
    }
}

fn signature_accepts_arguments<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    matched_parameter_indices(db, function.parameters(db), args, None).is_some()
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
    parameters: &[InferredFunctionParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    args.get(argument_index)?;
    if args[..argument_index]
        .iter()
        .all(|argument| matches!(argument, ResolvedCallArgument::Argument(_)))
    {
        return parameter_for_argument(parameters, argument_index)
            .map(|parameter| parameter_argument_type(db, parameter));
    }

    infer_parameter_type_for_argument(db, parameters, args, argument_index)
}

fn infer_parameter_type_for_argument<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[InferredFunctionParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    collected_type_result(
        db,
        matched_parameter_indices(db, parameters, args, Some(argument_index))?
            .into_iter()
            .filter_map(|index| parameter_for_argument(parameters, index))
            .map(|parameter| parameter_argument_type(db, parameter))
            .collect(),
    )
}

fn matched_parameter_indices<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[InferredFunctionParameter<'db>],
    args: &[ResolvedCallArgument<'db>],
    ignored_argument_index: Option<usize>,
) -> Option<Vec<usize>> {
    let mut pending = Vec::from([ArgumentSequenceState {
        parameter_index: 0,
        argument_index: 0,
        matched_parameter_index: None,
    }]);
    let mut seen = FxHashSet::default();
    let mut matched_parameter_indices = Vec::new();

    for _ in 0..MAX_ARGUMENT_SEQUENCE_STEPS {
        let Some(state) = pending.pop() else {
            return (!matched_parameter_indices.is_empty()).then_some(matched_parameter_indices);
        };
        if !seen.insert(state) {
            continue;
        }

        let Some(argument) = args.get(state.argument_index).copied() else {
            if remaining_parameters_accept_zero(parameters, state.parameter_index) {
                let Some(ignored_argument_index) = ignored_argument_index else {
                    return Some(Vec::new());
                };
                if state.argument_index > ignored_argument_index
                    && let Some(index) = state.matched_parameter_index
                    && !matched_parameter_indices.contains(&index)
                {
                    matched_parameter_indices.push(index);
                }
            }
            continue;
        };

        if ignored_argument_index == Some(state.argument_index) {
            let Some(parameter) = parameter_for_argument(parameters, state.parameter_index) else {
                continue;
            };
            if matches!(argument, ResolvedCallArgument::Optional(_)) {
                pending.push(ArgumentSequenceState {
                    matched_parameter_index: Some(state.parameter_index),
                    ..state.next_argument()
                });
            }
            pending.push(ArgumentSequenceState {
                matched_parameter_index: Some(state.parameter_index),
                ..state.next_parameter(parameter).next_argument()
            });
            continue;
        }

        match argument {
            ResolvedCallArgument::Argument(arg_ty) => {
                push_consumed_argument_state(db, parameters, state, arg_ty, &mut pending)
            }
            ResolvedCallArgument::Optional(arg_ty) => {
                pending.push(state.next_argument());
                push_consumed_argument_state(db, parameters, state, arg_ty, &mut pending);
            }
            ResolvedCallArgument::Spread(arg_ty) => {
                pending.push(state.next_argument());
                let arg_ty = spread_argument_element_type(db, arg_ty);
                push_consumed_spread_state(db, parameters, state, arg_ty, &mut pending);
            }
        }
    }

    if ignored_argument_index.is_none() {
        Some(Vec::new())
    } else {
        (!matched_parameter_indices.is_empty()).then_some(matched_parameter_indices)
    }
}

fn push_consumed_argument_state<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[InferredFunctionParameter<'db>],
    state: ArgumentSequenceState,
    arg_ty: InferredTypeData<'db>,
    pending: &mut Vec<ArgumentSequenceState>,
) {
    let Some(parameter) = parameter_for_argument(parameters, state.parameter_index) else {
        return;
    };
    if !argument_satisfies_parameter(db, parameter, arg_ty) {
        return;
    }

    pending.push(state.next_parameter(parameter).next_argument());
}

fn push_consumed_spread_state<'db>(
    db: &'db dyn ModuleDb,
    parameters: &[InferredFunctionParameter<'db>],
    state: ArgumentSequenceState,
    arg_ty: InferredTypeData<'db>,
    pending: &mut Vec<ArgumentSequenceState>,
) {
    let Some(parameter) = parameter_for_argument(parameters, state.parameter_index) else {
        return;
    };
    if !argument_satisfies_parameter(db, parameter, arg_ty) {
        return;
    }

    if parameter.is_rest() {
        pending.push(state.next_argument());
    } else {
        pending.push(state.next_parameter(parameter));
    }
}

fn argument_satisfies_parameter<'db>(
    db: &'db dyn ModuleDb,
    parameter: &InferredFunctionParameter<'db>,
    arg_ty: InferredTypeData<'db>,
) -> bool {
    let parameter_ty = parameter_argument_type(db, parameter);
    if !argument_may_match_parameter(db, parameter_ty, arg_ty) {
        return false;
    }

    match (
        parameter_ty.callable_function(db),
        arg_ty.callable_function(db),
    ) {
        (Some(parameter_function), Some(argument_function)) => {
            parameter_function.returns_promise(db) == argument_function.returns_promise(db)
        }
        _ => true,
    }
}

fn remaining_parameters_accept_zero(
    parameters: &[InferredFunctionParameter<'_>],
    parameter_index: usize,
) -> bool {
    parameters
        .iter()
        .skip(parameter_index)
        .all(|parameter| parameter.is_optional() || parameter.is_rest())
}

fn parameter_for_argument<'a, 'db>(
    parameters: &'a [InferredFunctionParameter<'db>],
    index: usize,
) -> Option<&'a InferredFunctionParameter<'db>> {
    parameters
        .get(index)
        .or_else(|| parameters.last().filter(|parameter| parameter.is_rest()))
}

fn parameter_argument_type<'db>(
    db: &'db dyn ModuleDb,
    parameter: &InferredFunctionParameter<'db>,
) -> InferredTypeData<'db> {
    if !parameter.is_rest() {
        return parameter.ty();
    }

    match parameter.ty() {
        InferredTypeData::InstanceOf(instance)
            if instance.ty(db).is_array_class(db) && !instance.type_parameters(db).is_empty() =>
        {
            instance.type_parameters(db)[0]
        }
        ty => ty,
    }
}

fn argument_may_match_parameter<'db>(
    db: &'db dyn ModuleDb,
    parameter_ty: InferredTypeData<'db>,
    arg_ty: InferredTypeData<'db>,
) -> bool {
    let mut pending = Vec::from([Vec::from([(parameter_ty, arg_ty)])]);
    let mut remaining_steps = MAX_ARGUMENT_MATCH_STEPS;

    while let Some(mut required_pairs) = pending.pop() {
        let Some((parameter_ty, arg_ty)) = required_pairs.pop() else {
            return true;
        };
        if remaining_steps == 0 {
            return true;
        }
        remaining_steps -= 1;

        if parameter_ty == arg_ty {
            pending.push(required_pairs);
            continue;
        }
        if let (InferredTypeData::Literal(parameter), InferredTypeData::Literal(argument)) =
            (parameter_ty, arg_ty)
            && !matches!(parameter.literal(db), InferredLiteral::Object(_))
            && !matches!(argument.literal(db), InferredLiteral::Object(_))
        {
            continue;
        }

        let parameter_ty = literal_base_type(db, parameter_ty).unwrap_or(parameter_ty);
        let arg_ty = literal_base_type(db, arg_ty).unwrap_or(arg_ty);

        if parameter_ty == arg_ty {
            pending.push(required_pairs);
            continue;
        }

        match argument_match_action(db, parameter_ty, arg_ty) {
            ArgumentMatchAction::Match => pending.push(required_pairs),
            ArgumentMatchAction::Mismatch => {}
            ArgumentMatchAction::All(pairs) => {
                required_pairs.extend(pairs);
                pending.push(required_pairs);
            }
            ArgumentMatchAction::Any(pairs) => {
                for pair in pairs.into_iter().rev() {
                    let mut branch_pairs = required_pairs.clone();
                    branch_pairs.push(pair);
                    pending.push(branch_pairs);
                }
            }
        }
    }

    false
}

fn argument_match_action<'db>(
    db: &'db dyn ModuleDb,
    parameter_ty: InferredTypeData<'db>,
    arg_ty: InferredTypeData<'db>,
) -> ArgumentMatchAction<'db> {
    match (parameter_ty, arg_ty) {
        (InferredTypeData::Generic(generic), arg_ty) => generic
            .constraint(db)
            .map_or(ArgumentMatchAction::Match, |constraint| {
                ArgumentMatchAction::All(Vec::from([(constraint, arg_ty)]))
            }),
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
        (InferredTypeData::Local(parameter), arg_ty) => {
            match argument_type_extends_parameter_local(db, arg_ty, parameter) {
                Some(true) => ArgumentMatchAction::Match,
                Some(false) => ArgumentMatchAction::Mismatch,
                None => ArgumentMatchAction::Match,
            }
        }
        (parameter_ty, InferredTypeData::Local(argument)) => {
            match argument_local_extends_parameter_type(db, argument, parameter_ty) {
                Some(true) => ArgumentMatchAction::Match,
                Some(false) => ArgumentMatchAction::Mismatch,
                None => ArgumentMatchAction::Match,
            }
        }
        (InferredTypeData::Class(_), InferredTypeData::Class(_)) => {
            if argument_type_extends_parameter_type(db, arg_ty, parameter_ty) {
                ArgumentMatchAction::Match
            } else {
                ArgumentMatchAction::Mismatch
            }
        }
        (InferredTypeData::Union(union), arg_ty) => ArgumentMatchAction::Any(
            union
                .types(db)
                .iter()
                .map(|parameter_ty| (*parameter_ty, arg_ty))
                .collect(),
        ),
        (parameter_ty, InferredTypeData::Union(union)) => ArgumentMatchAction::All(
            union
                .types(db)
                .iter()
                .map(|arg_ty| (parameter_ty, *arg_ty))
                .collect(),
        ),
        (InferredTypeData::Intersection(intersection), arg_ty) => ArgumentMatchAction::All(
            intersection
                .types(db)
                .iter()
                .map(|parameter_ty| (*parameter_ty, arg_ty))
                .collect(),
        ),
        (parameter_ty, InferredTypeData::Intersection(intersection)) => ArgumentMatchAction::Any(
            intersection
                .types(db)
                .iter()
                .map(|arg_ty| (parameter_ty, *arg_ty))
                .collect(),
        ),
        (InferredTypeData::InstanceOf(parameter), InferredTypeData::InstanceOf(argument)) => {
            let mut pairs = Vec::from([(parameter.ty(db), argument.ty(db))]);
            pairs.extend(
                parameter
                    .type_parameters(db)
                    .iter()
                    .zip(argument.type_parameters(db))
                    .map(|(parameter_ty, arg_ty)| (*parameter_ty, *arg_ty)),
            );
            ArgumentMatchAction::All(pairs)
        }
        (InferredTypeData::InstanceOf(parameter), arg_ty) => {
            ArgumentMatchAction::All(Vec::from([(parameter.ty(db), arg_ty)]))
        }
        (parameter_ty, InferredTypeData::InstanceOf(argument)) => {
            ArgumentMatchAction::All(Vec::from([(parameter_ty, argument.ty(db))]))
        }
        (InferredTypeData::MergedReference(reference), arg_ty) => ArgumentMatchAction::Any(
            merged_reference_targets(db, reference)
                .into_iter()
                .map(|parameter_ty| (parameter_ty, arg_ty))
                .collect(),
        ),
        (parameter_ty, InferredTypeData::MergedReference(reference)) => ArgumentMatchAction::Any(
            merged_reference_targets(db, reference)
                .into_iter()
                .map(|arg_ty| (parameter_ty, arg_ty))
                .collect(),
        ),
        (InferredTypeData::TypeofType(parameter), arg_ty) => {
            ArgumentMatchAction::All(Vec::from([(parameter.ty(db), arg_ty)]))
        }
        (parameter_ty, InferredTypeData::TypeofType(argument)) => {
            ArgumentMatchAction::All(Vec::from([(parameter_ty, argument.ty(db))]))
        }
        (InferredTypeData::TypeofValue(parameter), arg_ty) => {
            ArgumentMatchAction::All(Vec::from([(parameter.ty(db), arg_ty)]))
        }
        (parameter_ty, InferredTypeData::TypeofValue(argument)) => {
            ArgumentMatchAction::All(Vec::from([(parameter_ty, argument.ty(db))]))
        }
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
            | InferredTypeData::Divergent(_)
            | InferredTypeData::Global
            | InferredTypeData::Literal(_)
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::TypeofExpression(_),
            _,
        )
        | (
            _,
            InferredTypeData::Conditional
            | InferredTypeData::Divergent(_)
            | InferredTypeData::Global
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

fn spread_argument_element_type<'db>(
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

fn argument_type_extends_parameter_local<'db>(
    db: &'db dyn ModuleDb,
    argument_ty: InferredTypeData<'db>,
    parameter: InferredLocalTypeHandle<'db>,
) -> Option<bool> {
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
                if raw_local_class_name(db, parameter).is_some_and(|parameter_name| {
                    class
                        .name(db)
                        .as_ref()
                        .is_some_and(|argument_name| argument_name.text() == parameter_name)
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
            InferredTypeData::Unknown
            | InferredTypeData::Divergent(_)
            | InferredTypeData::Global
            | InferredTypeData::Conditional
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword => {}
        }
    }

    None
}

fn argument_local_extends_parameter_type<'db>(
    db: &'db dyn ModuleDb,
    argument: InferredLocalTypeHandle<'db>,
    parameter_ty: InferredTypeData<'db>,
) -> Option<bool> {
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
        InferredTypeData::Unknown
        | InferredTypeData::Divergent(_)
        | InferredTypeData::Global
        | InferredTypeData::Conditional
        | InferredTypeData::Constructor(_)
        | InferredTypeData::Function(_)
        | InferredTypeData::Interface(_)
        | InferredTypeData::Module(_)
        | InferredTypeData::Namespace(_)
        | InferredTypeData::Object(_)
        | InferredTypeData::Tuple(_)
        | InferredTypeData::Generic(_)
        | InferredTypeData::Local(_)
        | InferredTypeData::Intersection(_)
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
        | InferredTypeData::UnknownKeyword => None,
    }
}

fn argument_type_extends_parameter_type<'db>(
    db: &'db dyn ModuleDb,
    argument_ty: InferredTypeData<'db>,
    parameter_ty: InferredTypeData<'db>,
) -> bool {
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
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Function(_)
            | InferredTypeData::Interface(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Object(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::Union(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::TypeofType(_)
            | InferredTypeData::TypeofValue(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => {}
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

        if raw_local_class_name(db, local).as_deref() == Some(parameter_name) {
            return true;
        }
        if let Some(extends) = raw_local_class_extends(db, local) {
            pending.push(extends);
        }
    }

    false
}

fn raw_local_class_name<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
) -> Option<String> {
    let module_key = local.module(db);
    let current = module_for_key(db, module_key)?;
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return None;
    };
    let RawTypeData::Class(class) = js_info.raw_types.get(local.type_id(db).index())? else {
        return None;
    };
    class.name.as_ref().map(|name| name.text().to_string())
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
        ReturnType::Type(ty) => Some(infer_generic_return_type(db, function, *ty, args)),
        ReturnType::Predicate(_) | ReturnType::Asserts(_) => Some(InferredTypeData::Boolean),
    }
}

fn infer_generic_return_type<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    mut return_ty: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> InferredTypeData<'db> {
    for (parameter, arg) in function
        .parameters(db)
        .iter()
        .zip(args.iter().copied().map(ResolvedCallArgument::ty))
    {
        let parameter_ty = parameter.ty();
        if parameter_ty.is_generic_reference(db) {
            return_ty = return_ty.substitute_type(
                db,
                InferredTypeSubstitution {
                    generic: parameter_ty,
                    replacement: arg,
                },
            );
            continue;
        }

        let Some(parameter_function) = parameter_ty.callable_function(db) else {
            continue;
        };
        let ReturnType::Type(parameter_return_ty) = parameter_function.return_type(db) else {
            continue;
        };
        let Some(argument_function) = arg.callable_function(db) else {
            continue;
        };
        let ReturnType::Type(argument_return_ty) = argument_function.return_type(db) else {
            continue;
        };

        for substitution in
            collect_callback_return_replacements(db, *parameter_return_ty, *argument_return_ty)
        {
            return_ty = return_ty.substitute_type(db, substitution);
        }
    }

    return_ty
}

/// Determines the value produced by a callback with a generic return type.
///
/// When a callback may return either `T` or `Promise<T>`, a Promise result uses
/// its inner value. This prevents Promise methods from producing
/// `Promise<Promise<T>>`.
fn collect_callback_return_replacements<'db>(
    db: &'db dyn ModuleDb,
    parameter_return_ty: InferredTypeData<'db>,
    argument_return_ty: InferredTypeData<'db>,
) -> Vec<InferredTypeSubstitution<'db>> {
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
                        parameter.collect_generic_replacements(db, argument_return_ty);
                    if !replacements.is_empty() {
                        return replacements;
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
            return Vec::from([InferredTypeSubstitution {
                generic: *generic,
                replacement: argument_return_ty,
            }]);
        }
    }

    parameter_return_ty.collect_generic_replacements(db, argument_return_ty)
}

// #endregion

// #region EXPORTED TRACKED QUERIES

#[salsa::tracked(cycle_result=normalize_type_cycle_result)]
pub fn normalize_type<'db>(
    db: &'db dyn ModuleDb,
    input: NormalizeTypeInput<'db>,
) -> InferredTypeData<'db> {
    let module = input.module(db);
    let ty = input.ty(db);
    let Some(inferred) = infer_module_types(db, module) else {
        return ty;
    };

    let original = ty;
    let mut stack = Vec::from([TypeNormalizationItem::Type(ty)]);
    let mut results = Vec::new();
    let mut active_locals = FxHashSet::default();
    let mut remaining_steps = MAX_TYPE_NORMALIZATION_STEPS;

    while let Some(item) = stack.pop() {
        match item {
            TypeNormalizationItem::Type(ty) => {
                if remaining_steps == 0 {
                    return original;
                }
                remaining_steps -= 1;

                let mut exit_local = None;
                let ty = match ty {
                    InferredTypeData::Local(local) => {
                        let module = local.module(db);
                        let type_id = local.type_id(db);
                        if !active_locals.insert((module, type_id)) {
                            results.push(InferredTypeData::Local(local));
                            continue;
                        }
                        exit_local = Some((module, type_id));
                        inferred.resolve_type(db, ty)
                    }
                    ty @ (InferredTypeData::Unknown
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
                    | InferredTypeData::Object(_)
                    | InferredTypeData::Tuple(_)
                    | InferredTypeData::Generic(_)
                    | InferredTypeData::Intersection(_)
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
                    | InferredTypeData::VoidKeyword) => inferred.resolve_type(db, ty),
                };

                if let Some((module, type_id)) = exit_local {
                    stack.push(TypeNormalizationItem::ExitLocal { module, type_id });
                }

                match ty {
                    InferredTypeData::InstanceOf(instance) => {
                        stack.push(TypeNormalizationItem::RebuildInstance(
                            instance.type_parameters(db).len(),
                        ));
                        for type_parameter in instance.type_parameters(db).iter().rev() {
                            stack.push(TypeNormalizationItem::Type(*type_parameter));
                        }
                        stack.push(TypeNormalizationItem::Type(instance.ty(db)));
                    }
                    InferredTypeData::Intersection(intersection) => {
                        stack.push(TypeNormalizationItem::RebuildIntersection(
                            intersection.types(db).len(),
                        ));
                        for ty in intersection.types(db).iter().rev() {
                            stack.push(TypeNormalizationItem::Type(*ty));
                        }
                    }
                    InferredTypeData::MergedReference(reference) => {
                        let ty = reference.ty(db);
                        let value_ty = reference.value_ty(db);
                        let namespace_ty = reference.namespace_ty(db);
                        stack.push(TypeNormalizationItem::RebuildMergedReference {
                            has_ty: ty.is_some(),
                            has_value_ty: value_ty.is_some(),
                            has_namespace_ty: namespace_ty.is_some(),
                        });
                        if let Some(namespace_ty) = namespace_ty {
                            stack.push(TypeNormalizationItem::Type(namespace_ty));
                        }
                        if let Some(value_ty) = value_ty {
                            stack.push(TypeNormalizationItem::Type(value_ty));
                        }
                        if let Some(ty) = ty {
                            stack.push(TypeNormalizationItem::Type(ty));
                        }
                    }
                    InferredTypeData::TypeofType(ty) => {
                        stack.push(TypeNormalizationItem::Type(ty.ty(db)));
                    }
                    InferredTypeData::TypeofValue(value) => {
                        let ty = value.ty(db);
                        if ty == InferredTypeData::Unknown {
                            results.push(InferredTypeData::TypeofValue(value));
                        } else {
                            stack.push(TypeNormalizationItem::Type(ty));
                        }
                    }
                    InferredTypeData::Tuple(tuple) => {
                        stack.push(TypeNormalizationItem::RebuildTuple(tuple));
                        for element in tuple.elements(db).iter().rev() {
                            stack.push(TypeNormalizationItem::Type(element.ty));
                        }
                    }
                    InferredTypeData::Union(union) => {
                        stack.push(TypeNormalizationItem::RebuildUnion(union.types(db).len()));
                        for ty in union.types(db).iter().rev() {
                            stack.push(TypeNormalizationItem::Type(*ty));
                        }
                    }
                    ty @ (InferredTypeData::Unknown
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
                    | InferredTypeData::Object(_)
                    | InferredTypeData::Generic(_)
                    | InferredTypeData::Local(_)
                    | InferredTypeData::TypeOperator(_)
                    | InferredTypeData::Literal(_)
                    | InferredTypeData::TypeofExpression(_)
                    | InferredTypeData::AnyKeyword
                    | InferredTypeData::NeverKeyword
                    | InferredTypeData::ObjectKeyword
                    | InferredTypeData::ThisKeyword
                    | InferredTypeData::UnknownKeyword
                    | InferredTypeData::VoidKeyword) => results.push(ty),
                }
            }
            TypeNormalizationItem::ExitLocal { module, type_id } => {
                active_locals.remove(&(module, type_id));
            }
            TypeNormalizationItem::RebuildInstance(type_parameter_count) => {
                let Some(start) = results.len().checked_sub(type_parameter_count + 1) else {
                    return original;
                };
                let mut parts = results.split_off(start);
                let ty = parts.remove(0);
                if ty.should_flatten_instance(&parts) {
                    results.push(ty);
                } else {
                    results.push(InferredTypeData::instance_of(
                        db,
                        ty,
                        parts.into_boxed_slice(),
                    ));
                }
            }
            TypeNormalizationItem::RebuildIntersection(type_count) => {
                let Some(start) = results.len().checked_sub(type_count) else {
                    return original;
                };
                let types = results.split_off(start);
                results.push(InferredTypeData::intersection_from_types(db, types));
            }
            TypeNormalizationItem::RebuildMergedReference {
                has_ty,
                has_value_ty,
                has_namespace_ty,
            } => {
                let target_count = [has_ty, has_value_ty, has_namespace_ty]
                    .into_iter()
                    .filter(|has_target| *has_target)
                    .count();
                let Some(start) = results.len().checked_sub(target_count) else {
                    return original;
                };
                let mut targets = results.split_off(start).into_iter();
                let ty = has_ty.then(|| targets.next().unwrap_or(InferredTypeData::Unknown));
                let value_ty =
                    has_value_ty.then(|| targets.next().unwrap_or(InferredTypeData::Unknown));
                let namespace_ty =
                    has_namespace_ty.then(|| targets.next().unwrap_or(InferredTypeData::Unknown));
                let target_values = [ty, value_ty, namespace_ty]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                if let Some(first) = target_values.first().copied()
                    && target_values.iter().all(|target| *target == first)
                {
                    results.push(first);
                } else {
                    results.push(InferredTypeData::MergedReference(
                        InferredMergedReference::new(db, ty, value_ty, namespace_ty),
                    ));
                }
            }
            TypeNormalizationItem::RebuildTuple(tuple) => {
                let Some(start) = results.len().checked_sub(tuple.elements(db).len()) else {
                    return original;
                };
                let types = results.split_off(start);
                let elements = tuple
                    .elements(db)
                    .iter()
                    .zip(types)
                    .map(|(element, ty)| {
                        let mut element = element.clone();
                        element.ty = ty;
                        element
                    })
                    .collect::<Vec<_>>();
                results.push(InferredTypeData::Tuple(InferredTuple::new(
                    db,
                    elements.into_boxed_slice(),
                )));
            }
            TypeNormalizationItem::RebuildUnion(type_count) => {
                let Some(start) = results.len().checked_sub(type_count) else {
                    return original;
                };
                let types = results.split_off(start);
                results.push(InferredTypeData::union_from_types(db, types));
            }
        }
    }

    results.pop().unwrap_or(original)
}

// #endregion

// #region QUERY HELPER FUNCTIONS

#[derive(Clone, Copy, Debug)]
pub(crate) enum ResolvedCallArgument<'db> {
    Argument(InferredTypeData<'db>),
    Optional(InferredTypeData<'db>),
    Spread(InferredTypeData<'db>),
}

impl<'db> ResolvedCallArgument<'db> {
    pub(crate) fn ty(self) -> InferredTypeData<'db> {
        match self {
            Self::Argument(ty) | Self::Optional(ty) | Self::Spread(ty) => ty,
        }
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct ArgumentSequenceState {
    parameter_index: usize,
    argument_index: usize,
    matched_parameter_index: Option<usize>,
}

impl ArgumentSequenceState {
    fn next_parameter(mut self, parameter: &InferredFunctionParameter<'_>) -> Self {
        if !parameter.is_rest() {
            self.parameter_index += 1;
        }
        self
    }

    fn next_argument(mut self) -> Self {
        self.argument_index += 1;
        self
    }
}

enum ArgumentMatchAction<'db> {
    Match,
    Mismatch,
    All(Vec<(InferredTypeData<'db>, InferredTypeData<'db>)>),
    Any(Vec<(InferredTypeData<'db>, InferredTypeData<'db>)>),
}

#[derive(Clone, Copy, Debug)]
enum TypeNormalizationItem<'db> {
    Type(InferredTypeData<'db>),
    ExitLocal {
        module: InferredModuleKey,
        type_id: InferredLocalTypeId,
    },
    RebuildInstance(usize),
    RebuildIntersection(usize),
    RebuildMergedReference {
        has_ty: bool,
        has_value_ty: bool,
        has_namespace_ty: bool,
    },
    RebuildTuple(InferredTuple<'db>),
    RebuildUnion(usize),
}

// #endregion

// #region INTERNED TYPES

#[salsa::interned]
#[derive(Debug)]
pub struct CallExpressionTypeInput<'db> {
    pub module: ModuleInfo,
    pub callee: InferredTypeData<'db>,
    #[returns(ref)]
    pub args: Box<[InferredTypeData<'db>]>,
}

/// The data needed to find the parameter type for one call argument.
#[salsa::interned]
#[derive(Debug)]
pub struct CallArgumentTypeInput<'db> {
    pub callee: InferredTypeData<'db>,
    #[returns(ref)]
    pub args: Box<[InferredCallArgumentType<'db>]>,
    pub argument_index: usize,
}

#[salsa::interned]
#[derive(Debug)]
pub struct NormalizeTypeInput<'db> {
    pub module: ModuleInfo,
    pub ty: InferredTypeData<'db>,
}

// #endregion
