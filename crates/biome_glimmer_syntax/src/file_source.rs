use biome_rowan::FileSourceError;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GlimmerFileSource {
    // For now, we don't need variants
    // In the future, could add strict/loose mode, etc.
}

impl GlimmerFileSource {
    pub fn glimmer() -> Self {
        Self::default()
    }

    /// Try to return the Glimmer file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        // Glimmer templates are always embedded in .gjs/.gts files
        // This is called when we extract template content
        match extension {
            "glimmer" | "hbs" | "handlebars" => Ok(Self::glimmer()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    /// Try to return the Glimmer file source corresponding to this language ID
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "glimmer" | "handlebars" => Ok(Self::glimmer()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}
