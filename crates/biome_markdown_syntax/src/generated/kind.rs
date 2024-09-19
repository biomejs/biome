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
    MARKDOWN_HARD_LINE_LITERAL,
    MARKDOWN_SOFT_BREAK_LITERAL,
    MARKDOWN_TEXTUAL_LITERAL,
    MARKDOWN_STRING_LITERAL,
    MARKDOWN_INDENT_CHUNK_LITERAL,
    MARKDOWN_BREAK_BLOCK_LITERAL,
    NEWLINE,
    WHITESPACE,
    TAB,
    BOGUS,
    MARKDOWN_BOGUS,
    MARKDOWN_DOCUMENT,
    MARKDOWN_BLOCK_LIST,
    MARKDOWN_HASH_LIST,
    MARKDOWN_HASH,
    MARKDOWN_HEADER,
    MARKDOWN_INDENT_CODE_BLOCK,
    MARKDOWN_FENCED_CODE_BLOCK,
    MARKDOWN_HTML_BLOCK,
    MARKDOWN_LINK_BLOCK,
    MARKDOWN_QUOTE,
    MARKDOWN_ORDER_LIST_ITEM,
    MARKDOWN_BULLET_LIST_ITEM,
    MARKDOWN_BULLET_LIST,
    MARKDOWN_ORDER_LIST,
    MARKDOWN_PARAGRAPH,
    MARKDOWN_PARAGRAPH_ITEM_LIST,
    MARKDOWN_INLINE_CODE,
    MARKDOWN_INLINE_EMPHASIS,
    MARKDOWN_INLINE_LINK,
    MARKDOWN_INLINE_IMAGE,
    MARKDOWN_HARD_LINE,
    MARKDOWN_SOFT_BREAK,
    MARKDOWN_TEXTUAL,
    MARKDOWN_SETEXT_HEADER,
    MARKDOWN_STRING,
    MARKDOWN_INDENT,
    MARKDOWN_BREAK_BLOCK,
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
            MARKDOWN_HARD_LINE_LITERAL
            | MARKDOWN_SOFT_BREAK_LITERAL
            | MARKDOWN_TEXTUAL_LITERAL
            | MARKDOWN_STRING_LITERAL
            | MARKDOWN_INDENT_CHUNK_LITERAL
            | MARKDOWN_BREAK_BLOCK_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            MARKDOWN_BLOCK_LIST
            | MARKDOWN_HASH_LIST
            | MARKDOWN_BULLET_LIST
            | MARKDOWN_ORDER_LIST
            | MARKDOWN_PARAGRAPH_ITEM_LIST => true,
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
