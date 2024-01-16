use crate::HtmlLanguage;
use biome_rowan::{FileSource, FileSourceError};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct HtmlFileSource {
    #[allow(unused)]
    variant: HtmlVariant,
}

#[derive(Debug, Default, Clone)]
enum HtmlVariant {
    #[default]
    Standard,
    Astro,
}

impl HtmlFileSource {
    pub fn html() -> Self {
        Self {
            variant: HtmlVariant::Standard,
        }
    }
    pub fn astro() -> Self {
        Self {
            variant: HtmlVariant::Astro,
        }
    }
}

impl<'a> FileSource<'a, HtmlLanguage> for HtmlFileSource {}

impl TryFrom<&Path> for HtmlFileSource {
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

/// It deduce the [HtmlFileSource] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<HtmlFileSource, FileSourceError> {
    Ok(match extension {
        "html" => HtmlFileSource::html(),
        "astro" => HtmlFileSource::astro(),
        _ => {
            return Err(FileSourceError::UnknownExtension(
                file_name.into(),
                extension.into(),
            ));
        }
    })
}
