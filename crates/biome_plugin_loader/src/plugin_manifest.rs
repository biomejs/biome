use biome_console::markup;
use biome_deserialize::{DeserializationContext, DeserializationDiagnostic};
use biome_deserialize_macros::Deserializable;
use biome_rowan::TextRange;
use serde::{Deserialize, Serialize};

/// Declares the Grit rules provided by a plugin package or directory.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deserializable(unknown_fields = "deny")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginManifest {
    /// An optional JSON Schema reference used by editors.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// The manifest format version.
    #[deserializable(required, validate = "supported_version")]
    pub version: u8,

    /// Normalized paths to Grit rules, relative to the manifest directory.
    #[deserializable(required)]
    pub rules: Vec<String>,
}

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
