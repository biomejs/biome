use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;
use std::sync::Arc;

use boa_engine::object::builtins::JsFunction;
use boa_engine::{JsNativeError, JsResult, JsString, JsValue};
use camino::{Utf8Path, Utf8PathBuf};

use biome_analyze::{AnalyzerPlugin, PluginTargetLanguage, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_js_runtime::JsExecContext;
use biome_js_syntax::AnyJsRoot;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AnySyntaxNode, AstNode, RawSyntaxKind, SyntaxKind};
use biome_text_size::TextRange;

use crate::PluginDiagnostic;
use crate::thread_local::ThreadLocalCell;

/// Already loaded plugin in a thread.
/// These values can't be shared with another threads.
struct LoadedPlugin {
    ctx: JsExecContext,
    entrypoint: JsFunction,
}

fn load_plugin(fs: Arc<dyn FsWithResolverProxy>, path: &Utf8Path) -> JsResult<LoadedPlugin> {
    let mut ctx = JsExecContext::new(fs)?;
    let module = ctx.import_module(path)?;
    let entrypoint = ctx.get_default_export(&module)?;

    let Some(entrypoint) = entrypoint.as_function() else {
        return Err(JsNativeError::typ()
            .with_message("The plugin entrypoint must be a function")
            .into());
    };

    Ok(LoadedPlugin { ctx, entrypoint })
}

/// A JS analyzer plugin.
/// As the JS engine is intended to run in single thread, plugins are lazily loaded in each thread
/// just before executing it.
pub struct AnalyzerJsPlugin {
    fs: Arc<dyn FsWithResolverProxy>,
    path: Utf8PathBuf,
    loaded: ThreadLocalCell<LoadedPlugin>,
}

impl Debug for AnalyzerJsPlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnalyzerJsPlugin")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl AnalyzerJsPlugin {
    pub fn load(
        fs: Arc<dyn FsWithResolverProxy>,
        path: &Utf8Path,
    ) -> Result<Self, PluginDiagnostic> {
        // Load the plugin in the main thread here to catch errors while loading.
        load_plugin(fs.clone(), path)?;

        Ok(Self {
            fs,
            path: path.to_owned(),
            loaded: ThreadLocalCell::new(),
        })
    }
}

impl AnalyzerPlugin for AnalyzerJsPlugin {
    fn language(&self) -> PluginTargetLanguage {
        PluginTargetLanguage::JavaScript
    }

    fn query(&self) -> Vec<RawSyntaxKind> {
        // TODO: Support granular query defined in the JS plugin.
        AnyJsRoot::KIND_SET
            .iter()
            .map(|kind| kind.to_raw())
            .collect()
    }

    fn evaluate(&self, _node: AnySyntaxNode, path: Arc<Utf8PathBuf>) -> Vec<RuleDiagnostic> {
        let mut plugin = match self
            .loaded
            .get_mut_or_try_init(|| load_plugin(self.fs.clone(), &self.path))
        {
            Ok(plugin) => plugin,
            Err(err) => {
                return vec![RuleDiagnostic::new(
                    category!("plugin"),
                    None::<TextRange>,
                    markup!("Could not load the plugin: "<Error>{err.to_string()}</Error>),
                )];
            }
        };

        let plugin = plugin.deref_mut();

        // TODO: pass the AST to the plugin
        plugin
            .ctx
            .call_function(
                &plugin.entrypoint,
                &JsValue::undefined(),
                &[JsValue::String(JsString::from(path.as_str()))],
            )
            .map_or_else(
                |err| {
                    vec![RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!("Plugin errored: "<Error>{err.to_string()}</Error>),
                    )]
                },
                |_| plugin.ctx.pull_diagnostics(),
            )
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
            .map(print_diagnostic_to_string)
            .collect::<String>();

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
            r#"import { registerDiagnostic } from "@biomejs/plugin-api";
            export default function useMyPlugin() {
                registerDiagnostic("information", "Hello, world!");
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

                plugin.evaluate(parse.syntax().into(), Arc::new("/foo.js".into()))
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

                plugin.evaluate(parse.syntax().into(), Arc::new("/bar.js".into()))
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
