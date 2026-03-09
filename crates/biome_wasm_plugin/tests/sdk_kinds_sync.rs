//! Verify that the hand-maintained constants in `biome_plugin_sdk::{js,css,json}_kinds`
//! match the actual discriminant values of the generated syntax kind enums.
//!
//! If a codegen run reorders or inserts variants, these assertions will catch
//! the drift at test time rather than causing silent ABI mismatches.

use biome_css_syntax::CssSyntaxKind;
use biome_js_syntax::JsSyntaxKind;
use biome_json_syntax::JsonSyntaxKind;
use biome_plugin_sdk::{css_kinds, js_kinds, json_kinds};

macro_rules! assert_kind {
    ($module:ident :: $constant:ident == $enum:ident :: $variant:ident) => {
        assert_eq!(
            $module::$constant,
            $enum::$variant as u32,
            "{} (= {}) does not match {}::{} (= {})",
            stringify!($constant),
            $module::$constant,
            stringify!($enum),
            stringify!($variant),
            $enum::$variant as u32,
        );
    };
}

// ---------------------------------------------------------------
// JavaScript / TypeScript
// ---------------------------------------------------------------

#[test]
fn js_kinds_match_js_syntax_kind() {
    // Punctuation & operators
    assert_kind!(js_kinds::SEMICOLON == JsSyntaxKind::SEMICOLON);
    assert_kind!(js_kinds::L_PAREN == JsSyntaxKind::L_PAREN);
    assert_kind!(js_kinds::R_PAREN == JsSyntaxKind::R_PAREN);
    assert_kind!(js_kinds::L_CURLY == JsSyntaxKind::L_CURLY);
    assert_kind!(js_kinds::R_CURLY == JsSyntaxKind::R_CURLY);
    assert_kind!(js_kinds::L_ANGLE == JsSyntaxKind::L_ANGLE);
    assert_kind!(js_kinds::R_ANGLE == JsSyntaxKind::R_ANGLE);
    assert_kind!(js_kinds::DOT == JsSyntaxKind::DOT);
    assert_kind!(js_kinds::EQ2 == JsSyntaxKind::EQ2);
    assert_kind!(js_kinds::EQ3 == JsSyntaxKind::EQ3);
    assert_kind!(js_kinds::BANG == JsSyntaxKind::BANG);
    assert_kind!(js_kinds::NEQ == JsSyntaxKind::NEQ);
    assert_kind!(js_kinds::NEQ2 == JsSyntaxKind::NEQ2);
    assert_kind!(js_kinds::LTEQ == JsSyntaxKind::LTEQ);
    assert_kind!(js_kinds::GTEQ == JsSyntaxKind::GTEQ);

    // Keywords
    assert_kind!(js_kinds::CONST_KW == JsSyntaxKind::CONST_KW);
    assert_kind!(js_kinds::VAR_KW == JsSyntaxKind::VAR_KW);
    assert_kind!(js_kinds::LET_KW == JsSyntaxKind::LET_KW);
    assert_kind!(js_kinds::FALSE_KW == JsSyntaxKind::FALSE_KW);
    assert_kind!(js_kinds::TRUE_KW == JsSyntaxKind::TRUE_KW);

    // Identifiers
    assert_kind!(js_kinds::IDENT == JsSyntaxKind::IDENT);

    // Top-level
    assert_kind!(js_kinds::JS_MODULE == JsSyntaxKind::JS_MODULE);
    assert_kind!(js_kinds::JS_MODULE_ITEM_LIST == JsSyntaxKind::JS_MODULE_ITEM_LIST);

    // Statements
    assert_kind!(js_kinds::JS_BLOCK_STATEMENT == JsSyntaxKind::JS_BLOCK_STATEMENT);
    assert_kind!(js_kinds::JS_VARIABLE_STATEMENT == JsSyntaxKind::JS_VARIABLE_STATEMENT);
    assert_kind!(js_kinds::JS_VARIABLE_DECLARATION == JsSyntaxKind::JS_VARIABLE_DECLARATION);
    assert_kind!(
        js_kinds::JS_VARIABLE_DECLARATOR_LIST == JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST
    );
    assert_kind!(js_kinds::JS_VARIABLE_DECLARATOR == JsSyntaxKind::JS_VARIABLE_DECLARATOR);
    assert_kind!(js_kinds::JS_INITIALIZER_CLAUSE == JsSyntaxKind::JS_INITIALIZER_CLAUSE);
    assert_kind!(js_kinds::JS_EXPRESSION_STATEMENT == JsSyntaxKind::JS_EXPRESSION_STATEMENT);

    // Bindings
    assert_kind!(js_kinds::JS_IDENTIFIER_BINDING == JsSyntaxKind::JS_IDENTIFIER_BINDING);

    // Expressions
    assert_kind!(js_kinds::JS_IDENTIFIER_EXPRESSION == JsSyntaxKind::JS_IDENTIFIER_EXPRESSION);
    assert_kind!(js_kinds::JS_REFERENCE_IDENTIFIER == JsSyntaxKind::JS_REFERENCE_IDENTIFIER);
    assert_kind!(
        js_kinds::JS_STATIC_MEMBER_EXPRESSION == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
    );
    assert_kind!(
        js_kinds::JS_COMPUTED_MEMBER_EXPRESSION == JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
    );
    assert_kind!(js_kinds::JS_CALL_EXPRESSION == JsSyntaxKind::JS_CALL_EXPRESSION);
    assert_kind!(js_kinds::JS_UNARY_EXPRESSION == JsSyntaxKind::JS_UNARY_EXPRESSION);
    assert_kind!(js_kinds::JS_BINARY_EXPRESSION == JsSyntaxKind::JS_BINARY_EXPRESSION);
    assert_kind!(js_kinds::JS_INSTANCEOF_EXPRESSION == JsSyntaxKind::JS_INSTANCEOF_EXPRESSION);
    assert_kind!(js_kinds::JS_IN_EXPRESSION == JsSyntaxKind::JS_IN_EXPRESSION);
    assert_kind!(js_kinds::JS_LOGICAL_EXPRESSION == JsSyntaxKind::JS_LOGICAL_EXPRESSION);
    assert_kind!(
        js_kinds::JS_BOOLEAN_LITERAL_EXPRESSION == JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION
    );
}

// ---------------------------------------------------------------
// CSS
// ---------------------------------------------------------------

#[test]
fn css_kinds_match_css_syntax_kind() {
    assert_kind!(css_kinds::CSS_ROOT == CssSyntaxKind::CSS_ROOT);
    assert_kind!(css_kinds::CSS_ROOT_ITEM_LIST == CssSyntaxKind::CSS_ROOT_ITEM_LIST);
    assert_kind!(css_kinds::CSS_RULE_LIST == CssSyntaxKind::CSS_RULE_LIST);
    assert_kind!(css_kinds::CSS_QUALIFIED_RULE == CssSyntaxKind::CSS_QUALIFIED_RULE);
    assert_kind!(css_kinds::CSS_SELECTOR_LIST == CssSyntaxKind::CSS_SELECTOR_LIST);
    assert_kind!(css_kinds::CSS_DECLARATION_BLOCK == CssSyntaxKind::CSS_DECLARATION_BLOCK);
    assert_kind!(css_kinds::CSS_DECLARATION == CssSyntaxKind::CSS_DECLARATION);
}

// ---------------------------------------------------------------
// JSON
// ---------------------------------------------------------------

#[test]
fn json_kinds_match_json_syntax_kind() {
    assert_kind!(json_kinds::JSON_ROOT == JsonSyntaxKind::JSON_ROOT);
    assert_kind!(json_kinds::JSON_NUMBER_VALUE == JsonSyntaxKind::JSON_NUMBER_VALUE);
    assert_kind!(json_kinds::JSON_STRING_VALUE == JsonSyntaxKind::JSON_STRING_VALUE);
    assert_kind!(json_kinds::JSON_BOOLEAN_VALUE == JsonSyntaxKind::JSON_BOOLEAN_VALUE);
    assert_kind!(json_kinds::JSON_NULL_VALUE == JsonSyntaxKind::JSON_NULL_VALUE);
    assert_kind!(json_kinds::JSON_ARRAY_VALUE == JsonSyntaxKind::JSON_ARRAY_VALUE);
    assert_kind!(json_kinds::JSON_OBJECT_VALUE == JsonSyntaxKind::JSON_OBJECT_VALUE);
    assert_kind!(json_kinds::JSON_MEMBER_LIST == JsonSyntaxKind::JSON_MEMBER_LIST);
    assert_kind!(json_kinds::JSON_MEMBER == JsonSyntaxKind::JSON_MEMBER);
}
