//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_array_index_key;
pub(crate) mod no_catch_assign;
pub(crate) mod no_class_assign;
pub(crate) mod no_console_log;
pub(crate) mod no_duplicate_parameters;
pub(crate) mod no_function_assign;
pub(crate) mod no_import_assign;
pub(crate) mod no_label_var;
pub(crate) mod no_redeclare;

declare_group! {
    pub (crate) Suspicious {
        name : "suspicious" ,
        rules : [
            self :: no_array_index_key :: NoArrayIndexKey ,
            self :: no_catch_assign :: NoCatchAssign ,
            self :: no_class_assign :: NoClassAssign ,
            self :: no_console_log :: NoConsoleLog ,
            self :: no_duplicate_parameters :: NoDuplicateParameters ,
            self :: no_function_assign :: NoFunctionAssign ,
            self :: no_import_assign :: NoImportAssign ,
            self :: no_label_var :: NoLabelVar ,
            self :: no_redeclare :: NoRedeclare ,
        ]
     }
}
