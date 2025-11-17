use crate::restricted_regex::RestrictedRegex;
use biome_console::markup;
use biome_deserialize::{DeserializationContext, TextRange};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_string_case::{Case, Cases};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::str::FromStr;

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseFilenamingConventionOptions {
    /// If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
    /// This does not affect other [Case].
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub strict_case: Option<bool>,

    /// If `false`, then non-ASCII characters are allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub require_ascii: Option<bool>,

    /// Regular expression to enforce
    #[serde(rename = "match", skip_serializing_if = "Option::<_>::is_none")]
    pub matching: Option<RestrictedRegex>,

    /// Allowed cases for file names.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub filename_cases: Option<FilenameCases>,
}

impl UseFilenamingConventionOptions {
    pub const DEFAULT_STRICT_CASE: bool = true;
    pub const DEFAULT_REQUIRE_ASCII: bool = true;

    /// Returns [`Self::strict_case`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_STRICT_CASE`].
    pub fn strict_case(&self) -> bool {
        self.strict_case.unwrap_or(Self::DEFAULT_STRICT_CASE)
    }

    /// Returns [`Self::require_ascii`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_REQUIRE_ASCII`].
    pub fn require_ascii(&self) -> bool {
        self.require_ascii.unwrap_or(Self::DEFAULT_REQUIRE_ASCII)
    }
}

#[derive(
    Clone, Copy, Debug, Deserializable, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[serde(
    from = "SmallVec<[FilenameCase; 5]>",
    into = "SmallVec<[FilenameCase; 5]>"
)]
#[deserializable(with_validator)]
pub struct FilenameCases {
    pub cases: Cases,
    /// `true` is the filename can be equal to the name of an export.
    pub allow_export: bool,
}
impl From<SmallVec<[FilenameCase; 5]>> for FilenameCases {
    fn from(values: SmallVec<[FilenameCase; 5]>) -> Self {
        Self::from_iter(values)
    }
}
impl FromIterator<FilenameCase> for FilenameCases {
    fn from_iter<T: IntoIterator<Item = FilenameCase>>(values: T) -> Self {
        let mut result = Self {
            cases: Cases::empty(),
            allow_export: false,
        };
        for filename_case in values {
            if let Ok(case) = Case::try_from(filename_case) {
                result.cases |= case;
            } else {
                result.allow_export = true;
            }
        }
        result
    }
}
impl From<FilenameCases> for SmallVec<[FilenameCase; 5]> {
    fn from(value: FilenameCases) -> Self {
        let maybe_export = if value.allow_export {
            &[FilenameCase::Export][..]
        } else {
            &[]
        };
        value
            .cases
            .into_iter()
            .filter_map(|case| FilenameCase::try_from(case).ok())
            .chain(maybe_export.iter().copied())
            .collect()
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for FilenameCases {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("FilenameCases")
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        <std::collections::HashSet<FilenameCase>>::json_schema(generator)
    }
}
impl Default for FilenameCases {
    fn default() -> Self {
        Self {
            cases: Case::Camel | Case::Kebab | Case::Snake,
            allow_export: true,
        }
    }
}

impl biome_deserialize::DeserializableValidator for FilenameCases {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        name: &str,
        range: TextRange,
    ) -> bool {
        if !self.allow_export && self.cases.is_empty() {
            ctx.report(
                biome_deserialize::DeserializationDiagnostic::new(markup! {
                    ""<Emphasis>{name}</Emphasis>" cannot be an empty array."
                })
                .with_range(range),
            );
            false
        } else {
            true
        }
    }
}

impl biome_deserialize::Merge for FilenameCases {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

/// Supported cases for file names.
#[derive(
    Clone, Copy, Debug, serde::Deserialize, Deserializable, Eq, Hash, PartialEq, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FilenameCase {
    /// camelCase
    #[serde(rename = "camelCase")]
    Camel,

    /// Match an export name
    #[serde(rename = "export")]
    Export,

    /// kebab-case
    #[serde(rename = "kebab-case")]
    Kebab,

    /// PascalCase
    #[serde(rename = "PascalCase")]
    Pascal,

    /// snake_case
    #[serde(rename = "snake_case")]
    Snake,
}
impl FilenameCase {
    pub const ALLOWED_VARIANTS: &'static [&'static str] = &[
        "camelCase",
        "export",
        "kebab-case",
        "PascalCase",
        "snake_case",
    ];
}
impl FromStr for FilenameCase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camelCase" => Ok(Self::Camel),
            "export" => Ok(Self::Export),
            "kebab-case" => Ok(Self::Kebab),
            "PascalCase" => Ok(Self::Pascal),
            "snake_case" => Ok(Self::Snake),
            _ => Err("Value not supported for enum member case"),
        }
    }
}
impl TryFrom<FilenameCase> for Case {
    type Error = &'static str;

    fn try_from(case: FilenameCase) -> Result<Self, Self::Error> {
        match case {
            FilenameCase::Camel => Ok(Self::Camel),
            FilenameCase::Export => Err("`export` is not a valid case"),
            FilenameCase::Kebab => Ok(Self::Kebab),
            FilenameCase::Pascal => Ok(Self::Pascal),
            FilenameCase::Snake => Ok(Self::Snake),
        }
    }
}
impl TryFrom<Case> for FilenameCase {
    type Error = &'static str;

    fn try_from(value: Case) -> Result<Self, Self::Error> {
        match value {
            Case::Camel => Ok(Self::Camel),
            Case::Kebab => Ok(Self::Kebab),
            Case::Pascal => Ok(Self::Pascal),
            Case::Snake => Ok(Self::Snake),
            Case::Constant
            | Case::Lower
            | Case::Number
            | Case::NumberableCapital
            | Case::Uni
            | Case::Upper
            | Case::Unknown => Err("Unsupported case"),
        }
    }
}
