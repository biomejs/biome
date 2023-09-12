//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_comma_operator;
pub(crate) mod no_implicit_boolean;
pub(crate) mod no_inferrable_types;
pub(crate) mod no_namespace;
pub(crate) mod no_negation_else;
pub(crate) mod no_non_null_assertion;
pub(crate) mod no_parameter_properties;
pub(crate) mod no_unused_template_literal;
pub(crate) mod use_block_statements;
pub(crate) mod use_default_parameter_last;
pub(crate) mod use_enum_initializers;
pub(crate) mod use_exponentiation_operator;
pub(crate) mod use_numeric_literals;
pub(crate) mod use_self_closing_elements;
pub(crate) mod use_shorthand_array_type;
pub(crate) mod use_single_case_statement;
pub(crate) mod use_single_var_declarator;
pub(crate) mod use_template;
pub(crate) mod use_while;

declare_group! {
    pub (crate) Style {
        name : "style" ,
        rules : [
            self :: no_comma_operator :: NoCommaOperator ,
            self :: no_implicit_boolean :: NoImplicitBoolean ,
            self :: no_inferrable_types :: NoInferrableTypes ,
            self :: no_namespace :: NoNamespace ,
            self :: no_negation_else :: NoNegationElse ,
            self :: no_non_null_assertion :: NoNonNullAssertion ,
            self :: no_parameter_properties :: NoParameterProperties ,
            self :: no_unused_template_literal :: NoUnusedTemplateLiteral ,
            self :: use_block_statements :: UseBlockStatements ,
            self :: use_default_parameter_last :: UseDefaultParameterLast ,
            self :: use_enum_initializers :: UseEnumInitializers ,
            self :: use_exponentiation_operator :: UseExponentiationOperator ,
            self :: use_numeric_literals :: UseNumericLiterals ,
            self :: use_self_closing_elements :: UseSelfClosingElements ,
            self :: use_shorthand_array_type :: UseShorthandArrayType ,
            self :: use_single_case_statement :: UseSingleCaseStatement ,
            self :: use_single_var_declarator :: UseSingleVarDeclarator ,
            self :: use_template :: UseTemplate ,
            self :: use_while :: UseWhile ,
        ]
     }
}
