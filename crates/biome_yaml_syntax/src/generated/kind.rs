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
    COMMENT,
    FLOW_START,
    FLOW_END,
    MAPPING_START,
    MAPPING_END,
    SEQUENCE_START,
    SEQUENCE_END,
    YAML_ROOT,
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
    YAML_FLOW_IN_BLOCK_NODE,
    YAML_BLOCK_SCALAR,
    YAML_BLOCK_SEQUENCE,
    YAML_BLOCK_SEQUENCE_ENTRY_LIST,
    YAML_BLOCK_SEQUENCE_ENTRY,
    YAML_BLOCK_MAPPING,
    YAML_BLOCK_MAP_ENTRY_LIST,
    YAML_BLOCK_MAP_EXPLICIT_ENTRY,
    YAML_BLOCK_MAP_IMPLICIT_ENTRY,
    YAML_INDENTED_BLOCK,
    YAML_COMPACT_MAPPING,
    YAML_COMPACT_MAPPING_INDENTED,
    YAML_COMPACT_SEQUENCE,
    YAML_COMPACT_SEQUENCE_INDENTED,
    YAML_SINGLE_QUOTED_SCALAR,
    YAML_DOUBLE_QUOTED_SCALAR,
    YAML_PLAIN_SCALAR,
    YAML_LITERAL_SCALAR,
    YAML_FOLDED_SCALAR,
    YAML_PROPERTIES_ANCHOR_FIRST,
    YAML_PROPERTIES_TAG_FIRST,
    YAML_ANCHOR_PROPERTY,
    YAML_TAG_PROPERTY,
    YAML_BOGUS,
    YAML_BOGUS_BLOCK_NODE,
    YAML_BOGUS_BLOCK_MAP_ENTRY,
    YAML_BOGUS_FLOW_NODE,
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
        )
    }
    pub fn from_keyword(_ident: &str) -> Option<Self> {
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
            FLOW_START => "start of a flow node",
            FLOW_END => "end of a flow node",
            MAPPING_START => "start of a block mapping",
            MAPPING_END => "end of a block mapping",
            SEQUENCE_START => "start of a block sequence",
            SEQUENCE_END => "end of a block sequence",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: YamlSyntaxKind :: COLON } ; [,] => { $ crate :: YamlSyntaxKind :: COMMA } ; ['{'] => { $ crate :: YamlSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: YamlSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: YamlSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: YamlSyntaxKind :: R_BRACK } ; [?] => { $ crate :: YamlSyntaxKind :: QUESTION } ; [-] => { $ crate :: YamlSyntaxKind :: DASH } ; [---] => { $ crate :: YamlSyntaxKind :: DIRECTIVE_END } ; [...] => { $ crate :: YamlSyntaxKind :: DOC_END } ; ['`'] => { $ crate :: YamlSyntaxKind :: BACKTICK } ; [@] => { $ crate :: YamlSyntaxKind :: AT } ; [ident] => { $ crate :: YamlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: YamlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: YamlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: YamlSyntaxKind :: HASH } ; }
