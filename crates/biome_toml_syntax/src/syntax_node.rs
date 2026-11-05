//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{TomlRoot, TomlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TomlLanguage;

impl Language for TomlLanguage {
    type Kind = TomlSyntaxKind;
    type Root = TomlRoot;
}

pub type TomlSyntaxNode = biome_rowan::SyntaxNode<TomlLanguage>;
pub type TomlSyntaxToken = biome_rowan::SyntaxToken<TomlLanguage>;
pub type TomlSyntaxElement = biome_rowan::SyntaxElement<TomlLanguage>;
pub type TomlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<TomlLanguage>;
pub type TomlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<TomlLanguage>;
pub type TomlSyntaxList = biome_rowan::SyntaxList<TomlLanguage>;
pub type TomlSyntaxNodeWithOffset = biome_rowan::SyntaxNodeWithOffset<TomlLanguage>;
