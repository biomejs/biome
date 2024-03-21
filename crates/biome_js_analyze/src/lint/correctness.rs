//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_children_prop;
pub mod no_const_assign;
pub mod no_constant_condition;
pub mod no_constructor_return;
pub mod no_empty_character_class_in_regex;
pub mod no_empty_pattern;
pub mod no_global_object_calls;
pub mod no_inner_declarations;
pub mod no_invalid_constructor_super;
pub mod no_invalid_new_builtin;
pub mod no_invalid_use_before_declaration;
pub mod no_new_symbol;
pub mod no_nonoctal_decimal_escape;
pub mod no_precision_loss;
pub mod no_render_return_value;
pub mod no_self_assign;
pub mod no_setter_return;
pub mod no_string_case_mismatch;
pub mod no_switch_declarations;
pub mod no_undeclared_variables;
pub mod no_unnecessary_continue;
pub mod no_unreachable;
pub mod no_unreachable_super;
pub mod no_unsafe_finally;
pub mod no_unsafe_optional_chaining;
pub mod no_unused_imports;
pub mod no_unused_labels;
pub mod no_unused_private_class_members;
pub mod no_unused_variables;
pub mod no_void_elements_with_children;
pub mod no_void_type_return;
pub mod use_exhaustive_dependencies;
pub mod use_hook_at_top_level;
pub mod use_is_nan;
pub mod use_valid_for_direction;
pub mod use_yield;

declare_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_children_prop :: NoChildrenProp ,
            self :: no_const_assign :: NoConstAssign ,
            self :: no_constant_condition :: NoConstantCondition ,
            self :: no_constructor_return :: NoConstructorReturn ,
            self :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegex ,
            self :: no_empty_pattern :: NoEmptyPattern ,
            self :: no_global_object_calls :: NoGlobalObjectCalls ,
            self :: no_inner_declarations :: NoInnerDeclarations ,
            self :: no_invalid_constructor_super :: NoInvalidConstructorSuper ,
            self :: no_invalid_new_builtin :: NoInvalidNewBuiltin ,
            self :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration ,
            self :: no_new_symbol :: NoNewSymbol ,
            self :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscape ,
            self :: no_precision_loss :: NoPrecisionLoss ,
            self :: no_render_return_value :: NoRenderReturnValue ,
            self :: no_self_assign :: NoSelfAssign ,
            self :: no_setter_return :: NoSetterReturn ,
            self :: no_string_case_mismatch :: NoStringCaseMismatch ,
            self :: no_switch_declarations :: NoSwitchDeclarations ,
            self :: no_undeclared_variables :: NoUndeclaredVariables ,
            self :: no_unnecessary_continue :: NoUnnecessaryContinue ,
            self :: no_unreachable :: NoUnreachable ,
            self :: no_unreachable_super :: NoUnreachableSuper ,
            self :: no_unsafe_finally :: NoUnsafeFinally ,
            self :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining ,
            self :: no_unused_imports :: NoUnusedImports ,
            self :: no_unused_labels :: NoUnusedLabels ,
            self :: no_unused_private_class_members :: NoUnusedPrivateClassMembers ,
            self :: no_unused_variables :: NoUnusedVariables ,
            self :: no_void_elements_with_children :: NoVoidElementsWithChildren ,
            self :: no_void_type_return :: NoVoidTypeReturn ,
            self :: use_exhaustive_dependencies :: UseExhaustiveDependencies ,
            self :: use_hook_at_top_level :: UseHookAtTopLevel ,
            self :: use_is_nan :: UseIsNan ,
            self :: use_valid_for_direction :: UseValidForDirection ,
            self :: use_yield :: UseYield ,
        ]
     }
}
