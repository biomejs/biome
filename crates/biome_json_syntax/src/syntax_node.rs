//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{JsonRoot, JsonSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JsonLanguage;

impl Language for JsonLanguage {
    type Kind = JsonSyntaxKind;
    type Root = JsonRoot;
}

pub type JsonSyntaxNode = biome_rowan::SyntaxNode<JsonLanguage>;
pub type JsonSyntaxToken = biome_rowan::SyntaxToken<JsonLanguage>;
pub type JsonSyntaxElement = biome_rowan::SyntaxElement<JsonLanguage>;
pub type JsonSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<JsonLanguage>;
pub type JsonSyntaxElementChildren = biome_rowan::SyntaxElementChildren<JsonLanguage>;
pub type JsonSyntaxList = biome_rowan::SyntaxList<JsonLanguage>;
