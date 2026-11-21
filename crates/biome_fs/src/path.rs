//! Defines filesystem path identity and path-derived behavior used throughout Biome.
//!
//! [`BiomePath`] preserves the logical path supplied by an adapter, normalizes `.` and `..`
//! components without filesystem access, and derives the path's processing role. It does not
//! canonicalize paths or resolve symbolic links.
use crate::{ConfigName, PathKind, utils::normalize_path_buf};
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs::File, io, io::Write, ops::Deref};

/// Returns whether `path` contains a directory component named `node_modules`.
#[inline]
pub fn is_node_modules_path(path: &Utf8Path) -> bool {
    path.components()
        .any(|component| component.as_str().as_bytes() == b"node_modules")
}

/// Classifies paths that require different processing priorities.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FileRole {
    /// A Biome configuration file that must be processed before other paths.
    Config,
    /// A project manifest, such as `package.json` or `tsconfig.json`.
    Manifest,
    /// An ignore file, such as `.gitignore` or `.ignore`.
    Ignore,
    /// A path that does not require configuration, manifest, or ignore-file priority.
    #[default]
    Handleable,
}

/// Describes how a candidate path relates to a scoped root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathRelation<'a> {
    /// The candidate and root identify the same logical path.
    Same,
    /// The candidate is below the root by the given normalized relative path.
    Descendant(&'a Utf8Path),
    /// The candidate is not contained by the root.
    Outside,
}

const VIRTUAL_PATH_MARKER: &str = "\0biome-virtual-document\0";

fn is_virtual_path(path: &Utf8Path) -> bool {
    let mut components = path.components();
    while let Some(component) = components.next() {
        if component.as_str() == VIRTUAL_PATH_MARKER {
            return components.next().is_some() && components.next().is_none();
        }
    }
    false
}

/// Identifies a logical path throughout Biome.
#[derive(Debug, Clone, Eq, Default)]
pub struct BiomePath {
    /// The logical path used for identity, policies, storage, and diagnostics.
    path: Utf8PathBuf,
    /// The processing role derived from the logical file name.
    role: FileRole,
    /// Compatibility metadata supplied through [`Self::new_with_kind`].
    path_kind: PathKind,
    /// Whether this path is an opaque key for a document without filesystem backing.
    is_virtual: bool,
}

impl PartialEq for BiomePath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl std::hash::Hash for BiomePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl BiomePath {
    /// Constructs a normalized filesystem-backed logical path.
    ///
    /// ## Panic
    ///
    /// If it's a virtual path (in debug)
    pub fn new(path: impl Into<Utf8PathBuf>) -> Self {
        let path = normalize_path_buf(path.into());
        debug_assert!(
            !is_virtual_path(&path),
            "virtual paths must be constructed with BiomePath::new_virtual"
        );
        Self::from_parts(path, false)
    }

    pub(crate) fn from_virtual_path(path: &Utf8Path) -> Option<Self> {
        is_virtual_path(path).then(|| Self::from_parts(path.to_path_buf(), true))
    }

    /// Constructs an opaque path key for a document without filesystem backing.
    pub fn new_virtual(key: impl AsRef<str>) -> Self {
        const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

        let key = key.as_ref();
        let mut path = String::with_capacity(VIRTUAL_PATH_MARKER.len() + key.len() * 2 + 2);
        path.push('/');
        path.push_str(VIRTUAL_PATH_MARKER);
        path.push_str("/v");
        for byte in key.bytes() {
            path.push(HEX_DIGITS[(byte >> 4) as usize] as char);
            path.push(HEX_DIGITS[(byte & 0xf) as usize] as char);
        }
        Self::from_parts(path.into(), true)
    }

    /// Constructs a normalized logical path with compatibility metadata about its entry kind.
    pub fn new_with_kind(path: impl Into<Utf8PathBuf>, path_kind: PathKind) -> Self {
        Self {
            path_kind,
            ..Self::new(path)
        }
    }

    fn from_parts(path: Utf8PathBuf, is_virtual: bool) -> Self {
        let role = path.file_name().map_or_default(Self::priority);
        Self {
            path,
            role,
            path_kind: PathKind::default(),
            is_virtual,
        }
    }

    #[cfg(feature = "serde")]
    fn from_serialized(path: Utf8PathBuf) -> Self {
        if is_virtual_path(&path) {
            Self::from_parts(path, true)
        } else {
            Self::new(path)
        }
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

    /// Returns the logical path without exposing its owned storage.
    pub fn as_path(&self) -> &Utf8Path {
        &self.path
    }

    /// Returns the normalized logical path as a UTF-8 string slice.
    pub fn as_str(&self) -> &str {
        self.path.as_str()
    }

    /// Returns whether this is an opaque key for a document without filesystem backing.
    pub fn is_virtual(&self) -> bool {
        self.is_virtual
    }

    /// Returns the final path component, if one exists.
    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name()
    }

    /// Returns the file extension without the leading dot, if one exists.
    pub fn extension(&self) -> Option<&str> {
        self.path.extension()
    }

    /// Returns the logical parent path, if one exists.
    pub fn parent(&self) -> Option<&Utf8Path> {
        self.path.parent()
    }

    /// Returns whether this logical path starts with `base` on component boundaries.
    pub fn starts_with(&self, base: impl AsRef<Utf8Path>) -> bool {
        self.path.starts_with(base.as_ref())
    }

    /// Returns whether this logical path ends with `child` on component boundaries.
    pub fn ends_with(&self, child: impl AsRef<Utf8Path>) -> bool {
        self.path.ends_with(child.as_ref())
    }

    /// Copies the normalized logical path into a raw UTF-8 path buffer.
    pub fn to_path_buf(&self) -> Utf8PathBuf {
        self.path.clone()
    }

    /// Relates `candidate` to this path as the scoped root.
    ///
    /// Descendant paths are returned relative to this root. Both paths are already lexically
    /// normalized, so callers can pass the relative path directly to scoped matchers.
    pub fn relation_to<'a>(&'a self, candidate: &'a Self) -> PathRelation<'a> {
        if self.path == candidate.path {
            PathRelation::Same
        } else if let Ok(relative) = candidate.path.strip_prefix(&self.path)
            // Normalized relative paths keep `..` only as leading components, which escape a
            // relative root instead of descending into it.
            && relative.components().next() != Some(Utf8Component::ParentDir)
        {
            PathRelation::Descendant(relative)
        } else {
            PathRelation::Outside
        }
    }

    /// Compares paths by processing role and then by logical path.
    ///
    /// General [`Ord`] remains lexical; traversal and scanning use this comparison when
    /// configuration, manifest, and ignore files must be handled first.
    pub fn cmp_processing_priority(&self, other: &Self) -> Ordering {
        self.role
            .cmp(&other.role)
            .then_with(|| self.path.cmp(&other.path))
    }

    fn priority(file_name: &str) -> FileRole {
        if ConfigName::matches_file_name(file_name) {
            FileRole::Config
        } else if matches!(
            file_name,
            "package.json"
                | "tsconfig.json"
                | "jsconfig.json"
                | "turbo.json"
                | "turbo.jsonc"
                | "pnpm-workspace.yaml"
        ) {
            FileRole::Manifest
        } else if matches!(file_name, ".gitignore" | ".ignore") {
            FileRole::Ignore
        } else {
            FileRole::Handleable
        }
    }

    #[inline(always)]
    pub fn is_config(&self) -> bool {
        matches!(self.role, FileRole::Config)
    }

    #[inline(always)]
    pub fn is_manifest(&self) -> bool {
        matches!(self.role, FileRole::Manifest)
    }

    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        matches!(self.role, FileRole::Ignore)
    }

    #[inline(always)]
    pub fn is_handleable(&self) -> bool {
        matches!(self.role, FileRole::Handleable)
    }

    /// Returns `true` for paths that Biome must inspect while scanning.
    #[inline(always)]
    pub fn is_required_during_scan(&self) -> bool {
        matches!(
            self.role,
            FileRole::Config | FileRole::Ignore | FileRole::Manifest
        )
    }

    /// Returns the compatibility entry kind supplied at construction.
    pub fn path_kind(&self) -> PathKind {
        self.path_kind
    }

    /// Returns `true` if the path is inside `node_modules`
    #[inline(always)]
    pub fn is_dependency(&self) -> bool {
        is_node_modules_path(&self.path)
    }

    /// Whether this is a file named `package.json`
    pub fn is_package_json(&self) -> bool {
        self.path.file_name() == Some("package.json")
    }

    /// Returns `true` if the path has one of the recognised extensions for
    /// TypeScript type declarations.
    pub fn is_type_declaration(&self) -> bool {
        let bytes = self.path.as_os_str().as_encoded_bytes();
        bytes.ends_with(b".d.ts") || bytes.ends_with(b".d.cts") || bytes.ends_with(b".d.mts")
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
        Utf8PathBuf::from_path_buf(value).map(Self::new)
    }
}

impl TryFrom<Cow<'_, Path>> for BiomePath {
    type Error = PathBuf;

    fn try_from(value: Cow<'_, Path>) -> Result<Self, Self::Error> {
        Utf8PathBuf::from_path_buf(value.as_ref().into()).map(Self::new)
    }
}

impl Deref for BiomePath {
    type Target = Utf8PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Utf8PathBuf> for BiomePath {
    fn as_ref(&self) -> &Utf8PathBuf {
        &self.path
    }
}

impl AsRef<Utf8Path> for BiomePath {
    fn as_ref(&self) -> &Utf8Path {
        self.path.as_ref()
    }
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
        Ok(Self::from_serialized(path))
    }
}

impl From<BiomePath> for Utf8PathBuf {
    fn from(path: BiomePath) -> Self {
        path.path
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for BiomePath {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("BiomePath")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        String::json_schema(generator)
    }
}

impl PartialOrd for BiomePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BiomePath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

#[cfg(test)]
mod test {
    use camino::Utf8Path;

    use crate::{FileSystem, OsFileSystem};

    use super::{BiomePath, FileRole, PathRelation, is_node_modules_path};

    #[test]
    fn detects_node_modules_paths() {
        assert!(is_node_modules_path(Utf8Path::new(
            "/project/node_modules/package/index.js"
        )));
        assert!(is_node_modules_path(Utf8Path::new(
            "project/packages/app/node_modules/package/index.js"
        )));
        assert!(!is_node_modules_path(Utf8Path::new(
            "/project/node_modules_backup/package/index.js"
        )));
    }

    #[test]
    fn test_biome_paths() {
        let biome_path = BiomePath::new("src/package.json");
        assert_eq!(biome_path.file_name(), Some("package.json"));
        assert_eq!(BiomePath::priority("package.json"), FileRole::Manifest);
        assert_eq!(BiomePath::priority("turbo.json"), FileRole::Manifest);
        assert_eq!(BiomePath::priority("turbo.jsonc"), FileRole::Manifest);
        assert_eq!(
            BiomePath::priority("pnpm-workspace.yaml"),
            FileRole::Manifest
        );
        assert_eq!(BiomePath::priority("biome.json"), FileRole::Config);
        assert_eq!(BiomePath::priority("biome.jsonc"), FileRole::Config);
        assert_eq!(BiomePath::priority(".biome.json"), FileRole::Config);
        assert_eq!(BiomePath::priority(".biome.jsonc"), FileRole::Config);
        assert_eq!(BiomePath::priority(".gitignore"), FileRole::Ignore);
        assert_eq!(BiomePath::priority(".ignore"), FileRole::Ignore);
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
        paths.sort_by(BiomePath::cmp_processing_priority);
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

        let mut paths = [path1, path2, path3, path4, path5, path6, path7];
        paths.sort_by(BiomePath::cmp_processing_priority);
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
    fn normalizes_paths_lexically() {
        assert_eq!(
            BiomePath::new("project/./src/../file.js").as_str(),
            "project/file.js"
        );
        assert_eq!(BiomePath::new("../../file.js").as_str(), "../../file.js");
        assert_eq!(BiomePath::new("/../../file.js").as_str(), "/file.js");
    }

    #[test]
    fn anchors_before_normalizing() {
        let fs = OsFileSystem::new("/repo/project".into());

        assert_eq!(
            fs.biome_path(Utf8Path::new("missing/../dist/file.js"))
                .as_str(),
            "/repo/project/dist/file.js"
        );
        assert_eq!(fs.biome_path(Utf8Path::new("")).as_str(), "/repo/project");

        let root = fs.biome_path(Utf8Path::new("config"));
        assert_eq!(
            fs.biome_path_from(&root, Utf8Path::new("../shared/file.js"))
                .as_str(),
            "/repo/project/shared/file.js"
        );
    }

    #[test]
    fn virtual_paths_preserve_identity_without_becoming_filesystem_paths() {
        let first = BiomePath::new_virtual("untitled:first/../document.js");
        let second = BiomePath::new_virtual("untitled:second/../document.js");

        assert!(first.is_virtual());
        assert!(second.is_virtual());
        assert_ne!(first, second);
        assert!(!BiomePath::new("/project/file.js").is_virtual());
    }

    #[test]
    fn filesystem_adapters_preserve_virtual_paths() {
        let fs = OsFileSystem::new("/repo/project".into());
        let path = BiomePath::new_virtual("untitled:document.js");

        let reconstructed = fs.biome_path(path.as_path());

        assert_eq!(reconstructed, path);
        assert!(reconstructed.is_virtual());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "virtual paths must be constructed with BiomePath::new_virtual")]
    fn filesystem_constructor_rejects_the_virtual_namespace() {
        let virtual_path = BiomePath::new_virtual("untitled:document.js");
        BiomePath::new(virtual_path.to_path_buf());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialized_virtual_paths_preserve_their_origin() {
        let path = BiomePath::new_virtual("untitled:document.js");
        let serialized = serde_json::to_string(&path).unwrap();
        let deserialized: BiomePath = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, path);
        assert!(deserialized.is_virtual());
    }

    #[test]
    fn relates_normalized_paths() {
        let root = BiomePath::new("/repo/project/../project");
        let same = BiomePath::new("/repo/project");
        let descendant = BiomePath::new("/repo/project/dist/file.js");
        let outside = BiomePath::new("/repo/other/file.js");

        assert_eq!(root.relation_to(&same), PathRelation::Same);
        assert_eq!(
            root.relation_to(&descendant),
            PathRelation::Descendant(Utf8Path::new("dist/file.js"))
        );
        assert_eq!(root.relation_to(&outside), PathRelation::Outside);
    }

    #[test]
    fn relative_roots_do_not_contain_parent_escapes() {
        assert_eq!(
            BiomePath::new("..").relation_to(&BiomePath::new("../../outside.js")),
            PathRelation::Outside
        );
        assert_eq!(
            BiomePath::new("").relation_to(&BiomePath::new("../outside.js")),
            PathRelation::Outside
        );
        assert_eq!(
            BiomePath::new("").relation_to(&BiomePath::new("inside.js")),
            PathRelation::Descendant(Utf8Path::new("inside.js"))
        );
    }
}
