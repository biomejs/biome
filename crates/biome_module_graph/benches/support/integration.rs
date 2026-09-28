use biome_fs::{BiomePath, MemoryFileSystem};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_languages::JsFileSource;
use biome_module_graph::{
    ModuleDb, ModuleInfo, ModuleInfoKind, TypeInferenceMode, resolve_js_module_with_inference_mode,
};
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

pub const EFFECT_ARKTYPE_FILES: &[(&str, &str)] = &[
    (
        "/vendor/effect.d.ts",
        include_str!("../fixtures/effect_arktype/vendor/effect.d.ts"),
    ),
    (
        "/vendor/arktype.d.ts",
        include_str!("../fixtures/effect_arktype/vendor/arktype.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/effect_arktype/schema.ts"),
    ),
    (
        "/program.ts",
        include_str!("../fixtures/effect_arktype/program.ts"),
    ),
];

pub const EFFECT_ARKTYPE_CASES: &[&str] = &["/schema.ts", "/program.ts"];

pub const KYSELY_TS_PATTERN_FILES: &[(&str, &str)] = &[
    (
        "/vendor/kysely.d.ts",
        include_str!("../fixtures/kysely_ts_pattern/vendor/kysely.d.ts"),
    ),
    (
        "/vendor/ts-pattern.d.ts",
        include_str!("../fixtures/kysely_ts_pattern/vendor/ts-pattern.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/kysely_ts_pattern/schema.ts"),
    ),
    (
        "/queries.ts",
        include_str!("../fixtures/kysely_ts_pattern/queries.ts"),
    ),
    (
        "/reports.ts",
        include_str!("../fixtures/kysely_ts_pattern/reports.ts"),
    ),
];

pub const KYSELY_TS_PATTERN_CASES: &[&str] = &["/queries.ts", "/reports.ts"];

pub const HONO_VALIBOT_FILES: &[(&str, &str)] = &[
    (
        "/vendor/valibot.d.ts",
        include_str!("../fixtures/hono_valibot/vendor/valibot.d.ts"),
    ),
    (
        "/vendor/hono.d.ts",
        include_str!("../fixtures/hono_valibot/vendor/hono.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/hono_valibot/schema.ts"),
    ),
    (
        "/context.ts",
        include_str!("../fixtures/hono_valibot/context.ts"),
    ),
    (
        "/server.ts",
        include_str!("../fixtures/hono_valibot/server.ts"),
    ),
    (
        "/client.ts",
        include_str!("../fixtures/hono_valibot/client.ts"),
    ),
];

pub const HONO_VALIBOT_CASES: &[&str] = &["/server.ts", "/client.ts"];

pub const TRPC_ZOD_QUERY_FILES: &[(&str, &str)] = &[
    (
        "/vendor/zod.d.ts",
        include_str!("../fixtures/trpc_zod_query/vendor/zod.d.ts"),
    ),
    (
        "/vendor/tanstack-query.d.ts",
        include_str!("../fixtures/trpc_zod_query/vendor/tanstack-query.d.ts"),
    ),
    (
        "/vendor/trpc-server.d.ts",
        include_str!("../fixtures/trpc_zod_query/vendor/trpc-server.d.ts"),
    ),
    (
        "/vendor/trpc-client.d.ts",
        include_str!("../fixtures/trpc_zod_query/vendor/trpc-client.d.ts"),
    ),
    (
        "/vendor/trpc-tanstack-react-query.d.ts",
        include_str!("../fixtures/trpc_zod_query/vendor/trpc-tanstack-react-query.d.ts"),
    ),
    (
        "/schema.ts",
        include_str!("../fixtures/trpc_zod_query/schema.ts"),
    ),
    (
        "/trpc.ts",
        include_str!("../fixtures/trpc_zod_query/trpc.ts"),
    ),
    (
        "/router.ts",
        include_str!("../fixtures/trpc_zod_query/router.ts"),
    ),
    (
        "/client.ts",
        include_str!("../fixtures/trpc_zod_query/client.ts"),
    ),
];

pub const TRPC_ZOD_QUERY_CASES: &[&str] = &["/router.ts", "/client.ts"];

pub fn build_db(files: &[(&str, &str)], entry: &str) -> (WorkspaceDb, ModuleInfo) {
    let fs = MemoryFileSystem::default();
    for (path, source) in files {
        fs.insert((*path).into(), *source);
    }
    let db = WorkspaceDb::new(Arc::new(fs));
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
            &db,
            root,
            &path,
            semantic,
            TypeInferenceMode::RawTypesOnly,
        );
        let module = ModuleInfo::new(&db, path.as_path().to_path_buf(), ModuleInfoKind::Js(info));
        db.modules
            .pin()
            .insert(path.as_path().to_path_buf(), module);
    }
    for (name, _) in files {
        let module = db
            .module_for_path(camino::Utf8Path::new(name))
            .expect("fixture module must exist");
        let ModuleInfoKind::Js(info) = module.kind(&db) else {
            panic!("fixture module must be JavaScript");
        };
        for import in info.all_import_paths() {
            let path = import
                .resolve_js(&db, module)
                .path()
                .as_path()
                .expect("import must resolve");
            assert!(db.module_for_path(path).is_some(), "missing module: {path}");
        }
    }
    let module = db
        .module_for_path(camino::Utf8Path::new(entry))
        .expect("entry module must exist");
    (db, module)
}
