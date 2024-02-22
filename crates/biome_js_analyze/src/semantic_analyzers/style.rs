//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_arguments;
pub mod no_parameter_assign;
pub mod no_restricted_globals;
pub mod no_shouty_constants;
pub mod no_var;
pub mod use_const;
pub mod use_fragment_syntax;
pub mod use_naming_convention;

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
            self :: use_fragment_syntax :: UseFragmentSyntax ,
            self :: use_naming_convention :: UseNamingConvention ,
        ]
     }
}
