//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_lint_group;

pub mod no_invalid_direction_in_linear_gradient;

declare_lint_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_invalid_direction_in_linear_gradient :: NoInvalidDirectionInLinearGradient ,
        ]
     }
}
