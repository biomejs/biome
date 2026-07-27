//! Promise-specific classification queries.
//!
//! These queries traverse only the type shapes needed to distinguish promises,
//! arrays of promises, and promise-returning functions. When the available type
//! information is insufficient for a conclusive result, they return
//! `Indeterminate` rather than `NoMatch`.

use super::ExpressionTypeInput;
use crate::ModuleDb;
use crate::db::type_inference::{
    PromiseClassification, classify_expression_array_promise, classify_expression_function_return,
    classify_expression_promise, resolve_local_type_on_demand,
};
use crate::module_graph::ModuleInfoKind;
use crate::type_inference::{
    TypeInferenceClassification,
    profiling::{TypeInferenceProfileOrigin, TypeInferenceQueryKind, execute_query},
};
use biome_js_type_info::{InferredType, interned_types::TypeData as InferredTypeData};

// #region PROMISE QUERIES

/// Classifies whether an expression is a Promise without resolving unrelated members.
///
/// Returns [`TypeInferenceClassification::Match`] for `Promise` and
/// `PromiseLike` instances and [`TypeInferenceClassification::NoMatch`] for
/// conclusive non-matches. Missing expressions, disabled inference, unsupported
/// or ambiguous type shapes, dependency cycles, and exhausted traversal budgets
/// are indeterminate.
#[salsa::tracked(cycle_result=infer_expression_is_promise_cycle_result)]
pub fn infer_expression_is_promise<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    let module = input.module(db);
    let expression = input.expression(db);
    execute_query(
        TypeInferenceQueryKind::Promises,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_is_promise",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return TypeInferenceClassification::Indeterminate;
            };
            if !js_info.infer_types {
                return TypeInferenceClassification::Indeterminate;
            }

            let Some(reference) = js_info.raw_expressions.get(&expression) else {
                return TypeInferenceClassification::Indeterminate;
            };
            classify_promise_result(classify_expression_promise(db, module, reference.clone()))
        },
    )
}

/// Classifies whether an expression is an array of Promise-like values without
/// resolving unrelated members.
///
/// Returns [`TypeInferenceClassification::Match`] for a supported array whose
/// element type is `Promise` or `PromiseLike`.
/// [`TypeInferenceClassification::NoMatch`] means the expression is
/// conclusively not an array of Promise-like values. Missing expressions,
/// disabled inference, unsupported or ambiguous shapes, dependency cycles, and
/// exhausted traversal budgets are indeterminate.
///
/// In this example, classifying the `pending` expression produces a match.
///
/// ```ts
/// const pending: Promise<number>[] = [Promise.resolve(1)];
/// pending;
/// ```
#[salsa::tracked(cycle_result=infer_expression_is_array_of_promises_cycle_result)]
pub fn infer_expression_is_array_of_promises<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    let module = input.module(db);
    let expression = input.expression(db);
    execute_query(
        TypeInferenceQueryKind::Promises,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_is_array_of_promises",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return TypeInferenceClassification::Indeterminate;
            };
            if !js_info.infer_types {
                return TypeInferenceClassification::Indeterminate;
            }

            let Some(reference) = js_info.raw_expressions.get(&expression) else {
                return TypeInferenceClassification::Indeterminate;
            };
            classify_promise_result(classify_expression_array_promise(
                db,
                module,
                reference.clone(),
            ))
        },
    )
}

/// Classifies whether an expression is a function that returns a Promise.
///
/// Returns [`TypeInferenceClassification::Match`] for a supported function whose
/// return type is a `Promise` or `PromiseLike`, and
/// [`TypeInferenceClassification::NoMatch`] for conclusive non-matches. Missing
/// expressions, disabled inference, unsupported or ambiguous type shapes,
/// dependency cycles, and exhausted traversal budgets are indeterminate.
///
/// Requesting `callbacks.callback` produces a match without resolving `other`.
///
/// ```ts
/// const callbacks = { callback: async () => {}, other: 0 };
/// callbacks.callback;
/// ```
#[salsa::tracked(cycle_result=infer_expression_function_returns_promise_cycle_result)]
pub fn infer_expression_function_returns_promise<'db>(
    db: &'db dyn ModuleDb,
    input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    let module = input.module(db);
    let expression = input.expression(db);
    execute_query(
        TypeInferenceQueryKind::Promises,
        TypeInferenceProfileOrigin::exact(module, expression),
        "infer_expression_function_returns_promise",
        || {
            let ModuleInfoKind::Js(js_info) = module.kind(db) else {
                return TypeInferenceClassification::Indeterminate;
            };
            if !js_info.infer_types {
                return TypeInferenceClassification::Indeterminate;
            }

            let Some(reference) = js_info.raw_expressions.get(&expression) else {
                return TypeInferenceClassification::Indeterminate;
            };
            classify_promise_result(classify_expression_function_return(
                db,
                module,
                reference.clone(),
            ))
        },
    )
}

// #endregion

// #region CYCLE RESULTS

fn infer_expression_is_promise_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    TypeInferenceClassification::Indeterminate
}

fn infer_expression_is_array_of_promises_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    TypeInferenceClassification::Indeterminate
}

fn infer_expression_function_returns_promise_cycle_result<'db>(
    _db: &'db dyn ModuleDb,
    _id: salsa::Id,
    _input: ExpressionTypeInput<'db>,
) -> TypeInferenceClassification {
    TypeInferenceClassification::Indeterminate
}

// #endregion

// #region QUERY HELPER FUNCTIONS

fn classify_promise_result(classification: PromiseClassification) -> TypeInferenceClassification {
    match classification {
        PromiseClassification::ReturnsPromise => TypeInferenceClassification::Match,
        PromiseClassification::DoesNotReturnPromise => TypeInferenceClassification::NoMatch,
        PromiseClassification::Indeterminate => TypeInferenceClassification::Indeterminate,
    }
}

/// Classifies `ty` as a Promise without resolving unrelated nested types.
///
/// Returns `Some(true)` when a reachable branch is a `Promise` or
/// `PromiseLike` instance. Returns `Some(false)` when every reachable branch is
/// conclusively not Promise-like. Returns `None` for unresolved or recursive
/// types, ambiguous callables, and exhausted traversal.
pub fn is_promise_type<'db>(db: &'db dyn ModuleDb, ty: InferredTypeData<'db>) -> Option<bool> {
    InferredType::new(db, ty).is_promise_instance_with(|ty| {
        resolve_local_type_on_demand(db, ty).expand_structural_global(db)
    })
}

/// Classifies `ty` as an array of Promises without resolving unrelated nested
/// types.
///
/// Returns `Some(true)` for a supported array with a Promise-like element type.
/// Returns `Some(false)` for a conclusive non-array or a supported array with a
/// non-Promise element type. Returns `None` when the outer type or element type
/// cannot be resolved conclusively, is recursive, or exhausts traversal.
pub fn is_array_of_promise_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<bool> {
    InferredType::new(db, ty).is_array_of_promise_with(|ty| {
        resolve_local_type_on_demand(db, ty).expand_structural_global(db)
    })
}

/// Classifies whether `ty` is callable and returns a Promise.
///
/// Returns `Some(true)` when a reachable callable has a Promise-like return
/// type. Returns `Some(false)` when traversal finds no supported direct function
/// with a Promise-like return. Callable object and interface shapes are not
/// inspected and may therefore return `Some(false)`. Returns `None` when an
/// unresolved or recursive branch, or exhausted traversal, prevents a
/// conclusive result. Callable parameters and unrelated nested types are not
/// resolved.
pub fn function_returns_promise<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
) -> Option<bool> {
    InferredType::new(db, ty).function_returns_promise_with(|ty| {
        resolve_local_type_on_demand(db, ty).expand_structural_global(db)
    })
}

// #endregion
