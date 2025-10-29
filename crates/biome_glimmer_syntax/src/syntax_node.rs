//! This module defines the Concrete Syntax Tree for Glimmer templates.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{GlimmerRoot, GlimmerSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GlimmerLanguage;

impl Language for GlimmerLanguage {
    type Kind = GlimmerSyntaxKind;
    type Root = GlimmerRoot;
}

pub type GlimmerSyntaxNode = biome_rowan::SyntaxNode<GlimmerLanguage>;
pub type GlimmerSyntaxToken = biome_rowan::SyntaxToken<GlimmerLanguage>;
pub type GlimmerSyntaxElement = biome_rowan::SyntaxElement<GlimmerLanguage>;
pub type GlimmerSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<GlimmerLanguage>;
pub type GlimmerSyntaxElementChildren = biome_rowan::SyntaxElementChildren<GlimmerLanguage>;
pub type GlimmerSyntaxList = biome_rowan::SyntaxList<GlimmerLanguage>;
