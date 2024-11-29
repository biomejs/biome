#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum MiniSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    QUESTION,
    ASTERISK,
    COLON,
    L_BRACKET,
    R_BRACKET,
    OR,
    DOUBLE_OR,
    DOUBLE_AND,
    COMPLEX_KW,
    SIMPLE_KW,
    MINI_STRING_LITERAL,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    COMMA,
    MINI_ROOT,
    MINI_GRAMMAR,
    MINI_NODE_LIST,
    MINI_SIMPLE_NODE,
    MINI_COMPLEX_NODE,
    MINI_BOGUS,
    MINI_NODE_LIST,
    #[doc(hidden)]
    __LAST,
}
use self::MiniSyntaxKind::*;
impl MiniSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            QUESTION | ASTERISK | COLON | L_BRACKET | R_BRACKET | OR | DOUBLE_OR | DOUBLE_AND => {
                true
            }
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            MINI_STRING_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            MINI_NODE_LIST | MINI_NODE_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<MiniSyntaxKind> {
        let kw = match ident {
            "complex" => COMPLEX_KW,
            "simple" => SIMPLE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            QUESTION => "?",
            ASTERISK => "*",
            COLON => ":",
            L_BRACKET => "(",
            R_BRACKET => ")",
            OR => "|",
            DOUBLE_OR => "||",
            DOUBLE_AND => "&&",
            COMPLEX_KW => "complex",
            SIMPLE_KW => "simple",
            MINI_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [?] => { $ crate :: MiniSyntaxKind :: QUESTION } ; [*] => { $ crate :: MiniSyntaxKind :: ASTERISK } ; [:] => { $ crate :: MiniSyntaxKind :: COLON } ; ['('] => { $ crate :: MiniSyntaxKind :: L_BRACKET } ; [')'] => { $ crate :: MiniSyntaxKind :: R_BRACKET } ; [|] => { $ crate :: MiniSyntaxKind :: OR } ; [||] => { $ crate :: MiniSyntaxKind :: DOUBLE_OR } ; [&&] => { $ crate :: MiniSyntaxKind :: DOUBLE_AND } ; [complex] => { $ crate :: MiniSyntaxKind :: COMPLEX_KW } ; [simple] => { $ crate :: MiniSyntaxKind :: SIMPLE_KW } ; [ident] => { $ crate :: MiniSyntaxKind :: IDENT } ; [EOF] => { $ crate :: MiniSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: MiniSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: MiniSyntaxKind :: HASH } ; }
