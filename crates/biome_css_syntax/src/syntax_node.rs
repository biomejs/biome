//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{CssRoot, CssSyntaxKind};
use rome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CssLanguage;

impl Language for CssLanguage {
    type Kind = CssSyntaxKind;
    type Root = CssRoot;
}

pub type CssSyntaxNode = rome_rowan::SyntaxNode<CssLanguage>;
pub type CssSyntaxToken = rome_rowan::SyntaxToken<CssLanguage>;
pub type CssSyntaxElement = rome_rowan::SyntaxElement<CssLanguage>;
pub type CssSyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<CssLanguage>;
pub type CssSyntaxElementChildren = rome_rowan::SyntaxElementChildren<CssLanguage>;
pub type CssSyntaxList = rome_rowan::SyntaxList<CssLanguage>;
