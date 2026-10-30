use crate::db::WorkspaceDb;
use biome_diagnostics::Error;
use biome_frameworks::{InstalledPackage, LoaderHost};
use biome_fs::{
    BiomePath, FileSystem, PathInterner, PathKind, TraversalContext, TraversalScope, normalize_path,
};
use biome_glob::Glob;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_module_graph::{ResolutionMode, ResolverDb, resolve_specifier};
use biome_package::{Manifest, PackageJson};
use biome_resolver::PathInfo;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::{HashSet, HashSetRef, LocalGuard};
use std::hash::RandomState;
use std::sync::Mutex;

/// Gives framework loaders access to the files of a project, through the
/// filesystem and the resolver of the workspace.
pub(crate) struct WorkspaceLoaderHost<'a> {
    pub(crate) fs: &'a dyn FileSystem,
    pub(crate) db: &'a WorkspaceDb,
}

impl LoaderHost for WorkspaceLoaderHost<'_> {
    fn is_file(&self, path: &Utf8Path) -> bool {
        self.fs.path_is_file(path)
    }

    fn js_file(&self, path: &Utf8Path) -> Option<(AnyJsRoot, SemanticModel)> {
        let text = self.fs.read_file_from_path(path).ok()?;
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
        package_path.ancestors().find_map(|directory| {
            let path = directory.join("node_modules").join(name);
            let manifest = PackageJson::read_manifest(self.fs, &path.join("package.json"))
                .into_deserialized()?;
            // Like module resolution, use the target of a symbolic link, such
            // as the links that pnpm creates, so that the paths of the files
            // of the package are the paths that imports resolve to.
            let path = match self
                .db
                .resolver_paths()
                .get_or_create(self.db, &path)
                .info(self.db)
            {
                Ok(PathInfo::Symlink {
                    canonicalized_target,
                }) => canonicalized_target.clone(),
                _ => path,
            };
            Some(InstalledPackage { path, manifest })
        })
    }

    fn resolve(
        &self,
        directory: &Utf8Path,
        specifier: &str,
        kind: biome_frameworks::ResolutionKind,
    ) -> Option<Utf8PathBuf> {
        let mode = match kind {
            biome_frameworks::ResolutionKind::Stylesheet => ResolutionMode::Css,
            biome_frameworks::ResolutionKind::Module => ResolutionMode::JavaScript,
        };
        let resolved = resolve_specifier(self.db, directory, specifier, mode);
        // No file exists at the path of a module that a framework declares.
        if resolved.kind() == biome_resolver::ResolutionKind::FrameworkModule {
            return None;
        }
        resolved.path().as_path().map(Utf8Path::to_path_buf)
    }

    fn glob(&self, directory: &Utf8Path, glob: &Glob) -> Vec<Utf8PathBuf> {
        let collector = GlobCollector {
            directory,
            glob,
            interner: PathInterner::new().0,
            evaluated_paths: HashSet::default(),
            matches: Mutex::default(),
        };
        self.fs.traversal(Box::new(|scope: &dyn TraversalScope| {
            scope.evaluate(&collector, directory.to_path_buf());
        }));
        let mut matches = collector.matches.into_inner().unwrap_or_default();
        matches.sort();
        matches.dedup();
        matches
    }
}

/// Collects the files under `directory` whose path relative to `directory`
/// matches `glob`, without entering `node_modules` directories.
struct GlobCollector<'a> {
    directory: &'a Utf8Path,
    glob: &'a Glob,
    interner: PathInterner,
    evaluated_paths: HashSet<BiomePath>,
    matches: Mutex<Vec<Utf8PathBuf>>,
}

impl TraversalContext for GlobCollector<'_> {
    fn interner(&self) -> &PathInterner {
        &self.interner
    }

    fn push_diagnostic(&self, _error: Error) {}

    fn can_handle(&self, path: &BiomePath) -> bool {
        match path.path_kind() {
            PathKind::Directory { .. } => path.file_name() != Some("node_modules"),
            PathKind::File { .. } => path
                .strip_prefix(self.directory)
                .is_ok_and(|relative| self.glob.is_match(relative)),
        }
    }

    fn handle_path(&self, _path: BiomePath) {}

    fn store_path(&self, path: BiomePath) {
        if let Ok(mut matches) = self.matches.lock() {
            matches.push(path.to_path_buf());
        }
    }

    fn evaluated_paths(&self) -> HashSetRef<'_, BiomePath, RandomState, LocalGuard<'_>> {
        self.evaluated_paths.pin()
    }
}

/// Returns the files that the `/// <reference path="..." />` directives at the
/// start of the declaration file at `path` name, or nothing when `path` isn't
/// a declaration file.
pub(crate) fn referenced_declaration_files(
    fs: &dyn FileSystem,
    path: &Utf8Path,
) -> Vec<Utf8PathBuf> {
    let is_declaration_file = path.file_name().is_some_and(|name| {
        [".d.ts", ".d.mts", ".d.cts"]
            .iter()
            .any(|ext| name.ends_with(ext))
    });
    let (true, Some(directory), Ok(text)) = (
        is_declaration_file,
        path.parent(),
        fs.read_file_from_path(path),
    ) else {
        return Vec::new();
    };

    let mut references = Vec::new();
    for line in text.lines().map(str::trim) {
        let Some(directive) = line.strip_prefix("///") else {
            // Directives come before the first statement, after comments.
            if line.is_empty()
                || line.starts_with("//")
                || line.starts_with("/*")
                || line.starts_with('*')
            {
                continue;
            }
            break;
        };
        let Some(target) = directive
            .trim()
            .strip_prefix("<reference")
            .and_then(|attributes| attribute_value(attributes, "path"))
        else {
            continue;
        };
        let reference = normalize_path(&directory.join(target));
        if fs.path_is_file(&reference) {
            references.push(reference);
        }
    }
    references
}

/// Returns the value of the attribute `name` of an XML-like tag, such as
/// `./types.d.ts` for `path` in `path="./types.d.ts" />`.
fn attribute_value<'a>(attributes: &'a str, name: &str) -> Option<&'a str> {
    let (_, rest) = attributes.split_once(&format!("{name}="))?;
    let quote = rest
        .chars()
        .next()
        .filter(|quote| matches!(quote, '"' | '\''))?;
    rest[1..].split(quote).next()
}
