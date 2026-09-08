//! Interned compound inputs for tracked type-inference queries.
//!
//! Salsa tracks an interned input by identity. These types keep related query
//! arguments in one key without implying that the query result itself is
//! cached by interning.

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
    pub module: ModuleInfo,
    /// Source location that identifies the expression in the module's raw table.
    pub expression: TextRange,
}

/// Interned input for [`super::infer_binding_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct BindingTypeInput<'db> {
    pub module: ModuleInfo,
    /// Source range used to identify the binding.
    pub range: TextRange,
}

/// Interned input for [`super::infer_local_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct LocalTypeInput<'db> {
    pub module: ModuleInfo,
    pub type_id: InferredLocalTypeId,
}

/// Interned input for [`super::infer_binding_type_with_import_budget`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct BindingTypeWithImportBudgetInput<'db> {
    pub lookup: BindingTypeInput<'db>,
    pub remaining: u8,
}

/// Interned input for [`super::infer_local_type_with_import_budget`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct LocalTypeWithImportBudgetInput<'db> {
    pub lookup: LocalTypeInput<'db>,
    pub remaining: u8,
}

/// Interned input for [`super::infer_call_expression_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct CallExpressionTypeInput<'db> {
    /// Module used to resolve local callee and return types.
    pub module: ModuleInfo,
    pub callee: InferredTypeData<'db>,
    /// Definite positional argument types in source order.
    #[returns(ref)]
    pub args: Box<[InferredTypeData<'db>]>,
}

/// Interned input for call and constructor argument-type inference.
#[salsa::interned]
#[derive(Debug)]
pub struct CallArgumentTypeInput<'db> {
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
    pub ty: InferredTypeData<'db>,
}

// #endregion
