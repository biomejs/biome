//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{GraphqlDocument, GraphqlSyntaxKind};
use biome_rowan::Language;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, schemars::JsonSchema))]
pub struct GraphqlLanguage;

impl Language for GraphqlLanguage {
    type Kind = GraphqlSyntaxKind;
    type Root = GraphqlDocument;
}

pub type GraphqlSyntaxNode = biome_rowan::SyntaxNode<GraphqlLanguage>;
pub type GraphqlSyntaxToken = biome_rowan::SyntaxToken<GraphqlLanguage>;
pub type GraphqlSyntaxElement = biome_rowan::SyntaxElement<GraphqlLanguage>;
pub type GraphqlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<GraphqlLanguage>;
pub type GraphqlSyntaxList = biome_rowan::SyntaxList<GraphqlLanguage>;
