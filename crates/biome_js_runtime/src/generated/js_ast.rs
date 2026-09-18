// Generated file, do not edit by hand, see `xtask/codegen`

use crate::ast::{JsAstField, JsAstFieldValue, JsAstNode, JsAstNodeFields};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_rowan::AstNode;
impl JsAstNode {
    /// Resolves a constructible token kind by its exact native enum name.
    pub(crate) fn token_kind_from_name(name: &str) -> Option<JsSyntaxKind> {
        Some(match name {
            "SEMICOLON" => JsSyntaxKind::SEMICOLON,
            "COMMA" => JsSyntaxKind::COMMA,
            "L_PAREN" => JsSyntaxKind::L_PAREN,
            "R_PAREN" => JsSyntaxKind::R_PAREN,
            "L_CURLY" => JsSyntaxKind::L_CURLY,
            "R_CURLY" => JsSyntaxKind::R_CURLY,
            "L_BRACK" => JsSyntaxKind::L_BRACK,
            "R_BRACK" => JsSyntaxKind::R_BRACK,
            "L_ANGLE" => JsSyntaxKind::L_ANGLE,
            "R_ANGLE" => JsSyntaxKind::R_ANGLE,
            "TILDE" => JsSyntaxKind::TILDE,
            "QUESTION" => JsSyntaxKind::QUESTION,
            "QUESTION2" => JsSyntaxKind::QUESTION2,
            "QUESTIONDOT" => JsSyntaxKind::QUESTIONDOT,
            "AMP" => JsSyntaxKind::AMP,
            "PIPE" => JsSyntaxKind::PIPE,
            "PLUS" => JsSyntaxKind::PLUS,
            "PLUS2" => JsSyntaxKind::PLUS2,
            "STAR" => JsSyntaxKind::STAR,
            "STAR2" => JsSyntaxKind::STAR2,
            "SLASH" => JsSyntaxKind::SLASH,
            "CARET" => JsSyntaxKind::CARET,
            "PERCENT" => JsSyntaxKind::PERCENT,
            "DOT" => JsSyntaxKind::DOT,
            "DOT3" => JsSyntaxKind::DOT3,
            "COLON" => JsSyntaxKind::COLON,
            "EQ" => JsSyntaxKind::EQ,
            "EQ2" => JsSyntaxKind::EQ2,
            "EQ3" => JsSyntaxKind::EQ3,
            "FAT_ARROW" => JsSyntaxKind::FAT_ARROW,
            "BANG" => JsSyntaxKind::BANG,
            "NEQ" => JsSyntaxKind::NEQ,
            "NEQ2" => JsSyntaxKind::NEQ2,
            "MINUS" => JsSyntaxKind::MINUS,
            "MINUS2" => JsSyntaxKind::MINUS2,
            "LTEQ" => JsSyntaxKind::LTEQ,
            "GTEQ" => JsSyntaxKind::GTEQ,
            "PLUSEQ" => JsSyntaxKind::PLUSEQ,
            "MINUSEQ" => JsSyntaxKind::MINUSEQ,
            "PIPEEQ" => JsSyntaxKind::PIPEEQ,
            "AMPEQ" => JsSyntaxKind::AMPEQ,
            "CARETEQ" => JsSyntaxKind::CARETEQ,
            "SLASHEQ" => JsSyntaxKind::SLASHEQ,
            "STAREQ" => JsSyntaxKind::STAREQ,
            "PERCENTEQ" => JsSyntaxKind::PERCENTEQ,
            "AMP2" => JsSyntaxKind::AMP2,
            "PIPE2" => JsSyntaxKind::PIPE2,
            "SHL" => JsSyntaxKind::SHL,
            "SHR" => JsSyntaxKind::SHR,
            "USHR" => JsSyntaxKind::USHR,
            "SHLEQ" => JsSyntaxKind::SHLEQ,
            "SHREQ" => JsSyntaxKind::SHREQ,
            "USHREQ" => JsSyntaxKind::USHREQ,
            "AMP2EQ" => JsSyntaxKind::AMP2EQ,
            "PIPE2EQ" => JsSyntaxKind::PIPE2EQ,
            "STAR2EQ" => JsSyntaxKind::STAR2EQ,
            "QUESTION2EQ" => JsSyntaxKind::QUESTION2EQ,
            "AT" => JsSyntaxKind::AT,
            "BACKTICK" => JsSyntaxKind::BACKTICK,
            "BREAK_KW" => JsSyntaxKind::BREAK_KW,
            "CASE_KW" => JsSyntaxKind::CASE_KW,
            "CATCH_KW" => JsSyntaxKind::CATCH_KW,
            "CLASS_KW" => JsSyntaxKind::CLASS_KW,
            "CONST_KW" => JsSyntaxKind::CONST_KW,
            "CONTINUE_KW" => JsSyntaxKind::CONTINUE_KW,
            "DEBUGGER_KW" => JsSyntaxKind::DEBUGGER_KW,
            "DEFAULT_KW" => JsSyntaxKind::DEFAULT_KW,
            "DELETE_KW" => JsSyntaxKind::DELETE_KW,
            "DO_KW" => JsSyntaxKind::DO_KW,
            "ELSE_KW" => JsSyntaxKind::ELSE_KW,
            "ENUM_KW" => JsSyntaxKind::ENUM_KW,
            "EXPORT_KW" => JsSyntaxKind::EXPORT_KW,
            "EXTENDS_KW" => JsSyntaxKind::EXTENDS_KW,
            "FALSE_KW" => JsSyntaxKind::FALSE_KW,
            "FINALLY_KW" => JsSyntaxKind::FINALLY_KW,
            "FOR_KW" => JsSyntaxKind::FOR_KW,
            "FUNCTION_KW" => JsSyntaxKind::FUNCTION_KW,
            "IF_KW" => JsSyntaxKind::IF_KW,
            "IN_KW" => JsSyntaxKind::IN_KW,
            "INSTANCEOF_KW" => JsSyntaxKind::INSTANCEOF_KW,
            "IMPORT_KW" => JsSyntaxKind::IMPORT_KW,
            "NEW_KW" => JsSyntaxKind::NEW_KW,
            "NULL_KW" => JsSyntaxKind::NULL_KW,
            "RETURN_KW" => JsSyntaxKind::RETURN_KW,
            "SUPER_KW" => JsSyntaxKind::SUPER_KW,
            "SWITCH_KW" => JsSyntaxKind::SWITCH_KW,
            "THIS_KW" => JsSyntaxKind::THIS_KW,
            "THROW_KW" => JsSyntaxKind::THROW_KW,
            "TRY_KW" => JsSyntaxKind::TRY_KW,
            "TRUE_KW" => JsSyntaxKind::TRUE_KW,
            "TYPEOF_KW" => JsSyntaxKind::TYPEOF_KW,
            "VAR_KW" => JsSyntaxKind::VAR_KW,
            "VOID_KW" => JsSyntaxKind::VOID_KW,
            "WHILE_KW" => JsSyntaxKind::WHILE_KW,
            "WITH_KW" => JsSyntaxKind::WITH_KW,
            "IMPLEMENTS_KW" => JsSyntaxKind::IMPLEMENTS_KW,
            "INTERFACE_KW" => JsSyntaxKind::INTERFACE_KW,
            "LET_KW" => JsSyntaxKind::LET_KW,
            "PACKAGE_KW" => JsSyntaxKind::PACKAGE_KW,
            "PRIVATE_KW" => JsSyntaxKind::PRIVATE_KW,
            "PROTECTED_KW" => JsSyntaxKind::PROTECTED_KW,
            "PUBLIC_KW" => JsSyntaxKind::PUBLIC_KW,
            "STATIC_KW" => JsSyntaxKind::STATIC_KW,
            "YIELD_KW" => JsSyntaxKind::YIELD_KW,
            "ABSTRACT_KW" => JsSyntaxKind::ABSTRACT_KW,
            "ACCESSOR_KW" => JsSyntaxKind::ACCESSOR_KW,
            "AS_KW" => JsSyntaxKind::AS_KW,
            "SATISFIES_KW" => JsSyntaxKind::SATISFIES_KW,
            "ASSERTS_KW" => JsSyntaxKind::ASSERTS_KW,
            "ASSERT_KW" => JsSyntaxKind::ASSERT_KW,
            "ANY_KW" => JsSyntaxKind::ANY_KW,
            "ASYNC_KW" => JsSyntaxKind::ASYNC_KW,
            "AWAIT_KW" => JsSyntaxKind::AWAIT_KW,
            "BOOLEAN_KW" => JsSyntaxKind::BOOLEAN_KW,
            "CONSTRUCTOR_KW" => JsSyntaxKind::CONSTRUCTOR_KW,
            "DECLARE_KW" => JsSyntaxKind::DECLARE_KW,
            "DEFER_KW" => JsSyntaxKind::DEFER_KW,
            "GET_KW" => JsSyntaxKind::GET_KW,
            "INFER_KW" => JsSyntaxKind::INFER_KW,
            "IS_KW" => JsSyntaxKind::IS_KW,
            "KEYOF_KW" => JsSyntaxKind::KEYOF_KW,
            "MODULE_KW" => JsSyntaxKind::MODULE_KW,
            "NAMESPACE_KW" => JsSyntaxKind::NAMESPACE_KW,
            "NEVER_KW" => JsSyntaxKind::NEVER_KW,
            "READONLY_KW" => JsSyntaxKind::READONLY_KW,
            "REQUIRE_KW" => JsSyntaxKind::REQUIRE_KW,
            "NUMBER_KW" => JsSyntaxKind::NUMBER_KW,
            "OBJECT_KW" => JsSyntaxKind::OBJECT_KW,
            "SET_KW" => JsSyntaxKind::SET_KW,
            "STRING_KW" => JsSyntaxKind::STRING_KW,
            "SOURCE_KW" => JsSyntaxKind::SOURCE_KW,
            "SYMBOL_KW" => JsSyntaxKind::SYMBOL_KW,
            "TYPE_KW" => JsSyntaxKind::TYPE_KW,
            "UNDEFINED_KW" => JsSyntaxKind::UNDEFINED_KW,
            "UNIQUE_KW" => JsSyntaxKind::UNIQUE_KW,
            "UNKNOWN_KW" => JsSyntaxKind::UNKNOWN_KW,
            "FROM_KW" => JsSyntaxKind::FROM_KW,
            "GLOBAL_KW" => JsSyntaxKind::GLOBAL_KW,
            "BIGINT_KW" => JsSyntaxKind::BIGINT_KW,
            "OVERRIDE_KW" => JsSyntaxKind::OVERRIDE_KW,
            "OF_KW" => JsSyntaxKind::OF_KW,
            "OUT_KW" => JsSyntaxKind::OUT_KW,
            "USING_KW" => JsSyntaxKind::USING_KW,
            "JS_NUMBER_LITERAL" => JsSyntaxKind::JS_NUMBER_LITERAL,
            "JS_BIGINT_LITERAL" => JsSyntaxKind::JS_BIGINT_LITERAL,
            "JS_STRING_LITERAL" => JsSyntaxKind::JS_STRING_LITERAL,
            "JS_REGEX_LITERAL" => JsSyntaxKind::JS_REGEX_LITERAL,
            "JSX_TEXT_LITERAL" => JsSyntaxKind::JSX_TEXT_LITERAL,
            "JSX_STRING_LITERAL" => JsSyntaxKind::JSX_STRING_LITERAL,
            "TARGET" => JsSyntaxKind::TARGET,
            "META" => JsSyntaxKind::META,
            "HASH" => JsSyntaxKind::HASH,
            "TEMPLATE_CHUNK" => JsSyntaxKind::TEMPLATE_CHUNK,
            "DOLLAR_CURLY" => JsSyntaxKind::DOLLAR_CURLY,
            "IDENT" => JsSyntaxKind::IDENT,
            "JSX_IDENT" => JsSyntaxKind::JSX_IDENT,
            "JS_SHEBANG" => JsSyntaxKind::JS_SHEBANG,
            _ => return None,
        })
    }
    /// Resolves a syntax kind from the name used in the plugin API type definitions,
    /// e.g. `"JS_CALL_EXPRESSION"`.
    pub(crate) fn syntax_kind_from_ast_name(name: &str) -> Option<JsSyntaxKind> {
        Some(match name {
            "ASTRO_IMPLICIT_FRAGMENT" => JsSyntaxKind::ASTRO_IMPLICIT_FRAGMENT,
            "JS_ACCESSOR_MODIFIER" => JsSyntaxKind::JS_ACCESSOR_MODIFIER,
            "JS_ARRAY_ASSIGNMENT_PATTERN" => JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN,
            "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT" => {
                JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT
            }
            "JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT" => {
                JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
            }
            "JS_ARRAY_BINDING_PATTERN" => JsSyntaxKind::JS_ARRAY_BINDING_PATTERN,
            "JS_ARRAY_BINDING_PATTERN_ELEMENT" => JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT,
            "JS_ARRAY_BINDING_PATTERN_REST_ELEMENT" => {
                JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
            }
            "JS_ARRAY_EXPRESSION" => JsSyntaxKind::JS_ARRAY_EXPRESSION,
            "JS_ARRAY_HOLE" => JsSyntaxKind::JS_ARRAY_HOLE,
            "JS_ARROW_FUNCTION_EXPRESSION" => JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
            "JS_ASSIGNMENT_EXPRESSION" => JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION,
            "JS_AWAIT_EXPRESSION" => JsSyntaxKind::JS_AWAIT_EXPRESSION,
            "JS_BIGINT_LITERAL_EXPRESSION" => JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION,
            "JS_BINARY_EXPRESSION" => JsSyntaxKind::JS_BINARY_EXPRESSION,
            "JS_BLOCK_STATEMENT" => JsSyntaxKind::JS_BLOCK_STATEMENT,
            "JS_BOOLEAN_LITERAL_EXPRESSION" => JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION,
            "JS_BREAK_STATEMENT" => JsSyntaxKind::JS_BREAK_STATEMENT,
            "JS_CALL_ARGUMENTS" => JsSyntaxKind::JS_CALL_ARGUMENTS,
            "JS_CALL_EXPRESSION" => JsSyntaxKind::JS_CALL_EXPRESSION,
            "JS_CASE_CLAUSE" => JsSyntaxKind::JS_CASE_CLAUSE,
            "JS_CATCH_CLAUSE" => JsSyntaxKind::JS_CATCH_CLAUSE,
            "JS_CATCH_DECLARATION" => JsSyntaxKind::JS_CATCH_DECLARATION,
            "JS_CLASS_DECLARATION" => JsSyntaxKind::JS_CLASS_DECLARATION,
            "JS_CLASS_EXPORT_DEFAULT_DECLARATION" => {
                JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION
            }
            "JS_CLASS_EXPRESSION" => JsSyntaxKind::JS_CLASS_EXPRESSION,
            "JS_COMPUTED_MEMBER_ASSIGNMENT" => JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT,
            "JS_COMPUTED_MEMBER_EXPRESSION" => JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION,
            "JS_COMPUTED_MEMBER_NAME" => JsSyntaxKind::JS_COMPUTED_MEMBER_NAME,
            "JS_CONDITIONAL_EXPRESSION" => JsSyntaxKind::JS_CONDITIONAL_EXPRESSION,
            "JS_CONSTRUCTOR_CLASS_MEMBER" => JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER,
            "JS_CONSTRUCTOR_PARAMETERS" => JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS,
            "JS_CONTINUE_STATEMENT" => JsSyntaxKind::JS_CONTINUE_STATEMENT,
            "JS_DEBUGGER_STATEMENT" => JsSyntaxKind::JS_DEBUGGER_STATEMENT,
            "JS_DECORATOR" => JsSyntaxKind::JS_DECORATOR,
            "JS_DEFAULT_CLAUSE" => JsSyntaxKind::JS_DEFAULT_CLAUSE,
            "JS_DEFAULT_IMPORT_SPECIFIER" => JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER,
            "JS_DIRECTIVE" => JsSyntaxKind::JS_DIRECTIVE,
            "JS_DO_WHILE_STATEMENT" => JsSyntaxKind::JS_DO_WHILE_STATEMENT,
            "JS_ELSE_CLAUSE" => JsSyntaxKind::JS_ELSE_CLAUSE,
            "JS_EMPTY_CLASS_MEMBER" => JsSyntaxKind::JS_EMPTY_CLASS_MEMBER,
            "JS_EMPTY_STATEMENT" => JsSyntaxKind::JS_EMPTY_STATEMENT,
            "JS_EXPORT" => JsSyntaxKind::JS_EXPORT,
            "JS_EXPORT_AS_CLAUSE" => JsSyntaxKind::JS_EXPORT_AS_CLAUSE,
            "JS_EXPORT_DEFAULT_DECLARATION_CLAUSE" => {
                JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE
            }
            "JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE" => {
                JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
            }
            "JS_EXPORT_FROM_CLAUSE" => JsSyntaxKind::JS_EXPORT_FROM_CLAUSE,
            "JS_EXPORT_NAMED_CLAUSE" => JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE,
            "JS_EXPORT_NAMED_FROM_CLAUSE" => JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE,
            "JS_EXPORT_NAMED_FROM_SPECIFIER" => JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER,
            "JS_EXPORT_NAMED_SHORTHAND_SPECIFIER" => {
                JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER
            }
            "JS_EXPORT_NAMED_SPECIFIER" => JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER,
            "JS_EXPRESSION_SNIPPET" => JsSyntaxKind::JS_EXPRESSION_SNIPPET,
            "JS_EXPRESSION_STATEMENT" => JsSyntaxKind::JS_EXPRESSION_STATEMENT,
            "JS_EXPRESSION_TEMPLATE_ROOT" => JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT,
            "JS_EXTENDS_CLAUSE" => JsSyntaxKind::JS_EXTENDS_CLAUSE,
            "JS_FINALLY_CLAUSE" => JsSyntaxKind::JS_FINALLY_CLAUSE,
            "JS_FOR_IN_STATEMENT" => JsSyntaxKind::JS_FOR_IN_STATEMENT,
            "JS_FOR_OF_STATEMENT" => JsSyntaxKind::JS_FOR_OF_STATEMENT,
            "JS_FOR_STATEMENT" => JsSyntaxKind::JS_FOR_STATEMENT,
            "JS_FOR_VARIABLE_DECLARATION" => JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION,
            "JS_FORMAL_PARAMETER" => JsSyntaxKind::JS_FORMAL_PARAMETER,
            "JS_FUNCTION_BODY" => JsSyntaxKind::JS_FUNCTION_BODY,
            "JS_FUNCTION_DECLARATION" => JsSyntaxKind::JS_FUNCTION_DECLARATION,
            "JS_FUNCTION_EXPORT_DEFAULT_DECLARATION" => {
                JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            }
            "JS_FUNCTION_EXPRESSION" => JsSyntaxKind::JS_FUNCTION_EXPRESSION,
            "JS_GETTER_CLASS_MEMBER" => JsSyntaxKind::JS_GETTER_CLASS_MEMBER,
            "JS_GETTER_OBJECT_MEMBER" => JsSyntaxKind::JS_GETTER_OBJECT_MEMBER,
            "JS_IDENTIFIER_ASSIGNMENT" => JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT,
            "JS_IDENTIFIER_BINDING" => JsSyntaxKind::JS_IDENTIFIER_BINDING,
            "JS_IDENTIFIER_EXPRESSION" => JsSyntaxKind::JS_IDENTIFIER_EXPRESSION,
            "JS_IF_STATEMENT" => JsSyntaxKind::JS_IF_STATEMENT,
            "JS_IMPORT" => JsSyntaxKind::JS_IMPORT,
            "JS_IMPORT_ASSERTION" => JsSyntaxKind::JS_IMPORT_ASSERTION,
            "JS_IMPORT_ASSERTION_ENTRY" => JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY,
            "JS_IMPORT_BARE_CLAUSE" => JsSyntaxKind::JS_IMPORT_BARE_CLAUSE,
            "JS_IMPORT_CALL_EXPRESSION" => JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION,
            "JS_IMPORT_COMBINED_CLAUSE" => JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE,
            "JS_IMPORT_DEFAULT_CLAUSE" => JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE,
            "JS_IMPORT_META_EXPRESSION" => JsSyntaxKind::JS_IMPORT_META_EXPRESSION,
            "JS_IMPORT_NAMED_CLAUSE" => JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE,
            "JS_IMPORT_NAMESPACE_CLAUSE" => JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE,
            "JS_IN_EXPRESSION" => JsSyntaxKind::JS_IN_EXPRESSION,
            "JS_INITIALIZER_CLAUSE" => JsSyntaxKind::JS_INITIALIZER_CLAUSE,
            "JS_INSTANCEOF_EXPRESSION" => JsSyntaxKind::JS_INSTANCEOF_EXPRESSION,
            "JS_LABEL" => JsSyntaxKind::JS_LABEL,
            "JS_LABELED_STATEMENT" => JsSyntaxKind::JS_LABELED_STATEMENT,
            "JS_LITERAL_EXPORT_NAME" => JsSyntaxKind::JS_LITERAL_EXPORT_NAME,
            "JS_LITERAL_MEMBER_NAME" => JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
            "JS_LOGICAL_EXPRESSION" => JsSyntaxKind::JS_LOGICAL_EXPRESSION,
            "JS_METAVARIABLE" => JsSyntaxKind::JS_METAVARIABLE,
            "JS_METHOD_CLASS_MEMBER" => JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
            "JS_METHOD_OBJECT_MEMBER" => JsSyntaxKind::JS_METHOD_OBJECT_MEMBER,
            "JS_MODULE" => JsSyntaxKind::JS_MODULE,
            "JS_MODULE_SOURCE" => JsSyntaxKind::JS_MODULE_SOURCE,
            "JS_NAME" => JsSyntaxKind::JS_NAME,
            "JS_NAMED_IMPORT_SPECIFIER" => JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER,
            "JS_NAMED_IMPORT_SPECIFIERS" => JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS,
            "JS_NAMESPACE_IMPORT_SPECIFIER" => JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER,
            "JS_NEW_EXPRESSION" => JsSyntaxKind::JS_NEW_EXPRESSION,
            "JS_NEW_TARGET_EXPRESSION" => JsSyntaxKind::JS_NEW_TARGET_EXPRESSION,
            "JS_NULL_LITERAL_EXPRESSION" => JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
            "JS_NUMBER_LITERAL_EXPRESSION" => JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
            "JS_OBJECT_ASSIGNMENT_PATTERN" => JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN,
            "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY" => {
                JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
            }
            "JS_OBJECT_ASSIGNMENT_PATTERN_REST" => JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST,
            "JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY" => {
                JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
            }
            "JS_OBJECT_BINDING_PATTERN" => JsSyntaxKind::JS_OBJECT_BINDING_PATTERN,
            "JS_OBJECT_BINDING_PATTERN_PROPERTY" => {
                JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY
            }
            "JS_OBJECT_BINDING_PATTERN_REST" => JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST,
            "JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY" => {
                JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            }
            "JS_OBJECT_EXPRESSION" => JsSyntaxKind::JS_OBJECT_EXPRESSION,
            "JS_PARAMETERS" => JsSyntaxKind::JS_PARAMETERS,
            "JS_PARENTHESIZED_ASSIGNMENT" => JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT,
            "JS_PARENTHESIZED_EXPRESSION" => JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION,
            "JS_POST_UPDATE_EXPRESSION" => JsSyntaxKind::JS_POST_UPDATE_EXPRESSION,
            "JS_PRE_UPDATE_EXPRESSION" => JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION,
            "JS_PRIVATE_CLASS_MEMBER_NAME" => JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME,
            "JS_PRIVATE_NAME" => JsSyntaxKind::JS_PRIVATE_NAME,
            "JS_PROPERTY_CLASS_MEMBER" => JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER,
            "JS_PROPERTY_OBJECT_MEMBER" => JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER,
            "JS_REFERENCE_IDENTIFIER" => JsSyntaxKind::JS_REFERENCE_IDENTIFIER,
            "JS_REGEX_LITERAL_EXPRESSION" => JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
            "JS_REST_PARAMETER" => JsSyntaxKind::JS_REST_PARAMETER,
            "JS_RETURN_STATEMENT" => JsSyntaxKind::JS_RETURN_STATEMENT,
            "JS_SCRIPT" => JsSyntaxKind::JS_SCRIPT,
            "JS_SEQUENCE_EXPRESSION" => JsSyntaxKind::JS_SEQUENCE_EXPRESSION,
            "JS_SETTER_CLASS_MEMBER" => JsSyntaxKind::JS_SETTER_CLASS_MEMBER,
            "JS_SETTER_OBJECT_MEMBER" => JsSyntaxKind::JS_SETTER_OBJECT_MEMBER,
            "JS_SHORTHAND_NAMED_IMPORT_SPECIFIER" => {
                JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
            }
            "JS_SHORTHAND_PROPERTY_OBJECT_MEMBER" => {
                JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
            }
            "JS_SPREAD" => JsSyntaxKind::JS_SPREAD,
            "JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER" => {
                JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            }
            "JS_STATIC_MEMBER_ASSIGNMENT" => JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
            "JS_STATIC_MEMBER_EXPRESSION" => JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION,
            "JS_STATIC_MODIFIER" => JsSyntaxKind::JS_STATIC_MODIFIER,
            "JS_STRING_LITERAL_EXPRESSION" => JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
            "JS_SUPER_EXPRESSION" => JsSyntaxKind::JS_SUPER_EXPRESSION,
            "JS_SVELTE_DECLARATION_ROOT" => JsSyntaxKind::JS_SVELTE_DECLARATION_ROOT,
            "JS_SVELTE_SNIPPET_ROOT" => JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT,
            "JS_SWITCH_STATEMENT" => JsSyntaxKind::JS_SWITCH_STATEMENT,
            "JS_TEMPLATE_CHUNK_ELEMENT" => JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT,
            "JS_TEMPLATE_ELEMENT" => JsSyntaxKind::JS_TEMPLATE_ELEMENT,
            "JS_TEMPLATE_EXPRESSION" => JsSyntaxKind::JS_TEMPLATE_EXPRESSION,
            "JS_THIS_EXPRESSION" => JsSyntaxKind::JS_THIS_EXPRESSION,
            "JS_THROW_STATEMENT" => JsSyntaxKind::JS_THROW_STATEMENT,
            "JS_TRY_FINALLY_STATEMENT" => JsSyntaxKind::JS_TRY_FINALLY_STATEMENT,
            "JS_TRY_STATEMENT" => JsSyntaxKind::JS_TRY_STATEMENT,
            "JS_UNARY_EXPRESSION" => JsSyntaxKind::JS_UNARY_EXPRESSION,
            "JS_VARIABLE_DECLARATION" => JsSyntaxKind::JS_VARIABLE_DECLARATION,
            "JS_VARIABLE_DECLARATION_CLAUSE" => JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE,
            "JS_VARIABLE_DECLARATOR" => JsSyntaxKind::JS_VARIABLE_DECLARATOR,
            "JS_VARIABLE_STATEMENT" => JsSyntaxKind::JS_VARIABLE_STATEMENT,
            "JS_WHILE_STATEMENT" => JsSyntaxKind::JS_WHILE_STATEMENT,
            "JS_WITH_STATEMENT" => JsSyntaxKind::JS_WITH_STATEMENT,
            "JS_YIELD_ARGUMENT" => JsSyntaxKind::JS_YIELD_ARGUMENT,
            "JS_YIELD_EXPRESSION" => JsSyntaxKind::JS_YIELD_EXPRESSION,
            "JSX_ATTRIBUTE" => JsSyntaxKind::JSX_ATTRIBUTE,
            "JSX_ATTRIBUTE_INITIALIZER_CLAUSE" => JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE,
            "JSX_CLOSING_ELEMENT" => JsSyntaxKind::JSX_CLOSING_ELEMENT,
            "JSX_CLOSING_FRAGMENT" => JsSyntaxKind::JSX_CLOSING_FRAGMENT,
            "JSX_ELEMENT" => JsSyntaxKind::JSX_ELEMENT,
            "JSX_EXPRESSION_ATTRIBUTE_VALUE" => JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE,
            "JSX_EXPRESSION_CHILD" => JsSyntaxKind::JSX_EXPRESSION_CHILD,
            "JSX_FRAGMENT" => JsSyntaxKind::JSX_FRAGMENT,
            "JSX_MEMBER_NAME" => JsSyntaxKind::JSX_MEMBER_NAME,
            "JSX_NAME" => JsSyntaxKind::JSX_NAME,
            "JSX_NAMESPACE_NAME" => JsSyntaxKind::JSX_NAMESPACE_NAME,
            "JSX_OPENING_ELEMENT" => JsSyntaxKind::JSX_OPENING_ELEMENT,
            "JSX_OPENING_FRAGMENT" => JsSyntaxKind::JSX_OPENING_FRAGMENT,
            "JSX_REFERENCE_IDENTIFIER" => JsSyntaxKind::JSX_REFERENCE_IDENTIFIER,
            "JSX_SELF_CLOSING_ELEMENT" => JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT,
            "JSX_SHORTHAND_ATTRIBUTE" => JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE,
            "JSX_SPREAD_ATTRIBUTE" => JsSyntaxKind::JSX_SPREAD_ATTRIBUTE,
            "JSX_SPREAD_CHILD" => JsSyntaxKind::JSX_SPREAD_CHILD,
            "JSX_STRING" => JsSyntaxKind::JSX_STRING,
            "JSX_TAG_EXPRESSION" => JsSyntaxKind::JSX_TAG_EXPRESSION,
            "JSX_TEXT" => JsSyntaxKind::JSX_TEXT,
            "TS_ABSTRACT_MODIFIER" => JsSyntaxKind::TS_ABSTRACT_MODIFIER,
            "TS_ACCESSIBILITY_MODIFIER" => JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER,
            "TS_ANY_TYPE" => JsSyntaxKind::TS_ANY_TYPE,
            "TS_ARRAY_TYPE" => JsSyntaxKind::TS_ARRAY_TYPE,
            "TS_AS_ASSIGNMENT" => JsSyntaxKind::TS_AS_ASSIGNMENT,
            "TS_AS_EXPRESSION" => JsSyntaxKind::TS_AS_EXPRESSION,
            "TS_ASSERTS_CONDITION" => JsSyntaxKind::TS_ASSERTS_CONDITION,
            "TS_ASSERTS_RETURN_TYPE" => JsSyntaxKind::TS_ASSERTS_RETURN_TYPE,
            "TS_BIGINT_LITERAL_TYPE" => JsSyntaxKind::TS_BIGINT_LITERAL_TYPE,
            "TS_BIGINT_TYPE" => JsSyntaxKind::TS_BIGINT_TYPE,
            "TS_BOOLEAN_LITERAL_TYPE" => JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE,
            "TS_BOOLEAN_TYPE" => JsSyntaxKind::TS_BOOLEAN_TYPE,
            "TS_CALL_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER,
            "TS_CONDITIONAL_TYPE" => JsSyntaxKind::TS_CONDITIONAL_TYPE,
            "TS_CONST_MODIFIER" => JsSyntaxKind::TS_CONST_MODIFIER,
            "TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER" => {
                JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER
            }
            "TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER" => {
                JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
            }
            "TS_CONSTRUCTOR_TYPE" => JsSyntaxKind::TS_CONSTRUCTOR_TYPE,
            "TS_DECLARATION_MODULE" => JsSyntaxKind::TS_DECLARATION_MODULE,
            "TS_DECLARE_FUNCTION_DECLARATION" => JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
            "TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION" => {
                JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION
            }
            "TS_DECLARE_MODIFIER" => JsSyntaxKind::TS_DECLARE_MODIFIER,
            "TS_DECLARE_STATEMENT" => JsSyntaxKind::TS_DECLARE_STATEMENT,
            "TS_DEFAULT_TYPE_CLAUSE" => JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE,
            "TS_DEFINITE_PROPERTY_ANNOTATION" => JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION,
            "TS_DEFINITE_VARIABLE_ANNOTATION" => JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION,
            "TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY" => {
                JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY
            }
            "TS_ENUM_DECLARATION" => JsSyntaxKind::TS_ENUM_DECLARATION,
            "TS_ENUM_MEMBER" => JsSyntaxKind::TS_ENUM_MEMBER,
            "TS_EXPORT_AS_NAMESPACE_CLAUSE" => JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE,
            "TS_EXPORT_ASSIGNMENT_CLAUSE" => JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE,
            "TS_EXPORT_DECLARE_CLAUSE" => JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE,
            "TS_EXTENDS_CLAUSE" => JsSyntaxKind::TS_EXTENDS_CLAUSE,
            "TS_EXTERNAL_MODULE_DECLARATION" => JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION,
            "TS_EXTERNAL_MODULE_REFERENCE" => JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE,
            "TS_FUNCTION_TYPE" => JsSyntaxKind::TS_FUNCTION_TYPE,
            "TS_GETTER_SIGNATURE_CLASS_MEMBER" => JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER,
            "TS_GETTER_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER,
            "TS_GLOBAL_DECLARATION" => JsSyntaxKind::TS_GLOBAL_DECLARATION,
            "TS_IDENTIFIER_BINDING" => JsSyntaxKind::TS_IDENTIFIER_BINDING,
            "TS_IMPLEMENTS_CLAUSE" => JsSyntaxKind::TS_IMPLEMENTS_CLAUSE,
            "TS_IMPORT_EQUALS_DECLARATION" => JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION,
            "TS_IMPORT_TYPE" => JsSyntaxKind::TS_IMPORT_TYPE,
            "TS_IMPORT_TYPE_ARGUMENTS" => JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS,
            "TS_IMPORT_TYPE_ASSERTION" => JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION,
            "TS_IMPORT_TYPE_ASSERTION_BLOCK" => JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK,
            "TS_IMPORT_TYPE_QUALIFIER" => JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER,
            "TS_IN_MODIFIER" => JsSyntaxKind::TS_IN_MODIFIER,
            "TS_INDEX_SIGNATURE_CLASS_MEMBER" => JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER,
            "TS_INDEX_SIGNATURE_PARAMETER" => JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER,
            "TS_INDEX_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER,
            "TS_INDEXED_ACCESS_TYPE" => JsSyntaxKind::TS_INDEXED_ACCESS_TYPE,
            "TS_INFER_TYPE" => JsSyntaxKind::TS_INFER_TYPE,
            "TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER" => {
                JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
            }
            "TS_INSTANTIATION_EXPRESSION" => JsSyntaxKind::TS_INSTANTIATION_EXPRESSION,
            "TS_INTERFACE_DECLARATION" => JsSyntaxKind::TS_INTERFACE_DECLARATION,
            "TS_INTERSECTION_TYPE" => JsSyntaxKind::TS_INTERSECTION_TYPE,
            "TS_LITERAL_ENUM_MEMBER_NAME" => JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME,
            "TS_MAPPED_TYPE" => JsSyntaxKind::TS_MAPPED_TYPE,
            "TS_MAPPED_TYPE_AS_CLAUSE" => JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE,
            "TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE" => {
                JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE
            }
            "TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE" => {
                JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE
            }
            "TS_METHOD_SIGNATURE_CLASS_MEMBER" => JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER,
            "TS_METHOD_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER,
            "TS_MODULE_BLOCK" => JsSyntaxKind::TS_MODULE_BLOCK,
            "TS_MODULE_DECLARATION" => JsSyntaxKind::TS_MODULE_DECLARATION,
            "TS_NAMED_TUPLE_TYPE_ELEMENT" => JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT,
            "TS_NEVER_TYPE" => JsSyntaxKind::TS_NEVER_TYPE,
            "TS_NON_NULL_ASSERTION_ASSIGNMENT" => JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT,
            "TS_NON_NULL_ASSERTION_EXPRESSION" => JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION,
            "TS_NON_PRIMITIVE_TYPE" => JsSyntaxKind::TS_NON_PRIMITIVE_TYPE,
            "TS_NULL_LITERAL_TYPE" => JsSyntaxKind::TS_NULL_LITERAL_TYPE,
            "TS_NUMBER_LITERAL_TYPE" => JsSyntaxKind::TS_NUMBER_LITERAL_TYPE,
            "TS_NUMBER_TYPE" => JsSyntaxKind::TS_NUMBER_TYPE,
            "TS_OBJECT_TYPE" => JsSyntaxKind::TS_OBJECT_TYPE,
            "TS_OPTIONAL_PROPERTY_ANNOTATION" => JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION,
            "TS_OPTIONAL_TUPLE_TYPE_ELEMENT" => JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT,
            "TS_OUT_MODIFIER" => JsSyntaxKind::TS_OUT_MODIFIER,
            "TS_OVERRIDE_MODIFIER" => JsSyntaxKind::TS_OVERRIDE_MODIFIER,
            "TS_PARENTHESIZED_TYPE" => JsSyntaxKind::TS_PARENTHESIZED_TYPE,
            "TS_PREDICATE_RETURN_TYPE" => JsSyntaxKind::TS_PREDICATE_RETURN_TYPE,
            "TS_PROPERTY_PARAMETER" => JsSyntaxKind::TS_PROPERTY_PARAMETER,
            "TS_PROPERTY_SIGNATURE_CLASS_MEMBER" => {
                JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER
            }
            "TS_PROPERTY_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER,
            "TS_QUALIFIED_MODULE_NAME" => JsSyntaxKind::TS_QUALIFIED_MODULE_NAME,
            "TS_QUALIFIED_NAME" => JsSyntaxKind::TS_QUALIFIED_NAME,
            "TS_READONLY_MODIFIER" => JsSyntaxKind::TS_READONLY_MODIFIER,
            "TS_REFERENCE_TYPE" => JsSyntaxKind::TS_REFERENCE_TYPE,
            "TS_REST_TUPLE_TYPE_ELEMENT" => JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT,
            "TS_RETURN_TYPE_ANNOTATION" => JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION,
            "TS_SATISFIES_ASSIGNMENT" => JsSyntaxKind::TS_SATISFIES_ASSIGNMENT,
            "TS_SATISFIES_EXPRESSION" => JsSyntaxKind::TS_SATISFIES_EXPRESSION,
            "TS_SETTER_SIGNATURE_CLASS_MEMBER" => JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER,
            "TS_SETTER_SIGNATURE_TYPE_MEMBER" => JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER,
            "TS_STRING_LITERAL_TYPE" => JsSyntaxKind::TS_STRING_LITERAL_TYPE,
            "TS_STRING_TYPE" => JsSyntaxKind::TS_STRING_TYPE,
            "TS_SYMBOL_TYPE" => JsSyntaxKind::TS_SYMBOL_TYPE,
            "TS_TEMPLATE_CHUNK_ELEMENT" => JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT,
            "TS_TEMPLATE_ELEMENT" => JsSyntaxKind::TS_TEMPLATE_ELEMENT,
            "TS_TEMPLATE_LITERAL_TYPE" => JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE,
            "TS_THIS_PARAMETER" => JsSyntaxKind::TS_THIS_PARAMETER,
            "TS_THIS_TYPE" => JsSyntaxKind::TS_THIS_TYPE,
            "TS_TUPLE_TYPE" => JsSyntaxKind::TS_TUPLE_TYPE,
            "TS_TYPE_ALIAS_DECLARATION" => JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION,
            "TS_TYPE_ANNOTATION" => JsSyntaxKind::TS_TYPE_ANNOTATION,
            "TS_TYPE_ARGUMENTS" => JsSyntaxKind::TS_TYPE_ARGUMENTS,
            "TS_TYPE_ASSERTION_ASSIGNMENT" => JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT,
            "TS_TYPE_ASSERTION_EXPRESSION" => JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION,
            "TS_TYPE_CONSTRAINT_CLAUSE" => JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE,
            "TS_TYPE_OPERATOR_TYPE" => JsSyntaxKind::TS_TYPE_OPERATOR_TYPE,
            "TS_TYPE_PARAMETER" => JsSyntaxKind::TS_TYPE_PARAMETER,
            "TS_TYPE_PARAMETER_NAME" => JsSyntaxKind::TS_TYPE_PARAMETER_NAME,
            "TS_TYPE_PARAMETERS" => JsSyntaxKind::TS_TYPE_PARAMETERS,
            "TS_TYPEOF_TYPE" => JsSyntaxKind::TS_TYPEOF_TYPE,
            "TS_UNDEFINED_TYPE" => JsSyntaxKind::TS_UNDEFINED_TYPE,
            "TS_UNION_TYPE" => JsSyntaxKind::TS_UNION_TYPE,
            "TS_UNKNOWN_TYPE" => JsSyntaxKind::TS_UNKNOWN_TYPE,
            "TS_VOID_TYPE" => JsSyntaxKind::TS_VOID_TYPE,
            "JS_BOGUS" => JsSyntaxKind::JS_BOGUS,
            "JS_BOGUS_ASSIGNMENT" => JsSyntaxKind::JS_BOGUS_ASSIGNMENT,
            "JS_BOGUS_BINDING" => JsSyntaxKind::JS_BOGUS_BINDING,
            "JS_BOGUS_EXPRESSION" => JsSyntaxKind::JS_BOGUS_EXPRESSION,
            "JS_BOGUS_IMPORT_ASSERTION_ENTRY" => JsSyntaxKind::JS_BOGUS_IMPORT_ASSERTION_ENTRY,
            "JS_BOGUS_MEMBER" => JsSyntaxKind::JS_BOGUS_MEMBER,
            "JS_BOGUS_NAMED_IMPORT_SPECIFIER" => JsSyntaxKind::JS_BOGUS_NAMED_IMPORT_SPECIFIER,
            "JS_BOGUS_PARAMETER" => JsSyntaxKind::JS_BOGUS_PARAMETER,
            "JS_BOGUS_STATEMENT" => JsSyntaxKind::JS_BOGUS_STATEMENT,
            "JS_BOGUS_VARIABLE_DECLARATION" => JsSyntaxKind::JS_BOGUS_VARIABLE_DECLARATION,
            "TS_BOGUS_TYPE" => JsSyntaxKind::TS_BOGUS_TYPE,
            "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST" => {
                JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST
            }
            "JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST" => {
                JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
            }
            "JS_ARRAY_ELEMENT_LIST" => JsSyntaxKind::JS_ARRAY_ELEMENT_LIST,
            "JS_CALL_ARGUMENT_LIST" => JsSyntaxKind::JS_CALL_ARGUMENT_LIST,
            "JS_CLASS_MEMBER_LIST" => JsSyntaxKind::JS_CLASS_MEMBER_LIST,
            "JS_CONSTRUCTOR_MODIFIER_LIST" => JsSyntaxKind::JS_CONSTRUCTOR_MODIFIER_LIST,
            "JS_CONSTRUCTOR_PARAMETER_LIST" => JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST,
            "JS_DECORATOR_LIST" => JsSyntaxKind::JS_DECORATOR_LIST,
            "JS_DIRECTIVE_LIST" => JsSyntaxKind::JS_DIRECTIVE_LIST,
            "JS_EXPORT_NAMED_FROM_SPECIFIER_LIST" => {
                JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER_LIST
            }
            "JS_EXPORT_NAMED_SPECIFIER_LIST" => JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER_LIST,
            "JS_IMPORT_ASSERTION_ENTRY_LIST" => JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY_LIST,
            "JS_METHOD_MODIFIER_LIST" => JsSyntaxKind::JS_METHOD_MODIFIER_LIST,
            "JS_MODULE_ITEM_LIST" => JsSyntaxKind::JS_MODULE_ITEM_LIST,
            "JS_NAMED_IMPORT_SPECIFIER_LIST" => JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER_LIST,
            "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST" => {
                JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST
            }
            "JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST" => {
                JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
            }
            "JS_OBJECT_MEMBER_LIST" => JsSyntaxKind::JS_OBJECT_MEMBER_LIST,
            "JS_PARAMETER_LIST" => JsSyntaxKind::JS_PARAMETER_LIST,
            "JS_PROPERTY_MODIFIER_LIST" => JsSyntaxKind::JS_PROPERTY_MODIFIER_LIST,
            "JS_STATEMENT_LIST" => JsSyntaxKind::JS_STATEMENT_LIST,
            "JS_SWITCH_CASE_LIST" => JsSyntaxKind::JS_SWITCH_CASE_LIST,
            "JS_TEMPLATE_ELEMENT_LIST" => JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST,
            "JS_VARIABLE_DECLARATOR_LIST" => JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST,
            "JSX_ATTRIBUTE_LIST" => JsSyntaxKind::JSX_ATTRIBUTE_LIST,
            "JSX_CHILD_LIST" => JsSyntaxKind::JSX_CHILD_LIST,
            "TS_ENUM_MEMBER_LIST" => JsSyntaxKind::TS_ENUM_MEMBER_LIST,
            "TS_INDEX_SIGNATURE_MODIFIER_LIST" => JsSyntaxKind::TS_INDEX_SIGNATURE_MODIFIER_LIST,
            "TS_INTERSECTION_TYPE_ELEMENT_LIST" => JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST,
            "TS_METHOD_SIGNATURE_MODIFIER_LIST" => JsSyntaxKind::TS_METHOD_SIGNATURE_MODIFIER_LIST,
            "TS_PROPERTY_PARAMETER_MODIFIER_LIST" => {
                JsSyntaxKind::TS_PROPERTY_PARAMETER_MODIFIER_LIST
            }
            "TS_PROPERTY_SIGNATURE_MODIFIER_LIST" => {
                JsSyntaxKind::TS_PROPERTY_SIGNATURE_MODIFIER_LIST
            }
            "TS_TEMPLATE_ELEMENT_LIST" => JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST,
            "TS_TUPLE_TYPE_ELEMENT_LIST" => JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST,
            "TS_TYPE_ARGUMENT_LIST" => JsSyntaxKind::TS_TYPE_ARGUMENT_LIST,
            "TS_TYPE_LIST" => JsSyntaxKind::TS_TYPE_LIST,
            "TS_TYPE_MEMBER_LIST" => JsSyntaxKind::TS_TYPE_MEMBER_LIST,
            "TS_TYPE_PARAMETER_LIST" => JsSyntaxKind::TS_TYPE_PARAMETER_LIST,
            "TS_TYPE_PARAMETER_MODIFIER_LIST" => JsSyntaxKind::TS_TYPE_PARAMETER_MODIFIER_LIST,
            "TS_UNION_TYPE_VARIANT_LIST" => JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST,
            _ => return None,
        })
    }
    /// Returns the plugin API fields of `kind`, one per slot in slot order.
    /// Node kinds without fields, lists, and bogus nodes have no descriptor.
    pub(crate) fn node_fields(kind: JsSyntaxKind) -> Option<&'static JsAstNodeFields> {
        Some(match kind {
            JsSyntaxKind::ASTRO_IMPLICIT_FRAGMENT => &ASTRO_IMPLICIT_FRAGMENT_FIELDS,
            JsSyntaxKind::JS_ACCESSOR_MODIFIER => &JS_ACCESSOR_MODIFIER_FIELDS,
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => &JS_ARRAY_ASSIGNMENT_PATTERN_FIELDS,
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT => {
                &JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_FIELDS
            }
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                &JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT_FIELDS
            }
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => &JS_ARRAY_BINDING_PATTERN_FIELDS,
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT => {
                &JS_ARRAY_BINDING_PATTERN_ELEMENT_FIELDS
            }
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                &JS_ARRAY_BINDING_PATTERN_REST_ELEMENT_FIELDS
            }
            JsSyntaxKind::JS_ARRAY_EXPRESSION => &JS_ARRAY_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => &JS_ARROW_FUNCTION_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => &JS_ASSIGNMENT_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_AWAIT_EXPRESSION => &JS_AWAIT_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION => &JS_BIGINT_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_BINARY_EXPRESSION => &JS_BINARY_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_BLOCK_STATEMENT => &JS_BLOCK_STATEMENT_FIELDS,
            JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => &JS_BOOLEAN_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_BREAK_STATEMENT => &JS_BREAK_STATEMENT_FIELDS,
            JsSyntaxKind::JS_CALL_ARGUMENTS => &JS_CALL_ARGUMENTS_FIELDS,
            JsSyntaxKind::JS_CALL_EXPRESSION => &JS_CALL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_CASE_CLAUSE => &JS_CASE_CLAUSE_FIELDS,
            JsSyntaxKind::JS_CATCH_CLAUSE => &JS_CATCH_CLAUSE_FIELDS,
            JsSyntaxKind::JS_CATCH_DECLARATION => &JS_CATCH_DECLARATION_FIELDS,
            JsSyntaxKind::JS_CLASS_DECLARATION => &JS_CLASS_DECLARATION_FIELDS,
            JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                &JS_CLASS_EXPORT_DEFAULT_DECLARATION_FIELDS
            }
            JsSyntaxKind::JS_CLASS_EXPRESSION => &JS_CLASS_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => &JS_COMPUTED_MEMBER_ASSIGNMENT_FIELDS,
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => &JS_COMPUTED_MEMBER_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => &JS_COMPUTED_MEMBER_NAME_FIELDS,
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => &JS_CONDITIONAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => &JS_CONSTRUCTOR_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => &JS_CONSTRUCTOR_PARAMETERS_FIELDS,
            JsSyntaxKind::JS_CONTINUE_STATEMENT => &JS_CONTINUE_STATEMENT_FIELDS,
            JsSyntaxKind::JS_DEBUGGER_STATEMENT => &JS_DEBUGGER_STATEMENT_FIELDS,
            JsSyntaxKind::JS_DECORATOR => &JS_DECORATOR_FIELDS,
            JsSyntaxKind::JS_DEFAULT_CLAUSE => &JS_DEFAULT_CLAUSE_FIELDS,
            JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => &JS_DEFAULT_IMPORT_SPECIFIER_FIELDS,
            JsSyntaxKind::JS_DIRECTIVE => &JS_DIRECTIVE_FIELDS,
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => &JS_DO_WHILE_STATEMENT_FIELDS,
            JsSyntaxKind::JS_ELSE_CLAUSE => &JS_ELSE_CLAUSE_FIELDS,
            JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => &JS_EMPTY_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_EMPTY_STATEMENT => &JS_EMPTY_STATEMENT_FIELDS,
            JsSyntaxKind::JS_EXPORT => &JS_EXPORT_FIELDS,
            JsSyntaxKind::JS_EXPORT_AS_CLAUSE => &JS_EXPORT_AS_CLAUSE_FIELDS,
            JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
                &JS_EXPORT_DEFAULT_DECLARATION_CLAUSE_FIELDS
            }
            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                &JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE_FIELDS
            }
            JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => &JS_EXPORT_FROM_CLAUSE_FIELDS,
            JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => &JS_EXPORT_NAMED_CLAUSE_FIELDS,
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => &JS_EXPORT_NAMED_FROM_CLAUSE_FIELDS,
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => &JS_EXPORT_NAMED_FROM_SPECIFIER_FIELDS,
            JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                &JS_EXPORT_NAMED_SHORTHAND_SPECIFIER_FIELDS
            }
            JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => &JS_EXPORT_NAMED_SPECIFIER_FIELDS,
            JsSyntaxKind::JS_EXPRESSION_SNIPPET => &JS_EXPRESSION_SNIPPET_FIELDS,
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => &JS_EXPRESSION_STATEMENT_FIELDS,
            JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT => &JS_EXPRESSION_TEMPLATE_ROOT_FIELDS,
            JsSyntaxKind::JS_EXTENDS_CLAUSE => &JS_EXTENDS_CLAUSE_FIELDS,
            JsSyntaxKind::JS_FINALLY_CLAUSE => &JS_FINALLY_CLAUSE_FIELDS,
            JsSyntaxKind::JS_FOR_IN_STATEMENT => &JS_FOR_IN_STATEMENT_FIELDS,
            JsSyntaxKind::JS_FOR_OF_STATEMENT => &JS_FOR_OF_STATEMENT_FIELDS,
            JsSyntaxKind::JS_FOR_STATEMENT => &JS_FOR_STATEMENT_FIELDS,
            JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => &JS_FOR_VARIABLE_DECLARATION_FIELDS,
            JsSyntaxKind::JS_FORMAL_PARAMETER => &JS_FORMAL_PARAMETER_FIELDS,
            JsSyntaxKind::JS_FUNCTION_BODY => &JS_FUNCTION_BODY_FIELDS,
            JsSyntaxKind::JS_FUNCTION_DECLARATION => &JS_FUNCTION_DECLARATION_FIELDS,
            JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                &JS_FUNCTION_EXPORT_DEFAULT_DECLARATION_FIELDS
            }
            JsSyntaxKind::JS_FUNCTION_EXPRESSION => &JS_FUNCTION_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER => &JS_GETTER_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => &JS_GETTER_OBJECT_MEMBER_FIELDS,
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => &JS_IDENTIFIER_ASSIGNMENT_FIELDS,
            JsSyntaxKind::JS_IDENTIFIER_BINDING => &JS_IDENTIFIER_BINDING_FIELDS,
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => &JS_IDENTIFIER_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_IF_STATEMENT => &JS_IF_STATEMENT_FIELDS,
            JsSyntaxKind::JS_IMPORT => &JS_IMPORT_FIELDS,
            JsSyntaxKind::JS_IMPORT_ASSERTION => &JS_IMPORT_ASSERTION_FIELDS,
            JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => &JS_IMPORT_ASSERTION_ENTRY_FIELDS,
            JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => &JS_IMPORT_BARE_CLAUSE_FIELDS,
            JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => &JS_IMPORT_CALL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => &JS_IMPORT_COMBINED_CLAUSE_FIELDS,
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => &JS_IMPORT_DEFAULT_CLAUSE_FIELDS,
            JsSyntaxKind::JS_IMPORT_META_EXPRESSION => &JS_IMPORT_META_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => &JS_IMPORT_NAMED_CLAUSE_FIELDS,
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => &JS_IMPORT_NAMESPACE_CLAUSE_FIELDS,
            JsSyntaxKind::JS_IN_EXPRESSION => &JS_IN_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => &JS_INITIALIZER_CLAUSE_FIELDS,
            JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => &JS_INSTANCEOF_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_LABEL => &JS_LABEL_FIELDS,
            JsSyntaxKind::JS_LABELED_STATEMENT => &JS_LABELED_STATEMENT_FIELDS,
            JsSyntaxKind::JS_LITERAL_EXPORT_NAME => &JS_LITERAL_EXPORT_NAME_FIELDS,
            JsSyntaxKind::JS_LITERAL_MEMBER_NAME => &JS_LITERAL_MEMBER_NAME_FIELDS,
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => &JS_LOGICAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_METAVARIABLE => &JS_METAVARIABLE_FIELDS,
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER => &JS_METHOD_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => &JS_METHOD_OBJECT_MEMBER_FIELDS,
            JsSyntaxKind::JS_MODULE => &JS_MODULE_FIELDS,
            JsSyntaxKind::JS_MODULE_SOURCE => &JS_MODULE_SOURCE_FIELDS,
            JsSyntaxKind::JS_NAME => &JS_NAME_FIELDS,
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => &JS_NAMED_IMPORT_SPECIFIER_FIELDS,
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => &JS_NAMED_IMPORT_SPECIFIERS_FIELDS,
            JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => &JS_NAMESPACE_IMPORT_SPECIFIER_FIELDS,
            JsSyntaxKind::JS_NEW_EXPRESSION => &JS_NEW_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_NEW_TARGET_EXPRESSION => &JS_NEW_TARGET_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => &JS_NULL_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => &JS_NUMBER_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => &JS_OBJECT_ASSIGNMENT_PATTERN_FIELDS,
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                &JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_FIELDS
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                &JS_OBJECT_ASSIGNMENT_PATTERN_REST_FIELDS
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                &JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY_FIELDS
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => &JS_OBJECT_BINDING_PATTERN_FIELDS,
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                &JS_OBJECT_BINDING_PATTERN_PROPERTY_FIELDS
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => &JS_OBJECT_BINDING_PATTERN_REST_FIELDS,
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                &JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY_FIELDS
            }
            JsSyntaxKind::JS_OBJECT_EXPRESSION => &JS_OBJECT_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_PARAMETERS => &JS_PARAMETERS_FIELDS,
            JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => &JS_PARENTHESIZED_ASSIGNMENT_FIELDS,
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => &JS_PARENTHESIZED_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => &JS_POST_UPDATE_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => &JS_PRE_UPDATE_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => &JS_PRIVATE_CLASS_MEMBER_NAME_FIELDS,
            JsSyntaxKind::JS_PRIVATE_NAME => &JS_PRIVATE_NAME_FIELDS,
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => &JS_PROPERTY_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => &JS_PROPERTY_OBJECT_MEMBER_FIELDS,
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => &JS_REFERENCE_IDENTIFIER_FIELDS,
            JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => &JS_REGEX_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_REST_PARAMETER => &JS_REST_PARAMETER_FIELDS,
            JsSyntaxKind::JS_RETURN_STATEMENT => &JS_RETURN_STATEMENT_FIELDS,
            JsSyntaxKind::JS_SCRIPT => &JS_SCRIPT_FIELDS,
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => &JS_SEQUENCE_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER => &JS_SETTER_CLASS_MEMBER_FIELDS,
            JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => &JS_SETTER_OBJECT_MEMBER_FIELDS,
            JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                &JS_SHORTHAND_NAMED_IMPORT_SPECIFIER_FIELDS
            }
            JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                &JS_SHORTHAND_PROPERTY_OBJECT_MEMBER_FIELDS
            }
            JsSyntaxKind::JS_SPREAD => &JS_SPREAD_FIELDS,
            JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                &JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => &JS_STATIC_MEMBER_ASSIGNMENT_FIELDS,
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => &JS_STATIC_MEMBER_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_STATIC_MODIFIER => &JS_STATIC_MODIFIER_FIELDS,
            JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => &JS_STRING_LITERAL_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_SUPER_EXPRESSION => &JS_SUPER_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_SVELTE_DECLARATION_ROOT => &JS_SVELTE_DECLARATION_ROOT_FIELDS,
            JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT => &JS_SVELTE_SNIPPET_ROOT_FIELDS,
            JsSyntaxKind::JS_SWITCH_STATEMENT => &JS_SWITCH_STATEMENT_FIELDS,
            JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => &JS_TEMPLATE_CHUNK_ELEMENT_FIELDS,
            JsSyntaxKind::JS_TEMPLATE_ELEMENT => &JS_TEMPLATE_ELEMENT_FIELDS,
            JsSyntaxKind::JS_TEMPLATE_EXPRESSION => &JS_TEMPLATE_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_THIS_EXPRESSION => &JS_THIS_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_THROW_STATEMENT => &JS_THROW_STATEMENT_FIELDS,
            JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => &JS_TRY_FINALLY_STATEMENT_FIELDS,
            JsSyntaxKind::JS_TRY_STATEMENT => &JS_TRY_STATEMENT_FIELDS,
            JsSyntaxKind::JS_UNARY_EXPRESSION => &JS_UNARY_EXPRESSION_FIELDS,
            JsSyntaxKind::JS_VARIABLE_DECLARATION => &JS_VARIABLE_DECLARATION_FIELDS,
            JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => &JS_VARIABLE_DECLARATION_CLAUSE_FIELDS,
            JsSyntaxKind::JS_VARIABLE_DECLARATOR => &JS_VARIABLE_DECLARATOR_FIELDS,
            JsSyntaxKind::JS_VARIABLE_STATEMENT => &JS_VARIABLE_STATEMENT_FIELDS,
            JsSyntaxKind::JS_WHILE_STATEMENT => &JS_WHILE_STATEMENT_FIELDS,
            JsSyntaxKind::JS_WITH_STATEMENT => &JS_WITH_STATEMENT_FIELDS,
            JsSyntaxKind::JS_YIELD_ARGUMENT => &JS_YIELD_ARGUMENT_FIELDS,
            JsSyntaxKind::JS_YIELD_EXPRESSION => &JS_YIELD_EXPRESSION_FIELDS,
            JsSyntaxKind::JSX_ATTRIBUTE => &JSX_ATTRIBUTE_FIELDS,
            JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE => {
                &JSX_ATTRIBUTE_INITIALIZER_CLAUSE_FIELDS
            }
            JsSyntaxKind::JSX_CLOSING_ELEMENT => &JSX_CLOSING_ELEMENT_FIELDS,
            JsSyntaxKind::JSX_CLOSING_FRAGMENT => &JSX_CLOSING_FRAGMENT_FIELDS,
            JsSyntaxKind::JSX_ELEMENT => &JSX_ELEMENT_FIELDS,
            JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => &JSX_EXPRESSION_ATTRIBUTE_VALUE_FIELDS,
            JsSyntaxKind::JSX_EXPRESSION_CHILD => &JSX_EXPRESSION_CHILD_FIELDS,
            JsSyntaxKind::JSX_FRAGMENT => &JSX_FRAGMENT_FIELDS,
            JsSyntaxKind::JSX_MEMBER_NAME => &JSX_MEMBER_NAME_FIELDS,
            JsSyntaxKind::JSX_NAME => &JSX_NAME_FIELDS,
            JsSyntaxKind::JSX_NAMESPACE_NAME => &JSX_NAMESPACE_NAME_FIELDS,
            JsSyntaxKind::JSX_OPENING_ELEMENT => &JSX_OPENING_ELEMENT_FIELDS,
            JsSyntaxKind::JSX_OPENING_FRAGMENT => &JSX_OPENING_FRAGMENT_FIELDS,
            JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => &JSX_REFERENCE_IDENTIFIER_FIELDS,
            JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => &JSX_SELF_CLOSING_ELEMENT_FIELDS,
            JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE => &JSX_SHORTHAND_ATTRIBUTE_FIELDS,
            JsSyntaxKind::JSX_SPREAD_ATTRIBUTE => &JSX_SPREAD_ATTRIBUTE_FIELDS,
            JsSyntaxKind::JSX_SPREAD_CHILD => &JSX_SPREAD_CHILD_FIELDS,
            JsSyntaxKind::JSX_STRING => &JSX_STRING_FIELDS,
            JsSyntaxKind::JSX_TAG_EXPRESSION => &JSX_TAG_EXPRESSION_FIELDS,
            JsSyntaxKind::JSX_TEXT => &JSX_TEXT_FIELDS,
            JsSyntaxKind::TS_ABSTRACT_MODIFIER => &TS_ABSTRACT_MODIFIER_FIELDS,
            JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER => &TS_ACCESSIBILITY_MODIFIER_FIELDS,
            JsSyntaxKind::TS_ANY_TYPE => &TS_ANY_TYPE_FIELDS,
            JsSyntaxKind::TS_ARRAY_TYPE => &TS_ARRAY_TYPE_FIELDS,
            JsSyntaxKind::TS_AS_ASSIGNMENT => &TS_AS_ASSIGNMENT_FIELDS,
            JsSyntaxKind::TS_AS_EXPRESSION => &TS_AS_EXPRESSION_FIELDS,
            JsSyntaxKind::TS_ASSERTS_CONDITION => &TS_ASSERTS_CONDITION_FIELDS,
            JsSyntaxKind::TS_ASSERTS_RETURN_TYPE => &TS_ASSERTS_RETURN_TYPE_FIELDS,
            JsSyntaxKind::TS_BIGINT_LITERAL_TYPE => &TS_BIGINT_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_BIGINT_TYPE => &TS_BIGINT_TYPE_FIELDS,
            JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => &TS_BOOLEAN_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_BOOLEAN_TYPE => &TS_BOOLEAN_TYPE_FIELDS,
            JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => &TS_CALL_SIGNATURE_TYPE_MEMBER_FIELDS,
            JsSyntaxKind::TS_CONDITIONAL_TYPE => &TS_CONDITIONAL_TYPE_FIELDS,
            JsSyntaxKind::TS_CONST_MODIFIER => &TS_CONST_MODIFIER_FIELDS,
            JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                &TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                &TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_CONSTRUCTOR_TYPE => &TS_CONSTRUCTOR_TYPE_FIELDS,
            JsSyntaxKind::TS_DECLARATION_MODULE => &TS_DECLARATION_MODULE_FIELDS,
            JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                &TS_DECLARE_FUNCTION_DECLARATION_FIELDS
            }
            JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                &TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION_FIELDS
            }
            JsSyntaxKind::TS_DECLARE_MODIFIER => &TS_DECLARE_MODIFIER_FIELDS,
            JsSyntaxKind::TS_DECLARE_STATEMENT => &TS_DECLARE_STATEMENT_FIELDS,
            JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => &TS_DEFAULT_TYPE_CLAUSE_FIELDS,
            JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                &TS_DEFINITE_PROPERTY_ANNOTATION_FIELDS
            }
            JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                &TS_DEFINITE_VARIABLE_ANNOTATION_FIELDS
            }
            JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                &TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY_FIELDS
            }
            JsSyntaxKind::TS_ENUM_DECLARATION => &TS_ENUM_DECLARATION_FIELDS,
            JsSyntaxKind::TS_ENUM_MEMBER => &TS_ENUM_MEMBER_FIELDS,
            JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE => &TS_EXPORT_AS_NAMESPACE_CLAUSE_FIELDS,
            JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE => &TS_EXPORT_ASSIGNMENT_CLAUSE_FIELDS,
            JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE => &TS_EXPORT_DECLARE_CLAUSE_FIELDS,
            JsSyntaxKind::TS_EXTENDS_CLAUSE => &TS_EXTENDS_CLAUSE_FIELDS,
            JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => &TS_EXTERNAL_MODULE_DECLARATION_FIELDS,
            JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE => &TS_EXTERNAL_MODULE_REFERENCE_FIELDS,
            JsSyntaxKind::TS_FUNCTION_TYPE => &TS_FUNCTION_TYPE_FIELDS,
            JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER => {
                &TS_GETTER_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                &TS_GETTER_SIGNATURE_TYPE_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_GLOBAL_DECLARATION => &TS_GLOBAL_DECLARATION_FIELDS,
            JsSyntaxKind::TS_IDENTIFIER_BINDING => &TS_IDENTIFIER_BINDING_FIELDS,
            JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => &TS_IMPLEMENTS_CLAUSE_FIELDS,
            JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => &TS_IMPORT_EQUALS_DECLARATION_FIELDS,
            JsSyntaxKind::TS_IMPORT_TYPE => &TS_IMPORT_TYPE_FIELDS,
            JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS => &TS_IMPORT_TYPE_ARGUMENTS_FIELDS,
            JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION => &TS_IMPORT_TYPE_ASSERTION_FIELDS,
            JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK => &TS_IMPORT_TYPE_ASSERTION_BLOCK_FIELDS,
            JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => &TS_IMPORT_TYPE_QUALIFIER_FIELDS,
            JsSyntaxKind::TS_IN_MODIFIER => &TS_IN_MODIFIER_FIELDS,
            JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER => {
                &TS_INDEX_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => &TS_INDEX_SIGNATURE_PARAMETER_FIELDS,
            JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => &TS_INDEX_SIGNATURE_TYPE_MEMBER_FIELDS,
            JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => &TS_INDEXED_ACCESS_TYPE_FIELDS,
            JsSyntaxKind::TS_INFER_TYPE => &TS_INFER_TYPE_FIELDS,
            JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                &TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_INSTANTIATION_EXPRESSION => &TS_INSTANTIATION_EXPRESSION_FIELDS,
            JsSyntaxKind::TS_INTERFACE_DECLARATION => &TS_INTERFACE_DECLARATION_FIELDS,
            JsSyntaxKind::TS_INTERSECTION_TYPE => &TS_INTERSECTION_TYPE_FIELDS,
            JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME => &TS_LITERAL_ENUM_MEMBER_NAME_FIELDS,
            JsSyntaxKind::TS_MAPPED_TYPE => &TS_MAPPED_TYPE_FIELDS,
            JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => &TS_MAPPED_TYPE_AS_CLAUSE_FIELDS,
            JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                &TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE_FIELDS
            }
            JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                &TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE_FIELDS
            }
            JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER => {
                &TS_METHOD_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                &TS_METHOD_SIGNATURE_TYPE_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_MODULE_BLOCK => &TS_MODULE_BLOCK_FIELDS,
            JsSyntaxKind::TS_MODULE_DECLARATION => &TS_MODULE_DECLARATION_FIELDS,
            JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => &TS_NAMED_TUPLE_TYPE_ELEMENT_FIELDS,
            JsSyntaxKind::TS_NEVER_TYPE => &TS_NEVER_TYPE_FIELDS,
            JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                &TS_NON_NULL_ASSERTION_ASSIGNMENT_FIELDS
            }
            JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                &TS_NON_NULL_ASSERTION_EXPRESSION_FIELDS
            }
            JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => &TS_NON_PRIMITIVE_TYPE_FIELDS,
            JsSyntaxKind::TS_NULL_LITERAL_TYPE => &TS_NULL_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => &TS_NUMBER_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_NUMBER_TYPE => &TS_NUMBER_TYPE_FIELDS,
            JsSyntaxKind::TS_OBJECT_TYPE => &TS_OBJECT_TYPE_FIELDS,
            JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                &TS_OPTIONAL_PROPERTY_ANNOTATION_FIELDS
            }
            JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => &TS_OPTIONAL_TUPLE_TYPE_ELEMENT_FIELDS,
            JsSyntaxKind::TS_OUT_MODIFIER => &TS_OUT_MODIFIER_FIELDS,
            JsSyntaxKind::TS_OVERRIDE_MODIFIER => &TS_OVERRIDE_MODIFIER_FIELDS,
            JsSyntaxKind::TS_PARENTHESIZED_TYPE => &TS_PARENTHESIZED_TYPE_FIELDS,
            JsSyntaxKind::TS_PREDICATE_RETURN_TYPE => &TS_PREDICATE_RETURN_TYPE_FIELDS,
            JsSyntaxKind::TS_PROPERTY_PARAMETER => &TS_PROPERTY_PARAMETER_FIELDS,
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                &TS_PROPERTY_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                &TS_PROPERTY_SIGNATURE_TYPE_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => &TS_QUALIFIED_MODULE_NAME_FIELDS,
            JsSyntaxKind::TS_QUALIFIED_NAME => &TS_QUALIFIED_NAME_FIELDS,
            JsSyntaxKind::TS_READONLY_MODIFIER => &TS_READONLY_MODIFIER_FIELDS,
            JsSyntaxKind::TS_REFERENCE_TYPE => &TS_REFERENCE_TYPE_FIELDS,
            JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => &TS_REST_TUPLE_TYPE_ELEMENT_FIELDS,
            JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => &TS_RETURN_TYPE_ANNOTATION_FIELDS,
            JsSyntaxKind::TS_SATISFIES_ASSIGNMENT => &TS_SATISFIES_ASSIGNMENT_FIELDS,
            JsSyntaxKind::TS_SATISFIES_EXPRESSION => &TS_SATISFIES_EXPRESSION_FIELDS,
            JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER => {
                &TS_SETTER_SIGNATURE_CLASS_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                &TS_SETTER_SIGNATURE_TYPE_MEMBER_FIELDS
            }
            JsSyntaxKind::TS_STRING_LITERAL_TYPE => &TS_STRING_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_STRING_TYPE => &TS_STRING_TYPE_FIELDS,
            JsSyntaxKind::TS_SYMBOL_TYPE => &TS_SYMBOL_TYPE_FIELDS,
            JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => &TS_TEMPLATE_CHUNK_ELEMENT_FIELDS,
            JsSyntaxKind::TS_TEMPLATE_ELEMENT => &TS_TEMPLATE_ELEMENT_FIELDS,
            JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => &TS_TEMPLATE_LITERAL_TYPE_FIELDS,
            JsSyntaxKind::TS_THIS_PARAMETER => &TS_THIS_PARAMETER_FIELDS,
            JsSyntaxKind::TS_THIS_TYPE => &TS_THIS_TYPE_FIELDS,
            JsSyntaxKind::TS_TUPLE_TYPE => &TS_TUPLE_TYPE_FIELDS,
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => &TS_TYPE_ALIAS_DECLARATION_FIELDS,
            JsSyntaxKind::TS_TYPE_ANNOTATION => &TS_TYPE_ANNOTATION_FIELDS,
            JsSyntaxKind::TS_TYPE_ARGUMENTS => &TS_TYPE_ARGUMENTS_FIELDS,
            JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT => &TS_TYPE_ASSERTION_ASSIGNMENT_FIELDS,
            JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => &TS_TYPE_ASSERTION_EXPRESSION_FIELDS,
            JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => &TS_TYPE_CONSTRAINT_CLAUSE_FIELDS,
            JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => &TS_TYPE_OPERATOR_TYPE_FIELDS,
            JsSyntaxKind::TS_TYPE_PARAMETER => &TS_TYPE_PARAMETER_FIELDS,
            JsSyntaxKind::TS_TYPE_PARAMETER_NAME => &TS_TYPE_PARAMETER_NAME_FIELDS,
            JsSyntaxKind::TS_TYPE_PARAMETERS => &TS_TYPE_PARAMETERS_FIELDS,
            JsSyntaxKind::TS_TYPEOF_TYPE => &TS_TYPEOF_TYPE_FIELDS,
            JsSyntaxKind::TS_UNDEFINED_TYPE => &TS_UNDEFINED_TYPE_FIELDS,
            JsSyntaxKind::TS_UNION_TYPE => &TS_UNION_TYPE_FIELDS,
            JsSyntaxKind::TS_UNKNOWN_TYPE => &TS_UNKNOWN_TYPE_FIELDS,
            JsSyntaxKind::TS_VOID_TYPE => &TS_VOID_TYPE_FIELDS,
            _ => return None,
        })
    }
}
static ASTRO_IMPLICIT_FRAGMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::ASTRO_IMPLICIT_FRAGMENT,
    name: "AstroImplicitFragment",
    fields: &[JsAstField {
        property: "elements",
        updater: "withElements",
        optional: false,
        value: JsAstFieldValue::List {
            ty: "JsxChildList",
            can_cast: JsxChildList::can_cast,
        },
    }],
};
static JS_ACCESSOR_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ACCESSOR_MODIFIER,
    name: "JsAccessorModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![accessor]],
            expected: "\"accessor\"",
        },
    }],
};
static JS_ARRAY_ASSIGNMENT_PATTERN_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN,
    name: "JsArrayAssignmentPattern",
    fields: &[
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsArrayAssignmentPatternElementList",
                can_cast: JsArrayAssignmentPatternElementList::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT,
    name: "JsArrayAssignmentPatternElement",
    fields: &[
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignmentPattern",
                can_cast: AnyJsAssignmentPattern::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT,
    name: "JsArrayAssignmentPatternRestElement",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignmentPattern",
                can_cast: AnyJsAssignmentPattern::can_cast,
            },
        },
    ],
};
static JS_ARRAY_BINDING_PATTERN_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_BINDING_PATTERN,
    name: "JsArrayBindingPattern",
    fields: &[
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsArrayBindingPatternElementList",
                can_cast: JsArrayBindingPatternElementList::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_ARRAY_BINDING_PATTERN_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT,
    name: "JsArrayBindingPatternElement",
    fields: &[
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_ARRAY_BINDING_PATTERN_REST_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT,
    name: "JsArrayBindingPatternRestElement",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
    ],
};
static JS_ARRAY_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARRAY_EXPRESSION,
    name: "JsArrayExpression",
    fields: &[
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsArrayElementList",
                can_cast: JsArrayElementList::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_ARROW_FUNCTION_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
    name: "JsArrowFunctionExpression",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsArrowFunctionParameters",
                can_cast: AnyJsArrowFunctionParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "fatArrowToken",
            updater: "withFatArrowToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=>]],
                expected: "\"=>\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFunctionBody",
                can_cast: AnyJsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_ASSIGNMENT_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION,
    name: "JsAssignmentExpression",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignmentPattern",
                can_cast: AnyJsAssignmentPattern::can_cast,
            },
        },
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[
                    T![=],
                    T![+=],
                    T![-=],
                    T![*=],
                    T![/=],
                    T![%=],
                    T![**=],
                    T![>>=],
                    T![<<=],
                    T![>>>=],
                    T![&=],
                    T![|=],
                    T![^=],
                    T![&&=],
                    T![||=],
                    T![??=],
                ],
                expected: "\"=\", \"+=\", \"-=\", \"*=\", \"/=\", \"%=\", \"**=\", \">>=\", \"<<=\", \">>>=\", \"&=\", \"|=\", \"^=\", \"&&=\", \"||=\", \"??=\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_AWAIT_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_AWAIT_EXPRESSION,
    name: "JsAwaitExpression",
    fields: &[
        JsAstField {
            property: "awaitToken",
            updater: "withAwaitToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![await]],
                expected: "\"await\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_BIGINT_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION,
    name: "JsBigintLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_BIGINT_LITERAL],
            expected: "\"js_bigint_literal\"",
        },
    }],
};
static JS_BINARY_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_BINARY_EXPRESSION,
    name: "JsBinaryExpression",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[
                    T![<],
                    T![>],
                    T![<=],
                    T![>=],
                    T![==],
                    T![===],
                    T![!=],
                    T![!==],
                    T![+],
                    T![-],
                    T![*],
                    T![/],
                    T![%],
                    T![**],
                    T![<<],
                    T![>>],
                    T![>>>],
                    T![&],
                    T![|],
                    T![^],
                ],
                expected: "\"<\", \">\", \"<=\", \">=\", \"==\", \"===\", \"!=\", \"!==\", \"+\", \"-\", \"*\", \"/\", \"%\", \"**\", \"<<\", \">>\", \">>>\", \"&\", \"|\", \"^\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_BLOCK_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_BLOCK_STATEMENT,
    name: "JsBlockStatement",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "statements",
            updater: "withStatements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_BOOLEAN_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION,
    name: "JsBooleanLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![true], T![false]],
            expected: "\"true\", \"false\"",
        },
    }],
};
static JS_BREAK_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_BREAK_STATEMENT,
    name: "JsBreakStatement",
    fields: &[
        JsAstField {
            property: "breakToken",
            updater: "withBreakToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![break]],
                expected: "\"break\"",
            },
        },
        JsAstField {
            property: "label",
            updater: "withLabel",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsLabel",
                can_cast: JsLabel::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_CALL_ARGUMENTS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CALL_ARGUMENTS,
    name: "JsCallArguments",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "args",
            updater: "withArgs",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsCallArgumentList",
                can_cast: JsCallArgumentList::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_CALL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CALL_EXPRESSION,
    name: "JsCallExpression",
    fields: &[
        JsAstField {
            property: "callee",
            updater: "withCallee",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "optionalChainToken",
            updater: "withOptionalChainToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?.]],
                expected: "\"?.\"",
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "arguments",
            updater: "withArguments",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsCallArguments",
                can_cast: JsCallArguments::can_cast,
            },
        },
    ],
};
static JS_CASE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CASE_CLAUSE,
    name: "JsCaseClause",
    fields: &[
        JsAstField {
            property: "caseToken",
            updater: "withCaseToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![case]],
                expected: "\"case\"",
            },
        },
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "consequent",
            updater: "withConsequent",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
    ],
};
static JS_CATCH_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CATCH_CLAUSE,
    name: "JsCatchClause",
    fields: &[
        JsAstField {
            property: "catchToken",
            updater: "withCatchToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![catch]],
                expected: "\"catch\"",
            },
        },
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsCatchDeclaration",
                can_cast: JsCatchDeclaration::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsBlockStatement",
                can_cast: JsBlockStatement::can_cast,
            },
        },
    ],
};
static JS_CATCH_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CATCH_DECLARATION,
    name: "JsCatchDeclaration",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "binding",
            updater: "withBinding",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_CLASS_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CLASS_DECLARATION,
    name: "JsClassDeclaration",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "abstractToken",
            updater: "withAbstractToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![abstract]],
                expected: "\"abstract\"",
            },
        },
        JsAstField {
            property: "classToken",
            updater: "withClassToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![class]],
                expected: "\"class\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "extendsClause",
            updater: "withExtendsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsExtendsClause",
                can_cast: JsExtendsClause::can_cast,
            },
        },
        JsAstField {
            property: "implementsClause",
            updater: "withImplementsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsImplementsClause",
                can_cast: TsImplementsClause::can_cast,
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsClassMemberList",
                can_cast: JsClassMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_CLASS_EXPORT_DEFAULT_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION,
    name: "JsClassExportDefaultDeclaration",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "abstractToken",
            updater: "withAbstractToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![abstract]],
                expected: "\"abstract\"",
            },
        },
        JsAstField {
            property: "classToken",
            updater: "withClassToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![class]],
                expected: "\"class\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "extendsClause",
            updater: "withExtendsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsExtendsClause",
                can_cast: JsExtendsClause::can_cast,
            },
        },
        JsAstField {
            property: "implementsClause",
            updater: "withImplementsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsImplementsClause",
                can_cast: TsImplementsClause::can_cast,
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsClassMemberList",
                can_cast: JsClassMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_CLASS_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CLASS_EXPRESSION,
    name: "JsClassExpression",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "classToken",
            updater: "withClassToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![class]],
                expected: "\"class\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "extendsClause",
            updater: "withExtendsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsExtendsClause",
                can_cast: JsExtendsClause::can_cast,
            },
        },
        JsAstField {
            property: "implementsClause",
            updater: "withImplementsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsImplementsClause",
                can_cast: TsImplementsClause::can_cast,
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsClassMemberList",
                can_cast: JsClassMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_COMPUTED_MEMBER_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT,
    name: "JsComputedMemberAssignment",
    fields: &[
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_COMPUTED_MEMBER_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION,
    name: "JsComputedMemberExpression",
    fields: &[
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "optionalChainToken",
            updater: "withOptionalChainToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?.]],
                expected: "\"?.\"",
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_COMPUTED_MEMBER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_COMPUTED_MEMBER_NAME,
    name: "JsComputedMemberName",
    fields: &[
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static JS_CONDITIONAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CONDITIONAL_EXPRESSION,
    name: "JsConditionalExpression",
    fields: &[
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "consequent",
            updater: "withConsequent",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "alternate",
            updater: "withAlternate",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_CONSTRUCTOR_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER,
    name: "JsConstructorClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsConstructorModifierList",
                can_cast: JsConstructorModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsLiteralMemberName",
                can_cast: JsLiteralMemberName::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsConstructorParameters",
                can_cast: JsConstructorParameters::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_CONSTRUCTOR_PARAMETERS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS,
    name: "JsConstructorParameters",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsConstructorParameterList",
                can_cast: JsConstructorParameterList::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_CONTINUE_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_CONTINUE_STATEMENT,
    name: "JsContinueStatement",
    fields: &[
        JsAstField {
            property: "continueToken",
            updater: "withContinueToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![continue]],
                expected: "\"continue\"",
            },
        },
        JsAstField {
            property: "label",
            updater: "withLabel",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsLabel",
                can_cast: JsLabel::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_DEBUGGER_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DEBUGGER_STATEMENT,
    name: "JsDebuggerStatement",
    fields: &[
        JsAstField {
            property: "debuggerToken",
            updater: "withDebuggerToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![debugger]],
                expected: "\"debugger\"",
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_DECORATOR_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DECORATOR,
    name: "JsDecorator",
    fields: &[
        JsAstField {
            property: "atToken",
            updater: "withAtToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![@]],
                expected: "\"@\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsDecorator",
                can_cast: AnyJsDecorator::can_cast,
            },
        },
    ],
};
static JS_DEFAULT_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DEFAULT_CLAUSE,
    name: "JsDefaultClause",
    fields: &[
        JsAstField {
            property: "defaultToken",
            updater: "withDefaultToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![default]],
                expected: "\"default\"",
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "consequent",
            updater: "withConsequent",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
    ],
};
static JS_DEFAULT_IMPORT_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER,
    name: "JsDefaultImportSpecifier",
    fields: &[JsAstField {
        property: "localName",
        updater: "withLocalName",
        optional: false,
        value: JsAstFieldValue::Node {
            ty: "AnyJsBinding",
            can_cast: AnyJsBinding::can_cast,
        },
    }],
};
static JS_DIRECTIVE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DIRECTIVE,
    name: "JsDirective",
    fields: &[
        JsAstField {
            property: "valueToken",
            updater: "withValueToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[JS_STRING_LITERAL],
                expected: "\"js_string_literal\"",
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_DO_WHILE_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_DO_WHILE_STATEMENT,
    name: "JsDoWhileStatement",
    fields: &[
        JsAstField {
            property: "doToken",
            updater: "withDoToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![do]],
                expected: "\"do\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
        JsAstField {
            property: "whileToken",
            updater: "withWhileToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![while]],
                expected: "\"while\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_ELSE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_ELSE_CLAUSE,
    name: "JsElseClause",
    fields: &[
        JsAstField {
            property: "elseToken",
            updater: "withElseToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![else]],
                expected: "\"else\"",
            },
        },
        JsAstField {
            property: "alternate",
            updater: "withAlternate",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_EMPTY_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EMPTY_CLASS_MEMBER,
    name: "JsEmptyClassMember",
    fields: &[JsAstField {
        property: "semicolonToken",
        updater: "withSemicolonToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![;]],
            expected: "\";\"",
        },
    }],
};
static JS_EMPTY_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EMPTY_STATEMENT,
    name: "JsEmptyStatement",
    fields: &[JsAstField {
        property: "semicolonToken",
        updater: "withSemicolonToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![;]],
            expected: "\";\"",
        },
    }],
};
static JS_EXPORT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT,
    name: "JsExport",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "exportToken",
            updater: "withExportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![export]],
                expected: "\"export\"",
            },
        },
        JsAstField {
            property: "exportClause",
            updater: "withExportClause",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExportClause",
                can_cast: AnyJsExportClause::can_cast,
            },
        },
    ],
};
static JS_EXPORT_AS_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_AS_CLAUSE,
    name: "JsExportAsClause",
    fields: &[
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "exportedName",
            updater: "withExportedName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsLiteralExportName",
                can_cast: AnyJsLiteralExportName::can_cast,
            },
        },
    ],
};
static JS_EXPORT_DEFAULT_DECLARATION_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE,
    name: "JsExportDefaultDeclarationClause",
    fields: &[
        JsAstField {
            property: "defaultToken",
            updater: "withDefaultToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![default]],
                expected: "\"default\"",
            },
        },
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExportDefaultDeclaration",
                can_cast: AnyJsExportDefaultDeclaration::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE,
    name: "JsExportDefaultExpressionClause",
    fields: &[
        JsAstField {
            property: "defaultToken",
            updater: "withDefaultToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![default]],
                expected: "\"default\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPORT_FROM_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_FROM_CLAUSE,
    name: "JsExportFromClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "exportAs",
            updater: "withExportAs",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsExportAsClause",
                can_cast: JsExportAsClause::can_cast,
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPORT_NAMED_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE,
    name: "JsExportNamedClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "specifiers",
            updater: "withSpecifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsExportNamedSpecifierList",
                can_cast: JsExportNamedSpecifierList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPORT_NAMED_FROM_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE,
    name: "JsExportNamedFromClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "specifiers",
            updater: "withSpecifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsExportNamedFromSpecifierList",
                can_cast: JsExportNamedFromSpecifierList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPORT_NAMED_FROM_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER,
    name: "JsExportNamedFromSpecifier",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "sourceName",
            updater: "withSourceName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsLiteralExportName",
                can_cast: AnyJsLiteralExportName::can_cast,
            },
        },
        JsAstField {
            property: "exportAs",
            updater: "withExportAs",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsExportAsClause",
                can_cast: JsExportAsClause::can_cast,
            },
        },
    ],
};
static JS_EXPORT_NAMED_SHORTHAND_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER,
    name: "JsExportNamedShorthandSpecifier",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsReferenceIdentifier",
                can_cast: JsReferenceIdentifier::can_cast,
            },
        },
    ],
};
static JS_EXPORT_NAMED_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER,
    name: "JsExportNamedSpecifier",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "localName",
            updater: "withLocalName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsReferenceIdentifier",
                can_cast: JsReferenceIdentifier::can_cast,
            },
        },
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "exportedName",
            updater: "withExportedName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsLiteralExportName",
                can_cast: AnyJsLiteralExportName::can_cast,
            },
        },
    ],
};
static JS_EXPRESSION_SNIPPET_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPRESSION_SNIPPET,
    name: "JsExpressionSnippet",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_EXPRESSION_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPRESSION_STATEMENT,
    name: "JsExpressionStatement",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_EXPRESSION_TEMPLATE_ROOT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT,
    name: "JsExpressionTemplateRoot",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_EXTENDS_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_EXTENDS_CLAUSE,
    name: "JsExtendsClause",
    fields: &[
        JsAstField {
            property: "extendsToken",
            updater: "withExtendsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![extends]],
                expected: "\"extends\"",
            },
        },
        JsAstField {
            property: "superClass",
            updater: "withSuperClass",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
    ],
};
static JS_FINALLY_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FINALLY_CLAUSE,
    name: "JsFinallyClause",
    fields: &[
        JsAstField {
            property: "finallyToken",
            updater: "withFinallyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![finally]],
                expected: "\"finally\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsBlockStatement",
                can_cast: JsBlockStatement::can_cast,
            },
        },
    ],
};
static JS_FOR_IN_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FOR_IN_STATEMENT,
    name: "JsForInStatement",
    fields: &[
        JsAstField {
            property: "forToken",
            updater: "withForToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![for]],
                expected: "\"for\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsForInOrOfInitializer",
                can_cast: AnyJsForInOrOfInitializer::can_cast,
            },
        },
        JsAstField {
            property: "inToken",
            updater: "withInToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![in]],
                expected: "\"in\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_FOR_OF_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FOR_OF_STATEMENT,
    name: "JsForOfStatement",
    fields: &[
        JsAstField {
            property: "forToken",
            updater: "withForToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![for]],
                expected: "\"for\"",
            },
        },
        JsAstField {
            property: "awaitToken",
            updater: "withAwaitToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![await]],
                expected: "\"await\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsForInOrOfInitializer",
                can_cast: AnyJsForInOrOfInitializer::can_cast,
            },
        },
        JsAstField {
            property: "ofToken",
            updater: "withOfToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![of]],
                expected: "\"of\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_FOR_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FOR_STATEMENT,
    name: "JsForStatement",
    fields: &[
        JsAstField {
            property: "forToken",
            updater: "withForToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![for]],
                expected: "\"for\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsForInitializer",
                can_cast: AnyJsForInitializer::can_cast,
            },
        },
        JsAstField {
            property: "firstSemiToken",
            updater: "withFirstSemiToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "secondSemiToken",
            updater: "withSecondSemiToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
        JsAstField {
            property: "update",
            updater: "withUpdate",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_FOR_VARIABLE_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION,
    name: "JsForVariableDeclaration",
    fields: &[
        JsAstField {
            property: "awaitToken",
            updater: "withAwaitToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![await]],
                expected: "\"await\"",
            },
        },
        JsAstField {
            property: "kindToken",
            updater: "withKindToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![var], T![let], T![const], T![using]],
                expected: "\"var\", \"let\", \"const\", \"using\"",
            },
        },
        JsAstField {
            property: "declarator",
            updater: "withDeclarator",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsVariableDeclarator",
                can_cast: JsVariableDeclarator::can_cast,
            },
        },
    ],
};
static JS_FORMAL_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FORMAL_PARAMETER,
    name: "JsFormalParameter",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "binding",
            updater: "withBinding",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_FUNCTION_BODY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FUNCTION_BODY,
    name: "JsFunctionBody",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "directives",
            updater: "withDirectives",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDirectiveList",
                can_cast: JsDirectiveList::can_cast,
            },
        },
        JsAstField {
            property: "statements",
            updater: "withStatements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_FUNCTION_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FUNCTION_DECLARATION,
    name: "JsFunctionDeclaration",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "functionToken",
            updater: "withFunctionToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![function]],
                expected: "\"function\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_FUNCTION_EXPORT_DEFAULT_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION,
    name: "JsFunctionExportDefaultDeclaration",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "functionToken",
            updater: "withFunctionToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![function]],
                expected: "\"function\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_FUNCTION_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_FUNCTION_EXPRESSION,
    name: "JsFunctionExpression",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "functionToken",
            updater: "withFunctionToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![function]],
                expected: "\"function\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_GETTER_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_GETTER_CLASS_MEMBER,
    name: "JsGetterClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsMethodModifierList",
                can_cast: JsMethodModifierList::can_cast,
            },
        },
        JsAstField {
            property: "getToken",
            updater: "withGetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![get]],
                expected: "\"get\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "returnType",
            updater: "withReturnType",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_GETTER_OBJECT_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_GETTER_OBJECT_MEMBER,
    name: "JsGetterObjectMember",
    fields: &[
        JsAstField {
            property: "getToken",
            updater: "withGetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![get]],
                expected: "\"get\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "returnType",
            updater: "withReturnType",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_IDENTIFIER_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT,
    name: "JsIdentifierAssignment",
    fields: &[JsAstField {
        property: "nameToken",
        updater: "withNameToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static JS_IDENTIFIER_BINDING_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IDENTIFIER_BINDING,
    name: "JsIdentifierBinding",
    fields: &[JsAstField {
        property: "nameToken",
        updater: "withNameToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static JS_IDENTIFIER_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IDENTIFIER_EXPRESSION,
    name: "JsIdentifierExpression",
    fields: &[JsAstField {
        property: "name",
        updater: "withName",
        optional: false,
        value: JsAstFieldValue::Node {
            ty: "JsReferenceIdentifier",
            can_cast: JsReferenceIdentifier::can_cast,
        },
    }],
};
static JS_IF_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IF_STATEMENT,
    name: "JsIfStatement",
    fields: &[
        JsAstField {
            property: "ifToken",
            updater: "withIfToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![if]],
                expected: "\"if\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "consequent",
            updater: "withConsequent",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
        JsAstField {
            property: "elseClause",
            updater: "withElseClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsElseClause",
                can_cast: JsElseClause::can_cast,
            },
        },
    ],
};
static JS_IMPORT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT,
    name: "JsImport",
    fields: &[
        JsAstField {
            property: "importToken",
            updater: "withImportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![import]],
                expected: "\"import\"",
            },
        },
        JsAstField {
            property: "importClause",
            updater: "withImportClause",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsImportClause",
                can_cast: AnyJsImportClause::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_IMPORT_ASSERTION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_ASSERTION,
    name: "JsImportAssertion",
    fields: &[
        JsAstField {
            property: "withToken",
            updater: "withWithToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![with]],
                expected: "\"with\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "assertions",
            updater: "withAssertions",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsImportAssertionEntryList",
                can_cast: JsImportAssertionEntryList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_IMPORT_ASSERTION_ENTRY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY,
    name: "JsImportAssertionEntry",
    fields: &[
        JsAstField {
            property: "key",
            updater: "withKey",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[IDENT, JS_STRING_LITERAL],
                expected: "\"ident\", \"js_string_literal\"",
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "valueToken",
            updater: "withValueToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[JS_STRING_LITERAL],
                expected: "\"js_string_literal\"",
            },
        },
    ],
};
static JS_IMPORT_BARE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_BARE_CLAUSE,
    name: "JsImportBareClause",
    fields: &[
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
    ],
};
static JS_IMPORT_CALL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION,
    name: "JsImportCallExpression",
    fields: &[
        JsAstField {
            property: "importToken",
            updater: "withImportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![import]],
                expected: "\"import\"",
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "phase",
            updater: "withPhase",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![source], T![defer]],
                expected: "\"source\", \"defer\"",
            },
        },
        JsAstField {
            property: "arguments",
            updater: "withArguments",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsCallArguments",
                can_cast: JsCallArguments::can_cast,
            },
        },
    ],
};
static JS_IMPORT_COMBINED_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE,
    name: "JsImportCombinedClause",
    fields: &[
        JsAstField {
            property: "defaultSpecifier",
            updater: "withDefaultSpecifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsDefaultImportSpecifier",
                can_cast: JsDefaultImportSpecifier::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "specifier",
            updater: "withSpecifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsCombinedSpecifier",
                can_cast: AnyJsCombinedSpecifier::can_cast,
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
    ],
};
static JS_IMPORT_DEFAULT_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE,
    name: "JsImportDefaultClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "phaseToken",
            updater: "withPhaseToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![source]],
                expected: "\"source\"",
            },
        },
        JsAstField {
            property: "defaultSpecifier",
            updater: "withDefaultSpecifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsDefaultImportSpecifier",
                can_cast: JsDefaultImportSpecifier::can_cast,
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
    ],
};
static JS_IMPORT_META_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_META_EXPRESSION,
    name: "JsImportMetaExpression",
    fields: &[
        JsAstField {
            property: "importToken",
            updater: "withImportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![import]],
                expected: "\"import\"",
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "metaToken",
            updater: "withMetaToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[META],
                expected: "\"meta\"",
            },
        },
    ],
};
static JS_IMPORT_NAMED_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE,
    name: "JsImportNamedClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "namedSpecifiers",
            updater: "withNamedSpecifiers",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsNamedImportSpecifiers",
                can_cast: JsNamedImportSpecifiers::can_cast,
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
    ],
};
static JS_IMPORT_NAMESPACE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE,
    name: "JsImportNamespaceClause",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "phaseToken",
            updater: "withPhaseToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![defer]],
                expected: "\"defer\"",
            },
        },
        JsAstField {
            property: "namespaceSpecifier",
            updater: "withNamespaceSpecifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsNamespaceImportSpecifier",
                can_cast: JsNamespaceImportSpecifier::can_cast,
            },
        },
        JsAstField {
            property: "fromToken",
            updater: "withFromToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![from]],
                expected: "\"from\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "assertion",
            updater: "withAssertion",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsImportAssertion",
                can_cast: JsImportAssertion::can_cast,
            },
        },
    ],
};
static JS_IN_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_IN_EXPRESSION,
    name: "JsInExpression",
    fields: &[
        JsAstField {
            property: "property",
            updater: "withProperty",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsInProperty",
                can_cast: AnyJsInProperty::can_cast,
            },
        },
        JsAstField {
            property: "inToken",
            updater: "withInToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![in]],
                expected: "\"in\"",
            },
        },
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_INITIALIZER_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_INITIALIZER_CLAUSE,
    name: "JsInitializerClause",
    fields: &[
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_INSTANCEOF_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_INSTANCEOF_EXPRESSION,
    name: "JsInstanceofExpression",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "instanceofToken",
            updater: "withInstanceofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![instanceof]],
                expected: "\"instanceof\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_LABEL_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_LABEL,
    name: "JsLabel",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static JS_LABELED_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_LABELED_STATEMENT,
    name: "JsLabeledStatement",
    fields: &[
        JsAstField {
            property: "label",
            updater: "withLabel",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsLabel",
                can_cast: JsLabel::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_LITERAL_EXPORT_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_LITERAL_EXPORT_NAME,
    name: "JsLiteralExportName",
    fields: &[JsAstField {
        property: "value",
        updater: "withValue",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT, JS_STRING_LITERAL],
            expected: "\"ident\", \"js_string_literal\"",
        },
    }],
};
static JS_LITERAL_MEMBER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
    name: "JsLiteralMemberName",
    fields: &[JsAstField {
        property: "value",
        updater: "withValue",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT, JS_STRING_LITERAL, JS_NUMBER_LITERAL],
            expected: "\"ident\", \"js_string_literal\", \"js_number_literal\"",
        },
    }],
};
static JS_LOGICAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_LOGICAL_EXPRESSION,
    name: "JsLogicalExpression",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![??], T![||], T![&&]],
                expected: "\"??\", \"||\", \"&&\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_METAVARIABLE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_METAVARIABLE,
    name: "JsMetavariable",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[GRIT_METAVARIABLE],
            expected: "\"grit_metavariable\"",
        },
    }],
};
static JS_METHOD_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
    name: "JsMethodClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsMethodModifierList",
                can_cast: JsMethodModifierList::can_cast,
            },
        },
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_METHOD_OBJECT_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_METHOD_OBJECT_MEMBER,
    name: "JsMethodObjectMember",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_MODULE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_MODULE,
    name: "JsModule",
    fields: &[
        JsAstField {
            property: "bomToken",
            updater: "withBomToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![UNICODE_BOM]],
                expected: "\"UNICODE_BOM\"",
            },
        },
        JsAstField {
            property: "interpreterToken",
            updater: "withInterpreterToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[JS_SHEBANG],
                expected: "\"js_shebang\"",
            },
        },
        JsAstField {
            property: "directives",
            updater: "withDirectives",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDirectiveList",
                can_cast: JsDirectiveList::can_cast,
            },
        },
        JsAstField {
            property: "items",
            updater: "withItems",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsModuleItemList",
                can_cast: JsModuleItemList::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_MODULE_SOURCE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_MODULE_SOURCE,
    name: "JsModuleSource",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_STRING_LITERAL],
            expected: "\"js_string_literal\"",
        },
    }],
};
static JS_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NAME,
    name: "JsName",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static JS_NAMED_IMPORT_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER,
    name: "JsNamedImportSpecifier",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsLiteralExportName",
                can_cast: AnyJsLiteralExportName::can_cast,
            },
        },
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "localName",
            updater: "withLocalName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
    ],
};
static JS_NAMED_IMPORT_SPECIFIERS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS,
    name: "JsNamedImportSpecifiers",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "specifiers",
            updater: "withSpecifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsNamedImportSpecifierList",
                can_cast: JsNamedImportSpecifierList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_NAMESPACE_IMPORT_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER,
    name: "JsNamespaceImportSpecifier",
    fields: &[
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "localName",
            updater: "withLocalName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
    ],
};
static JS_NEW_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NEW_EXPRESSION,
    name: "JsNewExpression",
    fields: &[
        JsAstField {
            property: "newToken",
            updater: "withNewToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![new]],
                expected: "\"new\"",
            },
        },
        JsAstField {
            property: "callee",
            updater: "withCallee",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "arguments",
            updater: "withArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsCallArguments",
                can_cast: JsCallArguments::can_cast,
            },
        },
    ],
};
static JS_NEW_TARGET_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NEW_TARGET_EXPRESSION,
    name: "JsNewTargetExpression",
    fields: &[
        JsAstField {
            property: "newToken",
            updater: "withNewToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![new]],
                expected: "\"new\"",
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "targetToken",
            updater: "withTargetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[TARGET],
                expected: "\"target\"",
            },
        },
    ],
};
static JS_NULL_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
    name: "JsNullLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![null]],
            expected: "\"null\"",
        },
    }],
};
static JS_NUMBER_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
    name: "JsNumberLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_NUMBER_LITERAL],
            expected: "\"js_number_literal\"",
        },
    }],
};
static JS_OBJECT_ASSIGNMENT_PATTERN_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN,
    name: "JsObjectAssignmentPattern",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "properties",
            updater: "withProperties",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsObjectAssignmentPatternPropertyList",
                can_cast: JsObjectAssignmentPatternPropertyList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY,
    name: "JsObjectAssignmentPatternProperty",
    fields: &[
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignmentPattern",
                can_cast: AnyJsAssignmentPattern::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_OBJECT_ASSIGNMENT_PATTERN_REST_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST,
    name: "JsObjectAssignmentPatternRest",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "target",
            updater: "withTarget",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
    ],
};
static JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY,
    name: "JsObjectAssignmentPatternShorthandProperty",
    fields: &[
        JsAstField {
            property: "identifier",
            updater: "withIdentifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsIdentifierAssignment",
                can_cast: JsIdentifierAssignment::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_OBJECT_BINDING_PATTERN_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_BINDING_PATTERN,
    name: "JsObjectBindingPattern",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "properties",
            updater: "withProperties",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsObjectBindingPatternPropertyList",
                can_cast: JsObjectBindingPatternPropertyList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_OBJECT_BINDING_PATTERN_PROPERTY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY,
    name: "JsObjectBindingPatternProperty",
    fields: &[
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "pattern",
            updater: "withPattern",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_OBJECT_BINDING_PATTERN_REST_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST,
    name: "JsObjectBindingPatternRest",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "binding",
            updater: "withBinding",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
    ],
};
static JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY,
    name: "JsObjectBindingPatternShorthandProperty",
    fields: &[
        JsAstField {
            property: "identifier",
            updater: "withIdentifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "init",
            updater: "withInit",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_OBJECT_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_OBJECT_EXPRESSION,
    name: "JsObjectExpression",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsObjectMemberList",
                can_cast: JsObjectMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_PARAMETERS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PARAMETERS,
    name: "JsParameters",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "items",
            updater: "withItems",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsParameterList",
                can_cast: JsParameterList::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_PARENTHESIZED_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT,
    name: "JsParenthesizedAssignment",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "assignment",
            updater: "withAssignment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_PARENTHESIZED_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION,
    name: "JsParenthesizedExpression",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static JS_POST_UPDATE_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_POST_UPDATE_EXPRESSION,
    name: "JsPostUpdateExpression",
    fields: &[
        JsAstField {
            property: "operand",
            updater: "withOperand",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![++], T![--]],
                expected: "\"++\", \"--\"",
            },
        },
    ],
};
static JS_PRE_UPDATE_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION,
    name: "JsPreUpdateExpression",
    fields: &[
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![++], T![--]],
                expected: "\"++\", \"--\"",
            },
        },
        JsAstField {
            property: "operand",
            updater: "withOperand",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
    ],
};
static JS_PRIVATE_CLASS_MEMBER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME,
    name: "JsPrivateClassMemberName",
    fields: &[
        JsAstField {
            property: "hashToken",
            updater: "withHashToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![#]],
                expected: "\"#\"",
            },
        },
        JsAstField {
            property: "idToken",
            updater: "withIdToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[IDENT],
                expected: "\"ident\"",
            },
        },
    ],
};
static JS_PRIVATE_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PRIVATE_NAME,
    name: "JsPrivateName",
    fields: &[
        JsAstField {
            property: "hashToken",
            updater: "withHashToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![#]],
                expected: "\"#\"",
            },
        },
        JsAstField {
            property: "valueToken",
            updater: "withValueToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[IDENT],
                expected: "\"ident\"",
            },
        },
    ],
};
static JS_PROPERTY_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER,
    name: "JsPropertyClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsPropertyModifierList",
                can_cast: JsPropertyModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "propertyAnnotation",
            updater: "withPropertyAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyTsPropertyAnnotation",
                can_cast: AnyTsPropertyAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "value",
            updater: "withValue",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_PROPERTY_OBJECT_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER,
    name: "JsPropertyObjectMember",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "value",
            updater: "withValue",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_REFERENCE_IDENTIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_REFERENCE_IDENTIFIER,
    name: "JsReferenceIdentifier",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static JS_REGEX_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
    name: "JsRegexLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_REGEX_LITERAL],
            expected: "\"js_regex_literal\"",
        },
    }],
};
static JS_REST_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_REST_PARAMETER,
    name: "JsRestParameter",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "binding",
            updater: "withBinding",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static JS_RETURN_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_RETURN_STATEMENT,
    name: "JsReturnStatement",
    fields: &[
        JsAstField {
            property: "returnToken",
            updater: "withReturnToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![return]],
                expected: "\"return\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_SCRIPT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SCRIPT,
    name: "JsScript",
    fields: &[
        JsAstField {
            property: "bomToken",
            updater: "withBomToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![UNICODE_BOM]],
                expected: "\"UNICODE_BOM\"",
            },
        },
        JsAstField {
            property: "interpreterToken",
            updater: "withInterpreterToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[JS_SHEBANG],
                expected: "\"js_shebang\"",
            },
        },
        JsAstField {
            property: "directives",
            updater: "withDirectives",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDirectiveList",
                can_cast: JsDirectiveList::can_cast,
            },
        },
        JsAstField {
            property: "statements",
            updater: "withStatements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_SEQUENCE_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SEQUENCE_EXPRESSION,
    name: "JsSequenceExpression",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_SETTER_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SETTER_CLASS_MEMBER,
    name: "JsSetterClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsMethodModifierList",
                can_cast: JsMethodModifierList::can_cast,
            },
        },
        JsAstField {
            property: "setToken",
            updater: "withSetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![set]],
                expected: "\"set\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFormalParameter",
                can_cast: AnyJsFormalParameter::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_SETTER_OBJECT_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SETTER_OBJECT_MEMBER,
    name: "JsSetterObjectMember",
    fields: &[
        JsAstField {
            property: "setToken",
            updater: "withSetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![set]],
                expected: "\"set\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFormalParameter",
                can_cast: AnyJsFormalParameter::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFunctionBody",
                can_cast: JsFunctionBody::can_cast,
            },
        },
    ],
};
static JS_SHORTHAND_NAMED_IMPORT_SPECIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER,
    name: "JsShorthandNamedImportSpecifier",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "localName",
            updater: "withLocalName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
    ],
};
static JS_SHORTHAND_PROPERTY_OBJECT_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
    name: "JsShorthandPropertyObjectMember",
    fields: &[JsAstField {
        property: "name",
        updater: "withName",
        optional: false,
        value: JsAstFieldValue::Node {
            ty: "JsReferenceIdentifier",
            can_cast: JsReferenceIdentifier::can_cast,
        },
    }],
};
static JS_SPREAD_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SPREAD,
    name: "JsSpread",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER,
    name: "JsStaticInitializationBlockClassMember",
    fields: &[
        JsAstField {
            property: "staticToken",
            updater: "withStaticToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![static]],
                expected: "\"static\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "statements",
            updater: "withStatements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsStatementList",
                can_cast: JsStatementList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_STATIC_MEMBER_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
    name: "JsStaticMemberAssignment",
    fields: &[
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsName",
                can_cast: AnyJsName::can_cast,
            },
        },
    ],
};
static JS_STATIC_MEMBER_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION,
    name: "JsStaticMemberExpression",
    fields: &[
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.], T![?.]],
                expected: "\".\", \"?.\"",
            },
        },
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsName",
                can_cast: AnyJsName::can_cast,
            },
        },
    ],
};
static JS_STATIC_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_STATIC_MODIFIER,
    name: "JsStaticModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![static]],
            expected: "\"static\"",
        },
    }],
};
static JS_STRING_LITERAL_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
    name: "JsStringLiteralExpression",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_STRING_LITERAL],
            expected: "\"js_string_literal\"",
        },
    }],
};
static JS_SUPER_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SUPER_EXPRESSION,
    name: "JsSuperExpression",
    fields: &[JsAstField {
        property: "superToken",
        updater: "withSuperToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![super]],
            expected: "\"super\"",
        },
    }],
};
static JS_SVELTE_DECLARATION_ROOT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SVELTE_DECLARATION_ROOT,
    name: "JsSvelteDeclarationRoot",
    fields: &[
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsSvelteDeclaration",
                can_cast: AnyJsSvelteDeclaration::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_SVELTE_SNIPPET_ROOT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT,
    name: "JsSvelteSnippetRoot",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static JS_SWITCH_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_SWITCH_STATEMENT,
    name: "JsSwitchStatement",
    fields: &[
        JsAstField {
            property: "switchToken",
            updater: "withSwitchToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![switch]],
                expected: "\"switch\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "discriminant",
            updater: "withDiscriminant",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "cases",
            updater: "withCases",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsSwitchCaseList",
                can_cast: JsSwitchCaseList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_TEMPLATE_CHUNK_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT,
    name: "JsTemplateChunkElement",
    fields: &[JsAstField {
        property: "templateChunkToken",
        updater: "withTemplateChunkToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[TEMPLATE_CHUNK],
            expected: "\"template_chunk\"",
        },
    }],
};
static JS_TEMPLATE_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_TEMPLATE_ELEMENT,
    name: "JsTemplateElement",
    fields: &[
        JsAstField {
            property: "dollarCurlyToken",
            updater: "withDollarCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[DOLLAR_CURLY],
                expected: "\"dollar_curly\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JS_TEMPLATE_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_TEMPLATE_EXPRESSION,
    name: "JsTemplateExpression",
    fields: &[
        JsAstField {
            property: "tag",
            updater: "withTag",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "lTickToken",
            updater: "withLTickToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['`']],
                expected: "\"'`'\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsTemplateElementList",
                can_cast: JsTemplateElementList::can_cast,
            },
        },
        JsAstField {
            property: "rTickToken",
            updater: "withRTickToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['`']],
                expected: "\"'`'\"",
            },
        },
    ],
};
static JS_THIS_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_THIS_EXPRESSION,
    name: "JsThisExpression",
    fields: &[JsAstField {
        property: "thisToken",
        updater: "withThisToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![this]],
            expected: "\"this\"",
        },
    }],
};
static JS_THROW_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_THROW_STATEMENT,
    name: "JsThrowStatement",
    fields: &[
        JsAstField {
            property: "throwToken",
            updater: "withThrowToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![throw]],
                expected: "\"throw\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_TRY_FINALLY_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_TRY_FINALLY_STATEMENT,
    name: "JsTryFinallyStatement",
    fields: &[
        JsAstField {
            property: "tryToken",
            updater: "withTryToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![try]],
                expected: "\"try\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsBlockStatement",
                can_cast: JsBlockStatement::can_cast,
            },
        },
        JsAstField {
            property: "catchClause",
            updater: "withCatchClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsCatchClause",
                can_cast: JsCatchClause::can_cast,
            },
        },
        JsAstField {
            property: "finallyClause",
            updater: "withFinallyClause",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsFinallyClause",
                can_cast: JsFinallyClause::can_cast,
            },
        },
    ],
};
static JS_TRY_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_TRY_STATEMENT,
    name: "JsTryStatement",
    fields: &[
        JsAstField {
            property: "tryToken",
            updater: "withTryToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![try]],
                expected: "\"try\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsBlockStatement",
                can_cast: JsBlockStatement::can_cast,
            },
        },
        JsAstField {
            property: "catchClause",
            updater: "withCatchClause",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsCatchClause",
                can_cast: JsCatchClause::can_cast,
            },
        },
    ],
};
static JS_UNARY_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_UNARY_EXPRESSION,
    name: "JsUnaryExpression",
    fields: &[
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![delete], T![void], T![typeof], T![+], T![-], T![~], T![!]],
                expected: "\"delete\", \"void\", \"typeof\", \"+\", \"-\", \"~\", \"!\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_VARIABLE_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_VARIABLE_DECLARATION,
    name: "JsVariableDeclaration",
    fields: &[
        JsAstField {
            property: "awaitToken",
            updater: "withAwaitToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![await]],
                expected: "\"await\"",
            },
        },
        JsAstField {
            property: "kindToken",
            updater: "withKindToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![var], T![const], T![let], T![using]],
                expected: "\"var\", \"const\", \"let\", \"using\"",
            },
        },
        JsAstField {
            property: "declarators",
            updater: "withDeclarators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsVariableDeclaratorList",
                can_cast: JsVariableDeclaratorList::can_cast,
            },
        },
    ],
};
static JS_VARIABLE_DECLARATION_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE,
    name: "JsVariableDeclarationClause",
    fields: &[
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsVariableDeclaration",
                can_cast: JsVariableDeclaration::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_VARIABLE_DECLARATOR_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_VARIABLE_DECLARATOR,
    name: "JsVariableDeclarator",
    fields: &[
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBindingPattern",
                can_cast: AnyJsBindingPattern::can_cast,
            },
        },
        JsAstField {
            property: "variableAnnotation",
            updater: "withVariableAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyTsVariableAnnotation",
                can_cast: AnyTsVariableAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static JS_VARIABLE_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_VARIABLE_STATEMENT,
    name: "JsVariableStatement",
    fields: &[
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsVariableDeclaration",
                can_cast: JsVariableDeclaration::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static JS_WHILE_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_WHILE_STATEMENT,
    name: "JsWhileStatement",
    fields: &[
        JsAstField {
            property: "whileToken",
            updater: "withWhileToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![while]],
                expected: "\"while\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "test",
            updater: "withTest",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_WITH_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_WITH_STATEMENT,
    name: "JsWithStatement",
    fields: &[
        JsAstField {
            property: "withToken",
            updater: "withWithToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![with]],
                expected: "\"with\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsStatement",
                can_cast: AnyJsStatement::can_cast,
            },
        },
    ],
};
static JS_YIELD_ARGUMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_YIELD_ARGUMENT,
    name: "JsYieldArgument",
    fields: &[
        JsAstField {
            property: "starToken",
            updater: "withStarToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![*]],
                expected: "\"*\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static JS_YIELD_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JS_YIELD_EXPRESSION,
    name: "JsYieldExpression",
    fields: &[
        JsAstField {
            property: "yieldToken",
            updater: "withYieldToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![yield]],
                expected: "\"yield\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsYieldArgument",
                can_cast: JsYieldArgument::can_cast,
            },
        },
    ],
};
static JSX_ATTRIBUTE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_ATTRIBUTE,
    name: "JsxAttribute",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxAttributeName",
                can_cast: AnyJsxAttributeName::can_cast,
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsxAttributeInitializerClause",
                can_cast: JsxAttributeInitializerClause::can_cast,
            },
        },
    ],
};
static JSX_ATTRIBUTE_INITIALIZER_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE,
    name: "JsxAttributeInitializerClause",
    fields: &[
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "value",
            updater: "withValue",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxAttributeValue",
                can_cast: AnyJsxAttributeValue::can_cast,
            },
        },
    ],
};
static JSX_CLOSING_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_CLOSING_ELEMENT,
    name: "JsxClosingElement",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "slashToken",
            updater: "withSlashToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![/]],
                expected: "\"/\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxElementName",
                can_cast: AnyJsxElementName::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static JSX_CLOSING_FRAGMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_CLOSING_FRAGMENT,
    name: "JsxClosingFragment",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "slashToken",
            updater: "withSlashToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![/]],
                expected: "\"/\"",
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static JSX_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_ELEMENT,
    name: "JsxElement",
    fields: &[
        JsAstField {
            property: "openingElement",
            updater: "withOpeningElement",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxOpeningElement",
                can_cast: JsxOpeningElement::can_cast,
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsxChildList",
                can_cast: JsxChildList::can_cast,
            },
        },
        JsAstField {
            property: "closingElement",
            updater: "withClosingElement",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxClosingElement",
                can_cast: JsxClosingElement::can_cast,
            },
        },
    ],
};
static JSX_EXPRESSION_ATTRIBUTE_VALUE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE,
    name: "JsxExpressionAttributeValue",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JSX_EXPRESSION_CHILD_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_EXPRESSION_CHILD,
    name: "JsxExpressionChild",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JSX_FRAGMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_FRAGMENT,
    name: "JsxFragment",
    fields: &[
        JsAstField {
            property: "openingFragment",
            updater: "withOpeningFragment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxOpeningFragment",
                can_cast: JsxOpeningFragment::can_cast,
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsxChildList",
                can_cast: JsxChildList::can_cast,
            },
        },
        JsAstField {
            property: "closingFragment",
            updater: "withClosingFragment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxClosingFragment",
                can_cast: JsxClosingFragment::can_cast,
            },
        },
    ],
};
static JSX_MEMBER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_MEMBER_NAME,
    name: "JsxMemberName",
    fields: &[
        JsAstField {
            property: "object",
            updater: "withObject",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxObjectName",
                can_cast: AnyJsxObjectName::can_cast,
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "member",
            updater: "withMember",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsName",
                can_cast: JsName::can_cast,
            },
        },
    ],
};
static JSX_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_NAME,
    name: "JsxName",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JSX_IDENT],
            expected: "\"jsx_ident\"",
        },
    }],
};
static JSX_NAMESPACE_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_NAMESPACE_NAME,
    name: "JsxNamespaceName",
    fields: &[
        JsAstField {
            property: "namespace",
            updater: "withNamespace",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxName",
                can_cast: JsxName::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsxName",
                can_cast: JsxName::can_cast,
            },
        },
    ],
};
static JSX_OPENING_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_OPENING_ELEMENT,
    name: "JsxOpeningElement",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxElementName",
                can_cast: AnyJsxElementName::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "attributes",
            updater: "withAttributes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsxAttributeList",
                can_cast: JsxAttributeList::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static JSX_OPENING_FRAGMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_OPENING_FRAGMENT,
    name: "JsxOpeningFragment",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static JSX_REFERENCE_IDENTIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_REFERENCE_IDENTIFIER,
    name: "JsxReferenceIdentifier",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JSX_IDENT],
            expected: "\"jsx_ident\"",
        },
    }],
};
static JSX_SELF_CLOSING_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT,
    name: "JsxSelfClosingElement",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsxElementName",
                can_cast: AnyJsxElementName::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "attributes",
            updater: "withAttributes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsxAttributeList",
                can_cast: JsxAttributeList::can_cast,
            },
        },
        JsAstField {
            property: "slashToken",
            updater: "withSlashToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![/]],
                expected: "\"/\"",
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static JSX_SHORTHAND_ATTRIBUTE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE,
    name: "JsxShorthandAttribute",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsReferenceIdentifier",
                can_cast: JsReferenceIdentifier::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JSX_SPREAD_ATTRIBUTE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_SPREAD_ATTRIBUTE,
    name: "JsxSpreadAttribute",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JSX_SPREAD_CHILD_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_SPREAD_CHILD,
    name: "JsxSpreadChild",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static JSX_STRING_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_STRING,
    name: "JsxString",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JSX_STRING_LITERAL],
            expected: "\"jsx_string_literal\"",
        },
    }],
};
static JSX_TAG_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_TAG_EXPRESSION,
    name: "JsxTagExpression",
    fields: &[JsAstField {
        property: "tag",
        updater: "withTag",
        optional: false,
        value: JsAstFieldValue::Node {
            ty: "AnyJsxTag",
            can_cast: AnyJsxTag::can_cast,
        },
    }],
};
static JSX_TEXT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::JSX_TEXT,
    name: "JsxText",
    fields: &[JsAstField {
        property: "valueToken",
        updater: "withValueToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JSX_TEXT_LITERAL],
            expected: "\"jsx_text_literal\"",
        },
    }],
};
static TS_ABSTRACT_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ABSTRACT_MODIFIER,
    name: "TsAbstractModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![abstract]],
            expected: "\"abstract\"",
        },
    }],
};
static TS_ACCESSIBILITY_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER,
    name: "TsAccessibilityModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![private], T![protected], T![public]],
            expected: "\"private\", \"protected\", \"public\"",
        },
    }],
};
static TS_ANY_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ANY_TYPE,
    name: "TsAnyType",
    fields: &[JsAstField {
        property: "anyToken",
        updater: "withAnyToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![any]],
            expected: "\"any\"",
        },
    }],
};
static TS_ARRAY_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ARRAY_TYPE,
    name: "TsArrayType",
    fields: &[
        JsAstField {
            property: "elementType",
            updater: "withElementType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static TS_AS_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_AS_ASSIGNMENT,
    name: "TsAsAssignment",
    fields: &[
        JsAstField {
            property: "assignment",
            updater: "withAssignment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_AS_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_AS_EXPRESSION,
    name: "TsAsExpression",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_ASSERTS_CONDITION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ASSERTS_CONDITION,
    name: "TsAssertsCondition",
    fields: &[
        JsAstField {
            property: "isToken",
            updater: "withIsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![is]],
                expected: "\"is\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_ASSERTS_RETURN_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ASSERTS_RETURN_TYPE,
    name: "TsAssertsReturnType",
    fields: &[
        JsAstField {
            property: "assertsToken",
            updater: "withAssertsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![asserts]],
                expected: "\"asserts\"",
            },
        },
        JsAstField {
            property: "parameterName",
            updater: "withParameterName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsTypePredicateParameterName",
                can_cast: AnyTsTypePredicateParameterName::can_cast,
            },
        },
        JsAstField {
            property: "predicate",
            updater: "withPredicate",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsAssertsCondition",
                can_cast: TsAssertsCondition::can_cast,
            },
        },
    ],
};
static TS_BIGINT_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_BIGINT_LITERAL_TYPE,
    name: "TsBigintLiteralType",
    fields: &[
        JsAstField {
            property: "minusToken",
            updater: "withMinusToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![-]],
                expected: "\"-\"",
            },
        },
        JsAstField {
            property: "literalToken",
            updater: "withLiteralToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[JS_BIGINT_LITERAL],
                expected: "\"js_bigint_literal\"",
            },
        },
    ],
};
static TS_BIGINT_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_BIGINT_TYPE,
    name: "TsBigintType",
    fields: &[JsAstField {
        property: "bigintToken",
        updater: "withBigintToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![bigint]],
            expected: "\"bigint\"",
        },
    }],
};
static TS_BOOLEAN_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE,
    name: "TsBooleanLiteralType",
    fields: &[JsAstField {
        property: "literal",
        updater: "withLiteral",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![true], T![false]],
            expected: "\"true\", \"false\"",
        },
    }],
};
static TS_BOOLEAN_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_BOOLEAN_TYPE,
    name: "TsBooleanType",
    fields: &[JsAstField {
        property: "booleanToken",
        updater: "withBooleanToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![boolean]],
            expected: "\"boolean\"",
        },
    }],
};
static TS_CALL_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER,
    name: "TsCallSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_CONDITIONAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CONDITIONAL_TYPE,
    name: "TsConditionalType",
    fields: &[
        JsAstField {
            property: "checkType",
            updater: "withCheckType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "extendsToken",
            updater: "withExtendsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![extends]],
                expected: "\"extends\"",
            },
        },
        JsAstField {
            property: "extendsType",
            updater: "withExtendsType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "trueType",
            updater: "withTrueType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "falseType",
            updater: "withFalseType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_CONST_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CONST_MODIFIER,
    name: "TsConstModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![const]],
            expected: "\"const\"",
        },
    }],
};
static TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER,
    name: "TsConstructSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "newToken",
            updater: "withNewToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![new]],
                expected: "\"new\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
    name: "TsConstructorSignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsConstructorModifierList",
                can_cast: JsConstructorModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsLiteralMemberName",
                can_cast: JsLiteralMemberName::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsConstructorParameters",
                can_cast: JsConstructorParameters::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_CONSTRUCTOR_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_CONSTRUCTOR_TYPE,
    name: "TsConstructorType",
    fields: &[
        JsAstField {
            property: "abstractToken",
            updater: "withAbstractToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![abstract]],
                expected: "\"abstract\"",
            },
        },
        JsAstField {
            property: "newToken",
            updater: "withNewToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![new]],
                expected: "\"new\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "fatArrowToken",
            updater: "withFatArrowToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=>]],
                expected: "\"=>\"",
            },
        },
        JsAstField {
            property: "returnType",
            updater: "withReturnType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_DECLARATION_MODULE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DECLARATION_MODULE,
    name: "TsDeclarationModule",
    fields: &[
        JsAstField {
            property: "bomToken",
            updater: "withBomToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![UNICODE_BOM]],
                expected: "\"UNICODE_BOM\"",
            },
        },
        JsAstField {
            property: "interpreterToken",
            updater: "withInterpreterToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[JS_SHEBANG],
                expected: "\"js_shebang\"",
            },
        },
        JsAstField {
            property: "directives",
            updater: "withDirectives",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDirectiveList",
                can_cast: JsDirectiveList::can_cast,
            },
        },
        JsAstField {
            property: "items",
            updater: "withItems",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsModuleItemList",
                can_cast: JsModuleItemList::can_cast,
            },
        },
        JsAstField {
            property: "eofToken",
            updater: "withEofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![EOF]],
                expected: "\"EOF\"",
            },
        },
    ],
};
static TS_DECLARE_FUNCTION_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
    name: "TsDeclareFunctionDeclaration",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "functionToken",
            updater: "withFunctionToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![function]],
                expected: "\"function\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION,
    name: "TsDeclareFunctionExportDefaultDeclaration",
    fields: &[
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "functionToken",
            updater: "withFunctionToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![function]],
                expected: "\"function\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_DECLARE_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DECLARE_MODIFIER,
    name: "TsDeclareModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![declare]],
            expected: "\"declare\"",
        },
    }],
};
static TS_DECLARE_STATEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DECLARE_STATEMENT,
    name: "TsDeclareStatement",
    fields: &[
        JsAstField {
            property: "declareToken",
            updater: "withDeclareToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![declare]],
                expected: "\"declare\"",
            },
        },
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsDeclarationClause",
                can_cast: AnyJsDeclarationClause::can_cast,
            },
        },
    ],
};
static TS_DEFAULT_TYPE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE,
    name: "TsDefaultTypeClause",
    fields: &[
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_DEFINITE_PROPERTY_ANNOTATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION,
    name: "TsDefinitePropertyAnnotation",
    fields: &[
        JsAstField {
            property: "exclToken",
            updater: "withExclToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![!]],
                expected: "\"!\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static TS_DEFINITE_VARIABLE_ANNOTATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION,
    name: "TsDefiniteVariableAnnotation",
    fields: &[
        JsAstField {
            property: "exclToken",
            updater: "withExclToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![!]],
                expected: "\"!\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY,
    name: "TsEmptyExternalModuleDeclarationBody",
    fields: &[JsAstField {
        property: "semicolonToken",
        updater: "withSemicolonToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![;]],
            expected: "\";\"",
        },
    }],
};
static TS_ENUM_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ENUM_DECLARATION,
    name: "TsEnumDeclaration",
    fields: &[
        JsAstField {
            property: "constToken",
            updater: "withConstToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![const]],
                expected: "\"const\"",
            },
        },
        JsAstField {
            property: "enumToken",
            updater: "withEnumToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![enum]],
                expected: "\"enum\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsEnumMemberList",
                can_cast: TsEnumMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_ENUM_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_ENUM_MEMBER,
    name: "TsEnumMember",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsEnumMemberName",
                can_cast: AnyTsEnumMemberName::can_cast,
            },
        },
        JsAstField {
            property: "initializer",
            updater: "withInitializer",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
    ],
};
static TS_EXPORT_AS_NAMESPACE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE,
    name: "TsExportAsNamespaceClause",
    fields: &[
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "namespaceToken",
            updater: "withNamespaceToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![namespace]],
                expected: "\"namespace\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsName",
                can_cast: JsName::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_EXPORT_ASSIGNMENT_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE,
    name: "TsExportAssignmentClause",
    fields: &[
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_EXPORT_DECLARE_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE,
    name: "TsExportDeclareClause",
    fields: &[
        JsAstField {
            property: "declareToken",
            updater: "withDeclareToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![declare]],
                expected: "\"declare\"",
            },
        },
        JsAstField {
            property: "declaration",
            updater: "withDeclaration",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsDeclarationClause",
                can_cast: AnyJsDeclarationClause::can_cast,
            },
        },
    ],
};
static TS_EXTENDS_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXTENDS_CLAUSE,
    name: "TsExtendsClause",
    fields: &[
        JsAstField {
            property: "extendsToken",
            updater: "withExtendsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![extends]],
                expected: "\"extends\"",
            },
        },
        JsAstField {
            property: "types",
            updater: "withTypes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeList",
                can_cast: TsTypeList::can_cast,
            },
        },
    ],
};
static TS_EXTERNAL_MODULE_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION,
    name: "TsExternalModuleDeclaration",
    fields: &[
        JsAstField {
            property: "moduleToken",
            updater: "withModuleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![module]],
                expected: "\"module\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyTsExternalModuleDeclarationBody",
                can_cast: AnyTsExternalModuleDeclarationBody::can_cast,
            },
        },
    ],
};
static TS_EXTERNAL_MODULE_REFERENCE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE,
    name: "TsExternalModuleReference",
    fields: &[
        JsAstField {
            property: "requireToken",
            updater: "withRequireToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![require]],
                expected: "\"require\"",
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "source",
            updater: "withSource",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsModuleSource",
                can_cast: AnyJsModuleSource::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static TS_FUNCTION_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_FUNCTION_TYPE,
    name: "TsFunctionType",
    fields: &[
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "fatArrowToken",
            updater: "withFatArrowToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=>]],
                expected: "\"=>\"",
            },
        },
        JsAstField {
            property: "returnType",
            updater: "withReturnType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsReturnType",
                can_cast: AnyTsReturnType::can_cast,
            },
        },
    ],
};
static TS_GETTER_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER,
    name: "TsGetterSignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsMethodSignatureModifierList",
                can_cast: TsMethodSignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "getToken",
            updater: "withGetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![get]],
                expected: "\"get\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "returnType",
            updater: "withReturnType",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_GETTER_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER,
    name: "TsGetterSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "getToken",
            updater: "withGetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![get]],
                expected: "\"get\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_GLOBAL_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_GLOBAL_DECLARATION,
    name: "TsGlobalDeclaration",
    fields: &[
        JsAstField {
            property: "globalToken",
            updater: "withGlobalToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![global]],
                expected: "\"global\"",
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsModuleBlock",
                can_cast: TsModuleBlock::can_cast,
            },
        },
    ],
};
static TS_IDENTIFIER_BINDING_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IDENTIFIER_BINDING,
    name: "TsIdentifierBinding",
    fields: &[JsAstField {
        property: "nameToken",
        updater: "withNameToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static TS_IMPLEMENTS_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPLEMENTS_CLAUSE,
    name: "TsImplementsClause",
    fields: &[
        JsAstField {
            property: "implementsToken",
            updater: "withImplementsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![implements]],
                expected: "\"implements\"",
            },
        },
        JsAstField {
            property: "types",
            updater: "withTypes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeList",
                can_cast: TsTypeList::can_cast,
            },
        },
    ],
};
static TS_IMPORT_EQUALS_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION,
    name: "TsImportEqualsDeclaration",
    fields: &[
        JsAstField {
            property: "importToken",
            updater: "withImportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![import]],
                expected: "\"import\"",
            },
        },
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsBinding",
                can_cast: AnyJsBinding::can_cast,
            },
        },
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "moduleReference",
            updater: "withModuleReference",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsModuleReference",
                can_cast: AnyTsModuleReference::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_IMPORT_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_TYPE,
    name: "TsImportType",
    fields: &[
        JsAstField {
            property: "typeofToken",
            updater: "withTypeofToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![typeof]],
                expected: "\"typeof\"",
            },
        },
        JsAstField {
            property: "importToken",
            updater: "withImportToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![import]],
                expected: "\"import\"",
            },
        },
        JsAstField {
            property: "arguments",
            updater: "withArguments",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsImportTypeArguments",
                can_cast: TsImportTypeArguments::can_cast,
            },
        },
        JsAstField {
            property: "qualifierClause",
            updater: "withQualifierClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsImportTypeQualifier",
                can_cast: TsImportTypeQualifier::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
    ],
};
static TS_IMPORT_TYPE_ARGUMENTS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS,
    name: "TsImportTypeArguments",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "argument",
            updater: "withArgument",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "tsImportTypeAssertionBlock",
            updater: "withTsImportTypeAssertionBlock",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsImportTypeAssertionBlock",
                can_cast: TsImportTypeAssertionBlock::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static TS_IMPORT_TYPE_ASSERTION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION,
    name: "TsImportTypeAssertion",
    fields: &[
        JsAstField {
            property: "withToken",
            updater: "withWithToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![with]],
                expected: "\"with\"",
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "assertions",
            updater: "withAssertions",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsImportAssertionEntryList",
                can_cast: JsImportAssertionEntryList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_IMPORT_TYPE_ASSERTION_BLOCK_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK,
    name: "TsImportTypeAssertionBlock",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "typeAssertion",
            updater: "withTypeAssertion",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsImportTypeAssertion",
                can_cast: TsImportTypeAssertion::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_IMPORT_TYPE_QUALIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER,
    name: "TsImportTypeQualifier",
    fields: &[
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsName",
                can_cast: AnyTsName::can_cast,
            },
        },
    ],
};
static TS_IN_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_IN_MODIFIER,
    name: "TsInModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![in]],
            expected: "\"in\"",
        },
    }],
};
static TS_INDEX_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER,
    name: "TsIndexSignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsIndexSignatureModifierList",
                can_cast: TsIndexSignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsIndexSignatureParameter",
                can_cast: TsIndexSignatureParameter::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_INDEX_SIGNATURE_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER,
    name: "TsIndexSignatureParameter",
    fields: &[
        JsAstField {
            property: "binding",
            updater: "withBinding",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsIdentifierBinding",
                can_cast: JsIdentifierBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static TS_INDEX_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER,
    name: "TsIndexSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "readonlyToken",
            updater: "withReadonlyToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![readonly]],
                expected: "\"readonly\"",
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsIndexSignatureParameter",
                can_cast: TsIndexSignatureParameter::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_INDEXED_ACCESS_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INDEXED_ACCESS_TYPE,
    name: "TsIndexedAccessType",
    fields: &[
        JsAstField {
            property: "objectType",
            updater: "withObjectType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "indexType",
            updater: "withIndexType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static TS_INFER_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INFER_TYPE,
    name: "TsInferType",
    fields: &[
        JsAstField {
            property: "inferToken",
            updater: "withInferToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![infer]],
                expected: "\"infer\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameterName",
                can_cast: TsTypeParameterName::can_cast,
            },
        },
        JsAstField {
            property: "constraint",
            updater: "withConstraint",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeConstraintClause",
                can_cast: TsTypeConstraintClause::can_cast,
            },
        },
    ],
};
static TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER,
    name: "TsInitializedPropertySignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsPropertySignatureModifierList",
                can_cast: TsPropertySignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "value",
            updater: "withValue",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsInitializerClause",
                can_cast: JsInitializerClause::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_INSTANTIATION_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INSTANTIATION_EXPRESSION,
    name: "TsInstantiationExpression",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "arguments",
            updater: "withArguments",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
    ],
};
static TS_INTERFACE_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INTERFACE_DECLARATION,
    name: "TsInterfaceDeclaration",
    fields: &[
        JsAstField {
            property: "interfaceToken",
            updater: "withInterfaceToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![interface]],
                expected: "\"interface\"",
            },
        },
        JsAstField {
            property: "id",
            updater: "withId",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsIdentifierBinding",
                can_cast: AnyTsIdentifierBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "extendsClause",
            updater: "withExtendsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsExtendsClause",
                can_cast: TsExtendsClause::can_cast,
            },
        },
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeMemberList",
                can_cast: TsTypeMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_INTERSECTION_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_INTERSECTION_TYPE,
    name: "TsIntersectionType",
    fields: &[
        JsAstField {
            property: "leadingSeparatorToken",
            updater: "withLeadingSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![&]],
                expected: "\"&\"",
            },
        },
        JsAstField {
            property: "types",
            updater: "withTypes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsIntersectionTypeElementList",
                can_cast: TsIntersectionTypeElementList::can_cast,
            },
        },
    ],
};
static TS_LITERAL_ENUM_MEMBER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME,
    name: "TsLiteralEnumMemberName",
    fields: &[JsAstField {
        property: "value",
        updater: "withValue",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT, JS_STRING_LITERAL],
            expected: "\"ident\", \"js_string_literal\"",
        },
    }],
};
static TS_MAPPED_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MAPPED_TYPE,
    name: "TsMappedType",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "readonlyModifier",
            updater: "withReadonlyModifier",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsMappedTypeReadonlyModifierClause",
                can_cast: TsMappedTypeReadonlyModifierClause::can_cast,
            },
        },
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "propertyName",
            updater: "withPropertyName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameterName",
                can_cast: TsTypeParameterName::can_cast,
            },
        },
        JsAstField {
            property: "inToken",
            updater: "withInToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![in]],
                expected: "\"in\"",
            },
        },
        JsAstField {
            property: "keysType",
            updater: "withKeysType",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "asClause",
            updater: "withAsClause",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsMappedTypeAsClause",
                can_cast: TsMappedTypeAsClause::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
        JsAstField {
            property: "optionalModifier",
            updater: "withOptionalModifier",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsMappedTypeOptionalModifierClause",
                can_cast: TsMappedTypeOptionalModifierClause::can_cast,
            },
        },
        JsAstField {
            property: "mappedType",
            updater: "withMappedType",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_MAPPED_TYPE_AS_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE,
    name: "TsMappedTypeAsClause",
    fields: &[
        JsAstField {
            property: "asToken",
            updater: "withAsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![as]],
                expected: "\"as\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE,
    name: "TsMappedTypeOptionalModifierClause",
    fields: &[
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![+], T![-]],
                expected: "\"+\", \"-\"",
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
    ],
};
static TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE,
    name: "TsMappedTypeReadonlyModifierClause",
    fields: &[
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![+], T![-]],
                expected: "\"+\", \"-\"",
            },
        },
        JsAstField {
            property: "readonlyToken",
            updater: "withReadonlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![readonly]],
                expected: "\"readonly\"",
            },
        },
    ],
};
static TS_METHOD_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER,
    name: "TsMethodSignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsMethodSignatureModifierList",
                can_cast: TsMethodSignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "asyncToken",
            updater: "withAsyncToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![async]],
                expected: "\"async\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_METHOD_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER,
    name: "TsMethodSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "optionalToken",
            updater: "withOptionalToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "parameters",
            updater: "withParameters",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsParameters",
                can_cast: JsParameters::can_cast,
            },
        },
        JsAstField {
            property: "returnTypeAnnotation",
            updater: "withReturnTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsReturnTypeAnnotation",
                can_cast: TsReturnTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_MODULE_BLOCK_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MODULE_BLOCK,
    name: "TsModuleBlock",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "items",
            updater: "withItems",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsModuleItemList",
                can_cast: JsModuleItemList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_MODULE_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_MODULE_DECLARATION,
    name: "TsModuleDeclaration",
    fields: &[
        JsAstField {
            property: "moduleOrNamespace",
            updater: "withModuleOrNamespace",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![module], T![namespace]],
                expected: "\"module\", \"namespace\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsModuleName",
                can_cast: AnyTsModuleName::can_cast,
            },
        },
        JsAstField {
            property: "body",
            updater: "withBody",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsModuleBlock",
                can_cast: TsModuleBlock::can_cast,
            },
        },
    ],
};
static TS_NAMED_TUPLE_TYPE_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT,
    name: "TsNamedTupleTypeElement",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsName",
                can_cast: JsName::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_NEVER_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NEVER_TYPE,
    name: "TsNeverType",
    fields: &[JsAstField {
        property: "neverToken",
        updater: "withNeverToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![never]],
            expected: "\"never\"",
        },
    }],
};
static TS_NON_NULL_ASSERTION_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT,
    name: "TsNonNullAssertionAssignment",
    fields: &[
        JsAstField {
            property: "assignment",
            updater: "withAssignment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
        JsAstField {
            property: "exclToken",
            updater: "withExclToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![!]],
                expected: "\"!\"",
            },
        },
    ],
};
static TS_NON_NULL_ASSERTION_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION,
    name: "TsNonNullAssertionExpression",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "exclToken",
            updater: "withExclToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![!]],
                expected: "\"!\"",
            },
        },
    ],
};
static TS_NON_PRIMITIVE_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NON_PRIMITIVE_TYPE,
    name: "TsNonPrimitiveType",
    fields: &[JsAstField {
        property: "objectToken",
        updater: "withObjectToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![object]],
            expected: "\"object\"",
        },
    }],
};
static TS_NULL_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NULL_LITERAL_TYPE,
    name: "TsNullLiteralType",
    fields: &[JsAstField {
        property: "literalToken",
        updater: "withLiteralToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![null]],
            expected: "\"null\"",
        },
    }],
};
static TS_NUMBER_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NUMBER_LITERAL_TYPE,
    name: "TsNumberLiteralType",
    fields: &[
        JsAstField {
            property: "minusToken",
            updater: "withMinusToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![-]],
                expected: "\"-\"",
            },
        },
        JsAstField {
            property: "literalToken",
            updater: "withLiteralToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[JS_NUMBER_LITERAL],
                expected: "\"js_number_literal\"",
            },
        },
    ],
};
static TS_NUMBER_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_NUMBER_TYPE,
    name: "TsNumberType",
    fields: &[JsAstField {
        property: "numberToken",
        updater: "withNumberToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![number]],
            expected: "\"number\"",
        },
    }],
};
static TS_OBJECT_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_OBJECT_TYPE,
    name: "TsObjectType",
    fields: &[
        JsAstField {
            property: "lCurlyToken",
            updater: "withLCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['{']],
                expected: "\"'{'\"",
            },
        },
        JsAstField {
            property: "members",
            updater: "withMembers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeMemberList",
                can_cast: TsTypeMemberList::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_OPTIONAL_PROPERTY_ANNOTATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION,
    name: "TsOptionalPropertyAnnotation",
    fields: &[
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static TS_OPTIONAL_TUPLE_TYPE_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT,
    name: "TsOptionalTupleTypeElement",
    fields: &[
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "questionMarkToken",
            updater: "withQuestionMarkToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
    ],
};
static TS_OUT_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_OUT_MODIFIER,
    name: "TsOutModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![out]],
            expected: "\"out\"",
        },
    }],
};
static TS_OVERRIDE_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_OVERRIDE_MODIFIER,
    name: "TsOverrideModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![override]],
            expected: "\"override\"",
        },
    }],
};
static TS_PARENTHESIZED_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_PARENTHESIZED_TYPE,
    name: "TsParenthesizedType",
    fields: &[
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
    ],
};
static TS_PREDICATE_RETURN_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_PREDICATE_RETURN_TYPE,
    name: "TsPredicateReturnType",
    fields: &[
        JsAstField {
            property: "parameterName",
            updater: "withParameterName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsTypePredicateParameterName",
                can_cast: AnyTsTypePredicateParameterName::can_cast,
            },
        },
        JsAstField {
            property: "isToken",
            updater: "withIsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![is]],
                expected: "\"is\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_PROPERTY_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_PROPERTY_PARAMETER,
    name: "TsPropertyParameter",
    fields: &[
        JsAstField {
            property: "decorators",
            updater: "withDecorators",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "JsDecoratorList",
                can_cast: JsDecoratorList::can_cast,
            },
        },
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsPropertyParameterModifierList",
                can_cast: TsPropertyParameterModifierList::can_cast,
            },
        },
        JsAstField {
            property: "formalParameter",
            updater: "withFormalParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFormalParameter",
                can_cast: AnyJsFormalParameter::can_cast,
            },
        },
    ],
};
static TS_PROPERTY_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER,
    name: "TsPropertySignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsPropertySignatureModifierList",
                can_cast: TsPropertySignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "propertyAnnotation",
            updater: "withPropertyAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "AnyTsPropertySignatureAnnotation",
                can_cast: AnyTsPropertySignatureAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_PROPERTY_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER,
    name: "TsPropertySignatureTypeMember",
    fields: &[
        JsAstField {
            property: "readonlyToken",
            updater: "withReadonlyToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![readonly]],
                expected: "\"readonly\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "optionalToken",
            updater: "withOptionalToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![?]],
                expected: "\"?\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_QUALIFIED_MODULE_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_QUALIFIED_MODULE_NAME,
    name: "TsQualifiedModuleName",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsModuleName",
                can_cast: AnyTsModuleName::can_cast,
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsName",
                can_cast: JsName::can_cast,
            },
        },
    ],
};
static TS_QUALIFIED_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_QUALIFIED_NAME,
    name: "TsQualifiedName",
    fields: &[
        JsAstField {
            property: "left",
            updater: "withLeft",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsName",
                can_cast: AnyTsName::can_cast,
            },
        },
        JsAstField {
            property: "dotToken",
            updater: "withDotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![.]],
                expected: "\".\"",
            },
        },
        JsAstField {
            property: "right",
            updater: "withRight",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "JsName",
                can_cast: JsName::can_cast,
            },
        },
    ],
};
static TS_READONLY_MODIFIER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_READONLY_MODIFIER,
    name: "TsReadonlyModifier",
    fields: &[JsAstField {
        property: "modifierToken",
        updater: "withModifierToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![readonly]],
            expected: "\"readonly\"",
        },
    }],
};
static TS_REFERENCE_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_REFERENCE_TYPE,
    name: "TsReferenceType",
    fields: &[
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsName",
                can_cast: AnyTsName::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
    ],
};
static TS_REST_TUPLE_TYPE_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT,
    name: "TsRestTupleTypeElement",
    fields: &[
        JsAstField {
            property: "dotdotdotToken",
            updater: "withDotdotdotToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![...]],
                expected: "\"...\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_RETURN_TYPE_ANNOTATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION,
    name: "TsReturnTypeAnnotation",
    fields: &[
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsReturnType",
                can_cast: AnyTsReturnType::can_cast,
            },
        },
    ],
};
static TS_SATISFIES_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_SATISFIES_ASSIGNMENT,
    name: "TsSatisfiesAssignment",
    fields: &[
        JsAstField {
            property: "assignment",
            updater: "withAssignment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
        JsAstField {
            property: "satisfiesToken",
            updater: "withSatisfiesToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![satisfies]],
                expected: "\"satisfies\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_SATISFIES_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_SATISFIES_EXPRESSION,
    name: "TsSatisfiesExpression",
    fields: &[
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
        JsAstField {
            property: "satisfiesToken",
            updater: "withSatisfiesToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![satisfies]],
                expected: "\"satisfies\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_SETTER_SIGNATURE_CLASS_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER,
    name: "TsSetterSignatureClassMember",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsMethodSignatureModifierList",
                can_cast: TsMethodSignatureModifierList::can_cast,
            },
        },
        JsAstField {
            property: "setToken",
            updater: "withSetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![set]],
                expected: "\"set\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsClassMemberName",
                can_cast: AnyJsClassMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFormalParameter",
                can_cast: AnyJsFormalParameter::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_SETTER_SIGNATURE_TYPE_MEMBER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER,
    name: "TsSetterSignatureTypeMember",
    fields: &[
        JsAstField {
            property: "setToken",
            updater: "withSetToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![set]],
                expected: "\"set\"",
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsObjectMemberName",
                can_cast: AnyJsObjectMemberName::can_cast,
            },
        },
        JsAstField {
            property: "lParenToken",
            updater: "withLParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['(']],
                expected: "\"'('\"",
            },
        },
        JsAstField {
            property: "parameter",
            updater: "withParameter",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsFormalParameter",
                can_cast: AnyJsFormalParameter::can_cast,
            },
        },
        JsAstField {
            property: "commaToken",
            updater: "withCommaToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,]],
                expected: "\",\"",
            },
        },
        JsAstField {
            property: "rParenToken",
            updater: "withRParenToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![')']],
                expected: "\"')'\"",
            },
        },
        JsAstField {
            property: "separatorToken",
            updater: "withSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![,], T![;]],
                expected: "\",\", \";\"",
            },
        },
    ],
};
static TS_STRING_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_STRING_LITERAL_TYPE,
    name: "TsStringLiteralType",
    fields: &[JsAstField {
        property: "literalToken",
        updater: "withLiteralToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[JS_STRING_LITERAL],
            expected: "\"js_string_literal\"",
        },
    }],
};
static TS_STRING_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_STRING_TYPE,
    name: "TsStringType",
    fields: &[JsAstField {
        property: "stringToken",
        updater: "withStringToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![string]],
            expected: "\"string\"",
        },
    }],
};
static TS_SYMBOL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_SYMBOL_TYPE,
    name: "TsSymbolType",
    fields: &[JsAstField {
        property: "symbolToken",
        updater: "withSymbolToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![symbol]],
            expected: "\"symbol\"",
        },
    }],
};
static TS_TEMPLATE_CHUNK_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT,
    name: "TsTemplateChunkElement",
    fields: &[JsAstField {
        property: "templateChunkToken",
        updater: "withTemplateChunkToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[TEMPLATE_CHUNK],
            expected: "\"template_chunk\"",
        },
    }],
};
static TS_TEMPLATE_ELEMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TEMPLATE_ELEMENT,
    name: "TsTemplateElement",
    fields: &[
        JsAstField {
            property: "dollarCurlyToken",
            updater: "withDollarCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[DOLLAR_CURLY],
                expected: "\"dollar_curly\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "rCurlyToken",
            updater: "withRCurlyToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['}']],
                expected: "\"'}'\"",
            },
        },
    ],
};
static TS_TEMPLATE_LITERAL_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE,
    name: "TsTemplateLiteralType",
    fields: &[
        JsAstField {
            property: "lTickToken",
            updater: "withLTickToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['`']],
                expected: "\"'`'\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTemplateElementList",
                can_cast: TsTemplateElementList::can_cast,
            },
        },
        JsAstField {
            property: "rTickToken",
            updater: "withRTickToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['`']],
                expected: "\"'`'\"",
            },
        },
    ],
};
static TS_THIS_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_THIS_PARAMETER,
    name: "TsThisParameter",
    fields: &[
        JsAstField {
            property: "thisToken",
            updater: "withThisToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![this]],
                expected: "\"this\"",
            },
        },
        JsAstField {
            property: "typeAnnotation",
            updater: "withTypeAnnotation",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeAnnotation",
                can_cast: TsTypeAnnotation::can_cast,
            },
        },
    ],
};
static TS_THIS_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_THIS_TYPE,
    name: "TsThisType",
    fields: &[JsAstField {
        property: "thisToken",
        updater: "withThisToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![this]],
            expected: "\"this\"",
        },
    }],
};
static TS_TUPLE_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TUPLE_TYPE,
    name: "TsTupleType",
    fields: &[
        JsAstField {
            property: "lBrackToken",
            updater: "withLBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T!['[']],
                expected: "\"'['\"",
            },
        },
        JsAstField {
            property: "elements",
            updater: "withElements",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTupleTypeElementList",
                can_cast: TsTupleTypeElementList::can_cast,
            },
        },
        JsAstField {
            property: "rBrackToken",
            updater: "withRBrackToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![']']],
                expected: "\"']'\"",
            },
        },
    ],
};
static TS_TYPE_ALIAS_DECLARATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION,
    name: "TsTypeAliasDeclaration",
    fields: &[
        JsAstField {
            property: "typeToken",
            updater: "withTypeToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![type]],
                expected: "\"type\"",
            },
        },
        JsAstField {
            property: "bindingIdentifier",
            updater: "withBindingIdentifier",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsIdentifierBinding",
                can_cast: AnyTsIdentifierBinding::can_cast,
            },
        },
        JsAstField {
            property: "typeParameters",
            updater: "withTypeParameters",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameters",
                can_cast: TsTypeParameters::can_cast,
            },
        },
        JsAstField {
            property: "eqToken",
            updater: "withEqToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![=]],
                expected: "\"=\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "semicolonToken",
            updater: "withSemicolonToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![;]],
                expected: "\";\"",
            },
        },
    ],
};
static TS_TYPE_ANNOTATION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_ANNOTATION,
    name: "TsTypeAnnotation",
    fields: &[
        JsAstField {
            property: "colonToken",
            updater: "withColonToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![:]],
                expected: "\":\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_TYPE_ARGUMENTS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_ARGUMENTS,
    name: "TsTypeArguments",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "tsTypeArgumentList",
            updater: "withTsTypeArgumentList",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeArgumentList",
                can_cast: TsTypeArgumentList::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static TS_TYPE_ASSERTION_ASSIGNMENT_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT,
    name: "TsTypeAssertionAssignment",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
        JsAstField {
            property: "assignment",
            updater: "withAssignment",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsAssignment",
                can_cast: AnyJsAssignment::can_cast,
            },
        },
    ],
};
static TS_TYPE_ASSERTION_EXPRESSION_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION,
    name: "TsTypeAssertionExpression",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
        JsAstField {
            property: "expression",
            updater: "withExpression",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyJsExpression",
                can_cast: AnyJsExpression::can_cast,
            },
        },
    ],
};
static TS_TYPE_CONSTRAINT_CLAUSE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE,
    name: "TsTypeConstraintClause",
    fields: &[
        JsAstField {
            property: "extendsToken",
            updater: "withExtendsToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![extends]],
                expected: "\"extends\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_TYPE_OPERATOR_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_OPERATOR_TYPE,
    name: "TsTypeOperatorType",
    fields: &[
        JsAstField {
            property: "operatorToken",
            updater: "withOperatorToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![keyof], T![unique], T![readonly]],
                expected: "\"keyof\", \"unique\", \"readonly\"",
            },
        },
        JsAstField {
            property: "ty",
            updater: "withTy",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsType",
                can_cast: AnyTsType::can_cast,
            },
        },
    ],
};
static TS_TYPE_PARAMETER_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_PARAMETER,
    name: "TsTypeParameter",
    fields: &[
        JsAstField {
            property: "modifiers",
            updater: "withModifiers",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeParameterModifierList",
                can_cast: TsTypeParameterModifierList::can_cast,
            },
        },
        JsAstField {
            property: "name",
            updater: "withName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "TsTypeParameterName",
                can_cast: TsTypeParameterName::can_cast,
            },
        },
        JsAstField {
            property: "constraint",
            updater: "withConstraint",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeConstraintClause",
                can_cast: TsTypeConstraintClause::can_cast,
            },
        },
        JsAstField {
            property: "default",
            updater: "withDefault",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsDefaultTypeClause",
                can_cast: TsDefaultTypeClause::can_cast,
            },
        },
    ],
};
static TS_TYPE_PARAMETER_NAME_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_PARAMETER_NAME,
    name: "TsTypeParameterName",
    fields: &[JsAstField {
        property: "identToken",
        updater: "withIdentToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[IDENT],
            expected: "\"ident\"",
        },
    }],
};
static TS_TYPE_PARAMETERS_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPE_PARAMETERS,
    name: "TsTypeParameters",
    fields: &[
        JsAstField {
            property: "lAngleToken",
            updater: "withLAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![<]],
                expected: "\"<\"",
            },
        },
        JsAstField {
            property: "items",
            updater: "withItems",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsTypeParameterList",
                can_cast: TsTypeParameterList::can_cast,
            },
        },
        JsAstField {
            property: "rAngleToken",
            updater: "withRAngleToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![>]],
                expected: "\">\"",
            },
        },
    ],
};
static TS_TYPEOF_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_TYPEOF_TYPE,
    name: "TsTypeofType",
    fields: &[
        JsAstField {
            property: "typeofToken",
            updater: "withTypeofToken",
            optional: false,
            value: JsAstFieldValue::Token {
                kinds: &[T![typeof]],
                expected: "\"typeof\"",
            },
        },
        JsAstField {
            property: "expressionName",
            updater: "withExpressionName",
            optional: false,
            value: JsAstFieldValue::Node {
                ty: "AnyTsName",
                can_cast: AnyTsName::can_cast,
            },
        },
        JsAstField {
            property: "typeArguments",
            updater: "withTypeArguments",
            optional: true,
            value: JsAstFieldValue::Node {
                ty: "TsTypeArguments",
                can_cast: TsTypeArguments::can_cast,
            },
        },
    ],
};
static TS_UNDEFINED_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_UNDEFINED_TYPE,
    name: "TsUndefinedType",
    fields: &[JsAstField {
        property: "undefinedToken",
        updater: "withUndefinedToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![undefined]],
            expected: "\"undefined\"",
        },
    }],
};
static TS_UNION_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_UNION_TYPE,
    name: "TsUnionType",
    fields: &[
        JsAstField {
            property: "leadingSeparatorToken",
            updater: "withLeadingSeparatorToken",
            optional: true,
            value: JsAstFieldValue::Token {
                kinds: &[T![|]],
                expected: "\"|\"",
            },
        },
        JsAstField {
            property: "types",
            updater: "withTypes",
            optional: false,
            value: JsAstFieldValue::List {
                ty: "TsUnionTypeVariantList",
                can_cast: TsUnionTypeVariantList::can_cast,
            },
        },
    ],
};
static TS_UNKNOWN_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_UNKNOWN_TYPE,
    name: "TsUnknownType",
    fields: &[JsAstField {
        property: "unknownToken",
        updater: "withUnknownToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![unknown]],
            expected: "\"unknown\"",
        },
    }],
};
static TS_VOID_TYPE_FIELDS: JsAstNodeFields = JsAstNodeFields {
    kind: JsSyntaxKind::TS_VOID_TYPE,
    name: "TsVoidType",
    fields: &[JsAstField {
        property: "voidToken",
        updater: "withVoidToken",
        optional: false,
        value: JsAstFieldValue::Token {
            kinds: &[T![void]],
            expected: "\"void\"",
        },
    }],
};
