use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueBlockOrderOptions {
    /// The order of the top-level blocks of a Vue component.
    ///
    /// Each entry is either the tag name of a block, such as `"style"`, or a
    /// list of tag names whose blocks may appear in any order relative to each
    /// other. Blocks whose tag name isn't listed can appear anywhere.
    ///
    /// Defaults to `[["script", "template"], "style"]`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub order: Option<Box<[BlockOrderEntry]>>,
}

impl UseVueBlockOrderOptions {
    pub const DEFAULT_ORDER: &[BlockOrderEntry] = &[
        BlockOrderEntry::Group(Cow::Borrowed(&[
            Cow::Borrowed("script"),
            Cow::Borrowed("template"),
        ])),
        BlockOrderEntry::Tag(Cow::Borrowed("style")),
    ];

    /// Returns [`Self::order`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ORDER`].
    pub fn order(&self) -> &[BlockOrderEntry] {
        self.order.as_deref().unwrap_or(Self::DEFAULT_ORDER)
    }
}

impl biome_deserialize::Merge for UseVueBlockOrderOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(order) = other.order {
            self.order = Some(order);
        }
    }
}

/// One position in the configured block order.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum BlockOrderEntry {
    /// The tag name of a block, such as `"template"`.
    Tag(Cow<'static, str>),
    /// Tag names of blocks that share the same position, so they may appear
    /// in any order relative to each other.
    Group(Cow<'static, [Cow<'static, str>]>),
}

impl BlockOrderEntry {
    /// Returns the tag names at this position.
    pub fn tags(&self) -> &[Cow<'static, str>] {
        match self {
            Self::Tag(tag) => std::slice::from_ref(tag),
            Self::Group(tags) => tags,
        }
    }
}

impl Deserializable for BlockOrderEntry {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Str {
            let tag: String = Deserializable::deserialize(ctx, value, name)?;
            Some(Self::Tag(Cow::Owned(tag)))
        } else {
            let tags: Vec<String> = Deserializable::deserialize(ctx, value, name)?;
            Some(Self::Group(tags.into_iter().map(Cow::Owned).collect()))
        }
    }
}
