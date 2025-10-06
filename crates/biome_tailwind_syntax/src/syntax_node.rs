//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{TailwindSyntaxKind, TwRoot};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TailwindLanguage;

impl Language for TailwindLanguage {
    type Kind = TailwindSyntaxKind;
    type Root = TwRoot;
}

pub type TailwindSyntaxNode = biome_rowan::SyntaxNode<TailwindLanguage>;
pub type TailwindSyntaxToken = biome_rowan::SyntaxToken<TailwindLanguage>;
pub type TailwindSyntaxElement = biome_rowan::SyntaxElement<TailwindLanguage>;
pub type TailwindSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<TailwindLanguage>;
pub type TailwindSyntaxElementChildren = biome_rowan::SyntaxElementChildren<TailwindLanguage>;
pub type TailwindSyntaxList = biome_rowan::SyntaxList<TailwindLanguage>;
