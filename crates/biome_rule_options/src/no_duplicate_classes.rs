use std::ops::Deref;

use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

use crate::use_sorted_classes::UseSortedClassesOptions;

/// Options for the `noDuplicateClasses` assist action.
///
/// Controls which JSX attributes and utility functions are checked for duplicate classes.
#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDuplicateClassesOptions {
    /// Additional attributes that will be checked for duplicate classes.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of the functions or tagged templates that will be checked for duplicate classes.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoDuplicateClassesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(attributes) = other.attributes {
            self.attributes = Some(attributes);
        }
        if let Some(functions) = other.functions {
            self.functions = Some(functions);
        }
    }
}

impl Deref for NoDuplicateClassesOptions {
    type Target = UseSortedClassesOptions;

    fn deref(&self) -> &Self::Target {
        // SAFETY: NoDuplicateClassesOptions has the same memory layout as UseSortedClassesOptions
        // Both structs have identical fields in the same order with the same types.
        // This allows us to reuse UseSortedClassesOptions methods without code duplication.
        unsafe { &*(self as *const NoDuplicateClassesOptions as *const UseSortedClassesOptions) }
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for NoDuplicateClassesOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, NoDuplicateClassesOptionsVisitor, name)
    }
}

struct NoDuplicateClassesOptionsVisitor;

impl DeserializationVisitor for NoDuplicateClassesOptionsVisitor {
    type Output = NoDuplicateClassesOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut result = NoDuplicateClassesOptions::default();

        let mut attributes = Vec::new();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(ctx, &value, &key_text)
                    {
                        attributes.extend::<Vec<Box<str>>>(attributes_option);
                    }
                }
                "functions" => {
                    result.functions = Deserializable::deserialize(ctx, &value, &key_text)
                }
                unknown_key => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }
        result.attributes = if attributes.is_empty() {
            None
        } else {
            Some(attributes.into_boxed_slice())
        };
        Some(result)
    }
}
