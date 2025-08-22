use std::cell::RefCell;
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

/// Execute a function after loaded a plugin, or re-use the instance if the plugin is already loaded
/// in the thread.
fn with_plugin<F, R>(init: &JsPluginInit, f: F) -> JsResult<R>
where
    F: FnOnce(&mut LoadedPlugin) -> JsResult<R>,
{
    PLUGINS.with_borrow_mut(|plugins| {
        let plugin = match plugins.get_mut(&init.id) {
            Some(p) => p,
            _ => {
                let mut ctx = JsExecContext::new(init.fs.clone())?;
                let module = ctx.import_module(&init.path)?;
                let entrypoint = ctx.get_default_export(&module)?;

                if let JsValue::Object(entrypoint) = entrypoint
                    && let Some(entrypoint) = JsFunction::from_object(entrypoint)
                {
                    let plugin = LoadedPlugin { ctx, entrypoint };

                    plugins.insert(init.id, plugin);
                } else {
                    return Err(JsNativeError::typ()
                        .with_message("The plugin entrypoint must be a function")
                        .into());
                }

                plugins.get_mut(&init.id).unwrap()
            }
        };

        f(plugin)
    })
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
        with_plugin(&init, |_plugin| Ok(()))?;

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
