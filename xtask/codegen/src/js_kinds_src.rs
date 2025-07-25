//! Definitions for the ECMAScript AST used for codegen
//! Based on the rust analyzer parser and ast definitions

use crate::kind_src::KindsSrc;
use crate::language_kind::{LANGUAGE_PREFIXES, LanguageKind};
use quote::format_ident;
use std::collections::BTreeMap;

pub const JS_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("?", "QUESTION"),
        ("??", "QUESTION2"),
        // These are *not* question AND dot tokens, they are one
        // to distinguish between `? .3134` and `?.` per ecma specs
        ("?.", "QUESTIONDOT"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("+", "PLUS"),
        ("++", "PLUS2"),
        ("*", "STAR"),
        ("**", "STAR2"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        (".", "DOT"),
        ("...", "DOT3"),
        (":", "COLON"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("===", "EQ3"),
        ("=>", "FAT_ARROW"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("!==", "NEQ2"),
        ("-", "MINUS"),
        ("--", "MINUS2"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("<<", "SHL"),
        (">>", "SHR"),
        (">>>", "USHR"),
        ("<<=", "SHLEQ"),
        (">>=", "SHREQ"),
        (">>>=", "USHREQ"),
        ("&&=", "AMP2EQ"),
        ("||=", "PIPE2EQ"),
        ("**=", "STAR2EQ"),
        ("??=", "QUESTION2EQ"),
        ("@", "AT"),
        ("`", "BACKTICK"),
    ],
    keywords: &[
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "function",
        "if",
        "in",
        "instanceof",
        "import",
        "new",
        "null",
        "return",
        "super",
        "switch",
        "this",
        "throw",
        "try",
        "true",
        "typeof",
        "var",
        "void",
        "while",
        "with",
        // Strict mode contextual keywords
        "implements",
        "interface",
        "let",
        "package",
        "private",
        "protected",
        "public",
        "static",
        "yield",
        // contextual keywords
        "abstract",
        "accessor",
        "as",
        "satisfies",
        "asserts",
        "assert",
        "any",
        "async",
        "await",
        "boolean",
        "constructor",
        "declare",
        "defer",
        "get",
        "infer",
        "is",
        "keyof",
        "module",
        "namespace",
        "never",
        "readonly",
        "require",
        "number",
        "object",
        "set",
        "string",
        "symbol",
        "type",
        "undefined",
        "unique",
        "unknown",
        "from",
        "global",
        "bigint",
        "override",
        "of",
        "out",
        "using",
        "meta",
    ],
    literals: &[
        "JS_NUMBER_LITERAL",
        "JS_BIGINT_LITERAL",
        "JS_STRING_LITERAL",
        "JS_REGEX_LITERAL",
        "JSX_TEXT_LITERAL",
        "JSX_STRING_LITERAL",
    ],
    tokens: &[
        "TARGET",
        "META",
        "HASH", // #
        "TEMPLATE_CHUNK",
        "DOLLAR_CURLY", // ${
        "ERROR_TOKEN",
        "IDENT",
        "JSX_IDENT",
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
        "JS_SHEBANG",
        "GRIT_METAVARIABLE",
    ],
    nodes: &[
        "JS_MODULE",
        "JS_MODULE_ITEM_LIST",
        "JS_SCRIPT",
        "TS_DECLARATION_MODULE",
        "JS_EXPRESSION_SNIPPED",
        "JS_DIRECTIVE",
        "JS_DIRECTIVE_LIST",
        "JS_STATEMENT_LIST",
        "JS_BLOCK_STATEMENT",
        "JS_FUNCTION_BODY",
        "JS_VARIABLE_STATEMENT",
        "JS_VARIABLE_DECLARATION",
        "JS_VARIABLE_DECLARATOR_LIST",
        "JS_VARIABLE_DECLARATOR",
        "JS_VARIABLE_DECLARATION_CLAUSE",
        "TS_DEFINITE_VARIABLE_ANNOTATION",
        "JS_INITIALIZER_CLAUSE",
        "JS_EMPTY_STATEMENT",
        "JS_EXPRESSION_STATEMENT",
        "JS_IF_STATEMENT",
        "JS_ELSE_CLAUSE",
        "JS_DO_WHILE_STATEMENT",
        "JS_WHILE_STATEMENT",
        "JS_FOR_STATEMENT",
        "JS_FOR_IN_STATEMENT",
        "JS_FOR_OF_STATEMENT",
        "JS_FOR_VARIABLE_DECLARATION",
        "JS_CONTINUE_STATEMENT",
        "JS_BREAK_STATEMENT",
        "JS_RETURN_STATEMENT",
        "JS_WITH_STATEMENT",
        "JS_SWITCH_STATEMENT",
        "JS_SWITCH_CASE_LIST",
        "JS_CASE_CLAUSE",
        "JS_DEFAULT_CLAUSE",
        "JS_LABELED_STATEMENT",
        "JS_THROW_STATEMENT",
        "JS_TRY_STATEMENT",
        "JS_TRY_FINALLY_STATEMENT",
        "JS_CATCH_CLAUSE",
        "JS_CATCH_DECLARATION",
        "JS_FINALLY_CLAUSE",
        "JS_DEBUGGER_STATEMENT",
        "JS_FUNCTION_DECLARATION",
        "JS_PARAMETERS",
        "JS_PARAMETER_LIST",
        "JS_FORMAL_PARAMETER",
        "JS_REST_PARAMETER",
        "TS_THIS_PARAMETER",
        "TS_PROPERTY_PARAMETER",
        "TS_PROPERTY_PARAMETER_MODIFIER_LIST",
        "TS_TYPE_ANNOTATION",
        "TS_RETURN_TYPE_ANNOTATION",
        "JS_IDENTIFIER_BINDING",
        "JS_IDENTIFIER_EXPRESSION",
        "JS_REFERENCE_IDENTIFIER",
        "JS_NAME",
        "JS_PRIVATE_NAME",
        "JS_THIS_EXPRESSION",
        "JS_ARRAY_EXPRESSION",
        "JS_ARRAY_ELEMENT_LIST",
        "JS_ARRAY_HOLE",
        "JS_COMPUTED_MEMBER_NAME",
        "JS_LITERAL_MEMBER_NAME",
        "JS_OBJECT_EXPRESSION",
        "JS_OBJECT_MEMBER_LIST",
        "JS_PROPERTY_OBJECT_MEMBER",
        "JS_GETTER_OBJECT_MEMBER",
        "JS_SETTER_OBJECT_MEMBER",
        "JS_METHOD_OBJECT_MEMBER",
        "JS_SUPER_EXPRESSION",
        "JS_PARENTHESIZED_EXPRESSION",
        "JS_NEW_EXPRESSION",
        "JS_FUNCTION_EXPRESSION",
        "JS_STATIC_MEMBER_EXPRESSION",
        "JS_COMPUTED_MEMBER_EXPRESSION",
        "JS_CALL_EXPRESSION",
        "JS_UNARY_EXPRESSION",
        "JS_PRE_UPDATE_EXPRESSION",
        "JS_POST_UPDATE_EXPRESSION",
        "JS_BINARY_EXPRESSION",
        "JS_INSTANCEOF_EXPRESSION",
        "JS_IN_EXPRESSION",
        "JS_LOGICAL_EXPRESSION",
        "JS_CONDITIONAL_EXPRESSION",
        "JS_ASSIGNMENT_EXPRESSION",
        "JS_SEQUENCE_EXPRESSION",
        "JS_CALL_ARGUMENTS",
        "JS_CALL_ARGUMENT_LIST",
        "JS_STRING_LITERAL_EXPRESSION",
        "JS_NUMBER_LITERAL_EXPRESSION",
        "JS_BIGINT_LITERAL_EXPRESSION",
        "JS_BOOLEAN_LITERAL_EXPRESSION",
        "JS_NULL_LITERAL_EXPRESSION",
        "JS_REGEX_LITERAL_EXPRESSION",
        "JS_TEMPLATE_EXPRESSION",
        "JS_TEMPLATE_ELEMENT",
        "JS_TEMPLATE_CHUNK_ELEMENT",
        "JS_TEMPLATE_ELEMENT_LIST",
        "JS_IMPORT_CALL_EXPRESSION",
        "JS_NEW_TARGET_EXPRESSION",
        "JS_IMPORT_META_EXPRESSION",
        "JS_SHORTHAND_PROPERTY_OBJECT_MEMBER",
        "JS_SPREAD",
        "JS_OBJECT_BINDING_PATTERN",
        "JS_ARRAY_BINDING_PATTERN",
        "JS_ARRAY_BINDING_PATTERN_ELEMENT",
        "JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST",
        "JS_ARRAY_BINDING_PATTERN_REST_ELEMENT",
        "JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST",
        "JS_OBJECT_BINDING_PATTERN_REST",
        "JS_OBJECT_BINDING_PATTERN_PROPERTY",
        "JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY",
        "JS_ARROW_FUNCTION_EXPRESSION",
        "JS_YIELD_EXPRESSION",
        "JS_YIELD_ARGUMENT",
        "JS_CLASS_DECLARATION",
        "JS_CLASS_EXPRESSION",
        "JS_CLASS_MEMBER_LIST",
        "JS_STATIC_MODIFIER",
        "JS_ACCESSOR_MODIFIER",
        "TS_DECLARE_MODIFIER",
        "TS_READONLY_MODIFIER",
        "TS_ABSTRACT_MODIFIER",
        "TS_OVERRIDE_MODIFIER",
        "TS_ACCESSIBILITY_MODIFIER",
        "TS_CONST_MODIFIER",
        "TS_IN_MODIFIER",
        "TS_OUT_MODIFIER",
        "JS_EXTENDS_CLAUSE",
        "TS_IMPLEMENTS_CLAUSE",
        "JS_PRIVATE_CLASS_MEMBER_NAME",
        "JS_CONSTRUCTOR_CLASS_MEMBER",
        "TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER",
        "JS_CONSTRUCTOR_MODIFIER_LIST",
        "JS_CONSTRUCTOR_PARAMETER_LIST",
        "JS_CONSTRUCTOR_PARAMETERS",
        "JS_PROPERTY_CLASS_MEMBER",
        "JS_PROPERTY_MODIFIER_LIST",
        "TS_OPTIONAL_PROPERTY_ANNOTATION",
        "TS_DEFINITE_PROPERTY_ANNOTATION",
        "JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER",
        "JS_METHOD_CLASS_MEMBER",
        "JS_METHOD_MODIFIER_LIST",
        "JS_GETTER_CLASS_MEMBER",
        "JS_SETTER_CLASS_MEMBER",
        "JS_EMPTY_CLASS_MEMBER",
        "JS_PARENTHESIZED_ASSIGNMENT",
        "JS_IDENTIFIER_ASSIGNMENT",
        "JS_STATIC_MEMBER_ASSIGNMENT",
        "JS_COMPUTED_MEMBER_ASSIGNMENT",
        "TS_NON_NULL_ASSERTION_ASSIGNMENT",
        "TS_AS_ASSIGNMENT",
        "TS_SATISFIES_ASSIGNMENT",
        "TS_TYPE_ASSERTION_ASSIGNMENT",
        "JS_ARRAY_ASSIGNMENT_PATTERN",
        "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT",
        "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST",
        "JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT",
        "JS_OBJECT_ASSIGNMENT_PATTERN",
        "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST",
        "JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY",
        "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY",
        "JS_OBJECT_ASSIGNMENT_PATTERN_REST",
        "JS_IMPORT",
        "JS_IMPORT_BARE_CLAUSE",
        "JS_IMPORT_DEFAULT_CLAUSE",
        "JS_IMPORT_NAMESPACE_CLAUSE",
        "JS_IMPORT_NAMED_CLAUSE",
        "JS_IMPORT_COMBINED_CLAUSE",
        "JS_NAMED_IMPORT_SPECIFIERS",
        "JS_NAMED_IMPORT_SPECIFIER_LIST",
        "JS_NAMESPACE_IMPORT_SPECIFIER",
        "JS_DEFAULT_IMPORT_SPECIFIER",
        "JS_NAMED_IMPORT_SPECIFIER",
        "JS_SHORTHAND_NAMED_IMPORT_SPECIFIER",
        "JS_IMPORT_ASSERTION",
        "JS_IMPORT_ASSERTION_ENTRY_LIST",
        "JS_IMPORT_ASSERTION_ENTRY",
        "JS_MODULE_SOURCE",
        "JS_EXPORT",
        "JS_EXPORT_NAMED_CLAUSE",
        "JS_EXPORT_NAMED_SPECIFIER_LIST",
        "JS_EXPORT_NAMED_SHORTHAND_SPECIFIER",
        "JS_EXPORT_NAMED_SPECIFIER",
        "JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE",
        "JS_EXPORT_DEFAULT_DECLARATION_CLAUSE",
        "JS_CLASS_EXPORT_DEFAULT_DECLARATION",
        "JS_FUNCTION_EXPORT_DEFAULT_DECLARATION",
        "JS_EXPORT_FROM_CLAUSE",
        "JS_EXPORT_NAMED_FROM_CLAUSE",
        "JS_EXPORT_NAMED_FROM_SPECIFIER_LIST",
        "JS_EXPORT_NAMED_FROM_SPECIFIER",
        "JS_EXPORT_AS_CLAUSE",
        "TS_EXPORT_AS_NAMESPACE_CLAUSE",
        "TS_EXPORT_ASSIGNMENT_CLAUSE",
        "TS_EXPORT_DECLARE_CLAUSE",
        "JS_LITERAL_EXPORT_NAME",
        "JS_AWAIT_EXPRESSION",
        "JS_DECORATOR",
        "JS_DECORATOR_LIST",
        "JS_LABEL",
        // TypeScript
        "TS_IDENTIFIER_BINDING",
        "TS_ANY_TYPE",
        "TS_UNKNOWN_TYPE",
        "TS_NUMBER_TYPE",
        "TS_NON_PRIMITIVE_TYPE",
        "TS_BOOLEAN_TYPE",
        "TS_BIGINT_TYPE",
        "TS_STRING_TYPE",
        "TS_SYMBOL_TYPE",
        "TS_VOID_TYPE",
        "TS_UNDEFINED_TYPE",
        "TS_NEVER_TYPE",
        "TS_THIS_TYPE",
        "TS_TYPEOF_TYPE",
        "TS_PARENTHESIZED_TYPE",
        "TS_MAPPED_TYPE",
        "TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE",
        "TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE",
        "TS_MAPPED_TYPE_AS_CLAUSE",
        "TS_TYPE_ALIAS_DECLARATION",
        "TS_MODULE_DECLARATION",
        "TS_GLOBAL_DECLARATION",
        "TS_QUALIFIED_MODULE_NAME",
        "TS_MODULE_BLOCK",
        "TS_EXTERNAL_MODULE_DECLARATION",
        "TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY",
        "TS_QUALIFIED_NAME",
        "TS_REFERENCE_TYPE",
        "TS_UNION_TYPE",
        "TS_UNION_TYPE_VARIANT_LIST",
        "TS_INTERSECTION_TYPE",
        "TS_INTERSECTION_TYPE_ELEMENT_LIST",
        "TS_OBJECT_TYPE",
        "TS_TYPE_MEMBER_LIST",
        "TS_INTERFACE_DECLARATION",
        "TS_EXTENDS_CLAUSE",
        "TS_PROPERTY_SIGNATURE_TYPE_MEMBER",
        "TS_METHOD_SIGNATURE_TYPE_MEMBER",
        "TS_CALL_SIGNATURE_TYPE_MEMBER",
        "TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER",
        "TS_GETTER_SIGNATURE_TYPE_MEMBER",
        "TS_SETTER_SIGNATURE_TYPE_MEMBER",
        "TS_INDEX_SIGNATURE_TYPE_MEMBER",
        "TS_IMPORT_TYPE",
        "TS_IMPORT_TYPE_ARGUMENTS",
        "TS_IMPORT_TYPE_ASSERTION",
        "TS_IMPORT_TYPE_ASSERTION_BLOCK",
        "TS_IMPORT_TYPE_QUALIFIER",
        "TS_ARRAY_TYPE",
        "TS_INDEXED_ACCESS_TYPE",
        "TS_TUPLE_TYPE",
        "TS_TUPLE_TYPE_ELEMENT_LIST",
        "TS_REST_TUPLE_TYPE_ELEMENT",
        "TS_OPTIONAL_TUPLE_TYPE_ELEMENT",
        "TS_NAMED_TUPLE_TYPE_ELEMENT",
        "TS_TYPE_OPERATOR_TYPE",
        "TS_INFER_TYPE",
        "TS_CONSTRUCTOR_TYPE",
        "TS_FUNCTION_TYPE",
        "TS_PREDICATE_RETURN_TYPE",
        "TS_ASSERTS_RETURN_TYPE",
        "TS_ASSERTS_CONDITION",
        "TS_TYPE_PARAMETERS",
        "TS_TYPE_PARAMETER_LIST",
        "TS_TYPE_PARAMETER",
        "TS_TYPE_PARAMETER_MODIFIER_LIST",
        "TS_TYPE_PARAMETER_NAME",
        "TS_TYPE_CONSTRAINT_CLAUSE",
        "TS_DEFAULT_TYPE_CLAUSE",
        "TS_STRING_LITERAL_TYPE",
        "TS_NUMBER_LITERAL_TYPE",
        "TS_BIGINT_LITERAL_TYPE",
        "TS_BOOLEAN_LITERAL_TYPE",
        "TS_NULL_LITERAL_TYPE",
        "TS_TEMPLATE_LITERAL_TYPE",
        "TS_TEMPLATE_ELEMENT_LIST",
        "TS_TEMPLATE_CHUNK_ELEMENT",
        "TS_TEMPLATE_ELEMENT",
        "TS_TYPE_ARGUMENTS",
        "TS_TYPE_ARGUMENT_LIST",
        "TS_TYPE_LIST",
        "TS_EXTENDS",
        "TS_CONDITIONAL_TYPE",
        "TS_NON_NULL_ASSERTION_EXPRESSION",
        "TS_TYPE_ASSERTION_EXPRESSION",
        "TS_AS_EXPRESSION",
        "TS_SATISFIES_EXPRESSION",
        "TS_INSTANTIATION_EXPRESSION",
        "TS_ENUM_DECLARATION",
        "TS_ENUM_MEMBER_LIST",
        "TS_ENUM_MEMBER",
        "TS_LITERAL_ENUM_MEMBER_NAME",
        "TS_IMPORT_EQUALS_DECLARATION",
        "TS_EXTERNAL_MODULE_REFERENCE",
        "TS_DECLARE_FUNCTION_DECLARATION",
        "TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION",
        "TS_DECLARE_STATEMENT",
        "TS_INDEX_SIGNATURE_PARAMETER",
        "TS_PROPERTY_SIGNATURE_CLASS_MEMBER",
        "TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER",
        "TS_PROPERTY_SIGNATURE_MODIFIER_LIST",
        "TS_METHOD_SIGNATURE_CLASS_MEMBER",
        "TS_METHOD_SIGNATURE_MODIFIER_LIST",
        "TS_GETTER_SIGNATURE_CLASS_MEMBER",
        "TS_SETTER_SIGNATURE_CLASS_MEMBER",
        "TS_INDEX_SIGNATURE_CLASS_MEMBER",
        "TS_INDEX_SIGNATURE_MODIFIER_LIST",
        //JSX
        "JSX_NAME",
        "JSX_NAMESPACE_NAME",
        "JSX_REFERENCE_IDENTIFIER",
        "JSX_TAG_EXPRESSION",
        "JSX_ELEMENT",
        "JSX_FRAGMENT",
        "JSX_OPENING_FRAGMENT",
        "JSX_CLOSING_FRAGMENT",
        "JSX_SELF_CLOSING_ELEMENT",
        "JSX_OPENING_ELEMENT",
        "JSX_CLOSING_ELEMENT",
        "JSX_MEMBER_NAME",
        "JSX_TEXT",
        "JSX_ATTRIBUTE_LIST",
        "JSX_ATTRIBUTE",
        "JSX_SPREAD_ATTRIBUTE",
        "JSX_ATTRIBUTE_INITIALIZER_CLAUSE",
        "JSX_EXPRESSION_ATTRIBUTE_VALUE",
        "JSX_CHILD_LIST",
        "JSX_EXPRESSION_CHILD",
        "JSX_SPREAD_CHILD",
        "JSX_STRING",
        // Grit metavariable
        "JS_METAVARIABLE",
        // bogus nodes JS
        "JS_BOGUS",
        "JS_BOGUS_EXPRESSION",
        "JS_BOGUS_STATEMENT",
        "JS_BOGUS_MEMBER",
        "JS_BOGUS_BINDING",
        "JS_BOGUS_PARAMETER",
        "JS_BOGUS_IMPORT_ASSERTION_ENTRY",
        "JS_BOGUS_NAMED_IMPORT_SPECIFIER",
        "JS_BOGUS_ASSIGNMENT",
        "TS_BOGUS_TYPE",
    ],
};

#[derive(Default, Debug)]
pub struct AstSrc {
    pub nodes: Vec<AstNodeSrc>,
    pub unions: Vec<AstEnumSrc>,
    pub lists: BTreeMap<String, AstListSrc>,
    pub bogus: Vec<String>,
}

impl AstSrc {
    pub fn push_list(&mut self, name: &str, src: AstListSrc) {
        self.lists.insert(String::from(name), src);
    }

    pub fn lists(&self) -> std::collections::btree_map::Iter<String, AstListSrc> {
        self.lists.iter()
    }

    pub fn is_list(&self, name: &str) -> bool {
        self.lists.contains_key(name)
    }

    /// Sorts all nodes, enums, etc. for a stable code gen result
    pub fn sort(&mut self) {
        // No need to sort lists, they're stored in a btree
        self.nodes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.unions.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.bogus.sort_unstable();

        for union in self.unions.iter_mut() {
            union.variants.sort_unstable();
        }
    }
}

#[derive(Debug)]
pub struct AstListSrc {
    pub element_name: String,
    pub separator: Option<AstListSeparatorConfiguration>,
}

#[derive(Debug)]
pub struct AstListSeparatorConfiguration {
    /// Name of the separator token
    pub separator_token: String,
    /// Whatever the list allows a trailing comma or not
    pub allow_trailing: bool,
}

#[derive(Debug)]
pub struct AstNodeSrc {
    #[expect(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub fields: Vec<Field>,
    /// Whether the fields of the node should be ordered dynamically using a
    /// slot map for accesses.
    pub dynamic: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token {
        name: String,
        kind: TokenKind,
        optional: bool,
        unordered: bool,
    },
    Node {
        name: String,
        ty: String,
        optional: bool,
        unordered: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AstEnumSrc {
    #[expect(dead_code)]
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub variants: Vec<String>,
}

impl Field {
    pub fn method_name(&self, language_kind: LanguageKind) -> proc_macro2::Ident {
        match self {
            Self::Token { name, .. } => {
                let name = match (name.as_str(), language_kind) {
                    (";", _) => "semicolon",
                    ("'{'", _) => "l_curly",
                    ("'}'", _) => "r_curly",
                    ("'('", _) => "l_paren",
                    ("')'", _) => "r_paren",
                    ("'['", _) => "l_brack",
                    ("']'", _) => "r_brack",
                    ("'`'", _) => "backtick",
                    ("<", _) => "l_angle",
                    (">", _) => "r_angle",
                    ("=", _) => "eq",
                    ("!", _) => "excl",
                    ("*", _) => "star",
                    ("&", _) => "amp",
                    (".", _) => "dot",
                    ("...", _) => "dotdotdot",
                    ("=>", _) => "fat_arrow",
                    (":", _) => "colon",
                    ("::", _) => "double_colon",
                    ("?", _) => "question_mark",
                    ("+", _) => "plus",
                    ("-", _) => "minus",
                    ("#", _) => "hash",
                    ("@", _) => "at",
                    ("+=", _) => "add_assign",
                    ("-=", _) => "subtract_assign",
                    ("*=", _) => "times_assign",
                    ("%=", _) => "remainder_assign",
                    ("**=", _) => "exponent_assign",
                    (">>=", _) => "left_shift_assign",
                    ("<<=", _) => "right_shift_assign",
                    (">>>=", _) => "unsigned_right_shift_assign",
                    ("~", _) => "bitwise_not",
                    ("&=", _) => "bitwise_and_assign",
                    ("|=", LanguageKind::Js) => "bitwise_or_assign",
                    ("|=", LanguageKind::Css) => "exactly_or_hyphen",
                    ("^=", LanguageKind::Js) => "bitwise_xor_assign",
                    ("^=", LanguageKind::Css) => "prefix",
                    ("&&=", _) => "bitwise_logical_and_assign",
                    ("||=", _) => "bitwise_logical_or_assign",
                    ("??=", _) => "bitwise_nullish_coalescing_assign",
                    ("++", _) => "increment",
                    ("--", _) => "decrement",
                    ("<=", _) => "less_than_equal",
                    (">=", _) => "greater_than_equal",
                    ("==", _) => "equality",
                    ("===", _) => "strict_equality",
                    ("!=", _) => "inequality",
                    ("!==", _) => "strict_inequality",
                    ("/", _) => "slash",
                    ("%", LanguageKind::Css) => "percent",
                    ("%", _) => "remainder",
                    ("**", _) => "exponent",
                    ("<<", _) => "left_shift",
                    (">>", _) => "right_shift",
                    (">>>", _) => "unsigned_right_shift",
                    ("|", _) => "bitwise_or",
                    ("^", _) => "bitwise_xor",
                    ("??", _) => "nullish_coalescing",
                    ("||", _) => "logical_or",
                    ("&&", _) => "logical_and",
                    ("$=", _) => "suffix",
                    ("$", LanguageKind::Graphql) => "dollar",
                    ("~=", _) => "whitespace_like",
                    (",", _) => "comma",
                    ("---", LanguageKind::Yaml) => "dashdashdash",
                    ("---", LanguageKind::Html) => "fence",
                    ("<!--", LanguageKind::Html) => "comment_start",
                    ("-->", LanguageKind::Html) => "comment_end",
                    ("<![CDATA[", LanguageKind::Html) => "cdata_start",
                    ("]]>", LanguageKind::Html) => "cdata_end",
                    ("{{", LanguageKind::Html) => "l_double_curly",
                    ("}}", LanguageKind::Html) => "r_double_curly",

                    _ => name,
                };

                let kind_source = language_kind.kinds();

                // we need to replace "-" with "_" for the keywords
                // e.g. we have `color-profile` in css but it's an invalid ident in rust code
                if kind_source.keywords.contains(&name) {
                    format_ident!("{}_token", name.replace('-', "_"))
                } else {
                    format_ident!("{}_token", name)
                }
            }
            Self::Node { name, .. } => {
                let (prefix, tail) = name.split_once('_').unwrap_or(("", name));
                let final_name = if LANGUAGE_PREFIXES.contains(&prefix) {
                    tail
                } else {
                    name.as_str()
                };

                // this check here is to avoid emitting methods called "type()",
                // where "type" is a reserved word
                if final_name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", final_name)
                }
            }
        }
    }

    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Self::Token { .. } => format_ident!("SyntaxToken"),
            Self::Node { ty, .. } => format_ident!("{}", ty),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::Node { optional, .. } => *optional,
            Self::Token { optional, .. } => *optional,
        }
    }

    pub fn is_unordered(&self) -> bool {
        match self {
            Self::Node { unordered, .. } => *unordered,
            Self::Token { unordered, .. } => *unordered,
        }
    }
}
