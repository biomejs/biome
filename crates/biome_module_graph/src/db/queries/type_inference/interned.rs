//! Interned compound inputs for tracked type-inference queries.
//!
//! Salsa tracks an interned input by identity. These types keep related query
//! arguments in one key without implying that the query result itself is
//! cached by interning.

use crate::ModuleInfo;
use crate::js_module_info::flow::{FlowNodeId, FlowSubjectId};
use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, LocalTypeId as InferredLocalTypeId,
    TypeData as InferredTypeData, TypeSubstitution as InferredTypeSubstitution,
};
use biome_rowan::TextRange;

// #region INTERNED TYPES

/// Interned input for queries that project information from one expression.
#[salsa::interned]
#[derive(Debug)]
pub struct ExpressionTypeInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// Source location that identifies the expression in the module's raw table.
    #[returns(copy)]
    pub expression: TextRange,
}

/// Interned input for [`super::infer_binding_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct BindingTypeInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// Source range used to identify the binding.
    #[returns(copy)]
    pub range: TextRange,
}

/// Identifies a subject within one execution root, together with the type
/// that narrowing starts from.
///
/// Reads of the subject in that root share this key, and with it one solve of
/// the root's flow, when they start from the same type, whether a lookup or
/// complete module inference resolved that type.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowSubjectRootInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// Index of the execution root in the module's collected flow.
    #[returns(copy)]
    pub root: usize,
    /// Id of the subject in that root's subject table.
    #[returns(copy)]
    pub subject: FlowSubjectId,
    /// The type that narrowing starts from: the subject's declared type
    /// flattened for narrowing, or `None` when no runtime test can change that
    /// type. A variable's comes from [`super::infer_flow_binding_baseline`];
    /// a property path's is the type of the member access at the read, so
    /// reads of one path can start from different types.
    #[returns(copy)]
    pub baseline: Option<InferredTypeData<'db>>,
}

/// Identifies a subject's incoming state at one point of its execution root.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowSubjectTypeInput<'db> {
    /// The subject, its root, and the type that narrowing starts from.
    #[returns(copy)]
    pub states: FlowSubjectRootInput<'db>,
    /// Node index in that root's flow graph.
    #[returns(copy)]
    pub point: FlowNodeId,
}

/// Interned input for [`super::infer_local_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct LocalTypeInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    #[returns(copy)]
    pub type_id: InferredLocalTypeId,
}

/// Interned input for [`super::infer_binding_type_with_import_budget`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct BindingTypeWithImportBudgetInput<'db> {
    #[returns(copy)]
    pub lookup: BindingTypeInput<'db>,
    #[returns(copy)]
    pub remaining: u8,
}

/// Interned input for [`super::infer_local_type_with_import_budget`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct LocalTypeWithImportBudgetInput<'db> {
    #[returns(copy)]
    pub lookup: LocalTypeInput<'db>,
    #[returns(copy)]
    pub remaining: u8,
}

/// Interned input for [`super::infer_call_expression_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct CallExpressionTypeInput<'db> {
    /// Module used to resolve local callee and return types.
    #[returns(copy)]
    pub module: ModuleInfo,
    #[returns(copy)]
    pub callee: InferredTypeData<'db>,
    /// Definite positional argument types in source order.
    pub args: Box<[InferredTypeData<'db>]>,
}

/// Interned input for [`super::resolve_callable_function`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct CallableFunctionInput<'db> {
    #[returns(copy)]
    pub ty: InferredTypeData<'db>,
}

/// Interned input for [`super::substitute_types`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct TypeSubstitutionInput<'db> {
    #[returns(copy)]
    pub ty: InferredTypeData<'db>,
    pub substitutions: Box<[InferredTypeSubstitution<'db>]>,
}

/// Interned input for call and constructor argument-type inference.
#[salsa::interned]
#[derive(Debug)]
pub struct CallArgumentTypeInput<'db> {
    #[returns(copy)]
    pub callee: InferredTypeData<'db>,
    /// Source arguments in order, including unexpanded spread arguments.
    pub args: Box<[InferredCallArgumentType<'db>]>,
    /// Index of the source argument whose expected type is requested.
    #[returns(copy)]
    pub argument_index: usize,
}

/// Interned input for [`super::normalize_type`].
#[salsa::interned]
#[derive(Debug)]
pub struct NormalizeTypeInput<'db> {
    /// Module used to resolve local type handles.
    #[returns(copy)]
    pub module: ModuleInfo,
    #[returns(copy)]
    pub ty: InferredTypeData<'db>,
}

// #endregion
