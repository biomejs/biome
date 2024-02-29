//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{AnyJsRoot, JsSyntaxKind};
use biome_rowan::Language;
#[cfg(feature = "schema")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "schema", derive(Serialize, schemars::JsonSchema))]
pub struct JsLanguage;

impl Language for JsLanguage {
    type Kind = JsSyntaxKind;
    type Root = AnyJsRoot;
}

pub type JsSyntaxNode = biome_rowan::SyntaxNode<JsLanguage>;
pub type JsSyntaxToken = biome_rowan::SyntaxToken<JsLanguage>;
pub type JsSyntaxElement = biome_rowan::SyntaxElement<JsLanguage>;
pub type JsSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<JsLanguage>;
pub type JsSyntaxElementChildren = biome_rowan::SyntaxElementChildren<JsLanguage>;
pub type JsSyntaxList = biome_rowan::SyntaxList<JsLanguage>;
pub type JsSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<JsLanguage>;
