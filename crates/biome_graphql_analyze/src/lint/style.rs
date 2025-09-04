//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod use_deprecated_reason;
pub mod use_graphql_naming_convention;
declare_lint_group! { pub Style { name : "style" , rules : [self :: use_deprecated_reason :: UseDeprecatedReason , self :: use_graphql_naming_convention :: UseGraphqlNamingConvention ,] } }
