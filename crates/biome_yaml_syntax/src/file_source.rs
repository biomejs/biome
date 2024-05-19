use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[cfg_attr(feature = "schema", derive(schemars::YamlSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct YamlFileSource {
    // ?? Options
}

impl YamlFileSource {
    // Well-known YAML files
    // This list should be SORTED!
    // Source: https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L8081-L8083
    // Note: we shouldn't include machine generated files
    const WELL_KNOWN_YAML_FILES: &'static [&'static str] =
        &[".clang-format", ".clang-tidy", ".gemrc"];

    pub fn yaml() -> Self {
        Self {
            // ?? Options
        }
    }

    pub fn is_well_known_yaml_file(file_name: &str) -> bool {
        Self::WELL_KNOWN_YAML_FILES
            .binary_search(&file_name)
            .is_ok()
    }

    /// Try to return the Yaml file source corresponding to this file name from well-known files
    pub fn try_from_well_known(file_name: &str) -> Result<Self, FileSourceError> {
        if Self::is_well_known_yaml_file(file_name) {
            return Ok(Self::yaml());
        }
        Err(FileSourceError::UnknownFileName(file_name.into()))
    }

    /// Try to return the YAML file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            // https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L8070-L8079
            // https://yaml.org/spec/1.2.2/
            "yaml" | "yml" | "eyaml" | "eyml" | "cff" | "yaml-tmlanguage"
            | "yaml-tmpreferences" | "yaml-tmtheme" | "mir" | "reek" | "rviz"
            | "sublime-syntax" | "syntax" | "yaml.sed" | "yml.mysql" => Ok(Self::yaml()),
            _ => Err(FileSourceError::UnknownExtension(
                Default::default(),
                extension.into(),
            )),
        }
    }

    /// Try to return the YAML file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// The language ID for code snippets is registered by [VS Code built-in extensions]
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "yaml" => Ok(Self::yaml()),
            _ => Err(FileSourceError::UnknownLanguageId(language_id.into())),
        }
    }
}

impl TryFrom<&Path> for YamlFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        if let Ok(file_source) = Self::try_from_well_known(file_name) {
            return Ok(file_source);
        }

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        let extension = &path
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase)
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        Self::try_from_extension(extension)
    }
}

#[test]
fn test_order() {
    for items in YamlFileSource::WELL_KNOWN_YAML_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
