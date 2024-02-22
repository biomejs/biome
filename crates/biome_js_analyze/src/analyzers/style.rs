//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub mod no_comma_operator;
pub mod no_default_export;
pub mod no_implicit_boolean;
pub mod no_inferrable_types;
pub mod no_namespace;
pub mod no_negation_else;
pub mod no_non_null_assertion;
pub mod no_parameter_properties;
pub mod no_unused_template_literal;
pub mod no_useless_else;
pub mod use_as_const_assertion;
pub mod use_block_statements;
pub mod use_collapsed_else_if;
pub mod use_default_parameter_last;
pub mod use_enum_initializers;
pub mod use_exponentiation_operator;
pub mod use_literal_enum_members;
pub mod use_numeric_literals;
pub mod use_self_closing_elements;
pub mod use_shorthand_array_type;
pub mod use_shorthand_assign;
pub mod use_single_case_statement;
pub mod use_single_var_declarator;
pub mod use_template;
pub mod use_while;

declare_group! {
    pub Style {
        name : "style" ,
        rules : [
            self :: no_comma_operator :: NoCommaOperator ,
            self :: no_default_export :: NoDefaultExport ,
            self :: no_implicit_boolean :: NoImplicitBoolean ,
            self :: no_inferrable_types :: NoInferrableTypes ,
            self :: no_namespace :: NoNamespace ,
            self :: no_negation_else :: NoNegationElse ,
            self :: no_non_null_assertion :: NoNonNullAssertion ,
            self :: no_parameter_properties :: NoParameterProperties ,
            self :: no_unused_template_literal :: NoUnusedTemplateLiteral ,
            self :: no_useless_else :: NoUselessElse ,
            self :: use_as_const_assertion :: UseAsConstAssertion ,
            self :: use_block_statements :: UseBlockStatements ,
            self :: use_collapsed_else_if :: UseCollapsedElseIf ,
            self :: use_default_parameter_last :: UseDefaultParameterLast ,
            self :: use_enum_initializers :: UseEnumInitializers ,
            self :: use_exponentiation_operator :: UseExponentiationOperator ,
            self :: use_literal_enum_members :: UseLiteralEnumMembers ,
            self :: use_numeric_literals :: UseNumericLiterals ,
            self :: use_self_closing_elements :: UseSelfClosingElements ,
            self :: use_shorthand_array_type :: UseShorthandArrayType ,
            self :: use_shorthand_assign :: UseShorthandAssign ,
            self :: use_single_case_statement :: UseSingleCaseStatement ,
            self :: use_single_var_declarator :: UseSingleVarDeclarator ,
            self :: use_template :: UseTemplate ,
            self :: use_while :: UseWhile ,
        ]
     }
}
