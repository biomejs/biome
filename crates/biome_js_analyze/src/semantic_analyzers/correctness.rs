//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_children_prop;
pub mod no_const_assign;
pub mod no_constant_condition;
pub mod no_global_object_calls;
pub mod no_invalid_new_builtin;
pub mod no_new_symbol;
pub mod no_render_return_value;
pub mod no_undeclared_variables;
pub mod no_unused_variables;
pub mod no_void_elements_with_children;
pub mod use_exhaustive_dependencies;
pub mod use_hook_at_top_level;
pub mod use_is_nan;

declare_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_children_prop :: NoChildrenProp ,
            self :: no_const_assign :: NoConstAssign ,
            self :: no_constant_condition :: NoConstantCondition ,
            self :: no_global_object_calls :: NoGlobalObjectCalls ,
            self :: no_invalid_new_builtin :: NoInvalidNewBuiltin ,
            self :: no_new_symbol :: NoNewSymbol ,
            self :: no_render_return_value :: NoRenderReturnValue ,
            self :: no_undeclared_variables :: NoUndeclaredVariables ,
            self :: no_unused_variables :: NoUnusedVariables ,
            self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,
            self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,
            self :: use_hook_at_top_level :: UseHookAtTopLevel ,
            self :: use_is_nan :: UseIsNan ,
        ]
     }
}
