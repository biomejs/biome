//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum JsonSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    COLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    NULL_KW,
    TRUE_KW,
    FALSE_KW,
    JSON_STRING_LITERAL,
    JSON_NUMBER_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    MULTILINE_COMMENT,
    JSON_ROOT,
    JSON_NUMBER_VALUE,
    JSON_STRING_VALUE,
    JSON_BOOLEAN_VALUE,
    JSON_NULL_VALUE,
    JSON_ARRAY_VALUE,
    JSON_OBJECT_VALUE,
    JSON_MEMBER_LIST,
    JSON_MEMBER,
    JSON_MEMBER_NAME,
    JSON_ARRAY_ELEMENT_LIST,
    JSON_BOGUS,
    JSON_BOGUS_MEMBER_NAME,
    JSON_BOGUS_VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::JsonSyntaxKind::*;
impl JsonSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            COLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, JSON_STRING_LITERAL | JSON_NUMBER_LITERAL)
    }
    pub const fn is_list(self) -> bool {
        matches!(self, JSON_MEMBER_LIST | JSON_ARRAY_ELEMENT_LIST)
    }
    pub fn from_keyword(ident: &str) -> Option<JsonSyntaxKind> {
        let kw = match ident {
            "null" => NULL_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            COLON => ":",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            NULL_KW => "null",
            TRUE_KW => "true",
            FALSE_KW => "false",
            JSON_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: JsonSyntaxKind :: COLON } ; [,] => { $ crate :: JsonSyntaxKind :: COMMA } ; ['('] => { $ crate :: JsonSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: JsonSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: JsonSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: JsonSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: JsonSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: JsonSyntaxKind :: R_BRACK } ; [null] => { $ crate :: JsonSyntaxKind :: NULL_KW } ; [true] => { $ crate :: JsonSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: JsonSyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: JsonSyntaxKind :: IDENT } ; [EOF] => { $ crate :: JsonSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: JsonSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: JsonSyntaxKind :: HASH } ; }
