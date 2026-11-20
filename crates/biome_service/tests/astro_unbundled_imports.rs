#![cfg(feature = "html_embeds")]

use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::Configuration;
use biome_diagnostics::{Diagnostic, Severity};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_rowan::{TextRange, TextSize};
use biome_service::Workspace;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::test_utils::setup_workspace_and_open_project;
use biome_service::workspace::{
    ChangeFileParams, FileContent, OpenFileParams, PullDiagnosticsParams, UpdateSettingsParams,
};

#[test]
fn no_astro_unbundled_imports_updates_after_edits() {
    let (workspace, project_key) =
        setup_workspace_and_open_project(MemoryFileSystem::default(), "/project");
    let configuration: Configuration = serde_json::from_value(serde_json::json!({
        "html": { "experimentalFullSupportEnabled": true },
        "linter": { "rules": {
            "preset": "none",
            "nursery": { "noAstroUnbundledImports": "error" }
        }}
    }))
    .unwrap();
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: Some(BiomePath::new("/project")),
            configuration,
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();
    let path = BiomePath::new("/project/file.astro");
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: path.clone(),
            content: FileContent::FromClient {
                content: String::new(),
                version: 0,
            },
            document_file_source: None,
            persist_node_cache: true,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    for (version, (source, imports)) in [
        ("<script define:vars={{ value }}>import './first.js';</script>", vec!["import './first.js';"]),
        ("<p>日本語</p>\r\n<script define:vars={{ value }}>import './first.js';</script>", vec!["import './first.js';"]),
        ("<p>日本語</p>\r\n<script is:inline>\r\nimport './first.js';\r\nimport './second.js';\r\n</script>", vec!["import './first.js';", "import './second.js';"]),
        ("<script is:inline type=\"module\">import './first.js';</script>", vec![]),
        ("<script>import './first.js';</script>", vec![]),
        ("<script is:inline>import('./first.js');</script>", vec![]),
        ("<script is:inline>import './restored.js';</script>", vec!["import './restored.js';"]),
        ("<div>Removed script</div>", vec![]),
    ].into_iter().enumerate() {
        workspace.change_file(ChangeFileParams {
            project_key,
            path: path.clone(),
            content: source.into(),
            version: version as i32 + 1,
            inline_config: None,
            editor_features: None,
        }).unwrap();
        let result = workspace.pull_diagnostics(PullDiagnosticsParams {
            project_key,
            path: path.clone(),
            categories: RuleCategoriesBuilder::default().with_lint().build(),
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            include_code_fix: false,
            inline_config: None,
            max_diagnostics: None,
            diagnostic_level: Severity::Hint,
            enforce_assist: false,
        }).unwrap();
        assert_eq!(result.diagnostics.len(), imports.len(), "{source}\n{:#?}", result.diagnostics);
        for (diagnostic, import) in result.diagnostics.iter().zip(imports) {
            assert_eq!(diagnostic.category().unwrap().name(), "lint/nursery/noAstroUnbundledImports");
            assert_eq!(diagnostic.severity(), Severity::Error);
            assert_eq!(diagnostic.location().span, Some(TextRange::at(
                TextSize::from(source.find(import).unwrap() as u32), TextSize::of(import),
            )));
        }
    }
}
