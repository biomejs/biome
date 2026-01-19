//! Generated file, do not edit by hand, see `xtask/codegen`

//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;
pub mod no_duplicate_argument_names;
pub mod no_duplicate_enum_value_names;
pub mod no_duplicate_field_definition_names;
pub mod no_duplicate_graphql_operation_name;
pub mod no_duplicate_input_field_names;
pub mod no_duplicate_variable_names;
pub mod no_empty_source;
pub mod no_excessive_lines_per_file;
pub mod no_root_type;
pub mod use_consistent_graphql_descriptions;
pub mod use_deprecated_date;
pub mod use_lone_executable_definition;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: no_duplicate_argument_names :: NoDuplicateArgumentNames , self :: no_duplicate_enum_value_names :: NoDuplicateEnumValueNames , self :: no_duplicate_field_definition_names :: NoDuplicateFieldDefinitionNames , self :: no_duplicate_graphql_operation_name :: NoDuplicateGraphqlOperationName , self :: no_duplicate_input_field_names :: NoDuplicateInputFieldNames , self :: no_duplicate_variable_names :: NoDuplicateVariableNames , self :: no_empty_source :: NoEmptySource , self :: no_excessive_lines_per_file :: NoExcessiveLinesPerFile , self :: no_root_type :: NoRootType , self :: use_consistent_graphql_descriptions :: UseConsistentGraphqlDescriptions , self :: use_deprecated_date :: UseDeprecatedDate , self :: use_lone_executable_definition :: UseLoneExecutableDefinition ,] } }
