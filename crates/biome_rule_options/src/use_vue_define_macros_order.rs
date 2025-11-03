use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueDefineMacrosOrderOptions {
    /// The order of the Vue define macros.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub order: Option<Box<[Box<str>]>>,
}

impl UseVueDefineMacrosOrderOptions {
    pub const DEFAULT_ORDER: &[&str] = &["defineModel", "defineProps", "defineEmits"];

    /// Returns [`Self::order`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ORDER`].
    pub fn order_or_default(&self) -> impl Iterator<Item = &str> {
        let defaults: &[&str] = if self.order.is_none() {
            Self::DEFAULT_ORDER
        } else {
            &[]
        };
        self.order
            .iter()
            .flatten()
            .map(|val| val.as_ref())
            .chain(defaults.iter().copied())
    }
}

impl biome_deserialize::Merge for UseVueDefineMacrosOrderOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(order) = other.order {
            self.order = Some(order);
        }
    }
}
