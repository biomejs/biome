//! This module defines the Concrete Syntax Tree for Grit used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{GritRoot, GritSyntaxKind};
use biome_rowan::Language;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GritLanguage;

impl Language for GritLanguage {
    type Kind = GritSyntaxKind;
    type Root = GritRoot;
}

pub type GritSyntaxNode = biome_rowan::SyntaxNode<GritLanguage>;
pub type GritSyntaxToken = biome_rowan::SyntaxToken<GritLanguage>;
pub type GritSyntaxElement = biome_rowan::SyntaxElement<GritLanguage>;
pub type GritSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<GritLanguage>;
pub type GritSyntaxElementChildren = biome_rowan::SyntaxElementChildren<GritLanguage>;
pub type GritSyntaxList = biome_rowan::SyntaxList<GritLanguage>;
pub type GritSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<GritLanguage>;
