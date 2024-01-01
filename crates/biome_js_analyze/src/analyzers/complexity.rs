//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_excessive_cognitive_complexity;
pub(crate) mod no_extra_boolean_cast;
pub(crate) mod no_for_each;
pub(crate) mod no_multiple_spaces_in_regular_expression_literals;
pub(crate) mod no_static_only_class;
pub(crate) mod no_useless_catch;
pub(crate) mod no_useless_constructor;
pub(crate) mod no_useless_empty_export;
pub(crate) mod no_useless_label;
pub(crate) mod no_useless_rename;
pub(crate) mod no_useless_switch_case;
pub(crate) mod no_useless_type_constraint;
pub(crate) mod no_void;
pub(crate) mod no_with;
pub(crate) mod use_arrow_function;
pub(crate) mod use_flat_map;
pub(crate) mod use_literal_keys;
pub(crate) mod use_optional_chain;
pub(crate) mod use_regex_literals;
pub(crate) mod use_simple_number_keys;
pub(crate) mod use_simplified_logic_expression;

declare_group! {
    pub (crate) Complexity {
        name : "complexity" ,
        rules : [
            self :: no_excessive_cognitive_complexity :: NoExcessiveCognitiveComplexity ,
            self :: no_extra_boolean_cast :: NoExtraBooleanCast ,
            self :: no_for_each :: NoForEach ,
            self :: no_multiple_spaces_in_regular_expression_literals :: NoMultipleSpacesInRegularExpressionLiterals ,
            self :: no_static_only_class :: NoStaticOnlyClass ,
            self :: no_useless_catch :: NoUselessCatch ,
            self :: no_useless_constructor :: NoUselessConstructor ,
            self :: no_useless_empty_export :: NoUselessEmptyExport ,
            self :: no_useless_label :: NoUselessLabel ,
            self :: no_useless_rename :: NoUselessRename ,
            self :: no_useless_switch_case :: NoUselessSwitchCase ,
            self :: no_useless_type_constraint :: NoUselessTypeConstraint ,
            self :: no_void :: NoVoid ,
            self :: no_with :: NoWith ,
            self :: use_arrow_function :: UseArrowFunction ,
            self :: use_flat_map :: UseFlatMap ,
            self :: use_literal_keys :: UseLiteralKeys ,
            self :: use_optional_chain :: UseOptionalChain ,
            self :: use_regex_literals :: UseRegexLiterals ,
            self :: use_simple_number_keys :: UseSimpleNumberKeys ,
            self :: use_simplified_logic_expression :: UseSimplifiedLogicExpression ,
        ]
     }
}
