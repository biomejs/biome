use std::{ffi::OsStr, path::Path};

use biome_rowan::FileSourceError;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GritFileSource {
    #[allow(unused)]
    variant: GritVariant,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
enum GritVariant {
    #[default]
    Standard,
}

impl GritFileSource {
    pub fn grit() -> Self {
        Self {
            variant: GritVariant::Standard,
        }
    }

    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        match extension.as_encoded_bytes() {
            b"grit" => Ok(Self::grit()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }
}

impl TryFrom<&Path> for GritFileSource {
    type Error = FileSourceError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        todo!()
    }
}
