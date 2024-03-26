//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_barrel_file;
pub mod no_console;
pub mod no_done_callback;
pub mod no_duplicate_else_if;
pub mod no_duplicate_test_hooks;
pub mod no_evolving_any;
pub mod no_excessive_nested_test_suites;
pub mod no_exports_in_test;
pub mod no_focused_tests;
pub mod no_misplaced_assertion;
pub mod no_namespace_import;
pub mod no_nodejs_modules;
pub mod no_re_export_all;
pub mod no_restricted_imports;
pub mod no_semicolon_in_jsx;
pub mod no_skipped_tests;
pub mod no_undeclared_dependencies;
pub mod no_useless_ternary;
pub mod use_import_restrictions;
pub mod use_jsx_key_in_iterable;
pub mod use_node_assert_strict;
pub mod use_sorted_classes;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_barrel_file :: NoBarrelFile ,
            self :: no_console :: NoConsole ,
            self :: no_done_callback :: NoDoneCallback ,
            self :: no_duplicate_else_if :: NoDuplicateElseIf ,
            self :: no_duplicate_test_hooks :: NoDuplicateTestHooks ,
            self :: no_evolving_any :: NoEvolvingAny ,
            self :: no_excessive_nested_test_suites :: NoExcessiveNestedTestSuites ,
            self :: no_exports_in_test :: NoExportsInTest ,
            self :: no_focused_tests :: NoFocusedTests ,
            self :: no_misplaced_assertion :: NoMisplacedAssertion ,
            self :: no_namespace_import :: NoNamespaceImport ,
            self :: no_nodejs_modules :: NoNodejsModules ,
            self :: no_re_export_all :: NoReExportAll ,
            self :: no_restricted_imports :: NoRestrictedImports ,
            self :: no_semicolon_in_jsx :: NoSemicolonInJsx ,
            self :: no_skipped_tests :: NoSkippedTests ,
            self :: no_undeclared_dependencies :: NoUndeclaredDependencies ,
            self :: no_useless_ternary :: NoUselessTernary ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_jsx_key_in_iterable :: UseJsxKeyInIterable ,
            self :: use_node_assert_strict :: UseNodeAssertStrict ,
            self :: use_sorted_classes :: UseSortedClasses ,
        ]
     }
}
