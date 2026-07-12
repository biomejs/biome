#![deny(clippy::use_self)]

mod builders;
mod format_inferred_type_info;
mod format_type_info;
pub(crate) mod generated;
mod globals;
mod globals_builder;
pub(crate) mod globals_ids;
mod inferred_type;
mod interned_types;
mod local_inference;
mod misleading_return;
pub mod resolved;
mod type_data;
mod type_store;

pub use globals::{
    GlobalTypes, global_type_id_for_qualifier, global_type_id_for_value, global_types,
};
pub use inferred_type::{
    IgnoredPrimitiveTypes, InferredSwitchCase, InferredType, StringificationMode,
    StringificationUsefulness,
};
pub use interned_types::{RawTypeData, TypeDb};
pub use misleading_return::{MisleadingReturnType, ReturnTypeEvidence};
pub use type_data::*;
pub use type_store::{RawTypeCollector, TypeStore, UnionCollector};

pub use format_inferred_type_info::{
    FormatInferredTypeContext, InferredTypeDisplay, format_inferred_type,
};
pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
