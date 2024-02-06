//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_global_assign;
pub(crate) mod no_global_eval;
pub(crate) mod no_invalid_use_before_declaration;
pub(crate) mod no_misleading_character_class;
pub(crate) mod no_re_export_all;
pub(crate) mod no_then_property;
pub(crate) mod no_unused_imports;
pub(crate) mod use_export_type;
pub(crate) mod use_for_of;
pub(crate) mod use_import_type;
pub(crate) mod use_number_namespace;
pub(crate) mod use_sorted_classes;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_global_assign :: NoGlobalAssign ,
            self :: no_global_eval :: NoGlobalEval ,
            self :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration ,
            self :: no_misleading_character_class :: NoMisleadingCharacterClass ,
            self :: no_re_export_all :: NoReExportAll ,
            self :: no_then_property :: NoThenProperty ,
            self :: no_unused_imports :: NoUnusedImports ,
            self :: use_export_type :: UseExportType ,
            self :: use_for_of :: UseForOf ,
            self :: use_import_type :: UseImportType ,
            self :: use_number_namespace :: UseNumberNamespace ,
            self :: use_sorted_classes :: UseSortedClasses ,
        ]
     }
}
