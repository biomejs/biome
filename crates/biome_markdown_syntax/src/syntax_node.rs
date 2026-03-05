//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{MarkdownSyntaxKind, MdDocument};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MarkdownLanguage;

impl Language for MarkdownLanguage {
    type Kind = MarkdownSyntaxKind;
    type Root = MdDocument;
}

pub type MarkdownSyntaxNode = biome_rowan::SyntaxNode<MarkdownLanguage>;
pub type MarkdownSyntaxToken = biome_rowan::SyntaxToken<MarkdownLanguage>;
pub type MarkdownSyntaxElement = biome_rowan::SyntaxElement<MarkdownLanguage>;
pub type MarkdownSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<MarkdownLanguage>;
pub type MarkdownSyntaxElementChildren = biome_rowan::SyntaxElementChildren<MarkdownLanguage>;
pub type MarkdownSyntaxList = biome_rowan::SyntaxList<MarkdownLanguage>;
