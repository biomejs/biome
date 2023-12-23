//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_default_export;
pub(crate) mod no_empty_block_statements;
pub(crate) mod no_implicit_any_let;
pub(crate) mod no_unused_private_class_members;
pub(crate) mod no_useless_lone_block_statements;
pub(crate) mod no_useless_ternary;
pub(crate) mod use_await;
pub(crate) mod use_filenaming_convention;
pub(crate) mod use_grouped_type_import;
pub(crate) mod use_import_restrictions;
pub(crate) mod use_node_import_protocol;
pub(crate) mod use_regex_literals;
pub(crate) mod use_shorthand_function_type;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_default_export :: NoDefaultExport ,
            self :: no_empty_block_statements :: NoEmptyBlockStatements ,
            self :: no_implicit_any_let :: NoImplicitAnyLet ,
            self :: no_unused_private_class_members :: NoUnusedPrivateClassMembers ,
            self :: no_useless_lone_block_statements :: NoUselessLoneBlockStatements ,
            self :: no_useless_ternary :: NoUselessTernary ,
            self :: use_await :: UseAwait ,
            self :: use_filenaming_convention :: UseFilenamingConvention ,
            self :: use_grouped_type_import :: UseGroupedTypeImport ,
            self :: use_import_restrictions :: UseImportRestrictions ,
            self :: use_node_import_protocol :: UseNodeImportProtocol ,
            self :: use_regex_literals :: UseRegexLiterals ,
            self :: use_shorthand_function_type :: UseShorthandFunctionType ,
        ]
     }
}
