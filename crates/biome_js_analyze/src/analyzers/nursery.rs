//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_duplicate_test_hooks;
pub mod no_empty_block_statements;
pub mod no_empty_type_parameters;
pub mod no_excessive_nested_test_suites;
pub mod no_exports_in_test;
pub mod no_focused_tests;
pub mod no_namespace_import;
pub mod no_nodejs_modules;
pub mod no_restricted_imports;
pub mod no_skipped_tests;
pub mod no_undeclared_dependencies;
pub mod no_unused_private_class_members;
pub mod no_useless_lone_block_statements;
pub mod no_useless_ternary;
pub mod use_await;
pub mod use_consistent_array_type;
pub mod use_filenaming_convention;
pub mod use_grouped_type_import;
pub mod use_import_restrictions;
pub mod use_node_assert_strict;
pub mod use_nodejs_import_protocol;
pub mod use_shorthand_function_type;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_duplicate_test_hooks :: NoDuplicateTestHooks ,
            self :: no_empty_block_statements :: NoEmptyBlockStatements ,
            self :: no_empty_type_parameters :: NoEmptyTypeParameters ,
            self :: no_excessive_nested_test_suites :: NoExcessiveNestedTestSuites ,
            self :: no_exports_in_test :: NoExportsInTest ,
            self :: no_focused_tests :: NoFocusedTests ,
            self :: no_namespace_import :: NoNamespaceImport ,
            self :: no_nodejs_modules :: NoNodejsModules ,
            self :: no_restricted_imports :: NoRestrictedImports ,
            self :: no_skipped_tests :: NoSkippedTests ,
            self :: no_undeclared_dependencies :: NoUndeclaredDependencies ,
            self :: no_unused_private_class_members :: NoUnusedPrivateClassMembers ,
            self :: no_useless_lone_block_statements :: NoUselessLoneBlockStatements ,
            self :: no_useless_ternary :: NoUselessTernary ,
            self :: use_await :: UseAwait ,
            self :: use_consistent_array_type :: UseConsistentArrayType ,
            self :: use_filenaming_convention :: UseFilenamingConvention ,
            self :: use_grouped_type_import :: UseGroupedTypeImport ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_node_assert_strict :: UseNodeAssertStrict ,
            self :: use_nodejs_import_protocol :: UseNodejsImportProtocol ,
            self :: use_shorthand_function_type :: UseShorthandFunctionType ,
        ]
     }
}
