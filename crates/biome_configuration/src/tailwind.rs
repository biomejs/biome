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
    /// Defaults to `class` and `className`. HTML attribute names are matched
    /// case-insensitively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,

    /// Functions and tagged templates whose arguments contain Tailwind classes,
    /// such as `clsx` and `cn`.
    ///
    /// String arguments are class strings. In object arguments, the keys are
    /// class strings, as in `clsx({ "px-2": isActive })`. Member expressions
    /// rooted at a listed name are also recognized, so `tw` covers
    /// `` tw.div`...` `` and `tw.div("...")`.
    ///
    /// Defaults to `clsx`, `tw`, `twMerge`, `twJoin`, `cn`, `cc`, `cnb`, and
    /// `ctl`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_functions: Option<Box<[Box<str>]>>,

    /// Functions whose object arguments contain Tailwind classes as values,
    /// such as `cva` and `tv`.
    ///
    /// String arguments are class strings. Object arguments are read as variant
    /// configurations: classes are read from `base`, `slots`, `class`,
    /// `className`, and `variants`, and from the `class` and `className` of
    /// each `compoundVariants` and `compoundSlots` entry. Other keys, such as
    /// `defaultVariants`, are ignored.
    ///
    /// Defaults to `cva` and `tv`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_functions: Option<Box<[Box<str>]>>,
}

impl From<TailwindConfiguration> for TailwindOptions {
    fn from(configuration: TailwindConfiguration) -> Self {
        Self::new(
            configuration.attributes,
            configuration.merge_functions,
            configuration.variant_functions,
        )
    }
}
