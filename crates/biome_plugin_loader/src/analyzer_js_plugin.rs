use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use boa_engine::object::builtins::JsFunction;
use boa_engine::{JsNativeError, JsResult, JsString, JsValue};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

use biome_analyze::{AnalyzerPlugin, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::category;
use biome_js_runtime::JsExecContext;
use biome_parser::AnyParse;
use biome_resolver::FsWithResolverProxy;

use crate::PluginDiagnostic;

/// The global atomic store to generate a unique plugin ID.
static PLUGIN_ID: AtomicUsize = AtomicUsize::new(0);

/// A unique ID of the JS plugin across threads.
/// The same plugin will have the same ID, even in the different threads.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct JsPluginId(usize);

impl JsPluginId {
    /// Generate a unique plugin ID.
    fn new() -> Self {
        Self(PLUGIN_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Parameters for initialising a plugin in a thread.
struct JsPluginInit {
    id: JsPluginId,
    fs: Arc<dyn FsWithResolverProxy>,
    path: Utf8PathBuf,
}

impl Debug for JsPluginInit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"id", &self.id)
            .entry(&"path", &self.path)
            .finish()
    }
}

/// Already loaded plugin in a thread.
/// These values can't be shared with another threads.
struct LoadedPlugin {
    ctx: JsExecContext,
    entrypoint: JsFunction,
}

thread_local! {
    static PLUGINS: RefCell<FxHashMap<JsPluginId, LoadedPlugin>> = RefCell::new(FxHashMap::default());
}

fn load_plugin(init: &JsPluginInit) -> JsResult<LoadedPlugin> {
    let mut ctx = JsExecContext::new(init.fs.clone())?;
    let module = ctx.import_module(&init.path)?;
    let entrypoint = ctx.get_default_export(&module)?;

    let Some(entrypoint) = entrypoint.as_function() else {
        return Err(JsNativeError::typ()
            .with_message("The plugin entrypoint must be a function")
            .into());
    };

    Ok(LoadedPlugin { ctx, entrypoint })
}

/// Execute a function after loaded a plugin, or re-use the instance if the plugin is already loaded
/// in the thread.
fn with_plugin<F, R>(init: &JsPluginInit, f: F) -> JsResult<R>
where
    F: FnOnce(&mut LoadedPlugin) -> JsResult<R>,
{
    PLUGINS.with_borrow_mut(|plugins| {
        let plugin = match plugins.entry(init.id) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(load_plugin(init)?),
        };

        f(plugin)
    })
}

/// Unload all loaded plugin in the current thread.
#[allow(dead_code)]
fn unload_plugins() {
    PLUGINS.with_borrow_mut(|plugins| std::mem::take(plugins));
}

/// A JS analyzer plugin.
/// As the JS engine is intended to run in single thread, plugins are lazily loaded in each thread
/// just before executing it.
#[derive(Debug)]
pub struct AnalyzerJsPlugin {
    init: JsPluginInit,
}

impl AnalyzerJsPlugin {
    pub fn load(
        fs: Arc<dyn FsWithResolverProxy>,
        path: &Utf8Path,
    ) -> Result<Self, PluginDiagnostic> {
        let id = JsPluginId::new();
        let init = JsPluginInit {
            id,
            fs,
            path: path.to_owned(),
        };

        // Load the plugin in the main thread here to catch errors while loading.
        let _ = load_plugin(&init);

        Ok(Self { init })
    }
}

impl AnalyzerPlugin for AnalyzerJsPlugin {
    fn evaluate(&self, _root: AnyParse, path: Arc<Utf8PathBuf>) -> Vec<RuleDiagnostic> {
        let result = with_plugin(&self.init, |plugin| {
            // TODO: pass the AST to the plugin
            let _ = plugin.ctx.call_function(
                &plugin.entrypoint,
                &JsValue::undefined(),
                &[JsValue::String(JsString::from(path.as_str()))],
            )?;

            Ok(plugin.ctx.pull_diagnostics())
        });

        result.unwrap_or_else(|err| {
            vec![RuleDiagnostic::new(
                category!("plugin"),
                None::<TextRange>,
                markup!("Plugin errored: "<Error>{err.to_string()}</Error>),
            )]
        })
    }

    fn supports_css(&self) -> bool {
        false
    }

    fn supports_js(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
    use biome_fs::MemoryFileSystem;
    use biome_js_parser::{JsFileSource, JsParserOptions};

    fn snap_diagnostics(test_name: &str, diagnostics: Vec<Error>) {
        let content = diagnostics
            .iter()
            .map(|err| print_diagnostic_to_string(err))
            .collect::<Vec<_>>()
            .join("");

        // Normalize Windows paths...
        let content = content.replace('\\', "/");

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    #[test]
    fn evaluate_in_worker_threads() {
        let fs = MemoryFileSystem::default();
        fs.insert("/foo.js".into(), "let foo;");
        fs.insert("/bar.js".into(), "let bar;");
        fs.insert(
            "/plugin.js".into(),
            r#"import { addDiagnostic } from "@biomejs/plugin-api";
            export default function useMyPlugin() {
                addDiagnostic("information", "Hello, world!");
            }"#,
        );

        let fs = Arc::new(fs) as Arc<dyn FsWithResolverProxy>;
        let plugin = Arc::new(AnalyzerJsPlugin::load(fs.clone(), "/plugin.js".into()).unwrap());

        let worker1 = {
            let plugin = plugin.clone();

            std::thread::spawn(move || {
                let parse = biome_js_parser::parse(
                    "let foo;",
                    JsFileSource::js_module(),
                    JsParserOptions::default(),
                );

                let diagnostics = plugin.evaluate(parse.into(), Arc::new("/foo.js".into()));

                // FIXME: Unload plugins before exiting the thread to avoid heap corruption.
                unload_plugins();

                diagnostics
            })
        };

        let worker2 = {
            let plugin = plugin.clone();

            std::thread::spawn(move || {
                let parse = biome_js_parser::parse(
                    "let bar;",
                    JsFileSource::js_module(),
                    JsParserOptions::default(),
                );

                let diagnostics = plugin.evaluate(parse.into(), Arc::new("/bar.js".into()));

                // FIXME: Unload plugins before exiting the thread to avoid heap corruption.
                unload_plugins();

                diagnostics
            })
        };

        let mut diagnostics = worker1.join().unwrap();
        diagnostics.extend(worker2.join().unwrap());

        assert_eq!(diagnostics.len(), 2);
        snap_diagnostics(
            "evaluate_in_worker_threads",
            diagnostics.into_iter().map(|diag| diag.into()).collect(),
        );
    }
}
