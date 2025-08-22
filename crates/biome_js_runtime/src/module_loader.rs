use std::cell::RefCell;
use std::sync::Arc;

use boa_engine::module::{ModuleLoader, Referrer};
use boa_engine::{Context, JsNativeError, JsResult, JsString, Module, Source};
use camino::Utf8Path;
use rustc_hash::FxHashMap;

use biome_resolver::{FsWithResolverProxy, ResolveOptions, resolve};

pub struct JsModuleLoader {
    fs: Arc<dyn FsWithResolverProxy>,
    modules: RefCell<FxHashMap<JsString, Module>>,
}

impl JsModuleLoader {
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self {
            fs,
            modules: Default::default(),
        }
    }
}

impl ModuleLoader for JsModuleLoader {
    fn load_imported_module(
        &self,
        referrer: Referrer,
        specifier: JsString,
        finish_load: Box<dyn FnOnce(JsResult<Module>, &mut Context)>,
        context: &mut Context,
    ) {
        if let Some(module) = self.modules.borrow().get(&specifier) {
            finish_load(Ok(module.clone()), context);
            return;
        }

        let specifier = specifier.to_std_string_lossy();

        let base_dir = referrer
            .path()
            .and_then(|path| path.parent())
            .and_then(Utf8Path::from_path)
            .map(Utf8Path::to_path_buf)
            .or_else(|| self.fs.working_directory())
            .unwrap_or_default();

        let options = ResolveOptions {
            ..Default::default()
        };

        match resolve(&specifier, &base_dir, self.fs.as_ref(), &options) {
            Ok(path) => {
                let source = self.fs.read_file_from_path(&path);
                match source {
                    Ok(source) => {
                        let source = Source::from_bytes(source.as_bytes());
                        let module = Module::parse(source, None, context);
                        finish_load(module, context);
                    }
                    Err(err) => finish_load(
                        Err(JsNativeError::error().with_message(err.to_string()).into()),
                        context,
                    ),
                }
            }
            Err(err) => finish_load(
                Err(JsNativeError::error().with_message(err.to_string()).into()),
                context,
            ),
        }
    }

    fn register_module(&self, specifier: JsString, module: Module) {
        self.modules.borrow_mut().insert(specifier, module);
    }

    fn get_module(&self, specifier: JsString) -> Option<Module> {
        self.modules.borrow().get(&specifier).cloned()
    }
}
