//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_duplicated_fields;
pub mod use_deprecated_reason;
pub mod use_named_operation;
pub mod use_naming_convention;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_duplicated_fields :: NoDuplicatedFields , self :: use_deprecated_reason :: UseDeprecatedReason , self :: use_named_operation :: UseNamedOperation , self :: use_naming_convention :: UseNamingConvention ,] } }
