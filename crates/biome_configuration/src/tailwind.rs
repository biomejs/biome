use biome_analyze::options::TailwindOptions;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Configures how Biome recognizes Tailwind class strings.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct TailwindConfiguration {
    /// Attribute names whose values contain Tailwind classes.
    ///
    /// Defaults to `class` for HTML and to `class` and `className` for JSX.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,

    /// Function and tagged-template names whose arguments contain Tailwind classes.
    ///
    /// Defaults to `clsx`, `tw`, `twMerge`, `twJoin`, `cva`, `tv`, `cn`, `cc`,
    /// `cnb`, and `ctl`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}

impl From<TailwindConfiguration> for TailwindOptions {
    fn from(configuration: TailwindConfiguration) -> Self {
        Self::new(configuration.attributes, configuration.functions)
    }
}
