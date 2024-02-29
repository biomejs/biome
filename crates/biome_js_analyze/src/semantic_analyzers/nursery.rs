//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_console;
pub mod no_global_assign;
pub mod no_global_eval;
pub mod no_invalid_use_before_declaration;
pub mod no_misleading_character_class;
pub mod no_re_export_all;
pub mod no_semicolon_in_jsx;
pub mod no_then_property;
pub mod no_unused_imports;
pub mod use_export_type;
pub mod use_for_of;
pub mod use_import_type;
pub mod use_jsx_key_in_iterable;
pub mod use_number_namespace;
pub mod use_sorted_classes;

declare_group! {
    pub Nursery {
        name : "nursery" ,
        rules : [
            self :: no_console :: NoConsole ,
            self :: no_global_assign :: NoGlobalAssign ,
            self :: no_global_eval :: NoGlobalEval ,
            self :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration ,
            self :: no_misleading_character_class :: NoMisleadingCharacterClass ,
            self :: no_re_export_all :: NoReExportAll ,
            self :: no_semicolon_in_jsx :: NoSemicolonInJsx ,
            self :: no_then_property :: NoThenProperty ,
            self :: no_unused_imports :: NoUnusedImports ,
            self :: use_export_type :: UseExportType ,
            self :: use_for_of :: UseForOf ,
            self :: use_import_type :: UseImportType ,
            self :: use_jsx_key_in_iterable :: UseJsxKeyInIterable ,
            self :: use_number_namespace :: UseNumberNamespace ,
            self :: use_sorted_classes :: UseSortedClasses ,
        ]
     }
}
