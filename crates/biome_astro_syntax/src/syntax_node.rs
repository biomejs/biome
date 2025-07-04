use crate::{AstroLanguage, AstroSyntaxKind};
use biome_rowan::{SyntaxNode, SyntaxToken};

pub type AstroSyntaxNode = SyntaxNode<AstroLanguage>;
pub type AstroSyntaxToken = SyntaxToken<AstroLanguage>;
// pub type AstroSyntaxTrivia = SyntaxTrivia<AstroLanguage>;

impl AstroSyntaxNode {
    /// Returns `true` if this node is a frontmatter block
    pub fn is_frontmatter(&self) -> bool {
        self.kind().is_frontmatter()
    }

    /// Returns `true` if this node is an expression
    pub fn is_expression(&self) -> bool {
        self.kind().is_expression()
    }

    /// Returns `true` if this node is an element (HTML-like tag)
    pub fn is_element(&self) -> bool {
        self.kind().is_element()
    }

    /// Returns `true` if this node is a component (Astro/React component)
    pub fn is_component(&self) -> bool {
        self.kind().is_component()
    }

    /// Returns `true` if this node is an attribute
    pub fn is_attribute(&self) -> bool {
        self.kind().is_attribute()
    }
}

impl AstroSyntaxToken {
    /// Returns `true` if this token is part of a frontmatter block
    pub fn is_in_frontmatter(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.is_frontmatter())
    }

    /// Returns `true` if this token is part of an expression
    pub fn is_in_expression(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.is_expression())
    }
}