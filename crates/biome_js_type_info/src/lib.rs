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
mod globals_builder;
pub(crate) mod globals_ids;
mod inferred_type;
pub mod interned_types;
mod local_inference;
pub mod resolved;
mod return_type_relation;
mod type_data;
mod type_store;

pub use globals::{
    GlobalTypes, global_type_id_for_qualifier, global_type_id_for_value, global_types,
};
pub use globals_ids::GlobalTypeId;
pub use inferred_type::{
    IgnoredPrimitiveTypes, InferredSwitchCase, InferredType, StringificationMode,
    StringificationUsefulness,
};
pub use interned_types::{RawTypeData, TypeDb};
pub use return_type_relation::{
    NarrowedTypeCandidates, ReturnTypeRelation, ReturnTypeVerdict, compare_declared_return_type,
};
pub use type_data::*;
pub use type_store::{RawTypeCollector, TypeStore, UnionCollector};

pub use format_inferred_type_info::{
    FormatInferredTypeContext, InferredTypeDisplay, format_inferred_type,
};
pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
