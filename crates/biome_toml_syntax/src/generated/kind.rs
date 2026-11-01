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
    L_DOUBLE_BRACK,
    R_DOUBLE_BRACK,
    L_BRACK,
    R_BRACK,
    L_CURLY,
    R_CURLY,
    EQ,
    DOT,
    COMMA,
    TRUE_KW,
    FALSE_KW,
    TOML_BARE_KEY_LITERAL,
    TOML_BASIC_STRING_LITERAL,
    TOML_LITERAL_STRING_LITERAL,
    TOML_ML_BASIC_STRING_LITERAL,
    TOML_ML_LITERAL_STRING_LITERAL,
    TOML_INTEGER_LITERAL,
    TOML_FLOAT_LITERAL,
    TOML_OFFSET_DATE_TIME_LITERAL,
    TOML_LOCAL_DATE_TIME_LITERAL,
    TOML_LOCAL_DATE_LITERAL,
    TOML_LOCAL_TIME_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    TOML_ROOT,
    TOML_ROOT_ITEM_LIST,
    TOML_TABLE,
    TOML_ARRAY_TABLE,
    TOML_KEY_VALUE_LIST,
    TOML_KEY_VALUE,
    TOML_KEY,
    TOML_KEY_PART_LIST,
    TOML_BARE_KEY,
    TOML_QUOTED_KEY,
    TOML_STRING,
    TOML_INTEGER,
    TOML_FLOAT,
    TOML_BOOLEAN,
    TOML_DATE_TIME,
    TOML_ARRAY,
    TOML_ARRAY_ELEMENT_LIST,
    TOML_INLINE_TABLE,
    TOML_INLINE_TABLE_ENTRY_LIST,
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
            L_DOUBLE_BRACK
                | R_DOUBLE_BRACK
                | L_BRACK
                | R_BRACK
                | L_CURLY
                | R_CURLY
                | EQ
                | DOT
                | COMMA
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            TOML_BARE_KEY_LITERAL
                | TOML_BASIC_STRING_LITERAL
                | TOML_LITERAL_STRING_LITERAL
                | TOML_ML_BASIC_STRING_LITERAL
                | TOML_ML_LITERAL_STRING_LITERAL
                | TOML_INTEGER_LITERAL
                | TOML_FLOAT_LITERAL
                | TOML_OFFSET_DATE_TIME_LITERAL
                | TOML_LOCAL_DATE_TIME_LITERAL
                | TOML_LOCAL_DATE_LITERAL
                | TOML_LOCAL_TIME_LITERAL
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            TOML_ROOT_ITEM_LIST
                | TOML_KEY_VALUE_LIST
                | TOML_KEY_PART_LIST
                | TOML_ARRAY_ELEMENT_LIST
                | TOML_INLINE_TABLE_ENTRY_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<Self> {
        let kw = match ident {
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_DOUBLE_BRACK => "[[",
            R_DOUBLE_BRACK => "]]",
            L_BRACK => "[",
            R_BRACK => "]",
            L_CURLY => "{",
            R_CURLY => "}",
            EQ => "=",
            DOT => ".",
            COMMA => ",",
            TRUE_KW => "true",
            FALSE_KW => "false",
            EOF => "",
            TOML_BARE_KEY_LITERAL => "bare key",
            TOML_BASIC_STRING_LITERAL => "basic string",
            TOML_LITERAL_STRING_LITERAL => "literal string",
            TOML_ML_BASIC_STRING_LITERAL => "multi-line basic string",
            TOML_ML_LITERAL_STRING_LITERAL => "multi-line literal string",
            TOML_INTEGER_LITERAL => "integer",
            TOML_FLOAT_LITERAL => "float",
            TOML_OFFSET_DATE_TIME_LITERAL => "offset date-time",
            TOML_LOCAL_DATE_TIME_LITERAL => "local date-time",
            TOML_LOCAL_DATE_LITERAL => "local date",
            TOML_LOCAL_TIME_LITERAL => "local time",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { ["[["] => { $ crate :: TomlSyntaxKind :: L_DOUBLE_BRACK } ; ["]]"] => { $ crate :: TomlSyntaxKind :: R_DOUBLE_BRACK } ; ['['] => { $ crate :: TomlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: TomlSyntaxKind :: R_BRACK } ; ['{'] => { $ crate :: TomlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: TomlSyntaxKind :: R_CURLY } ; [=] => { $ crate :: TomlSyntaxKind :: EQ } ; [.] => { $ crate :: TomlSyntaxKind :: DOT } ; [,] => { $ crate :: TomlSyntaxKind :: COMMA } ; [true] => { $ crate :: TomlSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: TomlSyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: TomlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: TomlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: TomlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: TomlSyntaxKind :: HASH } ; }
