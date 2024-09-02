//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum MarkdownSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    PLUS,
    CALC_KW,
    NUMBER_LITERAL,
    COMMENT,
    NEWLINE,
    WHITESPACE,
    ERROR_TOKEN,
    ANY_VALUE,
    ROOT,
    BOGUS,
    NUMBER_VALUE,
    NUMBER_VALUE_LIST,
    #[doc(hidden)]
    __LAST,
}
use self::MarkdownSyntaxKind::*;
impl MarkdownSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            PLUS => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            NUMBER_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            NUMBER_VALUE_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<MarkdownSyntaxKind> {
        let kw = match ident {
            "calc" => CALC_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            PLUS => "+",
            CALC_KW => "calc",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [+] => { $ crate :: MarkdownSyntaxKind :: PLUS } ; [calc] => { $ crate :: MarkdownSyntaxKind :: CALC_KW } ; [ident] => { $ crate :: MarkdownSyntaxKind :: IDENT } ; [EOF] => { $ crate :: MarkdownSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: MarkdownSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: MarkdownSyntaxKind :: HASH } ; }
