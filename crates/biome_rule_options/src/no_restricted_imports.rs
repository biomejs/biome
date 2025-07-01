use crate::organize_imports::import_groups::SourcesMatcher;
use crate::restricted_regex::RestrictedRegex;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, TextRange,
};
use biome_deserialize_macros::Deserializable;
use biome_rowan::{SyntaxNode, SyntaxToken};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedImportsOptions {
    /// A list of import paths that should trigger the rule.
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    pub paths: FxHashMap<Box<str>, Paths>,

    /// gitignore-style patterns that should trigger the rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Box<[Patterns]>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Paths {
    /// The message to display when this module is imported.
    Plain(Box<str>),
    /// Additional options to configure the message and allowed/disallowed import names.
    WithOptions(PathOptions),
}

impl From<Paths> for PathOptions {
    fn from(paths: Paths) -> Self {
        match paths {
            Paths::Plain(message) => Self {
                message,
                import_names: [].into(),
                allow_import_names: [].into(),
            },
            Paths::WithOptions(path_options) => path_options,
        }
    }
}

impl Deserializable for Paths {
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
pub struct PathOptions {
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

impl PathOptions {
    fn has_import_name_constraints(&self) -> bool {
        !self.import_names.is_empty() || !self.allow_import_names.is_empty()
    }

    fn check_restriction(&self, imported_name: &str) -> Restriction {
        // Deny all imports except for the names specified in allow_import_names
        if !self.allow_import_names.is_empty() {
            if self
                .allow_import_names
                .iter()
                .any(|n| n.as_ref() == imported_name)
            {
                Restriction::allowed(Cause::AllowImportNames)
            } else {
                Restriction::forbidden(Cause::AllowImportNames)
            }
        // Allow all imports except for the names specified in import_names
        } else if !self.import_names.is_empty() {
            if self
                .import_names
                .iter()
                .any(|n| n.as_ref() == imported_name)
            {
                Restriction::forbidden(Cause::ImportNames)
            } else {
                Restriction::allowed(Cause::ImportNames)
            }
        } else {
            // Deny all imports from this module
            Restriction::forbidden(Cause::ImportSource)
        }
    }

    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        if !self.message.is_empty() {
            return self.message.to_string();
        }
        default_message(import_source, imported_name, cause)
    }
}

fn default_message(import_source: &str, imported_name: &str, cause: Cause) -> String {
    match cause {
        Cause::ImportSource => format!("Do not import '{import_source}'."),
        Cause::ImportNames | Cause::AllowImportNames => {
            if imported_name == RestrictedImportVisitor::BARE_IMPORT_ALIAS {
                format!("Do not import '{import_source}' through a side-effect import.")
            } else {
                format!("Do not import '{imported_name}' from '{import_source}'.")
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum Patterns {
    WithOptions(PatternOptions),
}

impl From<Patterns> for PatternOptions {
    fn from(patterns: Patterns) -> Self {
        match patterns {
            Patterns::WithOptions(pattern_options) => pattern_options,
        }
    }
}

impl Deserializable for Patterns {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
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
pub struct PatternOptions {
    /// An array of gitignore-style patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<SourcesMatcher>,

    /// A custom message for diagnostics related to this pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Box<str>>,

    /// A regex pattern for import names to forbid within the matched modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    import_name_pattern: Option<RestrictedRegex>,

    /// If true, the matched patterns in the importNamePattern will be allowed. Defaults to `false`.
    invert_import_name_pattern: bool,
}

/// Specifies whether a specific import is (dis)allowed, and why it is allowed/disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Restriction {
    pub allowed: bool,
    pub cause: Cause,
}

impl Restriction {
    pub const fn allowed(cause: Cause) -> Self {
        Self {
            allowed: true,
            cause,
        }
    }
    pub const fn forbidden(cause: Cause) -> Self {
        Self {
            allowed: false,
            cause,
        }
    }
    pub fn is_allowed(self) -> bool {
        self.allowed
    }
    pub fn is_forbidden(self) -> bool {
        !self.allowed
    }
}

/// Specifies why a specific import is allowed or disallowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cause {
    /// Reason: The import source is forbidden or allowed.
    ImportSource,
    /// Reason: A set of forbidden import names has been defined via `importNames`.
    ImportNames,
    /// Reason: A set of allowed import names has been defined via `allowImportNames`.
    AllowImportNames,
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

pub trait ImportRestrictions {
    fn check_restriction(&self, imported_name: &str) -> Restriction;
    fn has_import_name_constraints(&self) -> bool;
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String;
    fn options(&self) -> Options;
}

impl ImportRestrictions for PathOptions {
    fn check_restriction(&self, imported_name: &str) -> Restriction {
        self.check_restriction(imported_name)
    }
    fn has_import_name_constraints(&self) -> bool {
        self.has_import_name_constraints()
    }
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        self.message(import_source, imported_name, cause)
    }
    fn options(&self) -> Options {
        Options::PathOptions(self)
    }
}

pub enum Options<'a> {
    PathOptions(&'a PathOptions),
    PatternOptions(&'a PatternOptions),
}

impl ImportRestrictions for PatternOptions {
    fn check_restriction(&self, imported_name: &str) -> Restriction {
        self.check_restriction(imported_name)
    }
    fn has_import_name_constraints(&self) -> bool {
        self.has_import_name_constraints()
    }
    fn message(&self, import_source: &str, imported_name: &str, cause: Cause) -> String {
        self.message(import_source, imported_name, cause)
    }
    fn options(&self) -> Options {
        Options::PatternOptions(self)
    }
}

pub struct RestrictedImportVisitor<'a> {
    pub import_source: &'a str,
    pub options: Options<'a>,
    pub results: Vec<RestrictedImportMessage>,
}

impl RestrictedImportVisitor<'_> {
    pub const BARE_IMPORT_ALIAS: &'static str = "";
    pub const NAMESPACE_IMPORT_ALIAS: &'static str = "*";
    pub const DEFAULT_IMPORT_ALIAS: &'static str = "default";
}

pub struct RestrictedImportMessage {
    pub location: TextRange,
    pub message: String,
    pub import_source: String,
    pub allowed_import_names: Box<[Box<str>]>,
}

impl RestrictedImportMessage {
    pub fn new(
        token: TextRange,
        import_source: &str,
        message: String,
        allowed_import_names: Box<[Box<str>]>,
    ) -> Self {
        let allowed_names: Box<[Box<str>]> = if allowed_import_names.is_empty() {
            [].into()
        } else {
            allowed_import_names
        };
        Self {
            location: token,
            message,
            import_source: import_source.to_string(),
            allowed_import_names: allowed_names,
        }
    }
}
