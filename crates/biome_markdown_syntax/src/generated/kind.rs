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
    L_ANGLE,
    R_ANGLE,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    SLASH,
    EQ,
    BANG,
    MINUS,
    STAR,
    BACKTICK,
    TILDE,
    WHITESPACE3,
    UNDERSCORE,
    HASH,
    FALSE_KW,
    MD_HARD_LINE_LITERAL,
    MD_SOFT_BREAK_LITERAL,
    MD_TEXTUAL_LITERAL,
    MD_STRING_LITERAL,
    MD_INDENT_CHUNK_LITERAL,
    MD_BREAK_BLOCK_LITERAL,
    NEWLINE,
    WHITESPACE,
    TAB,
    BOGUS,
    MD_BOGUS,
    MD_DOCUMENT,
    MD_BLOCK_LIST,
    MD_HASH_LIST,
    MD_HASH,
    MD_HEADER,
    MD_INDENT_CODE_BLOCK,
    MD_FENCED_CODE_BLOCK,
    MD_HTML_BLOCK,
    MD_LINK_BLOCK,
    MD_QUOTE,
    MD_ORDER_LIST_ITEM,
    MD_BULLET_LIST_ITEM,
    MD_BULLET_LIST,
    MD_ORDER_LIST,
    MD_PARAGRAPH,
    MD_PARAGRAPH_ITEM_LIST,
    MD_INLINE_CODE,
    MD_INLINE_EMPHASIS,
    MD_INLINE_LINK,
    MD_INLINE_IMAGE,
    MD_HARD_LINE,
    MD_SOFT_BREAK,
    MD_TEXTUAL,
    MD_SETEXT_HEADER,
    MD_STRING,
    MD_INDENT,
    MD_MINUS_THEMATIC_BREAK_BLOCK,
    MD_MINUS_LIST,
    MD_MINUS,
    MD_UNDERSCORE_THEMATIC_BREAK_BLOCK,
    MD_UNDERSCORE_LIST,
    MD_UNDERSCORE,
    MD_STAR_THEMATIC_BREAK_BLOCK,
    MD_STAR_LIST,
    MD_STAR,
    #[doc(hidden)]
    __LAST,
}
use self::MarkdownSyntaxKind::*;
impl MarkdownSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            L_ANGLE | R_ANGLE | L_PAREN | R_PAREN | L_BRACK | R_BRACK | SLASH | EQ | BANG
            | MINUS | STAR | BACKTICK | TILDE | WHITESPACE3 | UNDERSCORE | HASH => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            MD_HARD_LINE_LITERAL
            | MD_SOFT_BREAK_LITERAL
            | MD_TEXTUAL_LITERAL
            | MD_STRING_LITERAL
            | MD_INDENT_CHUNK_LITERAL
            | MD_BREAK_BLOCK_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            MD_BLOCK_LIST
            | MD_HASH_LIST
            | MD_BULLET_LIST
            | MD_ORDER_LIST
            | MD_PARAGRAPH_ITEM_LIST
            | MD_MINUS_LIST
            | MD_UNDERSCORE_LIST
            | MD_STAR_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<MarkdownSyntaxKind> {
        let kw = match ident {
            "false" => FALSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_ANGLE => "<",
            R_ANGLE => ">",
            L_PAREN => "(",
            R_PAREN => ")",
            L_BRACK => "[",
            R_BRACK => "]",
            SLASH => "/",
            EQ => "=",
            BANG => "!",
            MINUS => "-",
            STAR => "*",
            BACKTICK => "`",
            TILDE => "~",
            WHITESPACE3 => "   ",
            UNDERSCORE => "_",
            HASH => "#",
            FALSE_KW => "false",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [<] => { $ crate :: MarkdownSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: MarkdownSyntaxKind :: R_ANGLE } ; ['('] => { $ crate :: MarkdownSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: MarkdownSyntaxKind :: R_PAREN } ; ['['] => { $ crate :: MarkdownSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: MarkdownSyntaxKind :: R_BRACK } ; [/] => { $ crate :: MarkdownSyntaxKind :: SLASH } ; [=] => { $ crate :: MarkdownSyntaxKind :: EQ } ; [!] => { $ crate :: MarkdownSyntaxKind :: BANG } ; [-] => { $ crate :: MarkdownSyntaxKind :: MINUS } ; [*] => { $ crate :: MarkdownSyntaxKind :: STAR } ; ['`'] => { $ crate :: MarkdownSyntaxKind :: BACKTICK } ; [~] => { $ crate :: MarkdownSyntaxKind :: TILDE } ; [   ] => { $ crate :: MarkdownSyntaxKind :: WHITESPACE3 } ; [_] => { $ crate :: MarkdownSyntaxKind :: UNDERSCORE } ; [#] => { $ crate :: MarkdownSyntaxKind :: HASH } ; [false] => { $ crate :: MarkdownSyntaxKind :: FALSE_KW } ; [ident] => { $ crate :: MarkdownSyntaxKind :: IDENT } ; [EOF] => { $ crate :: MarkdownSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: MarkdownSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: MarkdownSyntaxKind :: HASH } ; }
