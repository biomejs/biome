//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum TailwindSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SLASH,
    BANG,
    DASH,
    COLON,
    L_BRACKET,
    R_BRACKET,
    L_PAREN,
    R_PAREN,
    WHITESPACE,
    TW_BASE,
    TW_VALUE,
    TW_SELECTOR,
    TW_PROPERTY,
    ERROR_TOKEN,
    NEWLINE,
    TW_ROOT,
    TW_CANDIDATE_LIST,
    TW_FULL_CANDIDATE,
    TW_ARBITRARY_CANDIDATE,
    TW_STATIC_CANDIDATE,
    TW_FUNCTIONAL_CANDIDATE,
    TW_VARIANT_LIST,
    TW_ARBITRARY_VARIANT,
    TW_STATIC_VARIANT,
    TW_FUNCTIONAL_VARIANT,
    TW_NAMED_VALUE,
    TW_ARBITRARY_VALUE,
    TW_CSS_VARIABLE_VALUE,
    TW_MODIFIER,
    TW_BOGUS,
    TW_BOGUS_CANDIDATE,
    TW_BOGUS_VARIANT,
    TW_BOGUS_MODIFIER,
    TW_BOGUS_VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::TailwindSyntaxKind::*;
impl TailwindSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            SLASH | BANG | DASH | COLON | L_BRACKET | R_BRACKET | L_PAREN | R_PAREN | WHITESPACE
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, TW_BASE | TW_VALUE | TW_SELECTOR | TW_PROPERTY)
    }
    pub const fn is_list(self) -> bool {
        matches!(self, TW_CANDIDATE_LIST | TW_VARIANT_LIST)
    }
    pub fn from_keyword(_ident: &str) -> Option<Self> {
        None
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SLASH => "/",
            BANG => "!",
            DASH => "-",
            COLON => ":",
            L_BRACKET => "[",
            R_BRACKET => "]",
            L_PAREN => "(",
            R_PAREN => ")",
            WHITESPACE => " ",
            TW_BASE => "base",
            TW_VALUE => "value",
            TW_SELECTOR => "selector",
            TW_PROPERTY => "property",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [/] => { $ crate :: TailwindSyntaxKind :: SLASH } ; [!] => { $ crate :: TailwindSyntaxKind :: BANG } ; [-] => { $ crate :: TailwindSyntaxKind :: DASH } ; [:] => { $ crate :: TailwindSyntaxKind :: COLON } ; ['['] => { $ crate :: TailwindSyntaxKind :: L_BRACKET } ; [']'] => { $ crate :: TailwindSyntaxKind :: R_BRACKET } ; ['('] => { $ crate :: TailwindSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: TailwindSyntaxKind :: R_PAREN } ; [' '] => { $ crate :: TailwindSyntaxKind :: WHITESPACE } ; [ident] => { $ crate :: TailwindSyntaxKind :: IDENT } ; [EOF] => { $ crate :: TailwindSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: TailwindSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: TailwindSyntaxKind :: HASH } ; }
