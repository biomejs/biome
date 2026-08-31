//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum TomlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    EQ,
    DOT,
    COMMA,
    L_BRACK,
    R_BRACK,
    L_CURLY,
    R_CURLY,
    TOML_BARE_KEY,
    TOML_BASIC_STRING,
    TOML_LITERAL_STRING,
    TOML_INTEGER,
    TOML_FLOAT,
    TOML_BOOLEAN,
    TOML_OFFSET_DATE_TIME,
    TOML_LOCAL_DATE_TIME,
    TOML_LOCAL_DATE,
    TOML_LOCAL_TIME,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    TOML_ROOT,
    TOML_ITEM_LIST,
    TOML_KEY_VALUE,
    TOML_TABLE,
    TOML_ARRAY_TABLE,
    TOML_KEY,
    TOML_KEY_SEGMENT_LIST,
    TOML_KEY_SEGMENT,
    TOML_STRING_VALUE,
    TOML_INTEGER_VALUE,
    TOML_FLOAT_VALUE,
    TOML_BOOLEAN_VALUE,
    TOML_OFFSET_DATE_TIME_VALUE,
    TOML_LOCAL_DATE_TIME_VALUE,
    TOML_LOCAL_DATE_VALUE,
    TOML_LOCAL_TIME_VALUE,
    TOML_ARRAY,
    TOML_ARRAY_ELEMENT_LIST,
    TOML_INLINE_TABLE,
    TOML_INLINE_TABLE_ELEMENT_LIST,
    TOML_BOGUS,
    TOML_BOGUS_VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::TomlSyntaxKind::*;
impl TomlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            EQ | DOT | COMMA | L_BRACK | R_BRACK | L_CURLY | R_CURLY
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            TOML_BARE_KEY
                | TOML_BASIC_STRING
                | TOML_LITERAL_STRING
                | TOML_INTEGER
                | TOML_FLOAT
                | TOML_BOOLEAN
                | TOML_OFFSET_DATE_TIME
                | TOML_LOCAL_DATE_TIME
                | TOML_LOCAL_DATE
                | TOML_LOCAL_TIME
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            TOML_ITEM_LIST
                | TOML_KEY_SEGMENT_LIST
                | TOML_ARRAY_ELEMENT_LIST
                | TOML_INLINE_TABLE_ELEMENT_LIST
        )
    }
    pub fn from_keyword(_ident: &str) -> Option<Self> {
        None
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            EQ => "=",
            DOT => ".",
            COMMA => ",",
            L_BRACK => "[",
            R_BRACK => "]",
            L_CURLY => "{",
            R_CURLY => "}",
            EOF => "",
            TOML_BARE_KEY => "bare key",
            TOML_BASIC_STRING => "basic string",
            TOML_LITERAL_STRING => "literal string",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [=] => { $ crate :: TomlSyntaxKind :: EQ } ; [.] => { $ crate :: TomlSyntaxKind :: DOT } ; [,] => { $ crate :: TomlSyntaxKind :: COMMA } ; ['['] => { $ crate :: TomlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: TomlSyntaxKind :: R_BRACK } ; ['{'] => { $ crate :: TomlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: TomlSyntaxKind :: R_CURLY } ; [ident] => { $ crate :: TomlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: TomlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: TomlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: TomlSyntaxKind :: HASH } ; }
