//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod use_graphql_named_operations;
declare_lint_group! { pub Correctness { name : "correctness" , rules : [self :: use_graphql_named_operations :: UseGraphqlNamedOperations ,] } }
