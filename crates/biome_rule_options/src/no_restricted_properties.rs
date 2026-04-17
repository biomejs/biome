use biome_console::markup;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic, TextRange,
};
use biome_deserialize_macros::Deserializable;
use biome_unicode_table::is_js_ident;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct NoRestrictedPropertiesOptions {
    /// Restriction entries for object/property access.
    ///
    /// Each entry can describe one of these cases:
    ///
    /// - exact object/property match:
    ///   `{ "object": "require", "property": "ensure" }`
    /// - property-wide restriction with allowed objects:
    ///   `{ "property": "__defineGetter__", "allowObjects": ["Object"] }`
    /// - object-wide restriction with allowed properties:
    ///   `{ "object": "arguments", "allowProperties": ["length"] }`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub entries: Option<Box<[RestrictedPropertyEntry]>>,
}

impl biome_deserialize::Merge for NoRestrictedPropertiesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(entries) = other.entries {
            self.entries = Some(entries);
        }
    }
}

impl DeserializableValidator for NoRestrictedPropertiesOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        let mut is_valid = true;

        for entry in self.entries.iter().flatten() {
            if entry.object.is_none() && entry.property.is_none() {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"'entries[]'"</Emphasis>
                        " must define at least one of 'object' or 'property'."
                    })
                    .with_range(range),
                );
                is_valid = false;
            }

            if entry
                .object
                .as_deref()
                .is_some_and(|name| !is_js_ident(name))
            {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"'entries[].object'"</Emphasis>
                        " must be a valid JavaScript identifier."
                    })
                    .with_range(range),
                );
                is_valid = false;
            }

            if entry.object.is_some() && !entry.allow_objects.is_empty() {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"'entries[].allowObjects'"</Emphasis>
                        " cannot be used together with 'entries[].object'."
                    })
                    .with_range(range),
                );
                is_valid = false;
            }

            if entry.allow_objects.iter().any(|name| !is_js_ident(name)) {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"'entries[].allowObjects'"</Emphasis>
                        " only accepts valid JavaScript identifiers."
                    })
                    .with_range(range),
                );
                is_valid = false;
            }

            if entry.property.is_some() && !entry.allow_properties.is_empty() {
                ctx.report(
                    DeserializationDiagnostic::new(markup! {
                        <Emphasis>"'entries[].allowProperties'"</Emphasis>
                        " cannot be used together with 'entries[].property'."
                    })
                    .with_range(range),
                );
                is_valid = false;
            }
        }

        is_valid
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct RestrictedPropertyEntry {
    /// Object name to restrict.
    ///
    /// Example: `"require"` or `"Object"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub object: Option<Box<str>>,

    /// Property name to restrict.
    ///
    /// Example: `"ensure"` or `"__defineGetter__"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub property: Option<Box<str>>,

    /// Optional custom note appended to the diagnostic.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub message: Option<Box<str>>,

    /// Objects that are allowed when `property` is restricted globally.
    ///
    /// Example:
    /// `{ "property": "__defineGetter__", "allowObjects": ["Object"] }`
    #[serde(skip_serializing_if = "<[_]>::is_empty", default)]
    pub allow_objects: Box<[Box<str>]>,

    /// Properties that are allowed when `object` is restricted globally.
    ///
    /// Example:
    /// `{ "object": "arguments", "allowProperties": ["length"] }`
    #[serde(skip_serializing_if = "<[_]>::is_empty", default)]
    pub allow_properties: Box<[Box<str>]>,
}
