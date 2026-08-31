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
