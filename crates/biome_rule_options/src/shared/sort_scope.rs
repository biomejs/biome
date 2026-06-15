/// Controls how `sortOrder` and `ignoreCase` interact with `groups`.
///
/// - `global`: `sortOrder` and `ignoreCase` are applied over all props at once,
///   ignoring the `groups` order. This preserves the original flat-sort behavior.
/// - `group`: `sortOrder` and `ignoreCase` are applied independently within each
///   group defined by `groups`. Props are first partitioned by group, sorted
///   within each group, then concatenated in group order.
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
pub enum SortScope {
    /// Apply `sortOrder` and `ignoreCase` globally across all props (default).
    /// Group order is ignored; sorting is flat. Preserves existing behavior.
    #[default]
    Global,
    /// Apply `sortOrder` and `ignoreCase` within each group independently.
    /// Props are partitioned into groups, sorted within each group, then
    /// concatenated in group-index order.
    Group,
}
