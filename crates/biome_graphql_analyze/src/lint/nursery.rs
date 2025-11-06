//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_empty_source;
pub mod use_consistent_graphql_descriptions;
pub mod use_deprecated_date;
pub mod use_unique_graphql_operation_name;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_empty_source :: NoEmptySource , self :: use_consistent_graphql_descriptions :: UseConsistentGraphqlDescriptions , self :: use_deprecated_date :: UseDeprecatedDate , self :: use_unique_graphql_operation_name :: UseUniqueGraphqlOperationName ,] } }
