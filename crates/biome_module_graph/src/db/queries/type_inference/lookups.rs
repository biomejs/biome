//! On-demand type lookup queries.
//!
//! Each tracked query resolves one expression, binding, or local-type entry
//! from a module's raw inference tables. Member and callable lookups use the
//! same local-handle resolution path, allowing callers to inspect one type
//! without resolving every type collected for the module.

use super::{
    BindingTypeInput, BindingTypeWithImportBudgetInput, ExpressionTypeInput, LocalTypeInput,
    LocalTypeWithImportBudgetInput,
};
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
            let mut ctx = ResolutionCtx::new(db, module, &js_info, ImportResolution::on_demand());
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
        || infer_binding_type_impl(db, input, ImportResolution::on_demand()),
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
        || infer_local_type_impl(db, input, ImportResolution::on_demand()),
    )
}

#[salsa::tracked(cycle_result=infer_binding_type_with_import_budget_cycle_result)]
pub(crate) fn infer_binding_type_with_import_budget<'db>(
    db: &'db dyn ModuleDb,
    input: BindingTypeWithImportBudgetInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let lookup = input.lookup(db);
    let remaining = input.remaining(db);
    infer_binding_type_impl(db, lookup, ImportResolution::OnDemand { remaining })
}

#[salsa::tracked(cycle_result=infer_local_type_with_import_budget_cycle_result)]
pub(crate) fn infer_local_type_with_import_budget<'db>(
    db: &'db dyn ModuleDb,
    input: LocalTypeWithImportBudgetInput<'db>,
) -> Option<InferredTypeData<'db>> {
    let lookup = input.lookup(db);
    let remaining = input.remaining(db);
    infer_local_type_impl(db, lookup, ImportResolution::OnDemand { remaining })
}

fn infer_binding_type_impl<'db>(
    db: &'db dyn ModuleDb,
    input: BindingTypeInput<'db>,
    import_resolution: ImportResolution<'_>,
) -> Option<InferredTypeData<'db>> {
    let module = input.module(db);
    let range = input.range(db);
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    if !js_info.infer_types {
        return None;
    }

    let reference = js_info.raw_binding_types.get(&range)?.clone();
    let mut ctx = ResolutionCtx::new(db, module, &js_info, import_resolution);
    Some(ctx.resolve(&reference))
}

fn infer_local_type_impl<'db>(
    db: &'db dyn ModuleDb,
    input: LocalTypeInput<'db>,
    import_resolution: ImportResolution<'_>,
) -> Option<InferredTypeData<'db>> {
    let module = input.module(db);
    let ModuleInfoKind::Js(js_info) = module.kind(db) else {
        return None;
    };
    let type_id = input.type_id(db);
    if !js_info.infer_types || type_id.index() >= js_info.raw_types.len() {
        return None;
    }

    let mut ctx = ResolutionCtx::new(db, module, &js_info, import_resolution);
    Some(ctx.resolve_raw_type_id(TypeId::new(type_id.index())))
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

fn infer_binding_type_with_import_budget_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: BindingTypeWithImportBudgetInput<'db>,
) -> Option<InferredTypeData<'db>> {
    Some(InferredTypeData::Unknown)
}

fn infer_local_type_with_import_budget_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: LocalTypeWithImportBudgetInput<'db>,
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
/// The returned type is not normalized and may contain unresolved local handles
/// or structural wrappers. Call [`super::normalize_type`] when normalized output
/// is required.
/// Traversal is bounded. Reaching the work limit returns `Unknown`. `None` means
/// a completed traversal found no supported member.
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
/// type, generic substitution, work-limit, and normalization rules as
/// [`find_member_type`].
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
