//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_invalid_direction_in_linear_gradient;
pub mod no_invalid_grid_areas;
pub mod no_invalid_position_at_import_rule;
pub mod no_unknown_function;
pub mod no_unmatchable_anb_selector;

declare_lint_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_invalid_direction_in_linear_gradient :: NoInvalidDirectionInLinearGradient ,
            self :: no_invalid_grid_areas :: NoInvalidGridAreas ,
            self :: no_invalid_position_at_import_rule :: NoInvalidPositionAtImportRule ,
            self :: no_unknown_function :: NoUnknownFunction ,
            self :: no_unmatchable_anb_selector :: NoUnmatchableAnbSelector ,
        ]
     }
}
