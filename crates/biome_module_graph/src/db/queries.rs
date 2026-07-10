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

use crate::css_module_info::traverse::{CssClassStep, ImportTreeTraversal};
use crate::db::type_inference::{
    collected_type_result, infer_module_types_cycle_result, normalize_type_cycle_result,
    resolve_raw_types,
};
use crate::module_graph::{ModuleInfo, ModuleInfoKind};
use crate::{ImportTreeNode, JsExport, JsOwnExport, ModuleDb, ResolvedPath};
use biome_css_syntax::{TextRange, TextSize};
use biome_js_type_info::{
    ImportSymbol, RawTypeData, TypeReference, TypeResolverLevel,
    interned_types::{
        FunctionParameter as InferredFunctionParameter, InternedFunction as InferredFunction,
        InternedMergedReference as InferredMergedReference, Literal as InferredLiteral,
        LocalTypeHandle as InferredLocalTypeHandle, LocalTypeId as InferredLocalTypeId,
        ModuleKey as InferredModuleKey, ReturnType, TypeData as InferredTypeData,
        TypeMember as InferredTypeMember, TypeSubstitution as InferredTypeSubstitution,
    },
};
use biome_jsdoc_comment::JsdocComment;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};
use salsa::plumbing::{AsId, FromId};
use std::collections::VecDeque;

pub use crate::db::type_inference::InferredModuleTypes;

const MAX_TYPE_NORMALIZATION_STEPS: usize = 1024;
const MAX_ARGUMENT_MATCH_STEPS: usize = 1024;
const MAX_ARGUMENT_SEQUENCE_STEPS: usize = 1024;
const MAX_LOCAL_EXTENDS_STEPS: usize = 1024;

#[salsa::tracked(cycle_result=infer_module_types_cycle_result)]
#[deny(clippy::wildcard_enum_match_arm)]
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
#[deny(clippy::wildcard_enum_match_arm)]
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

#[salsa::interned]
#[derive(Debug)]
pub struct CallExpressionTypeInput<'db> {
    pub module: ModuleInfo,
    pub callee: InferredTypeData<'db>,
    #[returns(ref)]
    pub args: Box<[InferredTypeData<'db>]>,
}

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

#[salsa::tracked]
#[deny(clippy::wildcard_enum_match_arm)]
pub fn infer_call_expression_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallExpressionTypeInput<'db>,
) -> InferredTypeData<'db> {
    let module = input.module(db);
    let callee = input.callee(db);
    let args = input.args(db);
    let ty = infer_call_expression_return_type(db, callee, args);

    normalize_type(db, NormalizeTypeInput::new(db, module, ty))
}

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

#[deny(clippy::wildcard_enum_match_arm)]
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
                .filter_map(|callee| infer_function_call_type(db, *callee, args))
                .collect(),
        )
        .unwrap_or(InferredTypeData::Unknown),
        callee @ (InferredTypeData::Unknown
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
        | InferredTypeData::Local(_)
        | InferredTypeData::Intersection(_)
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
        | InferredTypeData::VoidKeyword) => {
            infer_function_call_type(db, callee, args).unwrap_or(InferredTypeData::Unknown)
        }
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
fn infer_function_call_type<'db>(
    db: &'db dyn ModuleDb,
    callee: InferredTypeData<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredTypeData<'db>> {
    match callee {
        InferredTypeData::Function(function) => infer_function_return_type(db, function, args),
        InferredTypeData::InstanceOf(instance) => match instance.ty(db) {
            InferredTypeData::Function(function) => infer_function_return_type(db, function, args),
            InferredTypeData::Interface(interface) => {
                infer_call_signature_type(db, interface.members(db), args)
            }
            InferredTypeData::Object(object) => {
                infer_call_signature_type(db, object.members(db), args)
            }
            InferredTypeData::Union(union) => {
                infer_function_call_type(db, InferredTypeData::Union(union), args)
            }
            InferredTypeData::TypeofType(typeof_type) => {
                infer_function_call_type(db, typeof_type.ty(db), args)
            }
            InferredTypeData::TypeofValue(typeof_value) => {
                infer_function_call_type(db, typeof_value.ty(db), args)
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
            | InferredTypeData::Class(_)
            | InferredTypeData::Constructor(_)
            | InferredTypeData::Module(_)
            | InferredTypeData::Namespace(_)
            | InferredTypeData::Tuple(_)
            | InferredTypeData::Generic(_)
            | InferredTypeData::Local(_)
            | InferredTypeData::Intersection(_)
            | InferredTypeData::TypeOperator(_)
            | InferredTypeData::Literal(_)
            | InferredTypeData::InstanceOf(_)
            | InferredTypeData::MergedReference(_)
            | InferredTypeData::TypeofExpression(_)
            | InferredTypeData::AnyKeyword
            | InferredTypeData::NeverKeyword
            | InferredTypeData::ObjectKeyword
            | InferredTypeData::ThisKeyword
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::VoidKeyword => None,
        },
        InferredTypeData::Interface(interface) => {
            infer_call_signature_type(db, interface.members(db), args)
        }
        InferredTypeData::Object(object) => infer_call_signature_type(db, object.members(db), args),
        InferredTypeData::Union(union) => collected_type_result(
            db,
            union
                .types(db)
                .iter()
                .filter_map(|callee| infer_function_call_type(db, *callee, args))
                .collect(),
        ),
        InferredTypeData::TypeofType(typeof_type) => {
            infer_function_call_type(db, typeof_type.ty(db), args)
        }
        InferredTypeData::TypeofValue(typeof_value) => {
            infer_function_call_type(db, typeof_value.ty(db), args)
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
        | InferredTypeData::Class(_)
        | InferredTypeData::Constructor(_)
        | InferredTypeData::Module(_)
        | InferredTypeData::Namespace(_)
        | InferredTypeData::Tuple(_)
        | InferredTypeData::Generic(_)
        | InferredTypeData::Local(_)
        | InferredTypeData::Intersection(_)
        | InferredTypeData::TypeOperator(_)
        | InferredTypeData::Literal(_)
        | InferredTypeData::MergedReference(_)
        | InferredTypeData::TypeofExpression(_)
        | InferredTypeData::AnyKeyword
        | InferredTypeData::NeverKeyword
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::ThisKeyword
        | InferredTypeData::UnknownKeyword
        | InferredTypeData::VoidKeyword => None,
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
fn infer_call_signature_type<'db>(
    db: &'db dyn ModuleDb,
    members: &[InferredTypeMember<'db>],
    args: &[ResolvedCallArgument<'db>],
) -> Option<InferredTypeData<'db>> {
    let signatures = members
        .iter()
        .filter(|member| member.kind.is_call_signature())
        .filter_map(|member| member.ty.callable_function(db))
        .collect::<Vec<_>>();

    if signatures.len() < 2 {
        return signatures
            .first()
            .and_then(|function| infer_function_return_type(db, *function, args));
    }

    signatures
        .into_iter()
        .find(|function| signature_accepts_arguments(db, *function, args))
        .and_then(|function| infer_function_return_type(db, function, args))
}

#[deny(clippy::wildcard_enum_match_arm)]
fn signature_accepts_arguments<'db>(
    db: &'db dyn ModuleDb,
    function: InferredFunction<'db>,
    args: &[ResolvedCallArgument<'db>],
) -> bool {
    let parameters = function.parameters(db);
    let mut pending = Vec::from([(0, 0)]);
    let mut seen = FxHashSet::default();

    for _ in 0..MAX_ARGUMENT_SEQUENCE_STEPS {
        let Some((parameter_index, argument_index)) = pending.pop() else {
            return false;
        };
        if !seen.insert((parameter_index, argument_index)) {
            continue;
        }

        let Some(argument) = args.get(argument_index).copied() else {
            if remaining_parameters_accept_zero(parameters, parameter_index) {
                return true;
            }
            continue;
        };

        match argument {
            ResolvedCallArgument::Argument(arg_ty) => push_consumed_argument_state(
                db,
                parameters,
                parameter_index,
                argument_index + 1,
                arg_ty,
                &mut pending,
            ),
            ResolvedCallArgument::Optional(arg_ty) => {
                pending.push((parameter_index, argument_index + 1));
                push_consumed_argument_state(
                    db,
                    parameters,
                    parameter_index,
                    argument_index + 1,
                    arg_ty,
                    &mut pending,
                );
            }
            ResolvedCallArgument::Spread(arg_ty) => {
                pending.push((parameter_index, argument_index + 1));
                let arg_ty = spread_argument_element_type(db, arg_ty);
                push_consumed_spread_state(
                    db,
                    parameters,
                    parameter_index,
                    argument_index,
                    arg_ty,
                    &mut pending,
                );
            }
        }
    }

    true
}

#[deny(clippy::wildcard_enum_match_arm)]
fn push_consumed_argument_state<'db>(
    db: &'db dyn ModuleDb,
    parameters: &'db [InferredFunctionParameter<'db>],
    parameter_index: usize,
    next_argument_index: usize,
    arg_ty: InferredTypeData<'db>,
    pending: &mut Vec<(usize, usize)>,
) {
    let Some(parameter) = parameter_for_argument(parameters, parameter_index) else {
        return;
    };
    if !argument_satisfies_parameter(db, parameter, arg_ty) {
        return;
    }

    pending.push((
        next_parameter_index(parameter, parameter_index),
        next_argument_index,
    ));
}

#[deny(clippy::wildcard_enum_match_arm)]
fn push_consumed_spread_state<'db>(
    db: &'db dyn ModuleDb,
    parameters: &'db [InferredFunctionParameter<'db>],
    parameter_index: usize,
    argument_index: usize,
    arg_ty: InferredTypeData<'db>,
    pending: &mut Vec<(usize, usize)>,
) {
    let Some(parameter) = parameter_for_argument(parameters, parameter_index) else {
        return;
    };
    if !argument_satisfies_parameter(db, parameter, arg_ty) {
        return;
    }

    if parameter.is_rest() {
        pending.push((parameter_index, argument_index + 1));
    } else {
        pending.push((parameter_index + 1, argument_index));
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
fn remaining_parameters_accept_zero<'db>(
    parameters: &'db [InferredFunctionParameter<'db>],
    parameter_index: usize,
) -> bool {
    parameters
        .iter()
        .skip(parameter_index)
        .all(|parameter| parameter.is_optional() || parameter.is_rest())
}

#[deny(clippy::wildcard_enum_match_arm)]
fn parameter_for_argument<'db>(
    parameters: &'db [InferredFunctionParameter<'db>],
    index: usize,
) -> Option<&'db InferredFunctionParameter<'db>> {
    parameters
        .get(index)
        .or_else(|| parameters.last().filter(|parameter| parameter.is_rest()))
}

#[deny(clippy::wildcard_enum_match_arm)]
fn next_parameter_index<'db>(parameter: &InferredFunctionParameter<'db>, index: usize) -> usize {
    if parameter.is_rest() {
        index
    } else {
        index + 1
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
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
        | InferredTypeData::UnknownKeyword
        | InferredTypeData::VoidKeyword) => ty,
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
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

enum ArgumentMatchAction<'db> {
    Match,
    Mismatch,
    All(Vec<(InferredTypeData<'db>, InferredTypeData<'db>)>),
    Any(Vec<(InferredTypeData<'db>, InferredTypeData<'db>)>),
}

#[deny(clippy::wildcard_enum_match_arm)]
fn argument_match_action<'db>(
    db: &'db dyn ModuleDb,
    parameter_ty: InferredTypeData<'db>,
    arg_ty: InferredTypeData<'db>,
) -> ArgumentMatchAction<'db> {
    match (parameter_ty, arg_ty) {
        (
            InferredTypeData::AnyKeyword
            | InferredTypeData::Unknown
            | InferredTypeData::UnknownKeyword
            | InferredTypeData::Generic(_)
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
        (parameter_ty, InferredTypeData::Union(union)) => ArgumentMatchAction::Any(
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

#[deny(clippy::wildcard_enum_match_arm)]
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
        ty @ (InferredTypeData::Divergent(_)
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
        | InferredTypeData::Intersection(_)
        | InferredTypeData::Union(_)
        | InferredTypeData::TypeOperator(_)
        | InferredTypeData::Literal(_)
        | InferredTypeData::InstanceOf(_)
        | InferredTypeData::MergedReference(_)
        | InferredTypeData::TypeofExpression(_)
        | InferredTypeData::TypeofType(_)
        | InferredTypeData::TypeofValue(_)
        | InferredTypeData::NeverKeyword
        | InferredTypeData::ObjectKeyword
        | InferredTypeData::ThisKeyword
        | InferredTypeData::VoidKeyword) => ty,
    }
}

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
fn raw_local_class_name<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
) -> Option<String> {
    let module_key = local.module(db);
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    if InferredModuleKey::new(current.as_id()) != module_key {
        return None;
    }
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return None;
    };
    let RawTypeData::Class(class) = js_info.raw_types.get(local.type_id(db).index())? else {
        return None;
    };
    class.name.as_ref().map(|name| name.text().to_string())
}

#[deny(clippy::wildcard_enum_match_arm)]
fn raw_local_class_extends<'db>(
    db: &'db dyn ModuleDb,
    local: InferredLocalTypeHandle<'db>,
) -> Option<InferredLocalTypeHandle<'db>> {
    let module_key = local.module(db);
    let module = ModuleInfo::from_id(module_key.as_id());
    let current = db.module_for_path(module.path(db))?;
    if InferredModuleKey::new(current.as_id()) != module_key {
        return None;
    };
    let ModuleInfoKind::Js(js_info) = current.kind(db) else {
        return None;
    };
    let RawTypeData::Class(class) = js_info.raw_types.get(local.type_id(db).index())? else {
        return None;
    };
    local_handle_from_reference(db, module_key, class.extends.as_ref()?)
}

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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

#[deny(clippy::wildcard_enum_match_arm)]
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
            parameter_return_ty.collect_generic_replacements(db, *argument_return_ty)
        {
            return_ty = return_ty.substitute_type(db, substitution);
        }
    }

    return_ty
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
    RebuildUnion(usize),
}

#[salsa::interned]
#[derive(Debug)]
pub struct NormalizeTypeInput<'db> {
    pub module: ModuleInfo,
    pub ty: InferredTypeData<'db>,
}

#[salsa::tracked(cycle_result=normalize_type_cycle_result)]
#[deny(clippy::wildcard_enum_match_arm)]
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
                    | InferredTypeData::Tuple(_)
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

/// Returns CSS class steps for a JS module by traversing its direct CSS imports.
///
/// Tracked: depends on `js_module_info(db, module)` and on the CSS modules it
/// imports. If any of those change, this recomputes.
#[salsa::tracked(no_eq, returns(deref))]
pub fn css_classes_for_module(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<CssClassStep> {
    let module_kind = module.kind(db);
    let Some(js_info) = module_kind.as_js_module_info() else {
        return Vec::new();
    };

    let mut results = Vec::new();
    for import_path in js_info.static_import_paths.values() {
        if let Some(path) = import_path.as_path()
            && let Some(target) = db.module_for_path(path)
            && let ModuleInfoKind::Css(css_info) = target.kind(db)
        {
            results.push(CssClassStep {
                css_path: path.to_path_buf(),
                css_classes: css_info.classes.clone(),
            });
        }
    }

    results
}

/// Returns all files that transitively import `path` (through CSS `@import`
/// chains and HTML `<link>` references).
///
/// The returned set includes only JS/HTML files (potential class consumers),
/// not intermediate CSS files.
#[salsa::tracked(returns(deref))]
pub fn transitive_importers_of(db: &dyn ModuleDb, module: ModuleInfo) -> Vec<Utf8PathBuf> {
    let mut result = Vec::new();
    let mut visited: FxHashSet<Utf8PathBuf> = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(module.path(db).to_path_buf());

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }

        db.for_each_module(&mut |file_path, module_info| {
            if file_path == current.as_path() {
                return;
            }
            let imports_current = match module_info {
                ModuleInfoKind::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                    .any(|p| p.as_path() == Some(current.as_path())),
                ModuleInfoKind::Css(css_info) => css_info
                    .imports
                    .values()
                    .any(|p| p.resolved_path.as_path() == Some(current.as_path())),
                ModuleInfoKind::Html(html_info) => {
                    html_info
                        .imported_stylesheets
                        .iter()
                        .any(|p| p.as_path() == Some(current.as_path()))
                        || html_info
                            .static_import_paths
                            .values()
                            .any(|p| p.as_path() == Some(current.as_path()))
                        || html_info
                            .dynamic_import_paths
                            .values()
                            .any(|p| p.as_path() == Some(current.as_path()))
                }
            };

            if imports_current && !visited.contains(file_path) {
                match module_info {
                    ModuleInfoKind::Js(_) | ModuleInfoKind::Html(_) => {
                        result.push(file_path.to_path_buf());
                    }
                    ModuleInfoKind::Css(_) => {
                        queue.push_back(file_path.to_path_buf());
                    }
                }
            }
        });
    }

    result
}

/// Returns CSS class steps for the given JS file by traversing its imports.
#[salsa::tracked(returns(deref))]
pub fn traverse_import_tree_for_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut results = Vec::new();

    if let Some(js_info) = db.js_module_info_for_path(module.path(db)) {
        for import_path in js_info.static_import_paths.values() {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                results.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }
    }

    let stack = vec![module.path(db).to_path_buf()];
    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());

    let traversal = ImportTreeTraversal {
        module_database: db,
        stack,
        visited,
        current_css_iter: None,
    };
    results.extend(traversal);
    results
}

/// Returns CSS class steps for the given HTML file.
#[salsa::tracked(returns(deref))]
pub fn traverse_import_tree_for_html_classes(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Vec<CssClassStep> {
    let mut inline_steps = Vec::new();
    let mut linked_steps = Vec::new();

    if let Some(html_info) = db.html_module_info_for_path(module.path(db)) {
        let all_inline_classes: IndexMap<_, _> = html_info
            .style_classes
            .iter()
            .map(|c| (c.range, c.name.clone()))
            .collect();
        if !all_inline_classes.is_empty() {
            inline_steps.push(CssClassStep {
                css_path: module.path(db).to_path_buf(),
                css_classes: all_inline_classes,
            });
        }

        for stylesheet_path in &html_info.imported_stylesheets {
            if let Some(path) = stylesheet_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                linked_steps.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }

        for import_path in html_info
            .static_import_paths
            .values()
            .chain(html_info.dynamic_import_paths.values())
        {
            if let Some(path) = import_path.as_path()
                && let Some(css_info) = db.css_module_info_for_path(path)
            {
                linked_steps.push(CssClassStep {
                    css_path: path.to_path_buf(),
                    css_classes: css_info.classes.clone(),
                });
            }
        }
    }

    let stack = vec![module.path(db).to_path_buf()];
    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());

    inline_steps
        .into_iter()
        .chain(linked_steps)
        .chain(ImportTreeTraversal {
            module_database: db,
            stack,
            visited,
            current_css_iter: None,
        })
        .collect()
}

#[salsa::interned]
/// Generic symbol used by queries to track a generic "symbol", which can represent everything (variable name, class name, etc.)
pub struct SymbolFromModuleInfo {
    #[returns(clone)]
    name: String,

    #[returns(ref)]
    module: ModuleInfo,
}

/// Finds the default exported symbol.
#[salsa::tracked]
pub fn find_js_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Option<JsOwnExport> {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![symbol];

    while let Some(symbol) = stack.pop() {
        let ModuleInfoKind::Js(module) = symbol.module(db).kind(db) else {
            continue;
        };
        match &module.exports.get(symbol.name(db).as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return Some(own_export.clone());
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => break,
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.module_for_path(path) {
                                    stack.push(SymbolFromModuleInfo::new(
                                        db,
                                        lookup.clone(),
                                        module,
                                    ));
                                }
                            }
                            _ => break,
                        }
                    }
                    ImportSymbol::Default => {
                        if let Ok(path) = reexport.import.resolved_path.as_deref()
                            && seen_paths.insert(path.to_path_buf())
                            && let Some(module) = db.module_for_path(path)
                        {
                            stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                        }
                    }
                }
            }
            None => {
                for reexport in module.blanket_reexports.iter() {
                    if let Ok(path) = reexport.import.resolved_path.as_deref()
                        && seen_paths.insert(path.to_path_buf())
                        && let Some(module) = db.module_for_path(path)
                    {
                        stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                    }
                }
            }
        }
    }

    None
}

/// Returns `true` if the given CSS `class_name` is referenced in any
/// JS or HTML file that transitively imports `css_path`.
#[salsa::tracked]
pub fn is_class_referenced_by_importers<'db>(
    db: &'db dyn ModuleDb,
    class_name: SymbolFromModuleInfo<'db>,
) -> bool {
    let importers = transitive_importers_of(db, *class_name.module(db));

    for importer_path in importers {
        let Some(module) = db.module_for_path(importer_path) else {
            continue;
        };
        if is_class_used_in_component_tree(db, module, class_name) {
            return true;
        }
    }
    false
}

/// Checks if a class is used in a file or any of its imported components (transitively).
#[salsa::tracked]
fn is_class_used_in_component_tree<'db>(
    db: &'db dyn ModuleDb,
    module: ModuleInfo,
    class_name: SymbolFromModuleInfo<'db>,
) -> bool {
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back(module);

    while let Some(module) = queue.pop_front() {
        if !visited.insert(module) {
            continue;
        }

        match module.kind(db) {
            ModuleInfoKind::Js(js_info) => {
                if js_info
                    .referenced_classes
                    .iter()
                    .any(|r| r.matches(class_name.name(db).as_str()))
                {
                    return true;
                }
                for import_path in js_info
                    .static_import_paths
                    .values()
                    .chain(js_info.dynamic_import_paths.values())
                {
                    if let Some(path) = import_path.as_path()
                        && let Some(module) = db.module_for_path(path)
                    {
                        queue.push_back(module);
                    }
                }
            }
            ModuleInfoKind::Html(html_info) => {
                if html_info
                    .referenced_classes
                    .iter()
                    .any(|r| r.matches(class_name.name(db).as_str()))
                {
                    return true;
                }
            }
            ModuleInfoKind::Css(_) => {}
        }
    }

    false
}

/// Finds JSDoc for an exported symbol by `name`, following re-exports through the db.
#[salsa::tracked(returns(ref))]
pub fn find_jsdoc_for_exported_symbol<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Option<JsdocComment> {
    let mut seen_paths = std::collections::BTreeSet::new();
    let mut stack = vec![symbol];

    while let Some(symbol) = stack.pop() {
        let ModuleInfoKind::Js(module) = symbol.module(db).kind(db) else {
            continue;
        };
        match &module.exports.get(symbol.name(db).as_str()) {
            Some(JsExport::Own(own_export) | JsExport::OwnType(own_export)) => {
                return match own_export {
                    JsOwnExport::Binding(binding_range) => module
                        .semantic_model
                        .as_binding_by_range(*binding_range)
                        .and_then(|binding| binding.jsdoc().cloned()),
                    JsOwnExport::Type(_) => None,
                    JsOwnExport::Namespace(reexport) => reexport
                        .export_range
                        .and_then(|range| module.semantic_model.export_jsdoc(range).cloned()),
                };
            }
            Some(JsExport::Reexport(reexport) | JsExport::ReexportType(reexport)) => {
                match &reexport.import.symbol {
                    ImportSymbol::All => break,
                    ImportSymbol::Named(source_name) => {
                        let lookup = source_name.text().to_string();
                        match reexport.import.resolved_path.as_deref() {
                            Ok(path) if seen_paths.insert(path.to_path_buf()) => {
                                if let Some(module) = db.module_for_path(path) {
                                    stack.push(SymbolFromModuleInfo::new(
                                        db,
                                        lookup.clone(),
                                        module,
                                    ));
                                }
                            }
                            _ => break,
                        }
                    }
                    ImportSymbol::Default => {
                        if let Ok(path) = reexport.import.resolved_path.as_deref()
                            && let Some(module) = db.module_for_path(path)
                        {
                            stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                        }
                    }
                }
            }
            None => {
                for reexport in module.blanket_reexports.iter() {
                    if let Ok(path) = reexport.import.resolved_path.as_deref()
                        && seen_paths.insert(path.to_path_buf())
                        && let Some(module) = db.module_for_path(path)
                    {
                        stack.push(SymbolFromModuleInfo::new(db, symbol.name(db), module));
                    }
                }
            }
        }
    }

    None
}

/// Finds the CSS file and text range where a class is defined.
#[salsa::tracked]
pub fn find_css_class_definition<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = Vec::new();
    let module = symbol.module(db);
    let mut visited_css = FxHashSet::default();
    // 1. Check inline style classes in HTML-like files (carry content_offset)
    if let ModuleInfoKind::Html(html_info) = module.kind(db) {
        for class_def in &html_info.style_classes {
            if class_def.name.text() == symbol.name(db) {
                result.push((
                    symbol.module(db).path(db).to_path_buf(),
                    class_def.range,
                    class_def.content_offset,
                ));
            }
        }
    }

    // 2. Check CSS files reachable from HTML (linked stylesheets + script imports)
    for step in traverse_import_tree_for_html_classes(db, *module) {
        if &step.css_path == module.path(db) {
            continue; // Already checked inline styles above
        }
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };

        let this_result = search_css_class_transitive(
            db,
            SymbolFromModuleInfo::new(db, symbol.name(db), module),
            &mut visited_css,
        );
        result.extend(this_result);
    }

    // 3. Check CSS files imported by JS (e.g., `import './styles.css'` in JSX)
    for step in traverse_import_tree_for_classes(db, *module) {
        let Some(module) = db.module_for_path(&step.css_path) else {
            continue;
        };
        let this_result = search_css_class_transitive(
            db,
            SymbolFromModuleInfo::new(db, symbol.name(db), module),
            &mut visited_css,
        );

        result.extend(this_result);
    }

    result
}

fn search_css_class_transitive<'db>(
    db: &'db dyn ModuleDb,
    symbol: SymbolFromModuleInfo<'db>,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<(Utf8PathBuf, TextRange, Option<TextSize>)> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(*symbol.module(db));

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.path(db).to_path_buf()) {
            continue;
        }

        let ModuleInfoKind::Css(css_info) = current.kind(db) else {
            continue;
        };

        for (range, token) in css_info.classes.iter() {
            if token.text() == symbol.name(db) {
                result.push((current.path(db).to_path_buf(), *range, None));
            }
        }

        // Follow @import edges
        for import in css_info.imports.values() {
            if let Some(imported_path) = import.resolved_path.as_path()
                && let Some(module) = db.module_for_path(imported_path)
            {
                queue.push_back(module);
            }
        }
    }

    result
}

/// Builds a tree structure representing the import relationships for diagnostic display.
#[salsa::tracked]
pub fn build_import_tree_for_js(db: &dyn ModuleDb, module: ModuleInfo) -> Option<ImportTreeNode> {
    let mut root = ImportTreeNode {
        file_path: module.path(db).to_path_buf(),
        css_imports: Vec::new(),
        parent_components: Vec::new(),
    };

    if let Some(js_info) = module.kind(db).as_js_module_info() {
        root.css_imports = js_info
            .static_import_paths
            .values()
            .filter_map(|import_path| {
                let path = import_path.as_path()?;
                db.css_module_info_for_path(path)?;
                Some(path.to_path_buf())
            })
            .collect();
    } else {
        return None;
    }

    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());
    root.parent_components = build_parent_nodes(db, module.path(db), &mut visited);

    Some(root)
}

/// Builds a tree structure for an HTML file's import relationships (diagnostic display).
#[salsa::tracked]
pub fn build_import_tree_for_html(db: &dyn ModuleDb, module: ModuleInfo) -> Option<ImportTreeNode> {
    let html_info = module.kind(db);
    let html_info = html_info.as_html_module_info()?;

    let css_imports: Vec<_> = html_info
        .imported_stylesheets
        .iter()
        .chain(html_info.static_import_paths.values())
        .filter_map(|stylesheet_path| {
            let path = stylesheet_path.as_path()?;
            db.css_module_info_for_path(path)?;
            Some(path.to_path_buf())
        })
        .collect();

    let mut root = ImportTreeNode {
        file_path: module.path(db).to_path_buf(),
        css_imports,
        parent_components: Vec::new(),
    };

    let mut visited = FxHashSet::default();
    visited.insert(module.path(db).to_path_buf());
    root.parent_components = build_parent_nodes(db, module.path(db), &mut visited);

    Some(root)
}

pub(crate) fn build_parent_nodes(
    db: &dyn ModuleDb,
    current_path: &Utf8Path,
    visited: &mut FxHashSet<Utf8PathBuf>,
) -> Vec<ImportTreeNode> {
    let all_modules = db.all_modules();
    let mut parents = Vec::new();

    for (file_path, module_info) in &all_modules {
        if visited.contains(file_path.as_path()) {
            continue;
        }

        let imports_current = match module_info {
            ModuleInfoKind::Js(js_info) => js_info
                .static_import_paths
                .values()
                .chain(js_info.dynamic_import_paths.values())
                .any(|p| p.as_path() == Some(current_path)),
            ModuleInfoKind::Html(html_info) => html_info
                .imported_stylesheets
                .iter()
                .chain(html_info.static_import_paths.values())
                .chain(html_info.dynamic_import_paths.values())
                .any(|p| p.as_path() == Some(current_path)),
            ModuleInfoKind::Css(_) => false,
        };

        if imports_current {
            let css_imports: Vec<Utf8PathBuf> = match module_info {
                ModuleInfoKind::Js(js_info) => js_info
                    .static_import_paths
                    .values()
                    .filter_map(|import_path| {
                        let path = import_path.as_path()?;
                        db.css_module_info_for_path(path)?;
                        Some(path.to_path_buf())
                    })
                    .collect(),
                ModuleInfoKind::Html(html_info) => html_info
                    .imported_stylesheets
                    .iter()
                    .chain(html_info.static_import_paths.values())
                    .chain(html_info.dynamic_import_paths.values())
                    .filter_map(|stylesheet_path| {
                        let path = stylesheet_path.as_path()?;
                        db.css_module_info_for_path(path)?;
                        Some(path.to_path_buf())
                    })
                    .collect(),
                ModuleInfoKind::Css(_) => Vec::new(),
            };

            let mut branch_visited = visited.clone();
            branch_visited.insert(file_path.clone());

            let parent_components = build_parent_nodes(db, file_path, &mut branch_visited);

            parents.push(ImportTreeNode {
                file_path: file_path.clone(),
                css_imports,
                parent_components,
            });
        }
    }

    parents
}
