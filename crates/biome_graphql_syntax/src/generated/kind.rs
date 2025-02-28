//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum GraphqlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    BANG,
    DOLLAR,
    AMP,
    L_PAREN,
    R_PAREN,
    DOT3,
    COLON,
    EQ,
    AT,
    L_BRACK,
    R_BRACK,
    L_CURLY,
    PIPE,
    R_CURLY,
    TRUE_KW,
    FALSE_KW,
    QUERY_KW,
    MUTATION_KW,
    SUBSCRIPTION_KW,
    FRAGMENT_KW,
    ON_KW,
    NULL_KW,
    SCHEMA_KW,
    EXTEND_KW,
    SCALAR_KW,
    TYPE_KW,
    IMPLEMENTS_KW,
    INTERFACE_KW,
    UNION_KW,
    ENUM_KW,
    INPUT_KW,
    DIRECTIVE_KW,
    REPEATABLE_KW,
    UPPER_QUERY_KW,
    UPPER_MUTATION_KW,
    UPPER_SUBSCRIPTION_KW,
    UPPER_FIELD_KW,
    FRAGMENT_DEFINITION_KW,
    FRAGMENT_SPREAD_KW,
    INLINE_FRAGMENT_KW,
    VARIABLE_DEFINITION_KW,
    UPPER_SCHEMA_KW,
    UPPER_SCALAR_KW,
    UPPER_OBJECT_KW,
    FIELD_DEFINITION_KW,
    ARGUMENT_DEFINITION_KW,
    UPPER_INTERFACE_KW,
    UPPER_UNION_KW,
    UPPER_ENUM_KW,
    ENUM_VALUE_KW,
    INPUT_OBJECT_KW,
    INPUT_FIELD_DEFINITION_KW,
    GRAPHQL_STRING_LITERAL,
    GRAPHQL_FLOAT_LITERAL,
    GRAPHQL_INT_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    COMMA,
    GRAPHQL_ROOT,
    GRAPHQL_LITERAL_NAME,
    GRAPHQL_NAME_BINDING,
    GRAPHQL_NAME_REFERENCE,
    GRAPHQL_DEFINITION_LIST,
    GRAPHQL_FRAGMENT_DEFINITION,
    GRAPHQL_DIRECTIVE_DEFINITION,
    GRAPHQL_SCHEMA_DEFINITION,
    GRAPHQL_SCALAR_TYPE_DEFINITION,
    GRAPHQL_OBJECT_TYPE_DEFINITION,
    GRAPHQL_INTERFACE_TYPE_DEFINITION,
    GRAPHQL_UNION_TYPE_DEFINITION,
    GRAPHQL_ENUM_TYPE_DEFINITION,
    GRAPHQL_INPUT_OBJECT_TYPE_DEFINITION,
    GRAPHQL_SCALAR_TYPE_EXTENSION,
    GRAPHQL_OPERATION_DEFINITION,
    GRAPHQL_OPERATION_TYPE,
    GRAPHQL_SELECTION_SET,
    GRAPHQL_SELECTION_LIST,
    GRAPHQL_FIELD,
    GRAPHQL_ALIAS,
    GRAPHQL_ARGUMENTS,
    GRAPHQL_ARGUMENT_LIST,
    GRAPHQL_ARGUMENT,
    GRAPHQL_FRAGMENT_SPREAD,
    GRAPHQL_INLINE_FRAGMENT,
    GRAPHQL_TYPE_CONDITION,
    GRAPHQL_VARIABLE_BINDING,
    GRAPHQL_VARIABLE_REFERENCE,
    GRAPHQL_ENUM_VALUE,
    GRAPHQL_LIST_VALUE,
    GRAPHQL_LIST_VALUE_ELEMENT_LIST,
    GRAPHQL_OBJECT_VALUE,
    GRAPHQL_OBJECT_VALUE_MEMBER_LIST,
    GRAPHQL_OBJECT_FIELD,
    GRAPHQL_VARIABLE_DEFINITIONS,
    GRAPHQL_VARIABLE_DEFINITION_LIST,
    GRAPHQL_VARIABLE_DEFINITION,
    GRAPHQL_DEFAULT_VALUE,
    GRAPHQL_NON_NULL_TYPE,
    GRAPHQL_LIST_TYPE,
    GRAPHQL_DIRECTIVE_LIST,
    GRAPHQL_DIRECTIVE,
    GRAPHQL_ROOT_OPERATION_TYPES,
    GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST,
    GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION,
    GRAPHQL_SCHEMA_EXTENSION,
    GRAPHQL_DESCRIPTION,
    GRAPHQL_OBJECT_TYPE_EXTENSION,
    GRAPHQL_IMPLEMENTS_INTERFACES,
    GRAPHQL_IMPLEMENTS_INTERFACE_LIST,
    GRAPHQL_FIELDS_DEFINITION,
    GRAPHQL_FIELD_DEFINITION_LIST,
    GRAPHQL_FIELD_DEFINITION,
    GRAPHQL_ARGUMENTS_DEFINITION,
    GRAPHQL_ARGUMENT_DEFINITION_LIST,
    GRAPHQL_INPUT_VALUE_DEFINITION,
    GRAPHQL_INTERFACE_TYPE_EXTENSION,
    GRAPHQL_UNION_MEMBER_TYPES,
    GRAPHQL_UNION_MEMBER_TYPE_LIST,
    GRAPHQL_UNION_TYPE_EXTENSION,
    GRAPHQL_ENUM_VALUES_DEFINITION,
    GRAPHQL_ENUM_VALUE_LIST,
    GRAPHQL_ENUM_VALUE_DEFINITION,
    GRAPHQL_ENUM_TYPE_EXTENSION,
    GRAPHQL_INPUT_FIELDS_DEFINITION,
    GRAPHQL_INPUT_FIELD_LIST,
    GRAPHQL_INPUT_OBJECT_TYPE_EXTENSION,
    GRAPHQL_DIRECTIVE_LOCATION_LIST,
    GRAPHQL_DIRECTIVE_LOCATION,
    GRAPHQL_STRING_VALUE,
    GRAPHQL_FLOAT_VALUE,
    GRAPHQL_INT_VALUE,
    GRAPHQL_BOOLEAN_VALUE,
    GRAPHQL_NULL_VALUE,
    GRAPHQL_BOGUS,
    GRAPHQL_BOGUS_DEFINITION,
    GRAPHQL_BOGUS_SELECTION,
    GRAPHQL_BOGUS_VALUE,
    GRAPHQL_BOGUS_TYPE,
    #[doc(hidden)]
    __LAST,
}
use self::GraphqlSyntaxKind::*;
impl GraphqlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            BANG | DOLLAR
                | AMP
                | L_PAREN
                | R_PAREN
                | DOT3
                | COLON
                | EQ
                | AT
                | L_BRACK
                | R_BRACK
                | L_CURLY
                | PIPE
                | R_CURLY
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            GRAPHQL_STRING_LITERAL | GRAPHQL_FLOAT_LITERAL | GRAPHQL_INT_LITERAL
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            GRAPHQL_DEFINITION_LIST
                | GRAPHQL_SELECTION_LIST
                | GRAPHQL_ARGUMENT_LIST
                | GRAPHQL_LIST_VALUE_ELEMENT_LIST
                | GRAPHQL_OBJECT_VALUE_MEMBER_LIST
                | GRAPHQL_VARIABLE_DEFINITION_LIST
                | GRAPHQL_DIRECTIVE_LIST
                | GRAPHQL_ROOT_OPERATION_TYPE_DEFINITION_LIST
                | GRAPHQL_IMPLEMENTS_INTERFACE_LIST
                | GRAPHQL_FIELD_DEFINITION_LIST
                | GRAPHQL_ARGUMENT_DEFINITION_LIST
                | GRAPHQL_UNION_MEMBER_TYPE_LIST
                | GRAPHQL_ENUM_VALUE_LIST
                | GRAPHQL_INPUT_FIELD_LIST
                | GRAPHQL_DIRECTIVE_LOCATION_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<GraphqlSyntaxKind> {
        let kw = match ident {
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "query" => QUERY_KW,
            "mutation" => MUTATION_KW,
            "subscription" => SUBSCRIPTION_KW,
            "fragment" => FRAGMENT_KW,
            "on" => ON_KW,
            "null" => NULL_KW,
            "schema" => SCHEMA_KW,
            "extend" => EXTEND_KW,
            "scalar" => SCALAR_KW,
            "type" => TYPE_KW,
            "implements" => IMPLEMENTS_KW,
            "interface" => INTERFACE_KW,
            "union" => UNION_KW,
            "enum" => ENUM_KW,
            "input" => INPUT_KW,
            "directive" => DIRECTIVE_KW,
            "repeatable" => REPEATABLE_KW,
            "QUERY" => UPPER_QUERY_KW,
            "MUTATION" => UPPER_MUTATION_KW,
            "SUBSCRIPTION" => UPPER_SUBSCRIPTION_KW,
            "FIELD" => UPPER_FIELD_KW,
            "FRAGMENT_DEFINITION" => FRAGMENT_DEFINITION_KW,
            "FRAGMENT_SPREAD" => FRAGMENT_SPREAD_KW,
            "INLINE_FRAGMENT" => INLINE_FRAGMENT_KW,
            "VARIABLE_DEFINITION" => VARIABLE_DEFINITION_KW,
            "SCHEMA" => UPPER_SCHEMA_KW,
            "SCALAR" => UPPER_SCALAR_KW,
            "OBJECT" => UPPER_OBJECT_KW,
            "FIELD_DEFINITION" => FIELD_DEFINITION_KW,
            "ARGUMENT_DEFINITION" => ARGUMENT_DEFINITION_KW,
            "INTERFACE" => UPPER_INTERFACE_KW,
            "UNION" => UPPER_UNION_KW,
            "ENUM" => UPPER_ENUM_KW,
            "ENUM_VALUE" => ENUM_VALUE_KW,
            "INPUT_OBJECT" => INPUT_OBJECT_KW,
            "INPUT_FIELD_DEFINITION" => INPUT_FIELD_DEFINITION_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            BANG => "!",
            DOLLAR => "$",
            AMP => "&",
            L_PAREN => "(",
            R_PAREN => ")",
            DOT3 => "...",
            COLON => ":",
            EQ => "=",
            AT => "@",
            L_BRACK => "[",
            R_BRACK => "]",
            L_CURLY => "{",
            PIPE => "|",
            R_CURLY => "}",
            TRUE_KW => "true",
            FALSE_KW => "false",
            QUERY_KW => "query",
            MUTATION_KW => "mutation",
            SUBSCRIPTION_KW => "subscription",
            FRAGMENT_KW => "fragment",
            ON_KW => "on",
            NULL_KW => "null",
            SCHEMA_KW => "schema",
            EXTEND_KW => "extend",
            SCALAR_KW => "scalar",
            TYPE_KW => "type",
            IMPLEMENTS_KW => "implements",
            INTERFACE_KW => "interface",
            UNION_KW => "union",
            ENUM_KW => "enum",
            INPUT_KW => "input",
            DIRECTIVE_KW => "directive",
            REPEATABLE_KW => "repeatable",
            UPPER_QUERY_KW => "QUERY",
            UPPER_MUTATION_KW => "MUTATION",
            UPPER_SUBSCRIPTION_KW => "SUBSCRIPTION",
            UPPER_FIELD_KW => "FIELD",
            FRAGMENT_DEFINITION_KW => "FRAGMENT_DEFINITION",
            FRAGMENT_SPREAD_KW => "FRAGMENT_SPREAD",
            INLINE_FRAGMENT_KW => "INLINE_FRAGMENT",
            VARIABLE_DEFINITION_KW => "VARIABLE_DEFINITION",
            UPPER_SCHEMA_KW => "SCHEMA",
            UPPER_SCALAR_KW => "SCALAR",
            UPPER_OBJECT_KW => "OBJECT",
            FIELD_DEFINITION_KW => "FIELD_DEFINITION",
            ARGUMENT_DEFINITION_KW => "ARGUMENT_DEFINITION",
            UPPER_INTERFACE_KW => "INTERFACE",
            UPPER_UNION_KW => "UNION",
            UPPER_ENUM_KW => "ENUM",
            ENUM_VALUE_KW => "ENUM_VALUE",
            INPUT_OBJECT_KW => "INPUT_OBJECT",
            INPUT_FIELD_DEFINITION_KW => "INPUT_FIELD_DEFINITION",
            GRAPHQL_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [!] => { $ crate :: GraphqlSyntaxKind :: BANG } ; [$] => { $ crate :: GraphqlSyntaxKind :: DOLLAR } ; [&] => { $ crate :: GraphqlSyntaxKind :: AMP } ; ['('] => { $ crate :: GraphqlSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: GraphqlSyntaxKind :: R_PAREN } ; [...] => { $ crate :: GraphqlSyntaxKind :: DOT3 } ; [:] => { $ crate :: GraphqlSyntaxKind :: COLON } ; [=] => { $ crate :: GraphqlSyntaxKind :: EQ } ; [@] => { $ crate :: GraphqlSyntaxKind :: AT } ; ['['] => { $ crate :: GraphqlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: GraphqlSyntaxKind :: R_BRACK } ; ['{'] => { $ crate :: GraphqlSyntaxKind :: L_CURLY } ; [|] => { $ crate :: GraphqlSyntaxKind :: PIPE } ; ['}'] => { $ crate :: GraphqlSyntaxKind :: R_CURLY } ; [true] => { $ crate :: GraphqlSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: GraphqlSyntaxKind :: FALSE_KW } ; [query] => { $ crate :: GraphqlSyntaxKind :: QUERY_KW } ; [mutation] => { $ crate :: GraphqlSyntaxKind :: MUTATION_KW } ; [subscription] => { $ crate :: GraphqlSyntaxKind :: SUBSCRIPTION_KW } ; [fragment] => { $ crate :: GraphqlSyntaxKind :: FRAGMENT_KW } ; [on] => { $ crate :: GraphqlSyntaxKind :: ON_KW } ; [null] => { $ crate :: GraphqlSyntaxKind :: NULL_KW } ; [schema] => { $ crate :: GraphqlSyntaxKind :: SCHEMA_KW } ; [extend] => { $ crate :: GraphqlSyntaxKind :: EXTEND_KW } ; [scalar] => { $ crate :: GraphqlSyntaxKind :: SCALAR_KW } ; [type] => { $ crate :: GraphqlSyntaxKind :: TYPE_KW } ; [implements] => { $ crate :: GraphqlSyntaxKind :: IMPLEMENTS_KW } ; [interface] => { $ crate :: GraphqlSyntaxKind :: INTERFACE_KW } ; [union] => { $ crate :: GraphqlSyntaxKind :: UNION_KW } ; [enum] => { $ crate :: GraphqlSyntaxKind :: ENUM_KW } ; [input] => { $ crate :: GraphqlSyntaxKind :: INPUT_KW } ; [directive] => { $ crate :: GraphqlSyntaxKind :: DIRECTIVE_KW } ; [repeatable] => { $ crate :: GraphqlSyntaxKind :: REPEATABLE_KW } ; [UPPER_QUERY] => { $ crate :: GraphqlSyntaxKind :: UPPER_QUERY_KW } ; [UPPER_MUTATION] => { $ crate :: GraphqlSyntaxKind :: UPPER_MUTATION_KW } ; [UPPER_SUBSCRIPTION] => { $ crate :: GraphqlSyntaxKind :: UPPER_SUBSCRIPTION_KW } ; [UPPER_FIELD] => { $ crate :: GraphqlSyntaxKind :: UPPER_FIELD_KW } ; [FRAGMENT_DEFINITION] => { $ crate :: GraphqlSyntaxKind :: FRAGMENT_DEFINITION_KW } ; [FRAGMENT_SPREAD] => { $ crate :: GraphqlSyntaxKind :: FRAGMENT_SPREAD_KW } ; [INLINE_FRAGMENT] => { $ crate :: GraphqlSyntaxKind :: INLINE_FRAGMENT_KW } ; [VARIABLE_DEFINITION] => { $ crate :: GraphqlSyntaxKind :: VARIABLE_DEFINITION_KW } ; [UPPER_SCHEMA] => { $ crate :: GraphqlSyntaxKind :: UPPER_SCHEMA_KW } ; [UPPER_SCALAR] => { $ crate :: GraphqlSyntaxKind :: UPPER_SCALAR_KW } ; [UPPER_OBJECT] => { $ crate :: GraphqlSyntaxKind :: UPPER_OBJECT_KW } ; [FIELD_DEFINITION] => { $ crate :: GraphqlSyntaxKind :: FIELD_DEFINITION_KW } ; [ARGUMENT_DEFINITION] => { $ crate :: GraphqlSyntaxKind :: ARGUMENT_DEFINITION_KW } ; [UPPER_INTERFACE] => { $ crate :: GraphqlSyntaxKind :: UPPER_INTERFACE_KW } ; [UPPER_UNION] => { $ crate :: GraphqlSyntaxKind :: UPPER_UNION_KW } ; [UPPER_ENUM] => { $ crate :: GraphqlSyntaxKind :: UPPER_ENUM_KW } ; [ENUM_VALUE] => { $ crate :: GraphqlSyntaxKind :: ENUM_VALUE_KW } ; [INPUT_OBJECT] => { $ crate :: GraphqlSyntaxKind :: INPUT_OBJECT_KW } ; [INPUT_FIELD_DEFINITION] => { $ crate :: GraphqlSyntaxKind :: INPUT_FIELD_DEFINITION_KW } ; [ident] => { $ crate :: GraphqlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: GraphqlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: GraphqlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: GraphqlSyntaxKind :: HASH } ; }
