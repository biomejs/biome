//! Interned compound keys used by tracked type-inference queries.
//!
//! Salsa identifies compound query arguments by their interned identity. The
//! types in this module group the module, source range, and inferred-type data
//! needed by a query without making those fields part of every query signature.
//! Interning a key does not cache the query result.

use crate::ModuleInfo;
use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, LocalTypeId as InferredLocalTypeId,
    TypeData as InferredTypeData,
};
use biome_rowan::TextRange;

// #region INTERNED TYPES

/// Interned input for queries that project information from one expression.
#[salsa::interned]
#[derive(Debug)]
pub struct ExpressionTypeInput<'db> {
    /// Module containing the expression.
    pub module: ModuleInfo,
    /// Source location that identifies the expression in the module's raw table.
    pub expression: TextRange,
}

/// Interned input for [`super::infer_binding_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct BindingTypeInput<'db> {
    /// Module containing the binding.
    pub module: ModuleInfo,
    /// Source range used to identify the binding.
    pub range: TextRange,
}

/// Interned input for [`super::infer_local_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct LocalTypeInput<'db> {
    /// Module owning the local type table.
    pub module: ModuleInfo,
    /// Index of the requested local type.
    pub type_id: InferredLocalTypeId,
}

/// Interned input for [`super::infer_call_expression_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct CallExpressionTypeInput<'db> {
    /// Module used to resolve local callee and return types.
    pub module: ModuleInfo,
    /// Inferred type of the expression being called.
    pub callee: InferredTypeData<'db>,
    /// Definite positional argument types in source order.
    #[returns(ref)]
    pub args: Box<[InferredTypeData<'db>]>,
}

/// Interned input for call and constructor argument-type inference.
#[salsa::interned]
#[derive(Debug)]
pub struct CallArgumentTypeInput<'db> {
    /// Inferred type of the function or constructor being invoked.
    pub callee: InferredTypeData<'db>,
    /// Source arguments in order, including unexpanded spread arguments.
    #[returns(ref)]
    pub args: Box<[InferredCallArgumentType<'db>]>,
    /// Index of the source argument whose expected type is requested.
    pub argument_index: usize,
}

/// Interned input for [`super::normalize_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct NormalizeTypeInput<'db> {
    /// Module used to resolve local type handles.
    pub module: ModuleInfo,
    /// Root type to normalize.
    pub ty: InferredTypeData<'db>,
}

// #endregion
