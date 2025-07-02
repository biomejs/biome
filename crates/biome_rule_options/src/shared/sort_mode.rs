#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize, biome_deserialize_macros::Deserializable,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SortMode {
    #[default]
    Natural,
    Lexicographic,
}
