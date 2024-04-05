use biome_rowan::FileSourceError;
use std::path::Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct JsonFileSource {
    allow_trailing_commas: bool,
    allow_comments: bool,
}

impl JsonFileSource {
    pub fn json() -> Self {
        Self {
            allow_trailing_commas: false,
            allow_comments: false,
        }
    }

    pub fn with_allow_trailing_commas(mut self) -> Self {
        self.allow_trailing_commas = true;
        self
    }

    pub fn allow_trailing_commas(&self) -> bool {
        self.allow_trailing_commas
    }

    pub fn with_allow_comments(mut self) -> Self {
        self.allow_comments = true;
        self
    }

    pub fn allow_comments(&self) -> bool {
        self.allow_comments
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
            // We support comments and trailing commas for `.jsonc` files
            // https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L3230-L3246
            "jsonc"
            | "code-snippets"
            | "code-workspace"
            | "sublime-build"
            | "sublime-commands"
            | "sublime-completions"
            | "sublime-keymap"
            | "sublime-macro"
            | "sublime-menu"
            | "sublime-mousemap"
            | "sublime-project"
            | "sublime-settings"
            | "sublime-theme"
            | "sublime-workspace"
            | "sublime_metrics"
            | "sublime_session" => JsonFileSource::json()
                .with_allow_comments()
                .with_allow_trailing_commas(),
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
