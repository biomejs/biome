use biome_frameworks::{InstalledPackage, LoaderHost, ResolutionKind};
use biome_fs::{FileSystem, MemoryFileSystem};
use biome_glob::Glob;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_package::{Manifest, PackageJson};
use camino::{Utf8Path, Utf8PathBuf};

/// Reads files from memory. Packages are installed in the `node_modules`
/// directory of the package that uses them, and only relative specifiers
/// resolve.
pub struct TestHost(pub MemoryFileSystem);

impl TestHost {
    pub fn new(files: &[(&str, &str)]) -> Self {
        let fs = MemoryFileSystem::default();
        for (path, content) in files {
            fs.insert(Utf8PathBuf::from(*path), *content);
        }
        Self(fs)
    }
}

impl LoaderHost for TestHost {
    fn is_file(&self, path: &Utf8Path) -> bool {
        self.0.path_is_file(path)
    }

    fn js_file(&self, path: &Utf8Path) -> Option<(AnyJsRoot, SemanticModel)> {
        let text = self.0.read_file_from_path(path).ok()?;
        let source = JsFileSource::try_from(path).ok()?;
        let parsed = parse(&text, source, JsParserOptions::default());
        if parsed.has_errors() {
            return None;
        }
        let root = parsed.tree();
        let model = semantic_model(&root, SemanticModelOptions::default());
        Some((root, model))
    }

    fn installed_package(&self, package_path: &Utf8Path, name: &str) -> Option<InstalledPackage> {
        let path = package_path.join("node_modules").join(name);
        let manifest =
            PackageJson::read_manifest(&self.0, &path.join("package.json")).into_deserialized()?;
        Some(InstalledPackage { path, manifest })
    }

    fn resolve(
        &self,
        directory: &Utf8Path,
        specifier: &str,
        _kind: ResolutionKind,
    ) -> Option<Utf8PathBuf> {
        let path = directory.join(specifier.strip_prefix("./")?);
        self.0.path_is_file(&path).then_some(path)
    }

    fn glob(&self, directory: &Utf8Path, glob: &Glob) -> Vec<Utf8PathBuf> {
        let mut paths: Vec<_> = self
            .0
            .files
            .read()
            .keys()
            .filter(|path| {
                path.strip_prefix(directory)
                    .is_ok_and(|relative| glob.is_match(relative))
            })
            .cloned()
            .collect();
        paths.sort();
        paths
    }
}
