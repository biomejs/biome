use biome_astro_syntax::{
    AstroSyntaxKind::{self, *},
    AstroSyntaxNode, AstroSyntaxToken, T,
};
use biome_rowan::{AstNode, SyntaxNode, SyntaxToken};

mod generated;
pub mod make;

pub use generated::*;
pub use make::*;

/// Creates a new syntax token with the given kind and text
pub fn token(kind: AstroSyntaxKind) -> AstroSyntaxTokenBuilder {
    AstroSyntaxTokenBuilder::new(kind)
}

pub struct AstroSyntaxTokenBuilder {
    kind: AstroSyntaxKind,
    text: Option<String>,
}

impl AstroSyntaxTokenBuilder {
    pub fn new(kind: AstroSyntaxKind) -> Self {
        Self { kind, text: None }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn build(self) -> AstroSyntaxToken {
        let text = self.text.unwrap_or_else(|| match self.kind {
            T!['{'] => "{".into(),
            T!['}'] => "}".into(),
            T!['<'] => "<".into(),
            T!['>'] => ">".into(),
            T!['/'] => "/".into(),
            T!['='] => "=".into(),
            T!['...'] => "...".into(),
            T!['`'] => "`".into(),
            T!['---'] => "---".into(),
            T![doctype] => "doctype".into(),
            T![html] => "html".into(),
            T![null] => "null".into(),
            T![true] => "true".into(),
            T![false] => "false".into(),
            _ => String::new(),
        });

        AstroSyntaxToken::new_detached(self.kind, &text)
    }
}

/// Creates a trivia token (whitespace, newline)
pub fn trivia(kind: AstroSyntaxKind, text: &str) -> AstroSyntaxToken {
    debug_assert!(kind.is_trivia());
    AstroSyntaxToken::new_detached(kind, text)
}

/// Creates a whitespace token
pub fn whitespace(text: &str) -> AstroSyntaxToken {
    trivia(WHITESPACE, text)
}

/// Creates a newline token
pub fn newline() -> AstroSyntaxToken {
    trivia(NEWLINE, "\n")
}

/// Creates a space token
pub fn space() -> AstroSyntaxToken {
    whitespace(" ")
}

/// Creates an empty element list
pub fn empty_element_list() -> AstroSyntaxNode {
    AstroSyntaxNode::new_detached(ASTRO_ELEMENT_LIST, [])
}

/// Creates an empty attribute list
pub fn empty_attribute_list() -> AstroSyntaxNode {
    AstroSyntaxNode::new_detached(ASTRO_ATTRIBUTE_LIST, [])
}