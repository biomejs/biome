use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseClassNameHelperOptions {
    /// JSX attribute names to check for template literals.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Preferred helper function names shown in diagnostics.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub helper_functions: Option<Box<[Box<str>]>>,
}

const DEFAULT_ATTRIBUTES: [&str; 2] = ["className", "class"];
const DEFAULT_HELPER_FUNCTIONS: [&str; 5] = ["cn", "clsx", "cva", "cx", "classNames"];

impl UseClassNameHelperOptions {
    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.as_deref().map_or_else(
            || DEFAULT_ATTRIBUTES.contains(&name),
            |attributes| contains_name(attributes, name),
        )
    }

    pub fn helper_functions_for_diagnostic(&self) -> String {
        match self
            .helper_functions
            .as_deref()
            .filter(|helper_functions| !helper_functions.is_empty())
        {
            Some(helper_functions) => helper_functions
                .iter()
                .map(|helper_function| helper_function.as_ref())
                .collect::<Vec<_>>()
                .join(", "),
            _ => DEFAULT_HELPER_FUNCTIONS.join(", "),
        }
    }
}

fn contains_name(names: &[Box<str>], name: &str) -> bool {
    names.iter().any(|entry| entry.as_ref() == name)
}
