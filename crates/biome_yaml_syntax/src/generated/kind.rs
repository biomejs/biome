//! Generated file, do not edit by hand, see `xtask/codegen`

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
    STAR,
    HASH,
    BANG,
    AT,
    SHL,
    AMP,
    PIPE,
    R_ANGLE,
    TILDE,
    BACKTICK,
    DOC_START,
    DOC_END,
    NULL_KW,
    YAML_STRING_VALUE,
    YAML_NUMBER_VALUE,
    YAML_BOOLEAN_VALUE,
    YAML_NULL_VALUE,
    YAML_BLOCK_VALUE,
    YAML_IDENTIFIER,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    YAML_ROOT,
    YAML_DOCUMENT_LIST,
    YAML_DOCUMENT,
    YAML_ARRAY_INLINE,
    YAML_ARRAY_INLINE_LIST,
    YAML_OBJECT,
    YAML_OBJECT_MEMBER,
    YAML_OBJECT_MEMBER_LIST,
    YAML_ARRAY,
    YAML_ARRAY_ITEM,
    YAML_ARRAY_ITEM_LIST,
    YAML_BLOCK_LITERAL,
    YAML_BLOCK_FOLDED,
    YAML_BOGUS,
    YAML_BOGUS_VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::YamlSyntaxKind::*;
impl YamlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            COLON
                | COMMA
                | L_CURLY
                | R_CURLY
                | L_BRACK
                | R_BRACK
                | DASH
                | PERCENT
                | STAR
                | HASH
                | BANG
                | AT
                | SHL
                | AMP
                | PIPE
                | R_ANGLE
                | TILDE
                | BACKTICK
                | DOC_START
                | DOC_END
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            YAML_STRING_VALUE
                | YAML_NUMBER_VALUE
                | YAML_BOOLEAN_VALUE
                | YAML_NULL_VALUE
                | YAML_BLOCK_VALUE
                | YAML_IDENTIFIER
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            YAML_DOCUMENT_LIST
                | YAML_ARRAY_INLINE_LIST
                | YAML_OBJECT_MEMBER_LIST
                | YAML_ARRAY_ITEM_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<YamlSyntaxKind> {
        let kw = match ident {
            "null" => NULL_KW,
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
            STAR => "*",
            HASH => "#",
            BANG => "!",
            AT => "@",
            SHL => "<<",
            AMP => "&",
            PIPE => "|",
            R_ANGLE => ">",
            TILDE => "~",
            BACKTICK => "`",
            DOC_START => "---",
            DOC_END => "...",
            NULL_KW => "null",
            YAML_STRING_VALUE => "string value",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: YamlSyntaxKind :: COLON } ; [,] => { $ crate :: YamlSyntaxKind :: COMMA } ; ['{'] => { $ crate :: YamlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: YamlSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: YamlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: YamlSyntaxKind :: R_BRACK } ; [-] => { $ crate :: YamlSyntaxKind :: DASH } ; [%] => { $ crate :: YamlSyntaxKind :: PERCENT } ; [*] => { $ crate :: YamlSyntaxKind :: STAR } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; [!] => { $ crate :: YamlSyntaxKind :: BANG } ; [@] => { $ crate :: YamlSyntaxKind :: AT } ; [<<] => { $ crate :: YamlSyntaxKind :: SHL } ; [&] => { $ crate :: YamlSyntaxKind :: AMP } ; [|] => { $ crate :: YamlSyntaxKind :: PIPE } ; [>] => { $ crate :: YamlSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: YamlSyntaxKind :: TILDE } ; ['`'] => { $ crate :: YamlSyntaxKind :: BACKTICK } ; [---] => { $ crate :: YamlSyntaxKind :: DOC_START } ; [...] => { $ crate :: YamlSyntaxKind :: DOC_END } ; [null] => { $ crate :: YamlSyntaxKind :: NULL_KW } ; [ident] => { $ crate :: YamlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: YamlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: YamlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; }
