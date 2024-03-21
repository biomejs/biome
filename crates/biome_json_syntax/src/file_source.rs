use biome_rowan::FileSourceError;
use std::path::Path;

// TODO: Jsonc is not well-defined, so some files may only support comments
// but no trailing commas. We should reconsider whether to use Jsonc as a variant,
// or just use something like: JsonWithComments, JsonWithCommentsAndTrailingCommas
//
// Currently, the "variant" key and other properties are making things duplicated
// and error-prone.

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct JsonFileSource {
    variant: JsonVariant,
    allow_trailing_commas: bool,
    allow_comments: bool,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum JsonVariant {
    #[default]
    Standard,
    Jsonc,
}

impl JsonFileSource {
    pub fn json() -> Self {
        Self {
            variant: JsonVariant::Standard,
            allow_trailing_commas: false,
            allow_comments: false,
        }
    }

    pub fn jsonc() -> Self {
        Self {
            variant: JsonVariant::Jsonc,
            allow_trailing_commas: true,
            allow_comments: true,
        }
    }

    pub fn with_trailing_commas(mut self, option_value: bool) -> Self {
        self.allow_trailing_commas = option_value;
        self
    }

    pub fn set_allow_trailing_commas(&mut self, option_value: bool) {
        self.allow_trailing_commas = option_value;
    }

    pub fn get_allow_trailing_commas(&self) -> bool {
        self.allow_trailing_commas
    }

    pub fn with_comments(mut self, option_value: bool) -> Self {
        self.allow_comments = option_value;
        self
    }

    pub fn set_allow_comments(&mut self, option_value: bool) {
        self.allow_comments = option_value;
    }

    pub fn get_allow_comments(&self) -> bool {
        self.allow_comments
    }

    pub const fn is_jsonc(&self) -> bool {
        matches!(self.variant, JsonVariant::Jsonc)
    }
}

impl TryFrom<&Path> for JsonFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        let extension = path
            .extension()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        compute_source_type_from_path_or_extension(file_name, extension)
    }
}

/// It deduce the [JsonFileSource] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<JsonFileSource, FileSourceError> {
    let source_type = if file_name.ends_with(".json") {
        JsonFileSource::json()
    } else {
        match extension {
            "json" => JsonFileSource::json(),
            "jsonc" => JsonFileSource::jsonc(),
            _ => {
                return Err(FileSourceError::UnknownExtension(
                    file_name.into(),
                    extension.into(),
                ));
            }
        }
    };
    Ok(source_type)
}
