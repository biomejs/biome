//! Salsa-backed path info for the resolver.
//!
//! The resolver reads the filesystem through [`ResolverDbAdapter`], which
//! stores one [`ResolverPathData`] input per path in [`ResolverPaths`]. An
//! input is created the first time a path is looked up, using
//! [`ResolverDb::resolver_fs`], and it is never removed afterwards. A path
//! that doesn't exist is stored as [`ResolveError::NotFound`], so
//! queries that observed a missing path are invalidated when the path is
//! created.
//!
//! Creating the input of a new path is the only filesystem access that
//! happens inside queries, and it never changes existing path info. The content
//! of a manifest that isn't indexed as a parsed source is also read through
//! [`ResolverDb::resolver_fs`], by the query that depends on the manifest's
//! [`ResolverPathData::revision`].
//!
//! Queries never refresh path info. The owner of the database reports changes
//! using [`sync_resolver_paths`], which re-reads the filesystem for known paths
//! and only updates the inputs whose path info changed.

use biome_db::{Db, ParsedSource};
use biome_fs::is_node_modules_path;
use biome_package::{Manifest, PackageJson, TsConfigJson};
use biome_rowan::Language;
use camino::{Utf8Path, Utf8PathBuf};
use enumflags2::{BitFlags, bitflags};
use papaya::{HashMap, HashSet};
use salsa::{Durability, Setter};
use std::sync::Arc;

use crate::{PathInfo, ResolveError, ResolverFsProxy};

/// Provides the path info used by resolver queries.
#[salsa::db]
pub trait ResolverDb: Db {
    /// Returns the filesystem used to read the path info of paths that haven't
    /// been looked up yet, and to read manifests that aren't indexed.
    ///
    /// Other queries must not read the filesystem directly. Use
    /// [`ResolverPaths::get_or_create`] or [`ResolverDbAdapter`] instead, so the read is
    /// backed by an input.
    fn resolver_fs(&self) -> &dyn ResolverFsProxy;

    /// Returns the lookup table of the known resolver paths.
    fn resolver_paths(&self) -> &ResolverPaths;
}

/// The path info of a path used by the resolver.
#[salsa::input]
#[derive(Debug)]
pub struct ResolverPathData {
    #[returns(ref)]
    pub path: Utf8PathBuf,

    /// The kind of filesystem entry the path points at.
    #[returns(ref)]
    pub info: Result<PathInfo, ResolveError>,

    /// Changes when the content backing this path may have changed.
    ///
    /// Queries that read content associated with the path, such as a
    /// manifest, read this field so they are invalidated when the content is
    /// replaced outside the parsed source.
    pub revision: u64,
}

/// Lookup table that maps paths to their [`ResolverPathData`].
///
/// Clones share the same table.
#[derive(Clone, Default)]
pub struct ResolverPaths {
    by_path: Arc<HashMap<Utf8PathBuf, ResolverPathData>>,
    /// The paths the resolver has read as manifests.
    manifests: Arc<HashSet<Utf8PathBuf>>,
}

impl ResolverPaths {
    /// Returns the path info of `path`, reading it from the filesystem if the
    /// path hasn't been looked up yet.
    pub fn get_or_create(&self, db: &dyn ResolverDb, path: &Utf8Path) -> ResolverPathData {
        if let Some(data) = self.by_path.pin().get(path) {
            return *data;
        }

        let info = db.resolver_fs().path_info(path);
        // Dependencies rarely change. A higher durability lets Salsa skip
        // verifying queries that only depend on them after a project file
        // changes.
        let durability = if is_node_modules_path(path) {
            Durability::HIGH
        } else {
            Durability::LOW
        };
        // The path is the key of the input in this table, and inputs are
        // never removed or re-keyed: a path that stops existing keeps its
        // input with a `NotFound` info. The `path` field is therefore never
        // set again, which lets Salsa skip verifying it. Salsa panics if a
        // `NEVER_CHANGE` field is set, so a setter for it would be a bug.
        let data = ResolverPathData::builder(path.to_path_buf(), info, 0)
            .durability(durability)
            .path_durability(Durability::NEVER_CHANGE)
            .new(db);
        // Another thread may have created the input in the meantime. Only one
        // input is ever published for a path.
        *self.by_path.pin().get_or_insert(path.to_path_buf(), data)
    }

    /// Returns the path info of `path` if the path has been looked up before.
    pub fn get(&self, path: &Utf8Path) -> Option<ResolverPathData> {
        self.by_path.pin().get(path).copied()
    }

    /// Returns the known paths equal to or inside `path`.
    pub fn paths_within(&self, path: &Utf8Path) -> Vec<Utf8PathBuf> {
        self.paths_matching(|candidate| candidate.starts_with(path))
    }

    /// Returns the known paths that match `predicate`.
    pub fn paths_matching(&self, predicate: impl Fn(&Utf8Path) -> bool) -> Vec<Utf8PathBuf> {
        self.by_path
            .pin()
            .keys()
            .filter(|candidate| predicate(candidate))
            .cloned()
            .collect()
    }

    /// Returns whether the resolver has read `path` as a manifest, such as a
    /// `package.json` or a TypeScript configuration.
    ///
    /// Only the content of manifests affects resolution, so only their
    /// content changes must be reported.
    pub fn is_manifest(&self, path: &Utf8Path) -> bool {
        self.manifests.pin().contains(path)
    }

    /// Returns the changes to report when the filesystem reports that `path`
    /// was created or modified.
    pub fn changes_for_modified_path(&self, path: &Utf8Path) -> ResolverPathChanges {
        if self.is_manifest(path) {
            ResolverPathChange::Kind | ResolverPathChange::Content
        } else {
            ResolverPathChange::Kind.into()
        }
    }

    /// Records that a query reads `path` as a manifest.
    ///
    /// The query records the path before it looks up the parsed source, so a
    /// writer that publishes a parsed source for a path that isn't recorded
    /// yet can't invalidate a query that already read it.
    fn record_manifest(&self, path: &Utf8Path) {
        let manifests = self.manifests.pin();
        if !manifests.contains(path) {
            manifests.insert(path.to_path_buf());
        }
    }
}

/// A change that may have happened to a path.
#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolverPathChange {
    /// The path may have been created, removed or replaced by a different
    /// kind of entry.
    Kind = 1 << 0,
    /// The content of the path may have changed.
    Content = 1 << 1,
}

/// The set of changes that may have happened to a path.
pub type ResolverPathChanges = BitFlags<ResolverPathChange>;
/// Returns whether [`sync_resolver_paths`] would update any input for the
/// given `changes`.
///
/// Use this to skip acquiring write access to the database when the
/// path info is already up to date.
pub fn resolver_paths_need_sync<'a>(
    db: &dyn ResolverDb,
    changes: impl IntoIterator<Item = (&'a Utf8Path, ResolverPathChanges)>,
) -> bool {
    let paths = db.resolver_paths();
    changes.into_iter().any(|(path, changes)| {
        if changes.contains(ResolverPathChange::Content) && paths.get(path).is_some() {
            return true;
        }
        if !changes.contains(ResolverPathChange::Kind) {
            return false;
        }
        // An unknown path may be inside a directory the resolver recorded as
        // missing, so its nearest known ancestor decides.
        path.ancestors()
            .find_map(|ancestor| paths.get(ancestor).map(|data| (ancestor, data)))
            .is_some_and(|(known_path, data)| {
                data.info(db) != &db.resolver_fs().path_info(known_path)
            })
    })
}

/// Refreshes the path info of the known paths among `changes` from the
/// filesystem.
///
/// When the kind of a path changes, its parent directory is refreshed as well,
/// so creating a file inside a new directory also updates the path info of that
/// directory. A path that hasn't been looked up yet isn't stored, but it is
/// treated as a possible change of kind, so its parent is refreshed instead.
pub fn sync_resolver_paths(
    db: &mut dyn ResolverDb,
    changes: impl IntoIterator<Item = (Utf8PathBuf, ResolverPathChanges)>,
) {
    let paths = db.resolver_paths().clone();
    for (path, mut changes) in changes {
        for ancestor in path.ancestors() {
            if !sync_resolver_path(db, &paths, ancestor, changes) {
                break;
            }
            // Only the reported path may have new content. Its ancestors can
            // only change kind.
            changes = ResolverPathChange::Kind.into();
        }
    }
}

/// Refreshes the path info of `path` for the given `changes`.
///
/// Returns whether the kind of `path` may have changed, in which case its
/// parent directory must be refreshed as well. That's also the case for a path
/// that hasn't been looked up yet.
fn sync_resolver_path(
    db: &mut dyn ResolverDb,
    paths: &ResolverPaths,
    path: &Utf8Path,
    changes: ResolverPathChanges,
) -> bool {
    let Some(data) = paths.get(path) else {
        return changes.contains(ResolverPathChange::Kind);
    };

    let mut kind_changed = false;
    if changes.contains(ResolverPathChange::Kind) {
        let info = db.resolver_fs().path_info(path);
        kind_changed = data.info(db) != &info;
        if kind_changed {
            let _ = data.set_info(db).to(info);
        }
    }
    if changes.contains(ResolverPathChange::Content) {
        let revision = data.revision(db).wrapping_add(1);
        data.set_revision(db).to(revision);
    }
    kind_changed
}

/// Adapts the Salsa-backed path info to [`ResolverFsProxy`].
pub struct ResolverDbAdapter<'db> {
    db: &'db dyn ResolverDb,
}

impl<'db> ResolverDbAdapter<'db> {
    pub fn new(db: &'db dyn ResolverDb) -> Self {
        Self { db }
    }

    fn path_data(&self, path: &Utf8Path) -> ResolverPathData {
        self.db.resolver_paths().get_or_create(self.db, path)
    }
}

impl ResolverFsProxy for ResolverDbAdapter<'_> {
    fn find_package_json(
        &self,
        search_dir: &Utf8Path,
    ) -> Result<(Utf8PathBuf, PackageJson), ResolveError> {
        let package = package_json_for_directory(self.db, self.path_data(search_dir))
            .ok_or(ResolveError::ManifestNotFound)?;
        let manifest = package
            .manifest(self.db)
            .manifest(self.db)
            .clone()
            .ok_or(ResolveError::ErrorLoadingManifest)?;
        Ok((package.path(self.db).clone(), manifest))
    }

    fn path_info(&self, path: &Utf8Path) -> Result<PathInfo, ResolveError> {
        self.path_data(path).info(self.db).clone()
    }

    fn read_package_json_in_directory(
        &self,
        dir_path: &Utf8Path,
    ) -> Result<PackageJson, ResolveError> {
        package_json_for_path(self.db, self.path_data(&dir_path.join("package.json")))
            .and_then(|data| data.manifest(self.db).clone())
            .ok_or(ResolveError::ErrorLoadingManifest)
    }

    fn read_tsconfig_json(&self, path: &Utf8Path) -> Result<TsConfigJson, ResolveError> {
        tsconfig_json_for_path(self.db, self.path_data(path))
            .and_then(|data| data.manifest(self.db).clone())
            .ok_or(ResolveError::ErrorLoadingManifest)
    }
}

/// A package manifest derived from a parsed source or read from the
/// filesystem.
#[salsa::tracked]
pub struct PackageJsonData<'db> {
    #[tracked]
    #[no_eq]
    #[returns(ref)]
    pub manifest: Option<PackageJson>,
}

/// Deserializes a package manifest from an existing parsed source.
///
/// The returned data contains no manifest when the source is not a JSON
/// document or does not contain a valid package manifest.
#[salsa::tracked]
pub fn package_json_from_source<'db>(
    db: &'db dyn Db,
    source: ParsedSource,
) -> PackageJsonData<'db> {
    PackageJsonData::new(db, deserialize_manifest(db, source))
}

/// Returns the package manifest at `path`, or `None` if there is none.
///
/// The indexed parsed source is used when there is one, so unsaved editor
/// content is taken into account, even for a manifest that doesn't exist on
/// disk yet. Otherwise, the manifest is read through
/// [`ResolverDb::resolver_fs`] if the path is a file.
#[salsa::tracked]
pub fn package_json_for_path<'db>(
    db: &'db dyn ResolverDb,
    path: ResolverPathData,
) -> Option<PackageJsonData<'db>> {
    db.resolver_paths().record_manifest(path.path(db));
    let _ = path.revision(db);
    if let Some(source) = db.parsed_source_for_path(path.path(db)) {
        return Some(package_json_from_source(db, source));
    }
    if !matches!(path.info(db), Ok(PathInfo::File | PathInfo::Symlink { .. })) {
        return None;
    }

    let path = path.path(db);
    let manifest = path.parent().and_then(|directory| {
        db.resolver_fs()
            .read_package_json_in_directory(directory)
            .ok()
    });
    Some(PackageJsonData::new(db, manifest))
}

/// The nearest package manifest for a directory.
#[salsa::tracked]
pub struct ResolvedPackage<'db> {
    #[returns(ref)]
    pub path: Utf8PathBuf,
    pub manifest: PackageJsonData<'db>,
}

/// Finds the nearest package manifest for `directory`.
#[salsa::tracked]
pub fn package_json_for_directory<'db>(
    db: &'db dyn ResolverDb,
    directory: ResolverPathData,
) -> Option<ResolvedPackage<'db>> {
    let paths = db.resolver_paths();
    for ancestor in directory.path(db).ancestors() {
        let manifest_path = paths.get_or_create(db, &ancestor.join("package.json"));
        if let Some(manifest) = package_json_for_path(db, manifest_path) {
            return Some(ResolvedPackage::new(db, ancestor.to_path_buf(), manifest));
        }
    }

    None
}

/// A TypeScript configuration derived from a parsed source or read from the
/// filesystem.
#[salsa::tracked]
pub struct TsConfigJsonData<'db> {
    #[tracked]
    #[no_eq]
    #[returns(ref)]
    pub manifest: Option<TsConfigJson>,
}

/// Deserializes a TypeScript configuration from an existing parsed source.
///
/// The returned data contains no configuration when the source is not a JSON
/// document or does not contain a valid TypeScript configuration.
#[salsa::tracked]
pub fn tsconfig_json_from_source<'db>(
    db: &'db dyn Db,
    source: ParsedSource,
) -> TsConfigJsonData<'db> {
    TsConfigJsonData::new(db, deserialize_manifest(db, source))
}

/// Returns the TypeScript configuration at `path`, or `None` if there is
/// none.
///
/// The indexed parsed source is used when there is one. Otherwise, the
/// configuration is read through [`ResolverDb::resolver_fs`] if the path is a
/// file.
#[salsa::tracked]
pub fn tsconfig_json_for_path<'db>(
    db: &'db dyn ResolverDb,
    path: ResolverPathData,
) -> Option<TsConfigJsonData<'db>> {
    db.resolver_paths().record_manifest(path.path(db));
    let _ = path.revision(db);
    if let Some(source) = db.parsed_source_for_path(path.path(db)) {
        return Some(tsconfig_json_from_source(db, source));
    }
    if !matches!(path.info(db), Ok(PathInfo::File | PathInfo::Symlink { .. })) {
        return None;
    }

    let path = path.path(db);
    let manifest = db.resolver_fs().read_tsconfig_json(path).ok();
    Some(TsConfigJsonData::new(db, manifest))
}

fn deserialize_manifest<M>(db: &dyn Db, source: ParsedSource) -> Option<M>
where
    M: Manifest,
    M::Language: 'static,
{
    let root = source
        .parsed(db)
        .clone()
        .into_language_root::<<M::Language as Language>::Root>()?;
    let (manifest, _) = M::deserialize_manifest(&root, source.path(db)).consume();
    manifest
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_db::testing::{
        Events, assert_function_query_was_not_run, assert_function_query_was_run,
        function_query_will_execute_count_by_name,
    };
    use biome_fs::MemoryFileSystem;
    use biome_json_parser::{JsonParse, JsonParserOptions, parse_json};
    use salsa::Storage;
    use std::collections::BTreeMap;
    use std::sync::Mutex;

    #[salsa::db]
    struct TestDb {
        events: Events,
        fs: MemoryFileSystem,
        files: Arc<Mutex<BTreeMap<Utf8PathBuf, ParsedSource>>>,
        paths: ResolverPaths,
        storage: Storage<Self>,
    }

    impl TestDb {
        fn new() -> Self {
            let events = Events::default();
            Self {
                storage: Storage::new(Some(Box::new({
                    let events = events.clone();
                    move |event| events.0.lock().unwrap().push(event)
                }))),
                events,
                fs: MemoryFileSystem::default(),
                files: Arc::default(),
                paths: ResolverPaths::default(),
            }
        }

        fn clear_events(&self) {
            self.take_events();
        }

        fn take_events(&self) -> Vec<salsa::Event> {
            std::mem::take(&mut *self.events.0.lock().unwrap())
        }

        fn path(&self, path: &str) -> ResolverPathData {
            self.paths.get_or_create(self, Utf8Path::new(path))
        }

        /// Writes `content` to the filesystem and reports the change.
        fn write(&mut self, path: &str, content: &str) {
            self.fs.insert(path.into(), content);
            sync_resolver_paths(
                self,
                [(
                    path.into(),
                    ResolverPathChange::Kind | ResolverPathChange::Content,
                )],
            );
        }

        /// Removes `path` from the filesystem and reports the change.
        fn remove(&mut self, path: &str) {
            self.fs.remove(Utf8Path::new(path));
            sync_resolver_paths(self, [(path.into(), ResolverPathChange::Kind.into())]);
        }

        /// Indexes `source` for `path` and reports the new parsed source.
        fn index(&mut self, path: &str, source: ParsedSource) {
            self.files.lock().unwrap().insert(path.into(), source);
            sync_resolver_paths(self, [(path.into(), ResolverPathChange::Content.into())]);
        }
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl Db for TestDb {
        fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<ParsedSource> {
            self.files.lock().unwrap().get(path).copied()
        }
    }

    #[salsa::db]
    impl ResolverDb for TestDb {
        fn resolver_fs(&self) -> &dyn ResolverFsProxy {
            &self.fs
        }

        fn resolver_paths(&self) -> &ResolverPaths {
            &self.paths
        }
    }

    fn parse(source: &str) -> JsonParse {
        parse_json(source, JsonParserOptions::default())
    }

    fn package_name(db: &TestDb, data: Option<PackageJsonData>) -> Option<String> {
        data?
            .manifest(db)
            .as_ref()
            .and_then(|manifest| manifest.name.as_ref().map(ToString::to_string))
    }

    #[test]
    fn package_json_query_tracks_the_parsed_source() {
        let mut db = TestDb::new();
        let source = ParsedSource::new(
            &db,
            Utf8PathBuf::from("/package.json"),
            parse(r#"{"name":"first"}"#).into(),
            0,
            Vec::new(),
        );

        assert_eq!(
            package_name(&db, Some(package_json_from_source(&db, source))),
            Some("first".into())
        );
        db.clear_events();

        let _ = package_json_from_source(&db, source);
        let events = db.take_events();
        assert_function_query_was_not_run(&db, package_json_from_source, source, &events);

        source
            .set_parsed(&mut db)
            .to(parse(r#"{"name":"second"}"#).into());
        db.clear_events();

        assert_eq!(
            package_name(&db, Some(package_json_from_source(&db, source))),
            Some("second".into())
        );
        let events = db.take_events();
        assert_function_query_was_run(&db, package_json_from_source, source, &events);
    }

    #[test]
    fn missing_path_is_stored_and_invalidated_when_created() {
        let mut db = TestDb::new();
        let manifest = db.path("/package.json");
        assert_eq!(manifest.info(&db), &Err(ResolveError::NotFound));
        assert!(package_json_for_path(&db, manifest).is_none());
        db.clear_events();

        let _ = package_json_for_path(&db, manifest);
        let events = db.take_events();
        assert_function_query_was_not_run(&db, package_json_for_path, manifest, &events);

        db.write("/package.json", r#"{"name":"created"}"#);
        db.clear_events();

        assert_eq!(db.path("/package.json"), manifest, "the input is stable");
        assert_eq!(
            package_name(&db, package_json_for_path(&db, manifest)),
            Some("created".into())
        );
        let events = db.take_events();
        assert_function_query_was_run(&db, package_json_for_path, manifest, &events);
    }

    #[test]
    fn content_change_reported_for_an_unindexed_manifest_invalidates_it() {
        let mut db = TestDb::new();
        db.fs
            .insert("/package.json".into(), r#"{"name":"first"}"#.as_bytes());
        let manifest = db.path("/package.json");
        assert_eq!(
            package_name(&db, package_json_for_path(&db, manifest)),
            Some("first".into())
        );

        db.write("/package.json", r#"{"name":"second"}"#);

        assert_eq!(
            package_name(&db, package_json_for_path(&db, manifest)),
            Some("second".into())
        );
    }

    #[test]
    fn indexed_manifest_takes_precedence_over_the_filesystem() {
        let mut db = TestDb::new();
        db.fs
            .insert("/package.json".into(), r#"{"name":"on-disk"}"#.as_bytes());
        let manifest = db.path("/package.json");
        assert_eq!(
            package_name(&db, package_json_for_path(&db, manifest)),
            Some("on-disk".into())
        );

        let source = ParsedSource::new(
            &db,
            Utf8PathBuf::from("/package.json"),
            parse(r#"{"name":"unsaved"}"#).into(),
            0,
            Vec::new(),
        );
        db.index("/package.json", source);

        assert_eq!(
            package_name(&db, package_json_for_path(&db, manifest)),
            Some("unsaved".into())
        );
    }

    #[test]
    fn sync_skips_unchanged_and_unknown_paths() {
        let mut db = TestDb::new();
        db.fs.insert("/src/a.ts".into(), "".as_bytes());
        let known = db.path("/src/a.ts");
        assert_eq!(known.info(&db), &Ok(PathInfo::File));

        assert!(!resolver_paths_need_sync(
            &db,
            [(Utf8Path::new("/src/a.ts"), ResolverPathChange::Kind.into())]
        ));
        assert!(!resolver_paths_need_sync(
            &db,
            [(
                Utf8Path::new("/src/unknown.ts"),
                ResolverPathChange::Content.into()
            )]
        ));
        assert!(resolver_paths_need_sync(
            &db,
            [(
                Utf8Path::new("/src/a.ts"),
                ResolverPathChange::Content.into()
            )]
        ));

        db.remove("/src/a.ts");
        assert_eq!(known.info(&db), &Err(ResolveError::NotFound));
        assert!(db.paths.get(Utf8Path::new("/src/unknown.ts")).is_none());
    }

    #[test]
    fn creating_a_file_refreshes_its_new_parent_directory() {
        let mut db = TestDb::new();
        let directory = db.path("/src/new");
        let file = db.path("/src/new/index.ts");
        assert_eq!(directory.info(&db), &Err(ResolveError::NotFound));

        db.fs.insert("/src/new/index.ts".into(), "".as_bytes());
        sync_resolver_paths(
            &mut db,
            [("/src/new/index.ts".into(), ResolverPathChange::Kind.into())],
        );

        assert_eq!(file.info(&db), &Ok(PathInfo::File));
        assert_eq!(directory.info(&db), &Ok(PathInfo::Directory));
    }

    #[test]
    fn creating_an_unknown_file_refreshes_its_missing_ancestors() {
        let mut db = TestDb::new();
        let package = db.path("/node_modules/dependency");
        assert_eq!(package.info(&db), &Err(ResolveError::NotFound));

        db.fs
            .insert("/node_modules/dependency/index.ts".into(), "".as_bytes());
        assert!(resolver_paths_need_sync(
            &db,
            [(
                Utf8Path::new("/node_modules/dependency/index.ts"),
                ResolverPathChange::Kind.into()
            )]
        ));
        sync_resolver_paths(
            &mut db,
            [(
                "/node_modules/dependency/index.ts".into(),
                ResolverPathChange::Kind.into(),
            )],
        );

        assert_eq!(package.info(&db), &Ok(PathInfo::Directory));
        assert!(
            db.paths
                .get(Utf8Path::new("/node_modules/dependency/index.ts"))
                .is_none()
        );
    }

    #[test]
    fn nearest_package_lookup_invalidates_when_a_closer_package_is_created() {
        let mut db = TestDb::new();
        db.fs
            .insert("/package.json".into(), r#"{"name":"root"}"#.as_bytes());
        let directory = db.path("/project/src");
        let package = package_json_for_directory(&db, directory)
            .expect("the root package manifest must be found");
        assert_eq!(package.path(&db), Utf8Path::new("/"));

        db.write("/project/package.json", r#"{"name":"project"}"#);
        db.clear_events();

        let package = package_json_for_directory(&db, directory)
            .expect("the closer package manifest must be found");
        assert_eq!(package.path(&db), Utf8Path::new("/project"));
        let events = db.take_events();
        assert_function_query_was_run(&db, package_json_for_directory, directory, &events);
    }

    #[test]
    fn database_adapter_reuses_path_info() {
        let mut db = TestDb::new();
        db.fs.insert("/src/dependency.ts".into(), "".as_bytes());
        let options = crate::ResolveOptions {
            extensions: &["ts"],
            ..Default::default()
        };

        let resolve = |db: &TestDb| {
            crate::resolve(
                "./dependency",
                Utf8Path::new("/src"),
                &ResolverDbAdapter::new(db),
                &options,
            )
        };
        assert_eq!(resolve(&db), Ok(Utf8PathBuf::from("/src/dependency.ts")));
        let known = db.paths.paths_within(Utf8Path::new("/"));

        db.fs.remove(Utf8Path::new("/src/dependency.ts"));
        assert_eq!(
            resolve(&db),
            Ok(Utf8PathBuf::from("/src/dependency.ts")),
            "path info is not refreshed without a sync"
        );
        assert_eq!(db.paths.paths_within(Utf8Path::new("/")), known);
        db.clear_events();

        sync_resolver_paths(
            &mut db,
            [("/src/dependency.ts".into(), ResolverPathChange::Kind.into())],
        );
        assert_eq!(resolve(&db), Err(ResolveError::NotFound));
        let events = db.take_events();
        assert_eq!(
            function_query_will_execute_count_by_name(&db, "package_json_for_directory", &events),
            0,
            "removing a source file doesn't invalidate the manifest lookup"
        );
    }
}
