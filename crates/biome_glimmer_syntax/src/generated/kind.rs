//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum GlimmerSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    L_CURLY2,
    R_CURLY2,
    L_PAREN,
    R_PAREN,
    L_ANGLE,
    R_ANGLE,
    SLASH,
    DOT,
    PIPE,
    EQ,
    HASH,
    AT,
    AS_KW,
    IF_KW,
    ELSE_KW,
    EACH_KW,
    LET_KW,
    YIELD_KW,
    THIS_KW,
    TRUE_KW,
    FALSE_KW,
    NULL_KW,
    UNDEFINED_KW,
    STRING_LITERAL,
    NUMBER_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    TEXT,
    COMMENT,
    MUSTACHE_COMMENT,
    GLIMMER_ROOT,
    GLIMMER_BLOCK,
    GLIMMER_TEXT_NODE,
    GLIMMER_COMMENT_STATEMENT,
    GLIMMER_MUSTACHE_COMMENT_STATEMENT,
    GLIMMER_MUSTACHE_STATEMENT,
    GLIMMER_BLOCK_STATEMENT,
    GLIMMER_ELEMENT_NODE,
    GLIMMER_START_TAG,
    GLIMMER_END_TAG,
    GLIMMER_ATTRIBUTE_NODE,
    GLIMMER_ATTRIBUTE_VALUE,
    GLIMMER_CONCAT_STATEMENT,
    GLIMMER_ELEMENT_MODIFIER,
    GLIMMER_BLOCK_PARAMS,
    GLIMMER_PARAM_NAME,
    GLIMMER_PATH_EXPRESSION,
    GLIMMER_PATH_SEGMENT,
    GLIMMER_THIS_HEAD,
    GLIMMER_AT_HEAD,
    GLIMMER_VAR_HEAD,
    GLIMMER_SUB_EXPRESSION,
    GLIMMER_HASH,
    GLIMMER_HASH_PAIR,
    GLIMMER_STRING_LITERAL,
    GLIMMER_NUMBER_LITERAL,
    GLIMMER_BOOLEAN_LITERAL,
    GLIMMER_NULL_LITERAL,
    GLIMMER_UNDEFINED_LITERAL,
    GLIMMER_ELSE_BLOCK,
    SELF_CLOSING,
    GLIMMER_STATEMENT_LIST,
    GLIMMER_ATTRIBUTE_LIST,
    GLIMMER_CONCAT_PART_LIST,
    GLIMMER_ELEMENT_MODIFIER_LIST,
    GLIMMER_PARAM_NAME_LIST,
    GLIMMER_PARAMS_LIST,
    GLIMMER_PATH_SEGMENT_LIST,
    GLIMMER_HASH_PAIR_LIST,
    GLIMMER_BOGUS,
    GLIMMER_BOGUS_STATEMENT,
    GLIMMER_BOGUS_EXPRESSION,
    #[doc(hidden)]
    __LAST,
}
use self::GlimmerSyntaxKind::*;
impl GlimmerSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            L_CURLY2
                | R_CURLY2
                | L_PAREN
                | R_PAREN
                | L_ANGLE
                | R_ANGLE
                | SLASH
                | DOT
                | PIPE
                | EQ
                | HASH
                | AT
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, STRING_LITERAL | NUMBER_LITERAL)
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            GLIMMER_STATEMENT_LIST
                | GLIMMER_ATTRIBUTE_LIST
                | GLIMMER_CONCAT_PART_LIST
                | GLIMMER_ELEMENT_MODIFIER_LIST
                | GLIMMER_PARAM_NAME_LIST
                | GLIMMER_PARAMS_LIST
                | GLIMMER_PATH_SEGMENT_LIST
                | GLIMMER_HASH_PAIR_LIST
        )
    }
    pub fn from_keyword(ident: &str) -> Option<Self> {
        let kw = match ident {
            "as" => AS_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "each" => EACH_KW,
            "let" => LET_KW,
            "yield" => YIELD_KW,
            "this" => THIS_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "null" => NULL_KW,
            "undefined" => UNDEFINED_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_CURLY2 => "{{",
            R_CURLY2 => "}}",
            L_PAREN => "(",
            R_PAREN => ")",
            L_ANGLE => "<",
            R_ANGLE => ">",
            SLASH => "/",
            DOT => ".",
            PIPE => "|",
            EQ => "=",
            HASH => "#",
            AT => "@",
            AS_KW => "as",
            IF_KW => "if",
            ELSE_KW => "else",
            EACH_KW => "each",
            LET_KW => "let",
            YIELD_KW => "yield",
            THIS_KW => "this",
            TRUE_KW => "true",
            FALSE_KW => "false",
            NULL_KW => "null",
            UNDEFINED_KW => "undefined",
            EOF => "EOF",
            GLIMMER_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { ["{{"] => { $ crate :: GlimmerSyntaxKind :: L_CURLY2 } ; ["}}"] => { $ crate :: GlimmerSyntaxKind :: R_CURLY2 } ; ['('] => { $ crate :: GlimmerSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: GlimmerSyntaxKind :: R_PAREN } ; [<] => { $ crate :: GlimmerSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: GlimmerSyntaxKind :: R_ANGLE } ; [/] => { $ crate :: GlimmerSyntaxKind :: SLASH } ; [.] => { $ crate :: GlimmerSyntaxKind :: DOT } ; [|] => { $ crate :: GlimmerSyntaxKind :: PIPE } ; [=] => { $ crate :: GlimmerSyntaxKind :: EQ } ; [#] => { $ crate :: GlimmerSyntaxKind :: HASH } ; [@] => { $ crate :: GlimmerSyntaxKind :: AT } ; [as] => { $ crate :: GlimmerSyntaxKind :: AS_KW } ; [if] => { $ crate :: GlimmerSyntaxKind :: IF_KW } ; [else] => { $ crate :: GlimmerSyntaxKind :: ELSE_KW } ; [each] => { $ crate :: GlimmerSyntaxKind :: EACH_KW } ; [let] => { $ crate :: GlimmerSyntaxKind :: LET_KW } ; [yield] => { $ crate :: GlimmerSyntaxKind :: YIELD_KW } ; [this] => { $ crate :: GlimmerSyntaxKind :: THIS_KW } ; [true] => { $ crate :: GlimmerSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: GlimmerSyntaxKind :: FALSE_KW } ; [null] => { $ crate :: GlimmerSyntaxKind :: NULL_KW } ; [undefined] => { $ crate :: GlimmerSyntaxKind :: UNDEFINED_KW } ; [ident] => { $ crate :: GlimmerSyntaxKind :: IDENT } ; [EOF] => { $ crate :: GlimmerSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: GlimmerSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: GlimmerSyntaxKind :: HASH } ; }
