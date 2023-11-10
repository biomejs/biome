//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_approximative_numeric_constant;
pub(crate) mod no_empty_block_statements;
pub(crate) mod no_empty_character_class_in_regex;
pub(crate) mod no_misleading_instantiator;
pub(crate) mod no_misrefactored_shorthand_assign;
pub(crate) mod no_unused_private_class_members;
pub(crate) mod no_useless_else;
pub(crate) mod no_useless_lone_block_statements;
pub(crate) mod use_arrow_function;
pub(crate) mod use_as_const_assertion;
pub(crate) mod use_grouped_type_import;
pub(crate) mod use_import_restrictions;
pub(crate) mod use_shorthand_assign;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_approximative_numeric_constant :: NoApproximativeNumericConstant ,
            self :: no_empty_block_statements :: NoEmptyBlockStatements ,
            self :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegex ,
            self :: no_misleading_instantiator :: NoMisleadingInstantiator ,
            self :: no_misrefactored_shorthand_assign :: NoMisrefactoredShorthandAssign ,
            self :: no_unused_private_class_members :: NoUnusedPrivateClassMembers ,
            self :: no_useless_else :: NoUselessElse ,
            self :: no_useless_lone_block_statements :: NoUselessLoneBlockStatements ,
            self :: use_arrow_function :: UseArrowFunction ,
            self :: use_as_const_assertion :: UseAsConstAssertion ,
            self :: use_grouped_type_import :: UseGroupedTypeImport ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_shorthand_assign :: UseShorthandAssign ,
        ]
     }
}
