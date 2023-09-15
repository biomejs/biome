//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_arguments;
pub(crate) mod no_parameter_assign;
pub(crate) mod no_restricted_globals;
pub(crate) mod no_shouty_constants;
pub(crate) mod no_var;
pub(crate) mod use_const;
pub(crate) mod use_fragment_syntax;
pub(crate) mod use_naming_convention;

declare_group! {
    pub (crate) Style {
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
