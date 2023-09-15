//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_confusing_void_type;
pub(crate) mod no_excessive_complexity;
pub(crate) mod no_fallthrough_switch_clause;
pub(crate) mod no_misleading_instantiator;
pub(crate) mod no_void;
pub(crate) mod use_arrow_function;
pub(crate) mod use_collapsed_else_if;
pub(crate) mod use_grouped_type_import;
pub(crate) mod use_import_restrictions;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_confusing_void_type :: NoConfusingVoidType ,
            self :: no_excessive_complexity :: NoExcessiveComplexity ,
            self :: no_fallthrough_switch_clause :: NoFallthroughSwitchClause ,
            self :: no_misleading_instantiator :: NoMisleadingInstantiator ,
            self :: no_void :: NoVoid ,
            self :: use_arrow_function :: UseArrowFunction ,
            self :: use_collapsed_else_if :: UseCollapsedElseIf ,
            self :: use_grouped_type_import :: UseGroupedTypeImport ,
            self :: use_import_restrictions :: UseImportRestrictions ,
        ]
     }
}
