use biome_parser::AnyParse;
use biome_rowan::TextRange;

#[derive(Clone, Debug)]
pub struct InjectedSyntax {
    pub syntax: AnyParse,
    pub range_in_host: TextRange,
}
