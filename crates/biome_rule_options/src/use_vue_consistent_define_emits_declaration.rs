use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentDefineEmitsDeclarationOptions {
    /// The style that `defineEmits` calls must use to declare events.
    /// Defaults to `type`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<DefineEmitsDeclarationStyle>,
}

impl UseVueConsistentDefineEmitsDeclarationOptions {
    /// Returns [`Self::style`] if it is set.
    /// Otherwise, returns [`DefineEmitsDeclarationStyle::Type`].
    pub fn style(&self) -> DefineEmitsDeclarationStyle {
        self.style.unwrap_or_default()
    }
}

#[derive(
    Default, Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum DefineEmitsDeclarationStyle {
    /// Events are declared with a type argument, such as
    /// `defineEmits<{ change: [id: number] }>()` or
    /// `defineEmits<{ (e: 'change', id: number): void }>()`.
    #[default]
    Type,
    /// Events are declared with a type argument whose members are all
    /// properties, such as `defineEmits<{ change: [id: number] }>()`.
    TypeLiteral,
    /// Events are declared with an argument, such as
    /// `defineEmits(['change'])`.
    Runtime,
}
