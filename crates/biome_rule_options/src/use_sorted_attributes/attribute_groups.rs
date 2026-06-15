use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_js_syntax::JsxAttribute;
use biome_rowan::AstNode;

/// The set of attribute groups for `useSortedAttributes`.
///
/// Groups control the position of special prop categories relative to each other
/// when `sortScope` is set to `"group"`.
///
/// ## Predefined groups
///
/// - `:CALLBACK:` — Callback props: names beginning with `on` followed by an uppercase letter (e.g. `onClick`, `onChange`).
/// - `:IMPLICIT:` — Implicit (boolean shorthand) props: props with no value (e.g. `<Foo disabled />`).
/// - `:MULTILINE:` — Props whose value spans multiple lines.
/// - `:RESERVED:` — React reserved props: `key` and `ref`.
/// - `:DOM_RESERVED:` — DOM-only reserved props: `children` and `dangerouslySetInnerHTML`.
///
/// ## Default ordering
///
/// When `groups` is not configured, the following default ordering is used
/// (only active when `sortScope` is `"group"`):
///
/// ```json
/// [":IMPLICIT:", ":RESERVED:", ":DOM_RESERVED:", ":CALLBACK:", ":MULTILINE:"]
/// ```
///
/// Props that don't match any group are placed after all named groups.
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
    /// Returns the index of the first matching group, or `self.0.len()`
    /// (the implicit "rest" bucket) if no group matches.
    pub fn group_index(&self, attr: &JsxAttribute) -> usize {
        self.0
            .iter()
            .position(|group| group.is_match(attr))
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
        AttributeGroupPattern::Callback,
        AttributeGroupPattern::Multiline,
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
    /// Props whose value spans multiple lines.
    #[serde(rename = ":MULTILINE:")]
    Multiline,
    /// React reserved props: `key` and `ref`.
    #[serde(rename = ":RESERVED:")]
    Reserved,
    /// DOM-only reserved props: `children` and `dangerouslySetInnerHTML`.
    #[serde(rename = ":DOM_RESERVED:")]
    DomReserved,
}

impl AttributeGroupPattern {
    pub fn is_match(&self, attr: &JsxAttribute) -> bool {
        match self {
            Self::Callback => is_callback_prop(attr),
            Self::Implicit => is_implicit_prop(attr),
            Self::Multiline => is_multiline_prop(attr),
            Self::Reserved => is_reserved_prop(attr),
            Self::DomReserved => is_dom_reserved_prop(attr),
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
            ":MULTILINE:" => Some(Self::Multiline),
            ":RESERVED:" => Some(Self::Reserved),
            ":DOM_RESERVED:" => Some(Self::DomReserved),
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

/// Returns `true` if the prop's source text contains a newline character.
fn is_multiline_prop(attr: &JsxAttribute) -> bool {
    attr.syntax().text_with_trivia().contains_char('\n')
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
