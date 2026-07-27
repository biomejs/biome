//! On-demand type lookup queries.
//!
//! Each tracked query resolves one expression, binding, or local-type entry
//! from a module's raw inference tables. Member and callable lookups use the
//! same local-handle resolution path, allowing callers to inspect one type
//! without resolving every type collected for the module.

use super::{BindingTypeInput, ExpressionTypeInput, LocalTypeInput};
use crate::ModuleDb;
use crate::db::type_inference::{
    ImportResolution, ResolutionCtx, find_member_type_on_demand as find_member_type_impl,
    find_value_member_type_on_demand as find_value_member_type_impl, resolve_local_type_on_demand,
};
use crate::module_graph::ModuleInfoKind;
use crate::type_inference::profiling::{
    TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query,
};
use biome_js_type_info::{InferredType, TypeId, interned_types::TypeData as InferredTypeData};

// #region LOOKUP QUERIES

/// Infers the type collected for one expression.
///
/// Returns `None` when the module does not support inference or the expression
/// was not collected. An inference cycle returns `Unknown`.
///
/// Requesting the `promise` expression in the expression statement infers
/// `Promise<number>`.
///
/// ```ts
/// const promise = Promise.resolve(1);
/// promise;
/// ```
#[salsa::tracked(cycle_result=infer_expression_type_cycle_result)]
pub fn infer_expression_type<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let module = input.module(db);
    let expression = input.expression(db);
    execute_query(
        TypeInferenceQueryKind::Lookups,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_type",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            if !js_info.infer_types {
                return None;
            }

            let reference = js_info.raw_expressions.get(&expression)?.clone();
            let mut ctx = ResolutionCtx::new(db, module, &js_info, ImportResolution::OnDemand);
            Some(ctx.resolve(&reference))
        },
    )
}

/// Infers the type collected for one binding range.
///
/// Returns `None` when the module does not support inference or the range has
/// no collected binding. An inference cycle returns `Unknown`.
///
/// Requesting the declaration binding for `value` infers the numeric literal
/// type `1`.
///
/// ```ts
/// const value = 1;
/// ```
#[salsa::tracked(cycle_result=infer_binding_type_cycle_result)]
pub fn infer_binding_type<'db>(
    db: &'db dyn ModuleDb,
    input: BindingTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let module = input.module(db);
    let range = input.range(db);
    execute_query(
        TypeInferenceQueryKind::Lookups,
        TypeInferenceProfileOrigin::exact(module, range),
        "infer_binding_type",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            if !js_info.infer_types {
                return None;
            }

            let reference = js_info.raw_binding_types.get(&range)?.clone();
            let mut ctx = ResolutionCtx::new(db, module, &js_info, ImportResolution::OnDemand);
            Some(ctx.resolve(&reference))
        },
    )
}

/// Infers one entry from a module's local type table.
///
/// Returns `None` when the module does not support inference or the ID is out
/// of bounds. An inference cycle returns `Unknown`.
///
/// Requesting the local type for `Value` infers an interface with a `field`
/// member of type `string`.
///
/// ```ts
/// interface Value {
///     field: string;
/// }
/// ```
#[salsa::tracked(cycle_result=infer_local_type_cycle_result)]
pub fn infer_local_type<'db>(
    db: &'db dyn ModuleDb,
    input: LocalTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let module = input.module(db);
    execute_query(
        TypeInferenceQueryKind::Lookups,
        TypeInferenceProfileOrigin::document(module),
        "infer_local_type",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return None;
            };
            let type_id = input.type_id(db);
            if !js_info.infer_types || type_id.index() >= js_info.raw_types.len() {
                return None;
            }

            let mut ctx = ResolutionCtx::new(db, module, &js_info, ImportResolution::OnDemand);
            Some(ctx.resolve_raw_type_id(TypeId::new(type_id.index())))
        },
    )
}

// #endregion

// #region CYCLE RESULTS

fn infer_expression_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    Some(InferredTypeData::Unknown)
}

fn infer_binding_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: BindingTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    Some(InferredTypeData::Unknown)
}

fn infer_local_type_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: LocalTypeInput<'db>,
) -> Option<InferredTypeData<'db>> {
    Some(InferredTypeData::Unknown)
}

// #endregion

// #region TYPE HELPERS

/// Finds a named member on either the class or instance side of `ty`.
///
/// Lookup follows inheritance, prototypes, generic constraints, and compound
/// types. Generic arguments are substituted into the result. Members found on
/// multiple union, intersection, or merged-reference branches are combined.
/// Traversal is bounded, so a returned union may contain only members found
/// before the limit. `None` means no member was found in the supported portion
/// that was traversed; it does not prove that `name` is absent.
///
/// For this class, lookup may find either `create` or `value`.
///
/// ```ts
/// class Counter {
///     static create(): Counter {
///         return new Counter();
///     }
///     value = 0;
/// }
/// ```
pub fn find_member_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    find_member_type_impl(db, ty, name)
}

/// Finds a named member available on a value of type `ty`.
///
/// A class value exposes static members, while a class instance exposes
/// instance members. Lookup otherwise follows the same inheritance, compound
/// type, generic substitution, and bounded partial-result rules as
/// [`find_member_type`]. `None` does not prove that the value lacks `name` when
/// traversal reaches its work limit.
///
/// In this example, the value `Counter` exposes `create`, and `counter` exposes
/// `value`.
///
/// ```ts
/// class Counter {
///     static create(): Counter {
///         return new Counter();
///     }
///     value = 0;
/// }
/// const counter = Counter.create();
/// ```
pub fn find_value_member_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    name: &str,
) -> Option<InferredTypeData<'db>> {
    find_value_member_type_impl(db, ty, name)
}

/// Resolves only the wrappers needed to reach one unambiguous callable type.
///
/// An interface or object must have exactly one call signature. An interface
/// without an own call signature may extend exactly one type. Returns `None`
/// for ambiguous, recursive, unresolved, or over-budget traversal.
///
/// In this example, resolving `Formatter` reaches its single call signature.
///
/// ```ts
/// interface Formatter {
///     (value: number): string;
/// }
/// ```
pub fn resolve_callable_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<InferredTypeData<'db>> {
    InferredType::new(db, ty)
        .callable_type_with(|ty| resolve_local_type_on_demand(db, ty).expand_structural_global(db))
}

// #endregion
