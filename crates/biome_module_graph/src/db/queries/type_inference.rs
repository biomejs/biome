//! Salsa-backed queries for JavaScript type inference.
//!
//! Lookup queries resolve one expression, binding, or local-type entry from the
//! raw tables stored in [`crate::ModuleInfo`]. These are the primary query units
//! for consumers that do not need every inferred type in a module.
//!
//! Export discovery is tracked separately from type resolution. Origin and
//! namespace-name queries inspect only module graph inputs, so their results
//! can be shared by importers without caching provisional inferred types.
//!
//! [`infer_module_types`] resolves the complete type tables for compatibility
//! with bulk consumers. [`infer_module_types_bottom_up`] prepares its
//! dependencies iteratively so long import chains do not consume the Rust call
//! stack.
//!
//! Tracked functions record the inputs and queries they inspect. Salsa reruns a
//! query when one of those dependencies changes. Inputs with several fields use
//! interned structs so equal values share one database identity; interning an
//! input is separate from caching the query result.
//!
//! Query-family-specific algorithms may live in a child module, such as
//! `calls/implementation`. Resolution helpers shared by multiple query families
//! live under [`crate::db::type_inference`].
//!
//! Call and constructor inference is best effort. Unsupported or unresolved
//! shapes produce `Unknown` or `None` instead of a TypeScript diagnostic.

mod calls;
mod exports;
mod interned;
mod lookups;
mod module_types;
mod normalization;
mod promises;

pub(in crate::db) use calls::{
    ResolvedCallArgument, infer_call_expression_return_type_from_args, resolve_callable_function,
};
pub use calls::{
    infer_call_argument_type, infer_call_expression_type, infer_constructor_argument_type,
};
pub use exports::infer_export_type;
pub(crate) use exports::{namespace_export_names, resolved_export_origin};
pub use interned::{
    BindingTypeInput, CallArgumentTypeInput, CallExpressionTypeInput, ExpressionTypeInput,
    LocalTypeInput, NormalizeTypeInput,
};
pub(crate) use interned::{BindingTypeWithImportBudgetInput, LocalTypeWithImportBudgetInput};
pub use lookups::{
    find_member_type, find_value_member_type, infer_binding_type, infer_expression_type,
    infer_local_type, resolve_callable_type,
};
pub(crate) use lookups::{
    infer_binding_type_with_import_budget, infer_local_type_with_import_budget,
};
pub use module_types::{infer_module_types, infer_module_types_bottom_up};
pub(crate) use module_types::{
    infer_module_types_bottom_up_for_import_depth, infer_module_types_from_tables,
    inference_module_sccs,
};
pub use normalization::normalize_type;
pub use promises::{
    function_returns_promise, infer_expression_function_returns_promise,
    infer_expression_is_array_of_promises, infer_expression_is_promise, is_array_of_promise_type,
    is_promise_type,
};
