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
pub struct NoJsRestrictedPropertiesOptions {
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

impl biome_deserialize::Merge for NoJsRestrictedPropertiesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(entries) = other.entries {
            self.entries = Some(entries);
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
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

impl DeserializableValidator for RestrictedPropertyEntry {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        let mut is_valid = true;

        if self.object.is_none() && self.property.is_none() {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    <Emphasis>"'entries[]'"</Emphasis>
                    " must define at least one of 'object' or 'property'."
                })
                .with_range(range)
                .with_note(markup! {
                    "Add "<Emphasis>"object"</Emphasis>", "<Emphasis>"property"</Emphasis>", or both to this entry."
                }),
            );
            is_valid = false;
        }

        if let Some(name) = self.object.as_deref().filter(|name| !is_js_ident(name)) {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    <Emphasis>"\""{name}"\""</Emphasis>
                    " isn't a valid identifier."
                })
                .with_range(range)
                .with_note(markup! {
                    "Valid identifiers must start with a letter, underscore, or dollar sign, and can only contain letters, digits, underscores, or dollar signs."
                })
                .with_note(markup! {
                    "Use a plain identifier, such as "<Emphasis>"\"Foo\""</Emphasis>", for "<Emphasis>"object"</Emphasis>"."
                }),
            );
            is_valid = false;
        }

        if self.object.is_some() && !self.allow_objects.is_empty() {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    <Emphasis>"'entries[].allowObjects'"</Emphasis>
                    " cannot be used together with 'entries[].object'."
                })
                .with_range(range)
                .with_note(markup! {
                    "Remove either "<Emphasis>"object"</Emphasis>" or "<Emphasis>"allowObjects"</Emphasis>" from this entry."
                }),
            );
            is_valid = false;
        }

        if let Some(name) = self.allow_objects.iter().find(|name| !is_js_ident(name)) {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    <Emphasis>"\""{name}"\""</Emphasis>
                    " isn't a valid identifier in "<Emphasis>"allowObjects"</Emphasis>"."
                })
                .with_range(range)
                .with_note(markup! {
                    "Use plain identifiers, such as "<Emphasis>"\"Object\""</Emphasis>", in "<Emphasis>"allowObjects"</Emphasis>"."
                }),
            );
            is_valid = false;
        }

        if self.property.is_some() && !self.allow_properties.is_empty() {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    <Emphasis>"'entries[].allowProperties'"</Emphasis>
                    " cannot be used together with 'entries[].property'."
                })
                .with_range(range)
                .with_note(markup! {
                    "Remove either "<Emphasis>"property"</Emphasis>" or "<Emphasis>"allowProperties"</Emphasis>" from this entry."
                }),
            );
            is_valid = false;
        }

        is_valid
    }
}
