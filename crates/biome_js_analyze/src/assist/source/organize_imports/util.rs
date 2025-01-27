use std::{
    cmp::Ordering,
    path::{Component, Path},
};

use biome_string_case::AsciiCollator;

/// Type for comparing two import sources.
/// Import sources are first grouped and ordered by [ImportSourceKind].
///
/// [ImportSourceKind::Path] are ordered according to their proxoimity of the importing module.
/// For instance, the following order holds: `/` < `../..` < `..` < `.`
///
/// Other kinds are ordered with a natural string order tailored for paths.
/// See [ImportSourceAsciiCollator] for more details.
///
/// ```
/// use biome_js_analyze::assist::source::organize_imports::util::ImportSource;
///
/// assert!(ImportSource::from("https://example.org") < ImportSource::from("bun:test"));
/// assert!(ImportSource::from("node:test") < ImportSource::from("@scope/package"));
/// assert!(ImportSource::from("@scope/package") < ImportSource::from("package"));
/// assert!(ImportSource::from("package") < ImportSource::from("@/alias"));
/// assert!(ImportSource::from("@/alias") < ImportSource::from("/path"));
/// assert!(ImportSource::from("../..") < ImportSource::from(".."));
/// assert!(ImportSource::from("..") < ImportSource::from("."));
/// assert!(ImportSource::from("./path9") < ImportSource::from("./path10"));
/// assert!(ImportSource::from("./path/a") < ImportSource::from("./path-a"));
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportSource<T> {
    kind: ImportSourceKind,
    inner: T,
}
impl<T> ImportSource<T> {
    pub fn kind(&self) -> ImportSourceKind {
        self.kind
    }
}
impl<T: AsRef<str>> From<T> for ImportSource<T> {
    fn from(inner: T) -> Self {
        Self {
            kind: ImportSourceKind::from_source(inner.as_ref()),
            inner,
        }
    }
}
impl<T: AsRef<str> + Eq> PartialOrd for ImportSource<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: AsRef<str> + Eq> Ord for ImportSource<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind()) {
            Ordering::Equal => {
                if self.kind == ImportSourceKind::Path {
                    PathComponents::from(Path::new(self.inner.as_ref()))
                        .cmp(PathComponents::from(Path::new(other.inner.as_ref())))
                        // [PathComponents] normalizes paths.
                        // This lead to a partial order bwteen edge cases such as `./..` and `..`.
                        // To obtain a totak order, we apply a string order when the normalized paths are equal.
                        .then_with(|| {
                            ImportSourceAsciiCollator
                                .cmp_str(self.inner.as_ref(), other.inner.as_ref())
                        })
                } else {
                    ImportSourceAsciiCollator.cmp_str(self.inner.as_ref(), other.inner.as_ref())
                }
            }
            result => result,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImportSourceKind {
    Unknown,
    // `https://example.org`
    Url,
    /// `node:test`, `npm:@scope/package`
    ProtocolPackage,
    /// `package`
    Package,
    /// Import sources that start with `@/`, `#`, `~`, or `%`
    /// Node.js subpath imports and TypeScript aliases.
    Alias,
    /// Import sources that start with `/`, `./`, or `../`
    Path,
}
impl ImportSourceKind {
    pub fn from_source(import_source: &str) -> Self {
        let mut iter = import_source.bytes();
        match iter.next() {
            Some(b'@') => {
                match iter.next() {
                    Some(b'/') | None => {
                        // TypeScript conventional path aliases
                        Self::Alias
                    }
                    Some(b'a'..=b'z' | b'0'..=b'9' | b'-') => Self::Package,
                    _ => Self::Unknown,
                }
            }
            // Node.js subpath imports
            Some(b'#' | b'~' | b'%') => Self::Alias,
            Some(b'/') => Self::Path,
            Some(b'.') => match iter.next() {
                Some(b'.') => {
                    if matches!(iter.next(), None | Some(b'/')) {
                        Self::Path
                    } else {
                        Self::Unknown
                    }
                }
                None | Some(b'/') => Self::Path,
                Some(_) => Self::Unknown,
            },
            Some(b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_') => {
                loop {
                    match iter.next() {
                        Some(b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.') => {}
                        Some(b':') => {
                            // Protocol
                            return match iter.next() {
                                Some(b'/') => Self::Url,
                                Some(b'@' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_') => {
                                    Self::ProtocolPackage
                                }
                                _ => Self::Unknown,
                            };
                        }
                        None | Some(b'/') => {
                            return Self::Package;
                        }
                        Some(_) => {
                            return Self::Unknown;
                        }
                    }
                }
            }
            _ => Self::Unknown,
        }
    }
}

/// This type is analog to [std::path::Component] with the following changes:
/// - [PathComponent::ParentDir] may represent several [Component::ParentDir] at once.
/// - [PathComponent::ParentDir] is ordered before [PathComponent::CurDir]
/// - Order between two [PathComponent::Normal] relies on [ImportSourceAsciiCollator]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PathComponent<'a> {
    /// See [Component::Prefix]
    Prefix(std::path::PrefixComponent<'a>),
    /// See [Component::RootDir]
    RootDir,
    /// See [Component::ParentDir]
    ParentDir(usize),
    /// See [Component::CurDir]
    CurDir,
    /// See [Component::Normal]
    Normal(&'a std::ffi::OsStr),
}
impl PartialOrd for PathComponent<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PathComponent<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PathComponent::RootDir, PathComponent::RootDir)
            | (PathComponent::CurDir, PathComponent::CurDir) => Ordering::Equal,
            (PathComponent::ParentDir(n1), PathComponent::ParentDir(n2)) => n1.cmp(n2).reverse(),
            (PathComponent::Prefix(c1), PathComponent::Prefix(c2)) => c1.cmp(c2),
            (
                PathComponent::Prefix(_),
                PathComponent::RootDir
                | PathComponent::ParentDir(_)
                | PathComponent::CurDir
                | PathComponent::Normal(_),
            ) => Ordering::Less,
            (
                PathComponent::RootDir
                | PathComponent::ParentDir(_)
                | PathComponent::CurDir
                | PathComponent::Normal(_),
                PathComponent::Prefix(_),
            ) => Ordering::Greater,
            (
                PathComponent::RootDir,
                PathComponent::CurDir | PathComponent::ParentDir(_) | PathComponent::Normal(_),
            ) => Ordering::Less,
            (PathComponent::ParentDir(_), PathComponent::CurDir | PathComponent::Normal(_)) => {
                Ordering::Less
            }
            (PathComponent::CurDir | PathComponent::Normal(_), PathComponent::ParentDir(_)) => {
                Ordering::Greater
            }
            (
                PathComponent::CurDir | PathComponent::ParentDir(_) | PathComponent::Normal(_),
                PathComponent::RootDir,
            ) => Ordering::Greater,
            (PathComponent::CurDir, PathComponent::Normal(_)) => Ordering::Less,
            (PathComponent::Normal(_), PathComponent::CurDir) => Ordering::Greater,
            (PathComponent::Normal(s1), PathComponent::Normal(s2)) => {
                ImportSourceAsciiCollator.cmp_osstr(s1, s2)
            }
        }
    }
}

/// This type is analog to [std::path::Components] with the following changes:
/// - The iterator yields [PathComponent] instead of [std::path::Component].
/// - Consecutive parent directories such as `../..` yields a single [PathComponent::ParentDir] with
///   the count of parent directories.
/// - `./..` is normalized to `..`
struct PathComponents<'a> {
    inner: std::path::Components<'a>,
}
impl<'a> From<&'a Path> for PathComponents<'a> {
    fn from(path: &'a Path) -> Self {
        Self {
            inner: path.components(),
        }
    }
}
impl<'a> Iterator for PathComponents<'a> {
    type Item = PathComponent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.inner.next()? {
            Component::Prefix(c) => PathComponent::Prefix(c),
            Component::RootDir => PathComponent::RootDir,
            Component::ParentDir => {
                let count = self
                    .inner
                    .clone()
                    .take_while(|c| matches!(c, Component::ParentDir))
                    .count();
                for _ in 1..=count {
                    self.inner.next();
                }
                PathComponent::ParentDir(count + 1)
            }
            Component::CurDir => {
                // Normalize `./../..` to `../..`.
                // Note that [std::path::Components] already normalizes `.././..` to `../..`
                let parent_dir_count = self
                    .inner
                    .clone()
                    .take_while(|c| matches!(c, Component::ParentDir))
                    .count();
                if parent_dir_count == 0 {
                    PathComponent::CurDir
                } else {
                    for _ in 1..=parent_dir_count {
                        self.inner.next();
                    }
                    PathComponent::ParentDir(parent_dir_count)
                }
            }
            Component::Normal(s) => PathComponent::Normal(s),
        })
    }
}

/// Custom collation order to get a natural order between import sources.
///
/// Non-printable characters and alphanumeric characters have the same order as [biome_string_case::CldrAsciiCollator].
/// Others are ordered differently, but still between non-printable characters and alphanumeric characters.
pub struct ImportSourceAsciiCollator;
impl ImportSourceAsciiCollator {
    const COLLATION: [u8; 128] = [
        b'\0', 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
        0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x7f, b'\t', b'\n',
        0x0b, 0x0c, b'\r', b'/', b'\\', b'?', b'#', b'=', b'&', b';', b',', b'@', b':', b'.', b' ',
        b'_', b'-', b'+', b'*', b'!', b'%', b'$', b'(', b')', b'[', b']', b'{', b'}', b'<', b'>',
        b'|', b'^', b'~', b'\'', b'"', b'`', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
        b'9', b'A', b'a', b'B', b'b', b'C', b'c', b'D', b'd', b'E', b'e', b'F', b'f', b'G', b'g',
        b'H', b'h', b'I', b'i', b'J', b'j', b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'O',
        b'o', b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's', b'T', b't', b'U', b'u', b'V', b'v',
        b'W', b'w', b'X', b'x', b'Y', b'y', b'Z', b'z',
    ];
}
impl biome_string_case::AsciiCollator for ImportSourceAsciiCollator {
    const WEIGHTS: [u8; 256] = biome_string_case::ascii_collation_weight_from(&Self::COLLATION);
}

#[cfg(test)]
mod test {
    use biome_string_case::AsciiCollator;

    use super::*;

    #[test]
    fn import_source_kind() {
        assert_eq!(
            ImportSourceKind::from_source("/absolute/path"),
            ImportSourceKind::Path
        );
    }

    #[test]
    fn import_source_ascii_collator() {
        let sorted = &[
            "/",
            "@scope/package/path",
            "@scope/package@>=1.0/path",
            "@scope/package@^1.0/path",
            "@scope/package@~1.0/path",
            "@scope/package@1.0/path",
            "./",
            "../",
            "a.js",
            "a a.js",
            "a_a.js",
            "a-a.js",
            "a+a.js",
            "a*a.js",
            "https://example.org/path?prop=val",
            "https://example.org/path?prop-1=val",
            "https://example.org/path?prop-1=val#frag",
            "https://example.org/path?prop-1=val&prop2=val",
            "https://example.org/path?prop-1=val-1&prop-2=val-2",
            "https://example.org/path#frag",
            "https://example.org/path-a",
        ];
        for items in sorted.windows(2) {
            let (x, y) = (items[0], items[1]);
            assert_eq!(
                ImportSourceAsciiCollator.cmp_str(x, y),
                Ordering::Less,
                "'{x}' < '{y}'"
            );
        }
    }

    #[test]
    fn test_cmp_path() {
        let cmp_path =
            |p1: &Path, p2: &Path| PathComponents::from(p1).cmp(PathComponents::from(p2));
        assert_eq!(cmp_path(Path::new("/"), Path::new("..")), Ordering::Less);
        assert_eq!(cmp_path(Path::new(".."), Path::new(".")), Ordering::Less);
        assert_eq!(cmp_path(Path::new("."), Path::new("test")), Ordering::Less);
    }

    #[test]
    fn test_import_source_cmp() {
        let sorted = [
            ImportSource::from("https://example.org/path?prop=val"),
            ImportSource::from("https://example.org/path?prop-1=val"),
            ImportSource::from("bun:test"),
            ImportSource::from("node:test"),
            ImportSource::from("npm:@scope/package/path"),
            ImportSource::from("npm:@scope/package@>=1.0/path"),
            ImportSource::from("npm:@scope/package@^1.0/path"),
            ImportSource::from("npm:@scope/package@~1.0/path"),
            ImportSource::from("npm:@scope/package@1.0/path"),
            ImportSource::from("npm:package/path"),
            ImportSource::from("npm:package@>=1.0/path"),
            ImportSource::from("npm:package@^1.0/path"),
            ImportSource::from("npm:package@~1.0/path"),
            ImportSource::from("npm:package@1.0/path"),
            ImportSource::from("#internal"),
            ImportSource::from("@/internal"),
            ImportSource::from("%/internal"),
            ImportSource::from("~/internal"),
            ImportSource::from("/"),
            ImportSource::from(".././.."),
            ImportSource::from("../.."),
            ImportSource::from(".."),
            ImportSource::from("."),
        ];
        for items in sorted.windows(2) {
            let (x, y) = (&items[0], &items[1]);
            assert!(x < y, "'{x:?}' < '{y:?}'");
        }
    }
}
