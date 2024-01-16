//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{HtmlRoot, HtmlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HtmlLanguage;

impl Language for HtmlLanguage {
    type Kind = HtmlSyntaxKind;
    type Root = HtmlRoot;
}

pub type HtmlSyntaxNode = biome_rowan::SyntaxNode<HtmlLanguage>;
pub type HtmlSyntaxToken = biome_rowan::SyntaxToken<HtmlLanguage>;
pub type HtmlSyntaxElement = biome_rowan::SyntaxElement<HtmlLanguage>;
pub type HtmlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<HtmlLanguage>;
pub type HtmlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<HtmlLanguage>;
pub type HtmlSyntaxList = biome_rowan::SyntaxList<HtmlLanguage>;
