use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, TextRange,
};
use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedImportsOptions {
    /// A list of import paths that should trigger the rule.
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub paths: FxHashMap<Box<str>, CustomRestrictedImport>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedImport {
    /// The message to display when this module is imported.
    Plain(Box<str>),
    /// Additional options to configure the message and allowed/disallowed import names.
    WithOptions(CustomRestrictedImportOptions),
}

impl From<CustomRestrictedImport> for CustomRestrictedImportOptions {
    fn from(options: CustomRestrictedImport) -> Self {
        match options {
            CustomRestrictedImport::Plain(message) => Self {
                message,
                import_names: [].into(),
                allow_import_names: [].into(),
            },
            CustomRestrictedImport::WithOptions(options) => options,
        }
    }
}

impl Deserializable for CustomRestrictedImport {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct CustomRestrictedImportOptions {
    /// The message to display when this module is imported.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub message: Box<str>,

    /// Names of the exported members that should not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub import_names: Box<[Box<str>]>,

    /// Names of the exported members that allowed to be not be used.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub allow_import_names: Box<[Box<str>]>,
}

/// Specifies why a specific import is allowed or disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportRestrictionCause {
    /// Reason: The import source is forbidden or allowed.
    ImportSource,
    /// Reason: A set of forbidden import names has been defined via `importNames`.
    ImportNames,
    /// Reason: A set of allowed import names has been defined via `allowImportNames`.
    AllowImportNames,
}

/// Specifies whether a specific import is (dis)allowed, and why it is allowed/disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportRestrictionStatus(bool, ImportRestrictionCause);

impl ImportRestrictionStatus {
    pub fn is_allowed(&self) -> bool {
        self.0
    }

    pub fn is_forbidden(&self) -> bool {
        !self.0
    }

    pub fn reason(&self) -> ImportRestrictionCause {
        self.1
    }
}

impl CustomRestrictedImportOptions {
    pub fn has_import_name_patterns(&self) -> bool {
        !self.import_names.is_empty() || !self.allow_import_names.is_empty()
    }

    pub fn is_import_allowed(&self, imported_name: &str) -> ImportRestrictionStatus {
        if !self.allow_import_names.is_empty() {
            // Deny all imports except for the names specified in allow_import_names
            let is_allowed = self
                .allow_import_names
                .iter()
                .any(|name| &**name == imported_name);

            ImportRestrictionStatus(is_allowed, ImportRestrictionCause::AllowImportNames)
        } else if !self.import_names.is_empty() {
            // Allow all imports except for the names specified in import_names
            let is_forbidden = self
                .import_names
                .iter()
                .any(|name| &**name == imported_name);

            ImportRestrictionStatus(!is_forbidden, ImportRestrictionCause::ImportNames)
        } else {
            // Deny all imports from this module
            ImportRestrictionStatus(false, ImportRestrictionCause::ImportSource)
        }
    }

    pub fn get_message_for_restriction(
        &self,
        import_source: &str,
        imported_name: &str,
        reason: ImportRestrictionCause,
    ) -> String {
        if !self.message.is_empty() {
            self.message.to_string()
        } else {
            match reason {
                ImportRestrictionCause::ImportSource => {
                    format!("Do not import '{import_source}'.")
                }
                ImportRestrictionCause::ImportNames | ImportRestrictionCause::AllowImportNames => {
                    if imported_name == RestrictedImportVisitor::BARE_IMPORT_ALIAS {
                        format!("Do not import '{import_source}' through a side-effect import.")
                    } else {
                        format!("Do not import '{imported_name}' from '{import_source}'.")
                    }
                }
            }
        }
    }
}

pub struct RestrictedImportVisitor<'a> {
    pub import_source: &'a str,
    pub restricted_import: CustomRestrictedImportOptions,
    pub results: Vec<RestrictedImportMessage>,
}

pub struct RestrictedImportMessage {
    pub location: TextRange,
    pub message: String,
    pub import_source: String,
    pub allowed_import_names: Box<[Box<str>]>,
}

impl RestrictedImportVisitor<'_> {
    pub const BARE_IMPORT_ALIAS: &'static str = "";
    pub const NAMESPACE_IMPORT_ALIAS: &'static str = "*";
    pub const DEFAULT_IMPORT_ALIAS: &'static str = "default";
}
