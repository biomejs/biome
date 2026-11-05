use super::*;
use crate::test_utils::setup_workspace_and_open_project;
use biome_fs::MemoryFileSystem;
use biome_plugin_loader::{PluginConfiguration, Plugins};

#[test]
fn missing_plugin_cache_reports_configured_plugins() {
    let (workspace, _) = setup_workspace_and_open_project(MemoryFileSystem::default(), "/project");
    let workspace = workspace.as_workspace();

    let plugins = workspace
        .get_analyzer_plugins_for_project(Utf8Path::new("/project"), &Plugins::default())
        .unwrap();
    assert!(plugins.is_empty());

    let plugins = Plugins(vec![PluginConfiguration::Path(
        "/project/plugin.grit".to_string(),
    )]);
    let diagnostics = workspace
        .get_analyzer_plugins_for_project(Utf8Path::new("/project"), &plugins)
        .unwrap_err();

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].to_string(),
        "Plugin is requested but not loaded: /project/plugin.grit"
    );
}
