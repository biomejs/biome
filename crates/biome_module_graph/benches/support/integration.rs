use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_languages::JsFileSource;
use biome_module_graph::{
    ModuleDb, ModuleInfo, ModuleInfoKind, PathInfoCache, TypeInferenceMode,
    resolve_js_module_with_inference_mode,
};
use biome_project_layout::ProjectLayout;
use biome_service::db::WorkspaceDb;
use std::sync::Arc;

pub const ZOD_TANSTACK_FORM_FILES: &[(&str, &str)] = &[
    (
        "/vendor/zod.d.ts",
        include_str!("../fixtures/zod_tanstack_form/vendor/zod.d.ts"),
    ),
    (
        "/vendor/tanstack-form.d.ts",
        include_str!("../fixtures/zod_tanstack_form/vendor/tanstack-form.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/zod_tanstack_form/schema.ts"),
    ),
    (
        "/form.ts",
        include_str!("../fixtures/zod_tanstack_form/form.ts"),
    ),
];

pub const ZOD_TANSTACK_FORM_CASES: &[&str] = &["/schema.ts", "/form.ts"];

pub const SVELTE_VALIBOT_FILES: &[(&str, &str)] = &[
    (
        "/vendor/svelte-store.d.ts",
        include_str!("../fixtures/svelte_valibot/vendor/svelte-store.d.ts"),
    ),
    (
        "/vendor/valibot.d.ts",
        include_str!("../fixtures/svelte_valibot/vendor/valibot.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/svelte_valibot/schema.ts"),
    ),
    (
        "/stores.ts",
        include_str!("../fixtures/svelte_valibot/stores.ts"),
    ),
];

pub const TYPE_CHALLENGES_FILES: &[(&str, &str)] = &[(
    "/query_string_parser.ts",
    include_str!("../fixtures/type_challenges/query_string_parser.ts"),
)];

pub const DRIZZLE_TYPEBOX_FILES: &[(&str, &str)] = &[
    (
        "/vendor/drizzle-typebox.d.ts",
        include_str!("../fixtures/drizzle_typebox/vendor/drizzle-typebox.d.ts"),
    ),
    (
        "/vendor/typebox.d.ts",
        include_str!("../fixtures/drizzle_typebox/vendor/typebox.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/drizzle_typebox/schema.ts"),
    ),
    (
        "/queries.ts",
        include_str!("../fixtures/drizzle_typebox/queries.ts"),
    ),
];

pub fn build_db(files: &[(&str, &str)], entry: &str) -> (WorkspaceDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert((*path).into(), *source);
    }
    let db = WorkspaceDb::default();
    let layout = ProjectLayout::default();
    let cache = PathInfoCache::default();
    for (name, source) in files {
        let path = BiomePath::new(*name);
        let parsed = biome_js_parser::parse(
            source,
            JsFileSource::try_from(path.as_path()).expect("fixture must be TypeScript"),
            JsParserOptions::default(),
        );
        assert!(
            parsed.diagnostics().is_empty(),
            "{name}: {:?}",
            parsed.diagnostics()
        );
        let root = parsed.tree();
        let semantic = Arc::new(semantic_model(&root, SemanticModelOptions::default()));
        let (info, _, _) = resolve_js_module_with_inference_mode(
            root,
            &path,
            &fs,
            &layout,
            semantic,
            &cache,
            TypeInferenceMode::RawTypesOnly,
        );
        let module = ModuleInfo::new(&db, path.as_path().to_path_buf(), ModuleInfoKind::Js(info));
        db.modules
            .pin()
            .insert(path.as_path().to_path_buf(), module);
    }
    for (name, _) in files {
        let info = db
            .js_module_info_for_path(camino::Utf8Path::new(name))
            .expect("fixture module must exist");
        for import in info.all_import_paths() {
            let path = import.resolved_path.as_path().expect("import must resolve");
            assert!(db.module_for_path(path).is_some(), "missing module: {path}");
        }
    }
    let module = db
        .module_for_path(camino::Utf8Path::new(entry))
        .expect("entry module must exist");
    (db, module)
}
