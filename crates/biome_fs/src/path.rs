//! This module is responsible to manage paths inside Biome.
//! It is a small wrapper around [path::Utf8PathBuf] but it is also able to
//! give additional information around the file that holds:
//! - the [FileHandlers] for the specific file
//! - shortcuts to open/write to the file
use crate::ConfigName;
use camino::{Utf8Path, Utf8PathBuf};
use enumflags2::{bitflags, BitFlags};
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::fs::read_to_string;
use std::hash::Hash;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::{fs::File, io, io::Write, ops::Deref};

/// The priority of the file
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[repr(u8)]
#[bitflags]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
// NOTE: The order of the variants is important, the one on the top has the highest priority
pub enum FileKind {
    /// A configuration file has the highest priority. It's usually `biome.json` and `biome.jsonc`
    ///
    /// Other third-party configuration files might be added in the future
    Config,
    /// It's usually `package.json` and `tsconfig.json`
    Manifest,
    /// An ignore file, like `.gitignore`
    Ignore,
    /// Files that are required to be inspected before handling other files.
    ///
    /// An example is the GraphQL schema
    Inspectable,
    /// A file to handle has the lowest priority. It's usually a traversed file, or a file opened by the LSP
    #[default]
    Handleable,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(
        from = "smallvec::SmallVec<[FileKind; 5]>",
        into = "smallvec::SmallVec<[FileKind; 5]>"
    )
)]
pub struct FileKinds(BitFlags<FileKind>);

impl From<SmallVec<[FileKind; 5]>> for FileKinds {
    fn from(value: SmallVec<[FileKind; 5]>) -> Self {
        value
            .into_iter()
            .fold(FileKinds::default(), |mut acc, kind| {
                acc.insert(kind);
                acc
            })
    }
}

impl From<FileKinds> for SmallVec<[FileKind; 5]> {
    fn from(value: FileKinds) -> Self {
        value.iter().collect()
    }
}

impl Deref for FileKinds {
    type Target = BitFlags<FileKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FileKinds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FileKind> for FileKinds {
    fn from(flag: FileKind) -> Self {
        Self(BitFlags::from(flag))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct BiomePath {
    /// The path to the file
    path: Utf8PathBuf,
    /// Determines the kind of the file inside Biome. Some files are considered as configuration files, others as manifest files, and others as files to handle
    kind: FileKinds,
    /// Whether this path (usually a file) was fixed as a result of a format/lint/check command with the `--write` filag.
    was_written: bool,
}

#[cfg(feature = "serde")]
impl serde::Serialize for BiomePath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.path.as_str())
    }
}

impl std::fmt::Display for BiomePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.path.as_str())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BiomePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let path: std::path::PathBuf = serde::Deserialize::deserialize(deserializer)?;
        let path = Utf8PathBuf::from_path_buf(path).map_err(|e| {
            serde::de::Error::custom(format!("Unable to deserialize path {}", e.display()))
        })?;
        Ok(BiomePath::new(path))
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for BiomePath {
    fn schema_name() -> String {
        "BiomePath".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

impl BiomePath {
    pub fn new(path_to_file: impl Into<Utf8PathBuf>) -> Self {
        let path = path_to_file.into();
        let kind = path.file_name().map(Self::priority).unwrap_or_default();
        Self {
            path,
            kind,
            was_written: false,
        }
    }

    pub fn new_written(path_to_file: impl Into<Utf8PathBuf>) -> Self {
        let path = path_to_file.into();
        let kind = path.file_name().map(Self::priority).unwrap_or_default();
        Self {
            path,
            kind,
            was_written: true,
        }
    }

    /// Creates a new [BiomePath], marked as fixed
    pub fn to_written(&self) -> Self {
        Self {
            path: self.path.clone(),
            kind: self.kind.clone(),
            was_written: true,
        }
    }

    pub fn was_written(&self) -> bool {
        self.was_written
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

    /// The priority of the file.
    /// - `biome.json` and `biome.jsonc` have the highest priority
    /// - `package.json` and `tsconfig.json`/`jsconfig.json` have the second-highest priority, and they are considered as manifest files
    /// - Other files are considered as files to handle
    fn priority(file_name: &str) -> FileKinds {
        if file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc() {
            FileKind::Config.into()
        } else if matches!(
            file_name,
            "package.json" | "tsconfig.json" | "jsconfig.json"
        ) {
            FileKind::Manifest.into()
        } else {
            FileKind::Handleable.into()
        }
    }

    pub fn is_config(&self) -> bool {
        self.kind.contains(FileKind::Config)
    }

    pub fn is_manifest(&self) -> bool {
        self.kind.contains(FileKind::Manifest)
    }

    pub fn is_ignore(&self) -> bool {
        self.kind.contains(FileKind::Ignore)
    }

    pub fn is_to_inspect(&self) -> bool {
        self.kind.contains(FileKind::Inspectable)
    }
}
impl From<Utf8PathBuf> for BiomePath {
    fn from(value: Utf8PathBuf) -> Self {
        Self::new(value)
    }
}

impl From<&Utf8Path> for BiomePath {
    fn from(value: &Utf8Path) -> Self {
        Self::new(value)
    }
}

impl TryFrom<PathBuf> for BiomePath {
    type Error = PathBuf;
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let path = Utf8PathBuf::from_path_buf(value)?;
        Ok(Self::new(path))
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for FileKinds {
    fn schema_name() -> String {
        String::from("FileKind")
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        <Vec<FileKind>>::json_schema(gen)
    }
}

impl Deref for BiomePath {
    type Target = Utf8PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl From<BiomePath> for Utf8PathBuf {
    fn from(path: BiomePath) -> Self {
        path.path
    }
}

impl PartialOrd for BiomePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BiomePath {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.path.cmp(&other.path),
            ordering => ordering,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::path::{FileKind, FileKinds};

    #[test]
    fn test_biome_paths() {
        use super::BiomePath;
        use camino::Utf8PathBuf;

        let path = Utf8PathBuf::from("src/package.json");
        let biome_path = BiomePath::new(path);
        assert_eq!(biome_path.file_name(), Some("package.json"));
        assert_eq!(
            BiomePath::priority("package.json"),
            FileKind::Manifest.into()
        );
        assert_eq!(BiomePath::priority("biome.json"), FileKind::Config.into());
        assert_eq!(BiomePath::priority("biome.jsonc"), FileKind::Config.into());
    }

    #[test]
    fn test_biome_file_names_order() {
        use super::BiomePath;
        use camino::Utf8PathBuf;

        let path1 = BiomePath::new(Utf8PathBuf::from("src/package.json"));
        let path2 = BiomePath::new(Utf8PathBuf::from("src/biome.json"));
        let path3 = BiomePath::new(Utf8PathBuf::from("src/biome.jsonc"));
        let path4 = BiomePath::new(Utf8PathBuf::from("src/tsconfig.json"));
        let path5 = BiomePath::new(Utf8PathBuf::from("src/README.md"));
        let path6 = BiomePath::new(Utf8PathBuf::from("src/frontend/biome.jsonc"));

        let mut paths = [path1, path2, path3, path4, path5, path6];
        paths.sort();
        let mut iter = paths.iter();
        assert_eq!(iter.next().unwrap().file_name(), Some("biome.json"));
        assert_eq!(iter.next().unwrap().file_name(), Some("biome.jsonc"));
        assert_eq!(iter.next().unwrap().file_name(), Some("biome.jsonc"));
        assert_eq!(iter.next().unwrap().file_name(), Some("package.json"));
        assert_eq!(iter.next().unwrap().file_name(), Some("tsconfig.json"));
        assert_eq!(iter.next().unwrap().file_name(), Some("README.md"));
    }

    #[test]
    fn test_biome_paths_order() {
        use super::BiomePath;
        use camino::Utf8PathBuf;

        let path1 = BiomePath::new(Utf8PathBuf::from("src/package.json"));
        let path2 = BiomePath::new(Utf8PathBuf::from("src/biome.json"));
        let path3 = BiomePath::new(Utf8PathBuf::from("src/biome.jsonc"));
        let path4 = BiomePath::new(Utf8PathBuf::from("src/tsconfig.json"));
        let path5 = BiomePath::new(Utf8PathBuf::from("src/README.md"));
        let path6 = BiomePath::new(Utf8PathBuf::from("src/frontend/biome.jsonc"));
        let path7 = BiomePath::new(Utf8PathBuf::from("src/frontend/package.json"));

        let mut paths = vec![path1, path2, path3, path4, path5, path6, path7];
        paths.sort();
        let mut iter = paths.iter();
        assert_eq!(iter.next().unwrap().to_string(), "src/biome.json");
        assert_eq!(iter.next().unwrap().to_string(), "src/biome.jsonc");
        assert_eq!(iter.next().unwrap().to_string(), "src/frontend/biome.jsonc");
        assert_eq!(
            iter.next().unwrap().to_string(),
            "src/frontend/package.json"
        );
        assert_eq!(iter.next().unwrap().to_string(), "src/package.json");
        assert_eq!(iter.next().unwrap().to_string(), "src/tsconfig.json");
        assert_eq!(iter.next().unwrap().to_string(), "src/README.md");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn deserialize_file_kind_from_str() {
        let result = serde_json::from_str::<FileKinds>("[\"config\"]");
        assert!(result.is_ok());
        let file_kinds = result.unwrap();
        assert!(file_kinds.contains(FileKind::Config));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialize_file_kind_into_vec() {
        let file_kinds = FileKinds::from(FileKind::Config);
        let result = serde_json::to_string(&file_kinds);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[\"config\"]");
    }
}
