use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use boa_engine::module::{ModuleLoader, Referrer};
use boa_engine::{JsNativeError, JsResult, JsString, Module, Source};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

use biome_resolver::{FsWithResolverProxy, ResolveOptions, resolve};

pub struct JsModuleLoader {
    fs: Arc<dyn FsWithResolverProxy>,
    builtins: RefCell<FxHashMap<JsString, Module>>,
    modules: RefCell<FxHashMap<Utf8PathBuf, Module>>,
}

impl JsModuleLoader {
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> Self {
        Self {
            fs,
            builtins: Default::default(),
            modules: Default::default(),
        }
    }

    pub(crate) fn register_module(&self, specifier: JsString, module: Module) {
        self.builtins.borrow_mut().insert(specifier, module);
    }
}

impl ModuleLoader for JsModuleLoader {
    async fn load_imported_module(
        self: Rc<JsModuleLoader>,
        referrer: Referrer,
        specifier: JsString,
        context: &RefCell<&mut boa_engine::Context>,
    ) -> JsResult<Module> {
        if let Some(module) = self.builtins.borrow().get(&specifier) {
            return Ok(module.clone());
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
                if let Some(module) = self.modules.borrow().get(&path).cloned() {
                    return Ok(module);
                }

                let source = self.fs.read_file_from_path(&path);
                match source {
                    Ok(source) => {
                        let source = source.as_bytes();
                        let source = Source::from_bytes(source).with_path(path.as_std_path());
                        let module = Module::parse(source, None, &mut context.borrow_mut());

                        // Insert the parsed module into the cache.
                        if let Ok(module) = &module {
                            self.modules.borrow_mut().insert(path, module.clone());
                        }

                        module
                    }
                    Err(err) => Err(JsNativeError::error().with_message(err.to_string()).into()),
                }
            }
            Err(err) => Err(JsNativeError::error().with_message(err.to_string()).into()),
        }
    }
}
