//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_array_index_key;
pub mod no_catch_assign;
pub mod no_class_assign;
pub mod no_console_log;
pub mod no_duplicate_parameters;
pub mod no_function_assign;
pub mod no_global_is_finite;
pub mod no_global_is_nan;
pub mod no_import_assign;
pub mod no_label_var;
pub mod no_redeclare;
pub mod no_unsafe_declaration_merging;
pub mod use_is_array;

declare_group! {
    pub Suspicious {
        name : "suspicious" ,
        rules : [
            self :: no_array_index_key :: NoArrayIndexKey ,
            self :: no_catch_assign :: NoCatchAssign ,
            self :: no_class_assign :: NoClassAssign ,
            self :: no_console_log :: NoConsoleLog ,
            self :: no_duplicate_parameters :: NoDuplicateParameters ,
            self :: no_function_assign :: NoFunctionAssign ,
            self :: no_global_is_finite :: NoGlobalIsFinite ,
            self :: no_global_is_nan :: NoGlobalIsNan ,
            self :: no_import_assign :: NoImportAssign ,
            self :: no_label_var :: NoLabelVar ,
            self :: no_redeclare :: NoRedeclare ,
            self :: no_unsafe_declaration_merging :: NoUnsafeDeclarationMerging ,
            self :: use_is_array :: UseIsArray ,
        ]
     }
}
