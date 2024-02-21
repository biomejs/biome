//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_excessive_cognitive_complexity;
pub mod no_extra_boolean_cast;
pub mod no_for_each;
pub mod no_multiple_spaces_in_regular_expression_literals;
pub mod no_static_only_class;
pub mod no_useless_catch;
pub mod no_useless_constructor;
pub mod no_useless_empty_export;
pub mod no_useless_label;
pub mod no_useless_rename;
pub mod no_useless_switch_case;
pub mod no_useless_type_constraint;
pub mod no_void;
pub mod no_with;
pub mod use_arrow_function;
pub mod use_flat_map;
pub mod use_literal_keys;
pub mod use_optional_chain;
pub mod use_regex_literals;
pub mod use_simple_number_keys;
pub mod use_simplified_logic_expression;

declare_group! {
    pub Complexity {
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
