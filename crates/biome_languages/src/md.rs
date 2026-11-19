use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum MarkdownVariant {
    #[default]
    Standard,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct MdFileSource {
    variant: MarkdownVariant,

    /// Whether bold, italic, and strikethrough markers follow the
    /// [CommonMark CJK-friendly amendments](https://github.com/tats-u/markdown-cjk-friendly/blob/main/specification.md).
    /// Biome sets it from the `markdown.parser.cjkFriendlyEmphasis` configuration when it opens
    /// the file, replacing any value sent by the client.
    #[serde(default)]
    cjk_friendly_emphasis: bool,
}

impl MdFileSource {
    pub fn markdown() -> Self {
        Self {
            variant: MarkdownVariant::Standard,
            cjk_friendly_emphasis: false,
        }
    }

    /// Returns whether bold, italic, and strikethrough markers follow the CommonMark
    /// CJK-friendly amendments. The amendments change the syntax tree, so the parser,
    /// the formatter, and the analyzer read this flag from the same file source.
    pub const fn cjk_friendly_emphasis(&self) -> bool {
        self.cjk_friendly_emphasis
    }

    pub fn set_cjk_friendly_emphasis(&mut self, cjk_friendly_emphasis: bool) {
        self.cjk_friendly_emphasis = cjk_friendly_emphasis;
    }

    /// Returns a possible file extension for this source without a leading dot.
    ///
    /// ## Warning
    ///
    /// Don't use this function to write files on disk, as it might support "multiple extensions for the same file"
    pub const fn file_extension(&self) -> &'static str {
        "md"
    }

    /// Try to return the Markdown file source corresponding to this file name from well-known files
    pub fn try_from_well_known(_: &Utf8Path) -> Result<Self, FileSourceError> {
        Err(FileSourceError::UnknownFileName)
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        match extension {
            "md" | "markdown" => Ok(Self::markdown()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "markdown" => Ok(Self::markdown()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for MdFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Utf8Path) -> Result<Self, Self::Error> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let Some(extension) = path.extension() else {
            return Err(FileSourceError::MissingFileExtension);
        };
        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        Self::try_from_extension(&extension.to_ascii_lowercase_cow())
    }
}
