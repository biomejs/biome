// This file contains minimal AST definitions for Astro
// In a full implementation, this would be auto-generated from the grammar

use biome_rowan::{
    Language, SyntaxKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum AstroSyntaxKind {
    // Technical tokens
    TOMBSTONE = 0,
    EOF,
    UNICODE_BOM,
    NEWLINE,
    WHITESPACE,

    // Punctuation
    L_CURLY,     // {
    R_CURLY,     // }
    L_ANGLE,     // <
    R_ANGLE,     // >
    SLASH,       // /
    EQUALS,      // =
    DOT3,        // ...
    BACKTICK,    // `
    MINUS3,      // ---

    // Comments
    ASTRO_COMMENT,

    // Literals
    ASTRO_STRING_LITERAL,
    ASTRO_TEXT,
    ASTRO_JS_CONTENT,
    ASTRO_COMMENT_CONTENT,
    ASTRO_TEMPLATE_LITERAL_CONTENT,
    ASTRO_LITERAL,

    // Names
    ASTRO_ELEMENT_NAME,
    ASTRO_COMPONENT_NAME,
    ASTRO_FRAGMENT_NAME,
    ASTRO_ATTRIBUTE_NAME,

    // Keywords
    DOCTYPE_KW,
    HTML_KW,
    NULL_KW,
    TRUE_KW,
    FALSE_KW,

    // Lists
    ASTRO_ELEMENT_LIST,
    ASTRO_ATTRIBUTE_LIST,

    // Nodes
    ASTRO_ROOT,
    ASTRO_FRONTMATTER,
    ASTRO_FRONTMATTER_CONTENT,
    ASTRO_ELEMENT,
    ASTRO_SELF_CLOSING_ELEMENT,
    ASTRO_COMPONENT,
    ASTRO_SELF_CLOSING_COMPONENT,
    ASTRO_FRAGMENT,
    ASTRO_OPENING_ELEMENT,
    ASTRO_OPENING_COMPONENT,
    ASTRO_OPENING_FRAGMENT,
    ASTRO_CLOSING_ELEMENT,
    ASTRO_CLOSING_COMPONENT,
    ASTRO_CLOSING_FRAGMENT,
    ASTRO_EXPRESSION,
    ASTRO_EXPRESSION_CONTENT,
    ASTRO_ATTRIBUTE,
    ASTRO_SHORTHAND_ATTRIBUTE,
    ASTRO_SPREAD_ATTRIBUTE,
    ASTRO_EXPRESSION_ATTRIBUTE,
    ASTRO_TEMPLATE_LITERAL_ATTRIBUTE,
    ASTRO_ATTRIBUTE_INITIALIZER_CLAUSE,
    ASTRO_ATTRIBUTE_VALUE,
    ASTRO_DOCTYPE,

    // Bogus nodes
    ASTRO_BOGUS,
    ASTRO_BOGUS_ELEMENT,
    ASTRO_BOGUS_ATTRIBUTE,
    ASTRO_BOGUS_EXPRESSION,

    __LAST,
}

use AstroSyntaxKind::*;

impl SyntaxKind for AstroSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            ASTRO_BOGUS | ASTRO_BOGUS_ELEMENT | ASTRO_BOGUS_ATTRIBUTE | ASTRO_BOGUS_EXPRESSION
        )
    }

    fn to_bogus(&self) -> Self {
        ASTRO_BOGUS
    }

    fn to_raw(&self) -> biome_rowan::RawSyntaxKind {
        biome_rowan::RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: biome_rowan::RawSyntaxKind) -> Self {
        assert!(raw.0 <= (__LAST as u16));
        unsafe { std::mem::transmute(raw.0) }
    }

    fn is_root(&self) -> bool {
        matches!(self, ASTRO_ROOT)
    }

    fn is_list(&self) -> bool {
        matches!(self, ASTRO_ELEMENT_LIST | ASTRO_ATTRIBUTE_LIST)
    }

    fn is_trivia(self) -> bool {
        matches!(self, NEWLINE | WHITESPACE)
    }

    fn to_string(&self) -> Option<&'static str> {
        let name = match self {
            TOMBSTONE => "TOMBSTONE",
            EOF => "EOF",
            UNICODE_BOM => "UNICODE_BOM",
            NEWLINE => "NEWLINE",
            WHITESPACE => "WHITESPACE",
            L_CURLY => "L_CURLY",
            R_CURLY => "R_CURLY",
            L_ANGLE => "L_ANGLE",
            R_ANGLE => "R_ANGLE",
            SLASH => "SLASH",
            EQUALS => "EQUALS",
            DOT3 => "DOT3",
            BACKTICK => "BACKTICK",
            MINUS3 => "MINUS3",
            ASTRO_COMMENT => "ASTRO_COMMENT",
            ASTRO_STRING_LITERAL => "ASTRO_STRING_LITERAL",
            ASTRO_TEXT => "ASTRO_TEXT",
            ASTRO_JS_CONTENT => "ASTRO_JS_CONTENT",
            ASTRO_COMMENT_CONTENT => "ASTRO_COMMENT_CONTENT",
            ASTRO_TEMPLATE_LITERAL_CONTENT => "ASTRO_TEMPLATE_LITERAL_CONTENT",
            ASTRO_LITERAL => "ASTRO_LITERAL",
            ASTRO_ELEMENT_NAME => "ASTRO_ELEMENT_NAME",
            ASTRO_COMPONENT_NAME => "ASTRO_COMPONENT_NAME",
            ASTRO_FRAGMENT_NAME => "ASTRO_FRAGMENT_NAME",
            ASTRO_ATTRIBUTE_NAME => "ASTRO_ATTRIBUTE_NAME",
            DOCTYPE_KW => "DOCTYPE_KW",
            HTML_KW => "HTML_KW",
            NULL_KW => "NULL_KW",
            TRUE_KW => "TRUE_KW",
            FALSE_KW => "FALSE_KW",
            ASTRO_ELEMENT_LIST => "ASTRO_ELEMENT_LIST",
            ASTRO_ATTRIBUTE_LIST => "ASTRO_ATTRIBUTE_LIST",
            ASTRO_ROOT => "ASTRO_ROOT",
            ASTRO_FRONTMATTER => "ASTRO_FRONTMATTER",
            ASTRO_FRONTMATTER_CONTENT => "ASTRO_FRONTMATTER_CONTENT",
            ASTRO_ELEMENT => "ASTRO_ELEMENT",
            ASTRO_SELF_CLOSING_ELEMENT => "ASTRO_SELF_CLOSING_ELEMENT",
            ASTRO_COMPONENT => "ASTRO_COMPONENT",
            ASTRO_SELF_CLOSING_COMPONENT => "ASTRO_SELF_CLOSING_COMPONENT",
            ASTRO_FRAGMENT => "ASTRO_FRAGMENT",
            ASTRO_OPENING_ELEMENT => "ASTRO_OPENING_ELEMENT",
            ASTRO_OPENING_COMPONENT => "ASTRO_OPENING_COMPONENT",
            ASTRO_OPENING_FRAGMENT => "ASTRO_OPENING_FRAGMENT",
            ASTRO_CLOSING_ELEMENT => "ASTRO_CLOSING_ELEMENT",
            ASTRO_CLOSING_COMPONENT => "ASTRO_CLOSING_COMPONENT",
            ASTRO_CLOSING_FRAGMENT => "ASTRO_CLOSING_FRAGMENT",
            ASTRO_EXPRESSION => "ASTRO_EXPRESSION",
            ASTRO_EXPRESSION_CONTENT => "ASTRO_EXPRESSION_CONTENT",
            ASTRO_ATTRIBUTE => "ASTRO_ATTRIBUTE",
            ASTRO_SHORTHAND_ATTRIBUTE => "ASTRO_SHORTHAND_ATTRIBUTE",
            ASTRO_SPREAD_ATTRIBUTE => "ASTRO_SPREAD_ATTRIBUTE",
            ASTRO_EXPRESSION_ATTRIBUTE => "ASTRO_EXPRESSION_ATTRIBUTE",
            ASTRO_TEMPLATE_LITERAL_ATTRIBUTE => "ASTRO_TEMPLATE_LITERAL_ATTRIBUTE",
            ASTRO_ATTRIBUTE_INITIALIZER_CLAUSE => "ASTRO_ATTRIBUTE_INITIALIZER_CLAUSE",
            ASTRO_ATTRIBUTE_VALUE => "ASTRO_ATTRIBUTE_VALUE",
            ASTRO_DOCTYPE => "ASTRO_DOCTYPE",
            ASTRO_BOGUS => "ASTRO_BOGUS",
            ASTRO_BOGUS_ELEMENT => "ASTRO_BOGUS_ELEMENT",
            ASTRO_BOGUS_ATTRIBUTE => "ASTRO_BOGUS_ATTRIBUTE",
            ASTRO_BOGUS_EXPRESSION => "ASTRO_BOGUS_EXPRESSION",
            __LAST => "__LAST",
        };
        Some(name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AstroLanguage;

impl Language for AstroLanguage {
    type Kind = AstroSyntaxKind;
}

// Token macros
#[macro_export]
macro_rules! T {
    ['{'] => { $crate::AstroSyntaxKind::L_CURLY };
    ['}'] => { $crate::AstroSyntaxKind::R_CURLY };
    ['<'] => { $crate::AstroSyntaxKind::L_ANGLE };
    ['>'] => { $crate::AstroSyntaxKind::R_ANGLE };
    ['/'] => { $crate::AstroSyntaxKind::SLASH };
    ['='] => { $crate::AstroSyntaxKind::EQUALS };
    ["..."] => { $crate::AstroSyntaxKind::DOT3 };
    ["`"] => { $crate::AstroSyntaxKind::BACKTICK };
    ["---"] => { $crate::AstroSyntaxKind::MINUS3 };
    [doctype] => { $crate::AstroSyntaxKind::DOCTYPE_KW };
    [html] => { $crate::AstroSyntaxKind::HTML_KW };
    [null] => { $crate::AstroSyntaxKind::NULL_KW };
    [true] => { $crate::AstroSyntaxKind::TRUE_KW };
    [false] => { $crate::AstroSyntaxKind::FALSE_KW };
}

// Simplified AST node definitions
use biome_rowan::{AstNode};

// Root node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstroRoot {
    pub(crate) syntax: crate::AstroSyntaxNode,
}

impl AstNode for AstroRoot {
    type Language = AstroLanguage;

    fn can_cast(kind: Self::Kind) -> bool {
        matches!(kind, ASTRO_ROOT)
    }

    fn cast(syntax: crate::AstroSyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &crate::AstroSyntaxNode {
        &self.syntax
    }

    fn into_syntax(self) -> crate::AstroSyntaxNode {
        self.syntax
    }
}

impl AstroRoot {
    pub fn bom(&self) -> Result<crate::AstroSyntaxToken, biome_rowan::SyntaxError> {
        Err(biome_rowan::SyntaxError::MissingRequiredChild)
    }

    pub fn frontmatter(&self) -> Result<AstroFrontmatter, biome_rowan::SyntaxError> {
        Err(biome_rowan::SyntaxError::MissingRequiredChild)
    }

    pub fn body(&self) -> Result<crate::AstroSyntaxNode, biome_rowan::SyntaxError> {
        Ok(self.syntax.clone())
    }
}

// Basic nodes for compilation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AstroFrontmatter {
    pub(crate) syntax: crate::AstroSyntaxNode,
}

impl AstNode for AstroFrontmatter {
    type Language = AstroLanguage;

    fn can_cast(kind: Self::Kind) -> bool {
        matches!(kind, ASTRO_FRONTMATTER)
    }

    fn cast(syntax: crate::AstroSyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }

    fn syntax(&self) -> &crate::AstroSyntaxNode {
        &self.syntax
    }

    fn into_syntax(self) -> crate::AstroSyntaxNode {
        self.syntax
    }
}

impl AstroFrontmatter {
    pub fn content(&self) -> Result<crate::AstroSyntaxNode, biome_rowan::SyntaxError> {
        Ok(self.syntax.clone())
    }
}