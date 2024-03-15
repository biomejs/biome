//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_arguments;
pub mod no_parameter_assign;
pub mod no_restricted_globals;
pub mod no_shouty_constants;
pub mod no_var;
pub mod use_const;
pub mod use_export_type;
pub mod use_for_of;
pub mod use_fragment_syntax;
pub mod use_import_type;
pub mod use_naming_convention;
pub mod use_number_namespace;

declare_group! {
    pub Style {
        name : "style" ,
        rules : [
            self :: no_arguments :: NoArguments ,
            self :: no_parameter_assign :: NoParameterAssign ,
            self :: no_restricted_globals :: NoRestrictedGlobals ,
            self :: no_shouty_constants :: NoShoutyConstants ,
            self :: no_var :: NoVar ,
            self :: use_const :: UseConst ,
            self :: use_export_type :: UseExportType ,
            self :: use_for_of :: UseForOf ,
            self :: use_fragment_syntax :: UseFragmentSyntax ,
            self :: use_import_type :: UseImportType ,
            self :: use_naming_convention :: UseNamingConvention ,
            self :: use_number_namespace :: UseNumberNamespace ,
        ]
     }
}
