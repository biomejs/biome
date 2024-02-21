//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_constructor_return;
pub mod no_empty_character_class_in_regex;
pub mod no_empty_pattern;
pub mod no_inner_declarations;
pub mod no_invalid_constructor_super;
pub mod no_nonoctal_decimal_escape;
pub mod no_precision_loss;
pub mod no_self_assign;
pub mod no_setter_return;
pub mod no_string_case_mismatch;
pub mod no_switch_declarations;
pub mod no_unnecessary_continue;
pub mod no_unreachable;
pub mod no_unreachable_super;
pub mod no_unsafe_finally;
pub mod no_unsafe_optional_chaining;
pub mod no_unused_labels;
pub mod no_void_type_return;
pub mod use_valid_for_direction;
pub mod use_yield;

declare_group! {
    pub Correctness {
        name : "correctness" ,
        rules : [
            self :: no_constructor_return :: NoConstructorReturn ,
            self :: no_empty_character_class_in_regex :: NoEmptyCharacterClassInRegex ,
            self :: no_empty_pattern :: NoEmptyPattern ,
            self :: no_inner_declarations :: NoInnerDeclarations ,
            self :: no_invalid_constructor_super :: NoInvalidConstructorSuper ,
            self :: no_nonoctal_decimal_escape :: NoNonoctalDecimalEscape ,
            self :: no_precision_loss :: NoPrecisionLoss ,
            self :: no_self_assign :: NoSelfAssign ,
            self :: no_setter_return :: NoSetterReturn ,
            self :: no_string_case_mismatch :: NoStringCaseMismatch ,
            self :: no_switch_declarations :: NoSwitchDeclarations ,
            self :: no_unnecessary_continue :: NoUnnecessaryContinue ,
            self :: no_unreachable :: NoUnreachable ,
            self :: no_unreachable_super :: NoUnreachableSuper ,
            self :: no_unsafe_finally :: NoUnsafeFinally ,
            self :: no_unsafe_optional_chaining :: NoUnsafeOptionalChaining ,
            self :: no_unused_labels :: NoUnusedLabels ,
            self :: no_void_type_return :: NoVoidTypeReturn ,
            self :: use_valid_for_direction :: UseValidForDirection ,
            self :: use_yield :: UseYield ,
        ]
     }
}
