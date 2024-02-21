use biome_rowan::FileSourceError;
use std::path::Path;

#[derive(Debug, Default, Clone, Copy)]
pub struct CssFileSource {
    // Unused until we potentially support postcss/less/sass
    #[allow(unused)]
    variant: CssVariant,
}

/// The style of CSS contained in the file.
///
/// Currently, Biome only supports plain CSS, and aims to be compatible with
/// the latest Recommendation level standards.
#[derive(Debug, Default, Clone, Copy)]
enum CssVariant {
    #[default]
    Standard,
}

impl CssFileSource {
    pub fn css() -> Self {
        Self {
            variant: CssVariant::Standard,
        }
    }
}

impl TryFrom<&Path> for CssFileSource {
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

/// It deduce the [CssFileSource] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<CssFileSource, FileSourceError> {
    let source_type = if file_name.ends_with(".css") {
        CssFileSource::css()
    } else {
        match extension {
            "css" => CssFileSource::css(),
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
