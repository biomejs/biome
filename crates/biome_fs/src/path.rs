//! This module is responsible to manage paths inside Biome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use std::ffi::OsStr;
use std::fs::{self, read_to_string};
use std::{fs::File, io, io::Write, ops::Deref, path::PathBuf};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct BiomePath {
    path: PathBuf,
}

impl Deref for BiomePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl BiomePath {
    pub fn new(path_to_file: impl Into<PathBuf>) -> Self {
        Self {
            path: path_to_file.into(),
        }
    }

    /// Accepts a file opened in read mode and saves into it
    pub fn save(&mut self, content: &str) -> Result<(), std::io::Error> {
        let mut file_to_write = File::create(&self.path).unwrap();
        // TODO: handle error with diagnostic
        file_to_write.write_all(content.as_bytes())
    }

    /// Returns the contents of a file, if it exists
    pub fn get_buffer_from_file(&mut self) -> String {
        // we assume we have permissions
        fs::read_to_string(&self.path).expect("cannot read the file to format")
    }

    /// Small wrapper for [read_to_string]
    pub fn read_to_string(&self) -> io::Result<String> {
        let path = self.path.as_path();
        read_to_string(path)
    }

    pub fn extension_as_str(&self) -> Option<&str> {
        self.extension().and_then(OsStr::to_str)
    }
}
