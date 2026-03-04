//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{MarkdownSyntaxKind, MdDocument};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MdLanguage;

impl Language for MdLanguage {
    type Kind = MarkdownSyntaxKind;
    type Root = MdDocument;
}

pub type MdSyntaxNode = biome_rowan::SyntaxNode<MdLanguage>;
pub type MdSyntaxToken = biome_rowan::SyntaxToken<MdLanguage>;
pub type MdSyntaxElement = biome_rowan::SyntaxElement<MdLanguage>;
pub type MdSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<MdLanguage>;
pub type MdSyntaxElementChildren = biome_rowan::SyntaxElementChildren<MdLanguage>;
pub type MdSyntaxList = biome_rowan::SyntaxList<MdLanguage>;
