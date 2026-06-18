pub use crate::shared::sort_order::SortOrder;
use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_js_syntax::JsxAttribute;
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedAttributesOptions {
    /// Sort order to apply within each group (or globally when `sortScope` is `"global"`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_order: Option<SortOrder>,

    /// Groups control the ordering of special prop categories.
    ///
    /// Only active when `sortScope` is `"group"`.
    /// When not configured, the default ordering is used:
    /// `[":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":REST:", ":CALLBACK:"]`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<AttributeGroups>,

    /// Whether to ignore case when comparing prop names.
    ///
    /// When `sortScope` is `"global"`, applied to the flat sort.
    /// When `sortScope` is `"group"`, applied independently within each group.
    ///
    /// Defaults to `false` (case-sensitive, preserving current behavior).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_case: Option<bool>,

    /// Controls how `sortOrder` and `ignoreCase` interact with `groups`.
    ///
    /// - `"global"` (default): flat sort across all props, groups ignored.
    /// - `"group"`: sort within each group independently, then order by group.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub sort_scope: Option<SortScope>,

    /// Controls how multiline props are ordered relative to single-line props.
    ///
    /// Only meaningful when `sortScope` is `"group"`. Defaults to `"group"` (mixed).
    ///
    /// - `"group"`: multiline props are sorted together with single-line props in their group.
    /// - `"groupFirst"`: within each group, multiline props are placed before single-line props.
    /// - `"groupLast"`: within each group, multiline props are placed after single-line props.
    /// - `"first"`: all multiline props are collected across all groups, sorted by group order,
    ///   and placed before all single-line groups.
    /// - `"last"`: all multiline props are collected across all groups, sorted by group order,
    ///   and placed after all single-line groups.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub multiline: Option<MultilineOrder>,
}

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

/// Controls where multiline props land relative to single-line props.
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
pub enum MultilineOrder {
    /// Multiline props are mixed with single-line props in their group (default).
    #[default]
    Group,
    /// Within each group, multiline props are sorted before single-line props.
    GroupFirst,
    /// Within each group, multiline props are sorted after single-line props.
    GroupLast,
    /// All multiline props are collected, sorted by group order, and placed before all non-multiline groups.
    First,
    /// All multiline props are collected, sorted by group order, and placed after all non-multiline groups.
    Last,
}

/// The set of attribute groups for `useSortedAttributes`.
///
/// Groups control the position of special prop categories relative to each other
/// when `sortScope` is set to `"group"`.
///
/// ## Predefined groups
///
/// - `:CALLBACK:` — Callback props: names beginning with `on` followed by an uppercase letter (e.g. `onClick`, `onChange`).
/// - `:IMPLICIT:` — Implicit (boolean shorthand) props: props with no value (e.g. `<Foo disabled />`).
/// - `:RESERVED:` — React reserved props: `key` and `ref`.
/// - `:DOM_RESERVED:` — DOM-only reserved props: `children` and `dangerouslySetInnerHTML`.
/// - `:REST:` — Catch-all for props that don't match any other configured group.
///   Sorted like a regular group, using `sortOrder` and `ignoreCase`.
///
/// ## Default ordering
///
/// When `groups` is not configured, the following default ordering is used
/// (only active when `sortScope` is `"group"`):
///
/// ```json
/// [":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":REST:", ":CALLBACK:"]
/// ```
///
/// If `:REST:` is omitted from a configured `groups` list, props that don't
/// match any named group are instead placed after all named groups, in their
/// original relative order (unsorted).
///
/// Multiline prop positioning is controlled separately via the `multiline` option.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct AttributeGroups(Box<[AttributeGroupPattern]>);

impl Deserializable for AttributeGroups {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Deserializable::deserialize(ctx, value, name).map(Self)
    }
}

impl AttributeGroups {
    /// Returns `true` if no explicit group is configured.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of explicitly configured groups.
    ///
    /// This is also the index of the implicit "rest" bucket used for props
    /// that don't match any named group.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the group index for the given attribute.
    ///
    /// Returns the index of the first explicitly matching group. If no group
    /// matches, returns the index of the configured `:REST:` group, if any.
    /// Otherwise, returns `self.0.len()` (the implicit unsorted "rest"
    /// bucket appended after all named groups).
    pub fn group_index(&self, attr: &JsxAttribute) -> usize {
        if let Some(pos) = self.0.iter().position(|group| group.is_match(attr)) {
            return pos;
        }
        self.0
            .iter()
            .position(|group| matches!(group, AttributeGroupPattern::Rest))
            .unwrap_or(self.0.len())
    }
}

impl biome_deserialize::Merge for AttributeGroups {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

/// The default group ordering used when `sortScope` is `"group"` but no
/// explicit `groups` configuration is provided.
pub fn default_attribute_groups() -> AttributeGroups {
    AttributeGroups(Box::new([
        AttributeGroupPattern::Implicit,
        AttributeGroupPattern::Reserved,
        AttributeGroupPattern::DomReserved,
        AttributeGroupPattern::Rest,
        AttributeGroupPattern::Callback,
    ]))
}

/// A predefined attribute group pattern.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum AttributeGroupPattern {
    /// Callback props: names beginning with `on` followed by an uppercase letter.
    #[serde(rename = ":CALLBACK:")]
    Callback,
    /// Implicit (boolean shorthand) props: props with no value.
    #[serde(rename = ":IMPLICIT:")]
    Implicit,
    /// React reserved props: `key` and `ref`.
    #[serde(rename = ":RESERVED:")]
    Reserved,
    /// DOM-only reserved props: `children` and `dangerouslySetInnerHTML`.
    #[serde(rename = ":DOM_RESERVED:")]
    DomReserved,
    /// Catch-all for props that don't match any other configured group.
    ///
    /// Unlike the other patterns, this never matches directly through
    /// [`Self::is_match`]: its position is resolved separately by
    /// [`AttributeGroups::group_index`] as a fallback for unmatched props.
    #[serde(rename = ":REST:")]
    Rest,
}

impl AttributeGroupPattern {
    pub fn is_match(&self, attr: &JsxAttribute) -> bool {
        match self {
            Self::Callback => is_callback_prop(attr),
            Self::Implicit => is_implicit_prop(attr),
            Self::Reserved => is_reserved_prop(attr),
            Self::DomReserved => is_dom_reserved_prop(attr),
            // Resolved separately in `AttributeGroups::group_index`.
            Self::Rest => false,
        }
    }
}

impl Deserializable for AttributeGroupPattern {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let text = Text::deserialize(ctx, value, name)?;
        match text.text() {
            ":CALLBACK:" => Some(Self::Callback),
            ":IMPLICIT:" => Some(Self::Implicit),
            ":RESERVED:" => Some(Self::Reserved),
            ":DOM_RESERVED:" => Some(Self::DomReserved),
            ":REST:" => Some(Self::Rest),
            _ => {
                // TODO: emit a proper deserialization diagnostic once the API stabilizes
                None
            }
        }
    }
}

impl biome_deserialize::Merge for AttributeGroupPattern {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

// ── Prop classification helpers ──────────────────────────────────────────────

/// Returns `true` if the prop is a callback: name starts with `on` followed
/// by an uppercase ASCII letter (e.g. `onClick`, `onChange`).
fn is_callback_prop(attr: &JsxAttribute) -> bool {
    let Ok(name) = attr.name() else {
        return false;
    };
    let Ok(name) = name.name() else {
        return false;
    };
    let text = name.text_trimmed();
    let bytes = text.as_bytes();
    bytes.len() > 2 && bytes[0] == b'o' && bytes[1] == b'n' && bytes[2].is_ascii_uppercase()
}

/// Returns `true` if the prop is implicit (boolean shorthand): has no value.
fn is_implicit_prop(attr: &JsxAttribute) -> bool {
    attr.initializer().is_none()
}

/// Returns `true` if the prop itself spans multiple lines (i.e. its value
/// contains a newline).
///
/// Uses `text_trimmed()` rather than `text_with_trivia()` so that leading and
/// trailing formatting trivia is excluded: in normally-formatted JSX every prop
/// sits on its own line and would otherwise be misclassified as multiline.
pub fn is_multiline_prop(attr: &JsxAttribute) -> bool {
    attr.syntax().text_trimmed().contains_char('\n')
}

const RESERVED_PROPS: [&str; 2] = ["key", "ref"];
const DOM_RESERVED_PROPS: [&str; 2] = ["children", "dangerouslySetInnerHTML"];

/// Returns `true` if the prop is a React reserved prop (`key` or `ref`).
fn is_reserved_prop(attr: &JsxAttribute) -> bool {
    prop_name_in(attr, &RESERVED_PROPS)
}

/// Returns `true` if the prop is a DOM-only reserved prop
/// (`children` or `dangerouslySetInnerHTML`).
fn is_dom_reserved_prop(attr: &JsxAttribute) -> bool {
    prop_name_in(attr, &DOM_RESERVED_PROPS)
}

fn prop_name_in(attr: &JsxAttribute, list: &[&str]) -> bool {
    let Ok(name) = attr.name() else {
        return false;
    };
    let Ok(name) = name.name() else {
        return false;
    };
    let text = name.text_trimmed();
    list.contains(&text)
}
