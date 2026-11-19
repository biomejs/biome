//! Type representations and inference queries for JavaScript and TypeScript.
//!
//! Collector-side types such as [`TypeData`] describe syntax-local information
//! without database access. Database-backed inferred types are exposed through
//! [`resolved`], where interned handles can refer to types from other modules.
//! Lint rules should normally use [`InferredType`], which provides conservative
//! read-only queries over those inferred values.

#![deny(clippy::use_self)]

mod builders;
mod format_inferred_type_info;
mod format_type_info;
pub(crate) mod generated;
mod globals;
pub(crate) mod globals_ids;
mod inferred_type;
pub mod interned_types;
mod literal_widening;
mod local_inference;
mod narrowing;
pub mod resolved;
mod return_type_relation;
mod stringification;
mod type_data;
mod type_operations;
mod type_relation;
mod type_store;
mod type_transform;
mod type_traversal;

pub use globals::{
    GlobalTypeInput, GlobalTypes, global_type_id_for_qualifier, global_type_id_for_type_name,
    global_type_id_for_value, global_types,
};
pub use globals_ids::GlobalTypeId;
pub use inferred_type::{
    AnyTypeKind, BooleanCoercion, IgnoredPrimitiveTypes, InferredSwitchCase, InferredType,
    ObjectSpreadMisuse, TypeTraversalError,
};
pub use interned_types::{RawTypeData, TypeDb};
pub use literal_widening::{FreshLiteralKinds, PrimitiveLiteralKind, widen_literal_of_kinds};
pub use local_inference::{
    AnyPredicateFunction, EnumTypes, inferred_predicate_parameter, own_returns, specialized_first,
};
pub use narrowing::{
    NarrowingPredicate, PropertyTest, RawPredicateTarget, TypeofKind, base_literal_type,
    evolving_array_element, evolving_assigned_type, falsy_values, has_readonly_property,
    is_narrowing_invariant, is_raw_any, is_raw_narrowing_invariant,
    is_type_parameter_or_intersection, may_narrow_by_instanceof, may_narrow_by_property,
    narrow_by_assignment, narrow_by_constructor, narrow_by_in_operator, narrow_by_instanceof,
    narrow_by_private_name, narrow_by_property, narrow_by_type_predicate, narrow_type,
    raw_may_be_nullish, raw_may_be_object_union, raw_may_be_union, raw_may_have_literal,
    raw_may_narrow_by_instanceof, raw_predicate_target,
};
pub use return_type_relation::{
    NarrowedTypeCandidates, ReturnTypeRelation, ReturnTypeVerdict, compare_declared_return_type,
};
pub use stringification::{StringificationMode, StringificationUsefulness};
pub use type_data::*;
pub use type_relation::{is_assignable_to, is_subtype_of};
pub use type_store::{RawTypeCollector, TypeStore, UnionCollector};
pub use type_transform::instantiate_intersection_alias;

pub use format_inferred_type_info::{
    FormatInferredTypeContext, InferredTypeDisplay, format_inferred_type,
};
pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
