//! Validates that the kind constants in `biome_plugin_sdk` match the actual
//! syntax kind enums in the corresponding `biome_*_syntax` crates.
//!
//! If a syntax kind enum is reordered or extended, these tests will catch the
//! mismatch at compile time or test time, preventing silent breakage in plugins.

use biome_css_syntax::CssSyntaxKind;
use biome_js_syntax::JsSyntaxKind;
use biome_json_syntax::JsonSyntaxKind;
use biome_plugin_sdk::{css_kinds, js_kinds, json_kinds};

#[test]
fn js_kind_constants_match_syntax_enum() {
    // Punctuation & operators
    assert_eq!(js_kinds::SEMICOLON, JsSyntaxKind::SEMICOLON as u32);
    assert_eq!(js_kinds::L_PAREN, JsSyntaxKind::L_PAREN as u32);
    assert_eq!(js_kinds::R_PAREN, JsSyntaxKind::R_PAREN as u32);
    assert_eq!(js_kinds::L_CURLY, JsSyntaxKind::L_CURLY as u32);
    assert_eq!(js_kinds::R_CURLY, JsSyntaxKind::R_CURLY as u32);
    assert_eq!(js_kinds::DOT, JsSyntaxKind::DOT as u32);
    assert_eq!(js_kinds::EQ2, JsSyntaxKind::EQ2 as u32);
    assert_eq!(js_kinds::EQ3, JsSyntaxKind::EQ3 as u32);
    assert_eq!(js_kinds::NEQ, JsSyntaxKind::NEQ as u32);
    assert_eq!(js_kinds::NEQ2, JsSyntaxKind::NEQ2 as u32);

    // Keywords
    assert_eq!(js_kinds::CONST_KW, JsSyntaxKind::CONST_KW as u32);
    assert_eq!(js_kinds::VAR_KW, JsSyntaxKind::VAR_KW as u32);
    assert_eq!(js_kinds::LET_KW, JsSyntaxKind::LET_KW as u32);
    assert_eq!(js_kinds::FALSE_KW, JsSyntaxKind::FALSE_KW as u32);
    assert_eq!(js_kinds::TRUE_KW, JsSyntaxKind::TRUE_KW as u32);

    // Identifiers & literals
    assert_eq!(js_kinds::IDENT, JsSyntaxKind::IDENT as u32);

    // Top-level
    assert_eq!(js_kinds::JS_MODULE, JsSyntaxKind::JS_MODULE as u32);
    assert_eq!(
        js_kinds::JS_MODULE_ITEM_LIST,
        JsSyntaxKind::JS_MODULE_ITEM_LIST as u32
    );

    // Statements
    assert_eq!(
        js_kinds::JS_BLOCK_STATEMENT,
        JsSyntaxKind::JS_BLOCK_STATEMENT as u32
    );
    assert_eq!(
        js_kinds::JS_VARIABLE_STATEMENT,
        JsSyntaxKind::JS_VARIABLE_STATEMENT as u32
    );
    assert_eq!(
        js_kinds::JS_VARIABLE_DECLARATION,
        JsSyntaxKind::JS_VARIABLE_DECLARATION as u32
    );
    assert_eq!(
        js_kinds::JS_VARIABLE_DECLARATOR_LIST,
        JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST as u32
    );
    assert_eq!(
        js_kinds::JS_VARIABLE_DECLARATOR,
        JsSyntaxKind::JS_VARIABLE_DECLARATOR as u32
    );
    assert_eq!(
        js_kinds::JS_INITIALIZER_CLAUSE,
        JsSyntaxKind::JS_INITIALIZER_CLAUSE as u32
    );
    assert_eq!(
        js_kinds::JS_EXPRESSION_STATEMENT,
        JsSyntaxKind::JS_EXPRESSION_STATEMENT as u32
    );

    // Bindings
    assert_eq!(
        js_kinds::JS_IDENTIFIER_BINDING,
        JsSyntaxKind::JS_IDENTIFIER_BINDING as u32
    );

    // Expressions
    assert_eq!(
        js_kinds::JS_IDENTIFIER_EXPRESSION,
        JsSyntaxKind::JS_IDENTIFIER_EXPRESSION as u32
    );
    assert_eq!(
        js_kinds::JS_REFERENCE_IDENTIFIER,
        JsSyntaxKind::JS_REFERENCE_IDENTIFIER as u32
    );
    assert_eq!(
        js_kinds::JS_STATIC_MEMBER_EXPRESSION,
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION as u32
    );
    assert_eq!(
        js_kinds::JS_COMPUTED_MEMBER_EXPRESSION,
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION as u32
    );
    assert_eq!(
        js_kinds::JS_CALL_EXPRESSION,
        JsSyntaxKind::JS_CALL_EXPRESSION as u32
    );
    assert_eq!(
        js_kinds::JS_BINARY_EXPRESSION,
        JsSyntaxKind::JS_BINARY_EXPRESSION as u32
    );
    assert_eq!(
        js_kinds::JS_BOOLEAN_LITERAL_EXPRESSION,
        JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION as u32
    );
}

#[test]
fn css_kind_constants_match_syntax_enum() {
    assert_eq!(css_kinds::CSS_ROOT, CssSyntaxKind::CSS_ROOT as u32);
    assert_eq!(
        css_kinds::CSS_ROOT_ITEM_LIST,
        CssSyntaxKind::CSS_ROOT_ITEM_LIST as u32
    );
    assert_eq!(
        css_kinds::CSS_RULE_LIST,
        CssSyntaxKind::CSS_RULE_LIST as u32
    );
    assert_eq!(
        css_kinds::CSS_QUALIFIED_RULE,
        CssSyntaxKind::CSS_QUALIFIED_RULE as u32
    );
    assert_eq!(
        css_kinds::CSS_SELECTOR_LIST,
        CssSyntaxKind::CSS_SELECTOR_LIST as u32
    );
    assert_eq!(
        css_kinds::CSS_DECLARATION_BLOCK,
        CssSyntaxKind::CSS_DECLARATION_BLOCK as u32
    );
    assert_eq!(
        css_kinds::CSS_DECLARATION,
        CssSyntaxKind::CSS_DECLARATION as u32
    );
}

#[test]
fn json_kind_constants_match_syntax_enum() {
    assert_eq!(json_kinds::JSON_ROOT, JsonSyntaxKind::JSON_ROOT as u32);
    assert_eq!(
        json_kinds::JSON_NUMBER_VALUE,
        JsonSyntaxKind::JSON_NUMBER_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_STRING_VALUE,
        JsonSyntaxKind::JSON_STRING_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_BOOLEAN_VALUE,
        JsonSyntaxKind::JSON_BOOLEAN_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_NULL_VALUE,
        JsonSyntaxKind::JSON_NULL_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_ARRAY_VALUE,
        JsonSyntaxKind::JSON_ARRAY_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_OBJECT_VALUE,
        JsonSyntaxKind::JSON_OBJECT_VALUE as u32
    );
    assert_eq!(
        json_kinds::JSON_MEMBER_LIST,
        JsonSyntaxKind::JSON_MEMBER_LIST as u32
    );
    assert_eq!(json_kinds::JSON_MEMBER, JsonSyntaxKind::JSON_MEMBER as u32);
}
