#![deny(clippy::use_self)]
#![expect(
    clippy::disallowed_methods,
    reason = "Type inference requires complete type syntax."
)]
#![allow(
    unused_lifetimes,
    reason = "salsa interned handle lifetimes are used by generated code"
)]

mod builders;
mod conditionals;
mod flattening;
mod format_inferred_type_info;
mod format_type_info;
pub(crate) mod generated;
mod globals;
mod globals_builder;
pub(crate) mod globals_ids;
mod helpers;
mod inferred_type;
pub mod interned_types;
mod local_inference;
mod resolver;
mod r#type;
mod type_data;
mod type_store;
mod type_traversal;

pub use conditionals::*;
pub use flattening::MAX_FLATTEN_DEPTH;
pub use globals::{GLOBAL_RESOLVER, GlobalsResolver};
pub use globals_ids::{GLOBAL_BOOLEAN_ID, GLOBAL_UNKNOWN_ID, NUM_PREDEFINED_TYPES};
pub use inferred_type::{
    IgnoredPrimitiveTypes, InferredSwitchCase, InferredType, MisleadingReturnType,
    ReturnTypeEvidence, StringificationMode, StringificationUsefulness,
};
pub use interned_types::TypeDb;
pub use resolver::*;
pub use r#type::Type;
pub use type_data::TypeData as RawTypeData;
pub use type_data::*;
pub use type_store::*;

pub use format_inferred_type_info::{
    FormatInferredTypeContext, InferredTypeDisplay, format_inferred_type,
};
pub use format_type_info::{FormatTypeContext, FormatTypeOptions};
