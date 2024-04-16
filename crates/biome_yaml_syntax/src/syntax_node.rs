use crate::{YamlRoot, YamlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct YamlLanguage;

impl Language for YamlLanguage {
    type Kind = YamlSyntaxKind;
    type Root = YamlRoot;
}

pub type YamlSyntaxNode = biome_rowan::SyntaxNode<YamlLanguage>;
pub type YamlSyntaxToken = biome_rowan::SyntaxToken<YamlLanguage>;
pub type YamlSyntaxElement = biome_rowan::SyntaxElement<YamlLanguage>;
pub type YamlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<YamlLanguage>;
pub type YamlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<YamlLanguage>;
pub type YamlSyntaxList = biome_rowan::SyntaxList<YamlLanguage>;
