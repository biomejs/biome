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
    QUESTION,
    DASH,
    DIRECTIVE_END,
    DOC_END,
    BACKTICK,
    AT,
    DIRECTIVE_LITERAL,
    ANCHOR_PROPERTY_LITERAL,
    TAG_PROPERTY_LITERAL,
    ALIAS_LITERAL,
    DOUBLE_QUOTED_LITERAL,
    SINGLE_QUOTED_LITERAL,
    PLAIN_LITERAL,
    LITERAL_BLOCK_LITERAL,
    FOLDED_BLOCK_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    INDENT,
    DEDENT,
    COMMENT,
    YAML_STREAM,
    YAML_DOCUMENT_LIST,
    YAML_DOCUMENT,
    YAML_DIRECTIVE_LIST,
    YAML_DIRECTIVE,
    YAML_FLOW_JSON_NODE,
    YAML_FLOW_YAML_NODE,
    YAML_FLOW_SEQUENCE,
    YAML_FLOW_SEQUENCE_ENTRY_LIST,
    YAML_FLOW_MAPPING,
    YAML_FLOW_MAP_ENTRY_LIST,
    YAML_FLOW_MAP_EXPLICIT_ENTRY,
    YAML_FLOW_MAP_IMPLICIT_ENTRY,
    YAML_ALIAS_NODE,
    YAML_BLOCK_COLLECTION,
    YAML_BLOCK_SEQUENCE,
    YAML_BLOCK_SEQUENCE_ENTRY_LIST,
    YAML_BLOCK_SEQUENCE_ENTRY,
    YAML_BLOCK_MAPPING,
    YAML_BLOCK_MAP_ENTRY_LIST,
    YAML_BLOCK_MAP_EXPLICIT_ENTRY,
    YAML_BLOCK_MAP_EXPLICIT_KEY,
    YAML_BLOCK_MAP_EXPLICIT_VALUE,
    YAML_BLOCK_MAP_IMPLICIT_ENTRY,
    YAML_BLOCK_MAP_IMPLICIT_VALUE,
    YAML_INDENTED_BLOCK,
    YAML_COMPACT_MAPPING,
    YAML_COMPACT_SEQUENCE,
    YAML_SINGLE_QUOTED_SCALAR,
    YAML_DOUBLE_QUOTED_SCALAR,
    YAML_PLAIN_SCALAR,
    YAML_LITERAL_SCALAR,
    YAML_FOLDED_SCALAR,
    YAML_PROPERTY_LIST,
    YAML_ANCHOR_PROPERTY,
    YAML_TAG_PROPERTY,
    YAML_BOGUS,
    YAML_BOGUS_NODE,
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
                | QUESTION
                | DASH
                | DIRECTIVE_END
                | DOC_END
                | BACKTICK
                | AT
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            DIRECTIVE_LITERAL
                | ANCHOR_PROPERTY_LITERAL
                | TAG_PROPERTY_LITERAL
                | ALIAS_LITERAL
                | DOUBLE_QUOTED_LITERAL
                | SINGLE_QUOTED_LITERAL
                | PLAIN_LITERAL
                | LITERAL_BLOCK_LITERAL
                | FOLDED_BLOCK_LITERAL
        )
    }
    pub const fn is_list(self) -> bool {
        matches!(
            self,
            YAML_DOCUMENT_LIST
                | YAML_DIRECTIVE_LIST
                | YAML_FLOW_SEQUENCE_ENTRY_LIST
                | YAML_FLOW_MAP_ENTRY_LIST
                | YAML_BLOCK_SEQUENCE_ENTRY_LIST
                | YAML_BLOCK_MAP_ENTRY_LIST
                | YAML_PROPERTY_LIST
        )
    }
    pub fn from_keyword(_ident: &str) -> Option<YamlSyntaxKind> {
        None
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            COLON => ":",
            COMMA => ",",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            QUESTION => "?",
            DASH => "-",
            DIRECTIVE_END => "---",
            DOC_END => "...",
            BACKTICK => "`",
            AT => "@",
            EOF => "EOF",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: YamlSyntaxKind :: COLON } ; [,] => { $ crate :: YamlSyntaxKind :: COMMA } ; ['{'] => { $ crate :: YamlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: YamlSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: YamlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: YamlSyntaxKind :: R_BRACK } ; [?] => { $ crate :: YamlSyntaxKind :: QUESTION } ; [-] => { $ crate :: YamlSyntaxKind :: DASH } ; [---] => { $ crate :: YamlSyntaxKind :: DIRECTIVE_END } ; [...] => { $ crate :: YamlSyntaxKind :: DOC_END } ; ['`'] => { $ crate :: YamlSyntaxKind :: BACKTICK } ; [@] => { $ crate :: YamlSyntaxKind :: AT } ; [ident] => { $ crate :: YamlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: YamlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: YamlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; }
