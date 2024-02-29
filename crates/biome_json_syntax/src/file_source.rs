use biome_rowan::FileSourceError;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct JsonFileSource {
    variant: JsonVariant,
}

#[derive(Debug, Default, Clone)]
enum JsonVariant {
    #[default]
    Standard,
    Jsonc,
}

impl JsonFileSource {
    pub fn json() -> Self {
        Self {
            variant: JsonVariant::Standard,
        }
    }

    pub fn jsonc() -> Self {
        Self {
            variant: JsonVariant::Jsonc,
        }
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
