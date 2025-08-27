use std::rc::Rc;
use std::sync::Arc;

use boa_engine::builtins::promise::PromiseState;
use boa_engine::module::ModuleLoader;
use boa_engine::object::builtins::JsFunction;
use boa_engine::{
    Context, JsError, JsNativeError, JsResult, JsValue, Module, NativeFunction, Source, js_string,
};
use camino::Utf8Path;

use biome_analyze::RuleDiagnostic;
use biome_resolver::FsWithResolverProxy;

use crate::JsModuleLoader;
use crate::plugin_api::JsPluginApi;

pub struct JsExecContext {
    ctx: Context,
    fs: Arc<dyn FsWithResolverProxy>,
    api: JsPluginApi,
}

impl JsExecContext {
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> JsResult<Self> {
        let module_loader = Rc::new(JsModuleLoader::new(fs.clone()));
        let api = JsPluginApi::new();
        let mut ctx = Context::builder()
            .module_loader(Rc::clone(&module_loader))
            .build()?;

        module_loader.register_module(
            js_string!("@biomejs/plugin-api"),
            api.create_module(&mut ctx),
        );

        Ok(Self { ctx, fs, api })
    }

    #[inline]
    pub fn pull_diagnostics(&mut self) -> Vec<RuleDiagnostic> {
        self.api.pull_diagnostics()
    }

    pub fn import_module(&mut self, path: impl AsRef<Utf8Path>) -> JsResult<Module> {
        let ctx = &mut self.ctx;
        let path = path.as_ref();
        let source = self.fs.read_file_from_path(path).map_err(|err| {
            JsNativeError::error().with_message(format!("Failed to read {path}: {err}"))
        })?;
        let source = Source::from_bytes(source.as_bytes()).with_path(path.as_std_path());
        let module = Module::parse(source, None, ctx)?;

        let promise_result = module
            .load(ctx)
            .then(
                Some(
                    NativeFunction::from_copy_closure_with_captures(
                        |_, _, module, context| {
                            module.link(context)?;
                            Ok(JsValue::undefined())
                        },
                        module.clone(),
                    )
                    .to_js_function(ctx.realm()),
                ),
                None,
                ctx,
            )
            .then(
                Some(
                    NativeFunction::from_copy_closure_with_captures(
                        |_, _, module, context| Ok(module.evaluate(context).into()),
                        module.clone(),
                    )
                    .to_js_function(ctx.realm()),
                ),
                None,
                ctx,
            );

        loop {
            match promise_result.state() {
                PromiseState::Pending => {
                    // Drive the job queue until the promise settles.
                    ctx.run_jobs();
                }
                PromiseState::Fulfilled(_) => break Ok(module),
                PromiseState::Rejected(err) => {
                    let opaque = JsError::from_opaque(err);
                    break match opaque.try_native(ctx) {
                        Ok(native) => Err(native.into()),
                        _ => Err(opaque),
                    };
                }
            }
        }
    }

    pub fn get_default_export(&mut self, module: &Module) -> JsResult<JsValue> {
        let ctx = &mut self.ctx;
        let namespace = module.namespace(ctx);

        namespace.get(js_string!("default"), ctx)
    }

    pub fn call_function(
        &mut self,
        function: &JsFunction,
        this: &JsValue,
        args: &[JsValue],
    ) -> JsResult<JsValue> {
        function.call(this, args, &mut self.ctx)
    }
}
