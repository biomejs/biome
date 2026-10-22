use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentVOnHandlerStyleOptions {
    /// The handler style to enforce in `v-on` directives.
    /// If omitted, `"methodOrInlineFunction"` is used.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<VueVOnHandlerStyle>,
}

/// A style of event handler in a `v-on` directive.
#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VueVOnHandlerStyle {
    /// Use the name of a method, as in `@click="handler"`. Where a method name can't be used, use
    /// an inline function, as in `@click="() => handler(item)"`.
    #[default]
    MethodOrInlineFunction,
    /// Use the name of a method, as in `@click="handler"`. Where a method name can't be used, use
    /// an inline handler, as in `@click="handler(item)"`.
    MethodOrInline,
    /// Use an inline function, as in `@click="() => handler()"`.
    InlineFunction,
    /// Use an inline handler, as in `@click="handler()"`.
    Inline,
}

impl UseVueConsistentVOnHandlerStyleOptions {
    pub const DEFAULT_STYLE: VueVOnHandlerStyle = VueVOnHandlerStyle::MethodOrInlineFunction;

    /// Returns [`Self::style`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_STYLE`].
    pub fn style(&self) -> VueVOnHandlerStyle {
        self.style.unwrap_or(Self::DEFAULT_STYLE)
    }
}
