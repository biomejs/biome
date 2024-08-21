//! This module is responsible to manage paths inside Biome.
//! It is a small wrapper around [path::PathBuf] but it is also able to
//! give additional information around the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use crate::ConfigName;
use enumflags2::{bitflags, BitFlags};
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::{fs::File, io, io::Write, ops::Deref, path::PathBuf};

/// The priority of the file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[bitflags]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileKindFlag {
    /// A configuration file has the highest priority. It's usually `biome.json` and `biome.jsonc`
    ///
    /// Other third-party configuration files might be added in the future
    Config,
    /// It's usually `package.json` and `tsconfig.json`
    Manifest,
    /// Files that are required to be inspected before handling other files.
    ///
    /// An example is the GraphQL schema
    ToInspect,
    /// A file to handle has the lowest priority. It's usually a traversed file, or a file opened by the LSP
    ToHandle,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileKind(BitFlags<FileKindFlag>);

impl From<FileKindFlag> for FileKind {
    fn from(flag: FileKindFlag) -> Self {
        Self(BitFlags::from(flag))
    }
}

#[derive(Debug, Clone, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
pub struct BiomePath {
    path: PathBuf,
    kind: FileKind,
    is_fixed: bool,
}

impl BiomePath {
    pub fn new(path_to_file: impl Into<PathBuf>) -> Self {
        let path = path_to_file.into();
        let kind = Self::priority(path.file_name().and_then(OsStr::to_str).unwrap_or(""));
        Self {
            path,
            kind,
            is_fixed: false,
        }
    }

    pub fn new_fixed(path_to_file: impl Into<PathBuf>) -> Self {
        let path = path_to_file.into();
        let kind = Self::priority(path.file_name().and_then(OsStr::to_str).unwrap_or(""));
        Self {
            path,
            kind,
            is_fixed: true,
        }
    }

    /// Creates a new [BiomePath], marked as fixed
    pub fn to_fixed(&self) -> Self {
        Self {
            path: self.path.clone(),
            kind: self.kind.clone(),
            is_fixed: true,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.is_fixed
    }

    /// Adds a file kind to the current file
    pub fn with_file_kind(mut self, kind: FileKindFlag) -> Self {
        self.kind.0.insert(kind);
        self
    }

    /// Accepts a file opened in read mode and saves into it
    pub fn save(&mut self, content: &str) -> Result<(), std::io::Error> {
        let mut file_to_write = File::create(&self.path).unwrap();
        // TODO: handle error with diagnostic
        file_to_write.write_all(content.as_bytes())
    }

    /// Returns the contents of a file, if it exists
    ///
    /// ## Error
    /// If Biome doesn't have permissions to read the file
    pub fn get_buffer_from_file(&mut self) -> String {
        // we assume we have permissions
        read_to_string(&self.path).expect("cannot read the file to format")
    }

    /// Small wrapper for [read_to_string]
    pub fn read_to_string(&self) -> io::Result<String> {
        let path = self.path.as_path();
        read_to_string(path)
    }

    /// Returns the extension of the path
    pub fn extension_as_str(&self) -> Option<&str> {
        self.extension().and_then(OsStr::to_str)
    }

    /// Returns the file name of the path
    fn get_file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(OsStr::to_str)
    }

    /// The priority of the file
    fn priority(file_name: &str) -> FileKind {
        if file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc() {
            FileKindFlag::Config.into()
        } else if matches!(file_name, "package.json" | "tsconfig.json") {
            FileKindFlag::Manifest.into()
        } else {
            FileKindFlag::ToHandle.into()
        }
    }
}

#[cfg(feature = "serde")]
impl schemars::JsonSchema for FileKind {
    fn schema_name() -> String {
        String::from("FileKind")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<FileKind>>::json_schema(gen)
    }
}

impl Deref for BiomePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Eq for BiomePath {}

impl PartialEq<Self> for BiomePath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd<Self> for BiomePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BiomePath {
    fn cmp(&self, other: &Self) -> Ordering {
        let current_file_name = self.get_file_name();
        let other_file_name = other.get_file_name();
        match (current_file_name, other_file_name) {
            (Some(current_file_name), Some(other_file_name)) => {
                if Self::priority(current_file_name) < Self::priority(other_file_name) {
                    Ordering::Less
                } else if Self::priority(current_file_name) > Self::priority(other_file_name) {
                    Ordering::Greater
                } else {
                    current_file_name.cmp(other_file_name)
                }
            }
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

impl From<&BiomePath> for BiomePath {
    fn from(value: &BiomePath) -> Self {
        BiomePath::new(value.path.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::path::FileKindFlag;

    #[test]
    fn test_biome_paths() {
        use super::BiomePath;
        use std::path::PathBuf;

        let path = PathBuf::from("src/package.json");
        let biome_path = BiomePath::new(path);
        assert_eq!(biome_path.get_file_name(), Some("package.json"));
        assert_eq!(
            BiomePath::priority("package.json"),
            FileKindFlag::Manifest.into()
        );
        assert_eq!(
            BiomePath::priority("biome.json"),
            FileKindFlag::Config.into()
        );
        assert_eq!(
            BiomePath::priority("biome.jsonc"),
            FileKindFlag::Config.into()
        );
    }

    #[test]
    fn test_biome_paths_order() {
        use super::BiomePath;
        use std::path::PathBuf;

        let path1 = PathBuf::from("src/package.json");
        let path2 = PathBuf::from("src/biome.json");
        let path3 = PathBuf::from("src/biome.jsonc");
        let path4 = PathBuf::from("src/tsconfig.json");
        let path5 = PathBuf::from("src/README.md");

        let dome_path1 = BiomePath::new(path1);
        let dome_path2 = BiomePath::new(path2);
        let dome_path3 = BiomePath::new(path3);
        let dome_path4 = BiomePath::new(path4);
        let dome_path5 = BiomePath::new(path5);

        let mut paths = vec![dome_path1, dome_path2, dome_path3, dome_path4, dome_path5];
        paths.sort();
        let mut iter = paths.iter();
        assert_eq!(iter.next().unwrap().get_file_name(), Some("biome.json"));
        assert_eq!(iter.next().unwrap().get_file_name(), Some("biome.jsonc"));
        assert_eq!(iter.next().unwrap().get_file_name(), Some("package.json"));
        assert_eq!(iter.next().unwrap().get_file_name(), Some("tsconfig.json"));
        assert_eq!(iter.next().unwrap().get_file_name(), Some("README.md"));
    }
}
