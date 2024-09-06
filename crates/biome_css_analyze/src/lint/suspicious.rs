//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_duplicate_at_import_rules;
pub mod no_duplicate_font_names;
pub mod no_duplicate_selectors_keyframe_block;
pub mod no_important_in_keyframe;

declare_lint_group! {
    pub Suspicious {
        name : "suspicious" ,
        rules : [
            self :: no_duplicate_at_import_rules :: NoDuplicateAtImportRules ,
            self :: no_duplicate_font_names :: NoDuplicateFontNames ,
            self :: no_duplicate_selectors_keyframe_block :: NoDuplicateSelectorsKeyframeBlock ,
            self :: no_important_in_keyframe :: NoImportantInKeyframe ,
        ]
     }
}
