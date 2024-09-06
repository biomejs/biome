//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_duplicate_at_import_rules;
pub mod no_duplicate_font_names;

declare_lint_group! {
    pub Suspicious {
        name : "suspicious" ,
        rules : [
            self :: no_duplicate_at_import_rules :: NoDuplicateAtImportRules ,
            self :: no_duplicate_font_names :: NoDuplicateFontNames ,
        ]
     }
}
