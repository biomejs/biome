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
fn astro_component_facts_update_after_edits() {
    let (workspace, project_key) =
        setup_workspace_and_open_project(MemoryFileSystem::default(), "/project");
    let configuration: Configuration = serde_json::from_value(serde_json::json!({
        "html": { "experimentalFullSupportEnabled": true },
        "linter": { "rules": {
            "preset": "none",
            "nursery": { "noAstroClientDirectiveOnAstroComponent": "error", "noAstroUnserializableClientProps": "error" }
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
        ("---\nimport Card from './Card.astro';\nconst click = () => {};\n---\n<Card client:load onClick={click} />", vec![("noAstroClientDirectiveOnAstroComponent", "client:load"), ("noAstroUnserializableClientProps", "click} ")]),
        ("---\nimport Card from './Card.jsx';\nconst click = 'text';\n---\n<Card client:load onClick={click} />", vec![]),
        ("---\r\n// 日本語\r\nimport Card from './Card.astro';\r\nconst click = () => {};\r\n---\r\n<Card client:load onClick={click} />", vec![("noAstroClientDirectiveOnAstroComponent", "client:load"), ("noAstroUnserializableClientProps", "click} ")]),
        ("---\nimport Card from './Card.astro';\nconst click = () => {};\n---\n<Card onClick={click} />", vec![]),
        ("<Button client:load onClick={() => {}} />", vec![("noAstroUnserializableClientProps", "() => {}")]),
        ("<p>日本語</p>\r\n<Button client:load onClick={() => {}} />", vec![("noAstroUnserializableClientProps", "() => {}")]),
        ("---\nfunction click() {}\n---\n{click = null}\n<Button client:load onClick={click} />", vec![]),
        ("---\nfunction click() {}\n---\n<Button client:load onClick={click} />", vec![("noAstroUnserializableClientProps", "click} ")]),
        ("<p>Removed component</p>", vec![]),
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
        for (rule, value) in imports {
            let category = format!("lint/nursery/{rule}");
            let diagnostic = result.diagnostics.iter().find(|diagnostic| diagnostic.category().unwrap().name() == category).unwrap();
            assert_eq!(diagnostic.severity(), Severity::Error);
            assert_eq!(diagnostic.location().span, Some(TextRange::at(
                TextSize::from(source.find(value).unwrap() as u32), TextSize::of(value.trim_end_matches("} ")),
            )));
        }
    }
}
