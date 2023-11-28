//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_constructor_return;
pub(crate) mod no_empty_character_class_in_regex;
pub(crate) mod no_empty_pattern;
pub(crate) mod no_inner_declarations;
pub(crate) mod no_invalid_constructor_super;
pub(crate) mod no_nonoctal_decimal_escape;
pub(crate) mod no_precision_loss;
pub(crate) mod no_self_assign;
pub(crate) mod no_setter_return;
pub(crate) mod no_string_case_mismatch;
pub(crate) mod no_switch_declarations;
pub(crate) mod no_unnecessary_continue;
pub(crate) mod no_unreachable;
pub(crate) mod no_unreachable_super;
pub(crate) mod no_unsafe_finally;
pub(crate) mod no_unsafe_optional_chaining;
pub(crate) mod no_unused_labels;
pub(crate) mod no_void_type_return;
pub(crate) mod use_valid_for_direction;
pub(crate) mod use_yield;

declare_group! {
    pub (crate) Correctness {
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
