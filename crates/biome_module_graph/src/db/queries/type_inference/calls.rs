//! Call and constructor inference queries.
//!
//! These tracked entry points map Salsa inputs to the call-inference algorithms
//! in [`implementation`]. Keeping the algorithms in a child module leaves this
//! file as the incremental boundary for the call query family.

mod implementation;

pub(in crate::db) use implementation::{
    ResolvedCallArgument, infer_call_expression_return_type_from_args,
};

use self::implementation::{
    infer_call_expression_return_type, infer_constructor_argument_type_inner,
    infer_function_argument_type, resolved_call_arguments,
};
use super::{CallArgumentTypeInput, CallExpressionTypeInput, NormalizeTypeInput, normalize_type};
use crate::ModuleDb;
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use biome_js_type_info::{global_types, interned_types::TypeData as InferredTypeData};

// #region CALL INFERENCE QUERIES

/// Infers the return type of a call expression.
///
/// The callee and final return type are normalized in `input.module`. The
/// query accepts plain positional arguments and supports functions, callable
/// interfaces and objects, and unions of callable types. Overloads are tested
/// in declaration order. If no supported signature matches, the result is
/// `Unknown`.
///
/// ```ts
/// declare function parse(value: string): number;
/// const result = parse("1"); // number
/// ```
#[salsa::tracked]
pub fn infer_call_expression_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallExpressionTypeInput<'db>,
) -> InferredTypeData<'db> {
    execute_query(
        TypeInferenceQueryKind::Calls,
        TypeInferenceProfileOrigin::Inherited,
        "infer_call_expression_type",
        || {
            let module = input.module(db);
            let callee = normalize_type(db, NormalizeTypeInput::new(db, module, input.callee(db)));
            let args = input.args(db);
            let ty = infer_call_expression_return_type(db, callee, args);

            normalize_type(db, NormalizeTypeInput::new(db, module, ty))
        },
    )
}

/// Infers the expected parameter type for one call argument.
///
/// `input.argument_index` refers to the source argument before tuple spreads
/// are expanded. The requested argument is ignored while overload candidates
/// are checked because its expected type is the result being inferred. Other
/// arguments still select among overloads. Returns `None` when no supported
/// call signature can provide an expected type.
///
/// ```ts
/// declare function consume(value: string, callback: () => void): void;
/// consume("value", async () => {});
/// // The expected type for argument 1 is `() => void`.
/// ```
#[salsa::tracked]
pub fn infer_call_argument_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallArgumentTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    execute_query(
        TypeInferenceQueryKind::Calls,
        TypeInferenceProfileOrigin::Inherited,
        "infer_call_argument_type",
        || {
            let (args, argument_index) =
                resolved_call_arguments(db, input.args(db), input.argument_index(db));
            infer_function_argument_type(db, input.callee(db), &args, argument_index)
        },
    )
}

/// Infers the expected constructor parameter type for one argument.
///
/// This has the same argument-index and overload-selection behavior as
/// [`infer_call_argument_type`], but searches class, interface, and object
/// constructor signatures. Returns `None` when no supported signature can
/// provide an expected type.
///
/// ```ts
/// declare class Job {
///     constructor(callback: () => void);
/// }
/// new Job(async () => {});
/// // The expected type for argument 0 is `() => void`.
/// ```
#[salsa::tracked]
pub fn infer_constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    input: CallArgumentTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    execute_query(
        TypeInferenceQueryKind::Calls,
        TypeInferenceProfileOrigin::Inherited,
        "infer_constructor_argument_type",
        || {
            let (args, argument_index) =
                resolved_call_arguments(db, input.args(db), input.argument_index(db));
            let ty =
                infer_constructor_argument_type_inner(db, input.callee(db), &args, argument_index)?;
            Some(match ty {
                InferredTypeData::GlobalType(id) => global_types(db).get(id),
                ty => ty,
            })
        },
    )
}

// #endregion
