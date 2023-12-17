//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum CssSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    HASH,
    AMP,
    PIPE,
    PIPE2,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    COLON,
    COLON2,
    EQ,
    BANG,
    NEQ,
    MINUS,
    LTEQ,
    GTEQ,
    PLUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AT,
    DOLLAR_EQ,
    TILDE_EQ,
    CDC,
    CDO,
    IMPORTANT_KW,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_CUSTOM_PROPERTY,
    CSS_SPACE_LITERAL,
    CSS_FUNCTION_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    FUNCTION_TOKEN,
    IDENT,
    AT_IDENT,
    BAD_URL,
    HASH_TOKEN,
    URL,
    DELIM,
    UNICODE_RANGE,
    CSS_STYLE_SHEET,
    CSS_STYLE_SHEET_CONTENT,
    CSS_QUALIFIED_RULE,
    CSS_QUALIFIED_RULE_PRELUDE,
    CSS_CURLY_BRACKETS_BLOCK,
    CSS_CURLY_BRACKETS_BLOCK_CONTENT,
    CSS_AT_RULE_COMPONENT_VALUE,
    CSS_AT_RULE_CONTENT,
    CSS_AT_RULE,
    CSS_DECLARATION_LIST,
    CSS_DECLARATION,
    CSS_LIST_OF_COMPONENT_VALUE,
    CSS_COMPONENT_VALUE,
    CSS_DECLARATION_IMPORTANT,
    CSS_SIMPLE_BLOCK,
    CSS_FUNCTION_BLOCK,
    CSS_SIMPLE_CURLY_BRACKETS_BLOCK,
    CSS_SIMPLE_PARENTHESES_BLOCK,
    CSS_SIMPLE_SQUARE_BRACKETS_BLOCK,
    CSS_PRESERVED_TOKEN,
    CSS_BLOCK_DECLARATION_LIST,
    CSS_IDENTIFIER,
    CSS_AT_KEYWORD,
    CSS_FUNCTION_TOKEN,
    CSS_STRING,
    CSS_NUMBER,
    CSS_COMPONENT_VALUE_LIST,
    CSS_SIMPLE_COMPONENT_VALUE_LIST,
    CSS_PERCENTAGE,
    CSS_DIMENSION,
    CSS_DELIM,
    CSS_HASH,
    CSS_PRESERVED_TOKEN_KEY,
    CSS_AT_RULE_SEMICOLON,
    CSS_BOGUS,
    #[doc(hidden)]
    __LAST,
}
use self::CssSyntaxKind::*;
impl CssSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | HASH | AMP | PIPE | PIPE2 | PLUS | STAR | SLASH
            | CARET | PERCENT | DOT | COLON | COLON2 | EQ | BANG | NEQ | MINUS | LTEQ | GTEQ
            | PLUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AT | DOLLAR_EQ
            | TILDE_EQ | CDC | CDO => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            CSS_STRING_LITERAL | CSS_NUMBER_LITERAL | CSS_CUSTOM_PROPERTY | CSS_SPACE_LITERAL
            | CSS_FUNCTION_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            CSS_DECLARATION_LIST
            | CSS_BLOCK_DECLARATION_LIST
            | CSS_COMPONENT_VALUE_LIST
            | CSS_SIMPLE_COMPONENT_VALUE_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<CssSyntaxKind> {
        let kw = match ident {
            "important" => IMPORTANT_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            TILDE => "~",
            HASH => "#",
            AMP => "&",
            PIPE => "|",
            PIPE2 => "||",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            DOT => ".",
            COLON => ":",
            COLON2 => "::",
            EQ => "=",
            BANG => "!",
            NEQ => "!=",
            MINUS => "-",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            PIPEEQ => "|=",
            AMPEQ => "&=",
            CARETEQ => "^=",
            SLASHEQ => "/=",
            STAREQ => "*=",
            PERCENTEQ => "%=",
            AT => "@",
            DOLLAR_EQ => "$=",
            TILDE_EQ => "~=",
            CDC => "-->",
            CDO => "<!--",
            IMPORTANT_KW => "important",
            CSS_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: CssSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: CssSyntaxKind :: COMMA } ; ['('] => { $ crate :: CssSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: CssSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: CssSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: CssSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: CssSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: CssSyntaxKind :: R_BRACK } ; [<] => { $ crate :: CssSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: CssSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: CssSyntaxKind :: TILDE } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; [&] => { $ crate :: CssSyntaxKind :: AMP } ; [|] => { $ crate :: CssSyntaxKind :: PIPE } ; [||] => { $ crate :: CssSyntaxKind :: PIPE2 } ; [+] => { $ crate :: CssSyntaxKind :: PLUS } ; [*] => { $ crate :: CssSyntaxKind :: STAR } ; [/] => { $ crate :: CssSyntaxKind :: SLASH } ; [^] => { $ crate :: CssSyntaxKind :: CARET } ; [%] => { $ crate :: CssSyntaxKind :: PERCENT } ; [.] => { $ crate :: CssSyntaxKind :: DOT } ; [:] => { $ crate :: CssSyntaxKind :: COLON } ; [::] => { $ crate :: CssSyntaxKind :: COLON2 } ; [=] => { $ crate :: CssSyntaxKind :: EQ } ; [!] => { $ crate :: CssSyntaxKind :: BANG } ; [!=] => { $ crate :: CssSyntaxKind :: NEQ } ; [-] => { $ crate :: CssSyntaxKind :: MINUS } ; [<=] => { $ crate :: CssSyntaxKind :: LTEQ } ; [>=] => { $ crate :: CssSyntaxKind :: GTEQ } ; [+=] => { $ crate :: CssSyntaxKind :: PLUSEQ } ; [|=] => { $ crate :: CssSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: CssSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: CssSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: CssSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: CssSyntaxKind :: STAREQ } ; [%=] => { $ crate :: CssSyntaxKind :: PERCENTEQ } ; [@] => { $ crate :: CssSyntaxKind :: AT } ; ["$="] => { $ crate :: CssSyntaxKind :: DOLLAR_EQ } ; [~=] => { $ crate :: CssSyntaxKind :: TILDE_EQ } ; [-->] => { $ crate :: CssSyntaxKind :: CDC } ; [<!--] => { $ crate :: CssSyntaxKind :: CDO } ; [important] => { $ crate :: CssSyntaxKind :: IMPORTANT_KW } ; [ident] => { $ crate :: CssSyntaxKind :: IDENT } ; [EOF] => { $ crate :: CssSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: CssSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; }
