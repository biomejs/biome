//! Interned compound inputs for tracked type-inference queries.
//!
//! Salsa tracks an interned input by identity. These types keep related query
//! arguments in one key without implying that the query result itself is
//! cached by interning.

use crate::ModuleInfo;
use crate::js_module_info::flow::{FlowEvolving, FlowNodeId, FlowSubjectId};
use biome_js_type_info::GlobalTypeId;
use biome_js_type_info::interned_types::{
    CallArgumentType as InferredCallArgumentType, LocalTypeId as InferredLocalTypeId,
    TypeData as InferredTypeData, TypeSubstitution as InferredTypeSubstitution,
};
use biome_rowan::{Text, TextRange};

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
    /// type. For a variable, lookups take it from
    /// [`super::infer_flow_binding_baseline`]. For a property path, it is the
    /// type of the member access at the read, so reads of one path can start
    /// from different types.
    #[returns(copy)]
    pub baseline: Option<InferredTypeData<'db>>,
    /// Whether the subject is a property path that is readonly at every
    /// step, so that the runtime tests that aliased conditions apply to it
    /// narrow it. Always false for other subjects and for paths that no such
    /// test narrows, so their reads share one solve.
    #[returns(copy)]
    pub readonly_path: bool,
    /// The exhaustive `switch` statements of the root, one bit each. Bit `i`
    /// stands for the statement at index `i` of the root's collected switches.
    /// The bypass of an exhaustive statement adds nothing to the subject.
    /// Only statements whose bypass reaches the read count, so the reads that
    /// no bypass reaches keep a mask of zero and share one solve.
    #[returns(copy)]
    pub exhaustive: u64,
    /// The types of the values that the writes of the subject assign, in the
    /// order of the root's collected writes of the subject. The context that
    /// reads the subject resolves them. `None` when no write with a value
    /// reduces the subject, so that other subjects keep sharing their solves.
    #[returns(copy)]
    pub assigned: Option<FlowAssignedTypes<'db>>,
    /// How the subject's type evolves with its writes, which selects the
    /// solve. With [`FlowEvolving::None`], the solve narrows `baseline` by the
    /// root's guards and writes. With the other kinds, it starts at
    /// `baseline`, which is `undefined`, and takes the subject's type from the
    /// values that the writes assign.
    #[returns(copy)]
    pub evolving: FlowEvolving,
    /// The effects of the root's call points that narrow the subject,
    /// resolved by the context that reads the subject. `None` when none of
    /// them narrows, so that other subjects keep sharing their solves.
    #[returns(copy)]
    pub calls: Option<FlowCallEffects<'db>>,
    /// The statement calls of the root found to never return, after which
    /// the path adds nothing to the subject: bit `i` stands for the call at
    /// index `i` of the root's collected ending calls. Only calls whose node
    /// reaches the read count, so the reads that none reaches keep a mask of
    /// zero and share one solve.
    #[returns(copy)]
    pub ends: u64,
    /// The declared types of the values that the root's comparisons narrow
    /// the subject by, resolved by the context that reads the subject. `None`
    /// when no comparison of the subject has such a value, so that other
    /// subjects keep sharing their solves.
    #[returns(copy)]
    pub compared: Option<FlowComparedTypes<'db>>,
}

/// The declared types of the values that the comparisons of one root
/// narrow a subject by, each with the start and the end of the range that
/// identifies its value, sorted by range; see
/// [`super::infer_flow_compared_types`]. A value whose type can't be used,
/// such as one that inference can't determine, is left out.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowComparedTypes<'db> {
    #[returns(ref)]
    pub types: Box<[(u32, u32, InferredTypeData<'db>)]>,
}

/// The effects of the call points of one root that narrow a subject, each
/// with its index in the root's collected calls, sorted by index.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowCallEffects<'db> {
    #[returns(ref)]
    pub effects: Box<[(u32, FlowCallEffect<'db>)]>,
}

/// Identifies a call point of a module by the range of its call expression,
/// whose effect [`super::infer_flow_call_effect`] resolves.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowCallInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    #[returns(copy)]
    pub call: TextRange,
}

/// How a call point narrows, which the signature of its callee decides.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, salsa::SalsaValue)]
pub(crate) enum FlowCallEffect<'db> {
    /// The call narrows nothing.
    #[default]
    None,
    /// A call in a condition whose signature returns a type predicate, such
    /// as `value is T`. The call narrows the operand that `parameter` names
    /// to `target` where it is truthy, and, with `narrows_falsy`, removes
    /// `target` from the operand where it is falsy. Without it, `target` may
    /// be wider than TypeScript's, as when Biome can't tell the type
    /// arguments that TypeScript infers, and removing it could remove values
    /// that TypeScript keeps.
    Predicate {
        parameter: FlowCallParameter,
        target: InferredTypeData<'db>,
        narrows_falsy: bool,
    },
    /// A statement call whose signature asserts, such as `asserts value is T`,
    /// or `asserts value` without a `target`. The call narrows the operand
    /// that `parameter` names once it returns.
    Assertion {
        parameter: FlowCallParameter,
        target: Option<InferredTypeData<'db>>,
    },
    /// A statement call that never returns, because its signature's return
    /// type annotation is `never` or because it asserts an argument that is
    /// always false, as `assert(false)`. The paths through the call end
    /// there, as they do in TypeScript.
    Ends,
}

impl FlowCallEffect<'_> {
    /// Returns whether the effect narrows the operands of the call: a type
    /// predicate or an assertion. A call that ends the path narrows nothing
    /// where it returns, because it never does.
    pub(crate) const fn narrows(&self) -> bool {
        matches!(self, Self::Predicate { .. } | Self::Assertion { .. })
    }
}

/// The operand of a call that its signature's type predicate or assertion
/// names.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, salsa::SalsaValue)]
pub(crate) enum FlowCallParameter {
    /// The argument at this position, as `value` in `value is T`.
    Argument(u32),
    /// The object that the callee reads its function from, as `this` in
    /// `this is T` names it.
    Receiver,
}

/// The types of the values that the writes of one subject assign, in the
/// order of the root's collected writes of the subject; see
/// [`super::infer_flow_assigned_types`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowAssignedTypes<'db> {
    #[returns(ref)]
    pub types: Box<[InferredTypeData<'db>]>,
}

/// Identifies a subject within one execution root, whose assigned types
/// [`super::infer_flow_assigned_types`] resolves.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowSubjectWritesInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// Index of the execution root in the module's collected flow.
    #[returns(copy)]
    pub root: usize,
    /// Id of the subject in that root's subject table.
    #[returns(copy)]
    pub subject: FlowSubjectId,
}

/// Identifies an execution root of a module by its range, to find which of
/// its statements no path reaches. See [`super::infer_flow_reachability`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowReachabilityInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// The trimmed range of the execution root.
    #[returns(copy)]
    pub root: TextRange,
}

/// Which statements of one execution root some path reaches, and whether a
/// path reaches the root's end. See [`super::infer_flow_reachability`].
#[derive(Clone, Debug, Eq, Hash, PartialEq, salsa::SalsaValue)]
pub(crate) struct FlowReachability {
    /// The ranges of the statements that the root's own code holds, sorted by
    /// start, each with whether some path reaches it. A statement nested in
    /// an unreachable one is never listed, because the unreachable ancestor
    /// stands for it.
    pub statements: Box<[(TextRange, bool)]>,
    /// Whether some path reaches the end of the root, where a function
    /// returns without a `return` statement.
    pub end: bool,
}

impl FlowReachability {
    /// Returns whether some path reaches the innermost listed statement that
    /// contains `range`, or `None` when no listed statement contains it.
    pub(crate) fn reaches(&self, range: TextRange) -> Option<bool> {
        // Statements are sorted by start, and the statements that contain
        // `range` nest, so the innermost of them is the last one listed.
        self.statements
            .iter()
            .take_while(|(statement, _)| statement.start() <= range.start())
            .filter(|(statement, _)| statement.contains_range(range))
            .last()
            .map(|(_, reached)| *reached)
    }
}

/// Identifies one `switch` statement of an execution root, for the query
/// that returns whether the statement is exhaustive
/// ([`super::infer_flow_switch_exhaustive`]).
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct FlowSwitchInput<'db> {
    #[returns(copy)]
    pub module: ModuleInfo,
    /// Index of the execution root in the module's collected flow.
    #[returns(copy)]
    pub root: usize,
    /// Index of the statement in that root's collected switches.
    #[returns(copy)]
    pub switch: u32,
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

/// Interned input for [`super::augmentations::augmentation_target`]: a name
/// that an augmentation of a module declares.
///
/// The name borrows the source text of the augmentation, so interning it
/// allocates no string.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct AugmentedExportInput<'db> {
    /// The module that the augmentation names.
    #[returns(copy)]
    pub module: ModuleInfo,
    /// The name of the interface that the augmentation declares.
    pub name: Text,
}

/// Interned input for [`super::augmentations::global_type_contributions`].
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct GlobalTypeContributionsInput<'db> {
    #[returns(copy)]
    pub id: GlobalTypeId,
}

/// Interned input for [`super::global_name_declaration`]: the name of a
/// global that the standard library doesn't declare. The name borrows the
/// source text that refers to the global.
#[salsa::interned]
#[derive(Debug)]
pub(crate) struct GlobalNameInput<'db> {
    pub name: Text,
}

// #endregion
