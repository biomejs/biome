//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_empty_source;
pub mod use_consistent_graphql_descriptions;
pub mod use_deprecated_date;
pub mod use_lone_executable_definition;
pub mod use_unique_argument_names;
pub mod use_unique_enum_value_names;
pub mod use_unique_field_definition_names;
pub mod use_unique_graphql_operation_name;
pub mod use_unique_input_field_names;
pub mod use_unique_variable_names;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_empty_source :: NoEmptySource , self :: use_consistent_graphql_descriptions :: UseConsistentGraphqlDescriptions , self :: use_deprecated_date :: UseDeprecatedDate , self :: use_lone_executable_definition :: UseLoneExecutableDefinition , self :: use_unique_argument_names :: UseUniqueArgumentNames , self :: use_unique_enum_value_names :: UseUniqueEnumValueNames , self :: use_unique_field_definition_names :: UseUniqueFieldDefinitionNames , self :: use_unique_graphql_operation_name :: UseUniqueGraphqlOperationName , self :: use_unique_input_field_names :: UseUniqueInputFieldNames , self :: use_unique_variable_names :: UseUniqueVariableNames ,] } }
