use biome_analyze::options::TailwindOptions;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Tells Biome where your code lists Tailwind CSS classes, so the Tailwind
/// lint rules know which strings to check.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct TailwindConfiguration {
    /// Attributes whose values are Tailwind classes, such as `class` in
    /// `<div class="px-2 py-2">`.
    ///
    /// Defaults to `class` and `className`. In HTML, they are case-insensitive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,

    /// Functions that combine Tailwind classes, such as `clsx` and `cn`.
    ///
    /// Biome checks every string passed to these functions, like both strings
    /// in `cn("px-2", "py-2")`. When an object is passed, Biome checks its
    /// keys: in `cn({ "px-2": isActive })`, it checks `"px-2"`.
    ///
    /// Properties of these functions are checked too, so listing `tw` also
    /// covers `` tw.div`px-2` `` and `tw.div("px-2")`.
    ///
    /// Defaults to `clsx`, `tw`, `twMerge`, `twJoin`, `cn`, `cc`, `cnb`, and
    /// `ctl`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_functions: Option<Box<[Box<str>]>>,

    /// Functions that describe a component's styles with an object, such as
    /// `cva` and `tv`.
    ///
    /// Biome checks every string passed directly to these functions. In an
    /// object passed to them, Biome checks the classes under these properties:
    ///
    /// - `base`, `class`, and `className`: classes that always apply.
    /// - `variants`: classes for each version of the component, such as each
    ///   button size.
    /// - `slots`: classes for each part of the component, such as its icon.
    /// - `compoundVariants` and `compoundSlots`: only the `class` and
    ///   `className` of each entry.
    ///
    /// Other properties, such as `defaultVariants`, hold names rather than
    /// classes, so Biome skips them. For example, Biome checks `"rounded"`,
    /// `"px-2"`, and `"px-4"` here, but not `"small"`:
    ///
    /// ```js
    /// cva("rounded", {
    ///     variants: {
    ///         size: { small: "px-2", large: "px-4" },
    ///     },
    ///     defaultVariants: { size: "small" },
    /// });
    /// ```
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
