use biome_rowan::Language;

use crate::GlimmerSyntaxKind;

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

use crate::GlimmerRoot;
