//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{PhpRoot, PhpSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PhpLanguage;

impl Language for PhpLanguage {
    type Kind = PhpSyntaxKind;
    type Root = PhpRoot;
}

pub type PhpSyntaxNode = biome_rowan::SyntaxNode<PhpLanguage>;
pub type PhpSyntaxToken = biome_rowan::SyntaxToken<PhpLanguage>;
pub type PhpSyntaxElement = biome_rowan::SyntaxElement<PhpLanguage>;
pub type PhpSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<PhpLanguage>;
pub type PhpSyntaxElementChildren = biome_rowan::SyntaxElementChildren<PhpLanguage>;
pub type PhpSyntaxList = biome_rowan::SyntaxList<PhpLanguage>;