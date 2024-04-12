//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum YamlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    COLON,
    COMMA,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    DASH,
    PERCENT,
    ASTERISK,
    HASH,
    SHL,
    AMP,
    DOC_START,
    DOC_END,
    YAML_SCALAR,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    YAML_ROOT,
    YAML_DOCUMENT_LIST,
    ANY_YAML_NODE,
    YAML_SCALAR,
    YAML_SEQUENCE,
    YAML_SEQUENCE_ELEMENTS,
    YAML_MAPPING,
    YAML_MAPPING_ENTRY,
    YAML_MAPPING_ENTRIES,
    YAML_ALIAS,
    YAML_BOGUS,
    YAML_COMMENT,
    #[doc(hidden)]
    __LAST,
}
use self::YamlSyntaxKind::*;
impl YamlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            COLON | COMMA | L_CURLY | R_CURLY | L_BRACK | R_BRACK | DASH | PERCENT | ASTERISK
            | HASH | SHL | AMP | DOC_START | DOC_END => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            YAML_SCALAR => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            YAML_DOCUMENT_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<YamlSyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            COLON => ":",
            COMMA => ",",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            DASH => "-",
            PERCENT => "%",
            ASTERISK => "*",
            HASH => "#",
            SHL => "<<",
            AMP => "&",
            DOC_START => "---",
            DOC_END => "...",
            YAML_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: YamlSyntaxKind :: COLON } ; [,] => { $ crate :: YamlSyntaxKind :: COMMA } ; ['{'] => { $ crate :: YamlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: YamlSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: YamlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: YamlSyntaxKind :: R_BRACK } ; [-] => { $ crate :: YamlSyntaxKind :: DASH } ; [%] => { $ crate :: YamlSyntaxKind :: PERCENT } ; [*] => { $ crate :: YamlSyntaxKind :: ASTERISK } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; [<<] => { $ crate :: YamlSyntaxKind :: SHL } ; [&] => { $ crate :: YamlSyntaxKind :: AMP } ; [---] => { $ crate :: YamlSyntaxKind :: DOC_START } ; [...] => { $ crate :: YamlSyntaxKind :: DOC_END } ; [ident] => { $ crate :: YamlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: YamlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: YamlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; }
