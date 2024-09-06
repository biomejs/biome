//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_invalid_direction_in_linear_gradient;
pub mod no_invalid_position_at_import_rule;

declare_lint_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_invalid_direction_in_linear_gradient :: NoInvalidDirectionInLinearGradient ,
            self :: no_invalid_position_at_import_rule :: NoInvalidPositionAtImportRule ,
        ]
     }
}
