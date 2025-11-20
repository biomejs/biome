#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    biome_deserialize_macros::Deserializable,
    biome_deserialize_macros::Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
    #[default]
    Natural,
    Lexicographic,
}
