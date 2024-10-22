use biome_console::markup;
use biome_deserialize::{DeserializationContext, DeserializationDiagnostic};
use biome_deserialize_macros::Deserializable;
use biome_rowan::TextRange;

use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserializable, Eq, PartialEq)]
pub struct PluginManifest {
    #[deserializable(required, validate = "supported_version")]
    pub version: u8,

    pub rules: Vec<PathBuf>,
}

// There's only one manifest version now.
pub fn supported_version(
    ctx: &mut impl DeserializationContext,
    value: &u8,
    name: &str,
    range: TextRange,
) -> bool {
    if *value == 1 {
        true
    } else {
        ctx.report(
            DeserializationDiagnostic::new(markup! {
                <Emphasis>{name}</Emphasis>" must be 1"
            })
            .with_range(range),
        );
        false
    }
}
