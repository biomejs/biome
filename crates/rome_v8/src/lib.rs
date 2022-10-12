use std::{cell::Ref, sync::Once};

use anyhow::{bail, ensure, Context, Result};
use rome_js_syntax::{JsAnyRoot, JsLanguage, JsSyntaxNode};
use rome_rowan::BatchMutation;
use v8::inspector::{StringView, V8Inspector};

mod bindings;
mod convert;
mod inspector;
mod registry;
mod support;

use crate::{
    bindings::setup_context,
    convert::{FromV8, ToV8},
    registry::{take_native, EnumRegistry},
};

pub struct Instance {
    global: v8::Global<v8::ObjectTemplate>,
    inspector: v8::UniqueRef<v8::inspector::V8Inspector>,
    _client: Box<inspector::DebugClient>,
    isolate: v8::OwnedIsolate,
}

pub struct Script {
    context: v8::Global<v8::Context>,
    function: v8::Global<v8::Function>,
}

impl Instance {
    pub fn new() -> Result<Self> {
        // Setup the V8 platform once
        static INIT_ONCE: Once = Once::new();
        INIT_ONCE.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        // Create a new `Isolate`
        let mut isolate = v8::Isolate::new(v8::Isolate::create_params());

        // Create a `DebugClient` to collect the console messages
        let mut client = Box::new(inspector::DebugClient::new());
        let inspector = V8Inspector::create(&mut isolate, client.as_mut());

        let global = {
            // Create a new `HandleScope` to manage the garbage collection of values accessed from Rust
            let mut scope = v8::HandleScope::new(&mut isolate);

            // Create the "global object" template containing references to all the binding functions
            let global = setup_context(&mut scope)?;
            v8::Global::new(&mut scope, global)
        };

        Ok(Self {
            global,
            inspector,
            _client: client,
            isolate,
        })
    }

    pub fn load(&mut self, resource_name: &str, source_string: &str) -> Result<Script> {
        // Create a new `HandleScope` to manage the garbage collection of values accessed from Rust
        let scope = &mut v8::HandleScope::new(&mut self.isolate);
        let global = v8::Local::new(scope, &self.global);

        // Create a new `ContextScope` to sandbox everything from now on to a specific context
        let context = v8::Context::new_from_template(scope, global);
        let scope = &mut v8::ContextScope::new(scope, context);

        // Setup the per-context enum cache
        EnumRegistry::install(scope)?;

        let name = StringView::from(b"context".as_slice());
        self.inspector.context_created(context, 1, name);

        // Create a new `TryCatch` scope to catch JS exceptions in the following code
        let scope = &mut v8::TryCatch::new(scope);

        let resource_name =
            v8::String::new(scope, resource_name).context("could not allocate string")?;
        let source_string =
            v8::String::new(scope, source_string).context("could not allocate string")?;

        let source_map_url = v8::undefined(scope);

        let origin = v8::ScriptOrigin::new(
            scope,
            resource_name.into(),
            0,
            0,
            false,
            0,
            source_map_url.into(),
            false,
            false,
            true,
        );

        let source = v8::script_compiler::Source::new(source_string, Some(&origin));
        let module = v8::script_compiler::compile_module(scope, source)
            .context("failed to compile module")?;

        let result = module
            .instantiate_module(scope, |_, _, _, _| {
                eprintln!("import callback");
                None
            })
            .context("failed to instantiate module")?;

        ensure!(result, "instantiate_module returned false");

        if let Some(promise) = module.evaluate(scope) {
            if let Ok(promise) = v8::Local::<v8::Promise>::try_from(promise) {
                let state = promise.state();
                ensure!(
                    state == v8::PromiseState::Fulfilled,
                    "top-level await is unsupported"
                );
            }
        }

        if let Some(err) = scope.exception() {
            let err = err.to_string(scope).context("could not read string")?;
            bail!("JS Exception: {}", err.to_rust_string_lossy(scope));
        }

        let namespace = module.get_module_namespace();
        let namespace = namespace
            .to_object(scope)
            .context("could not read string")?;

        let key = v8::String::new(scope, "default").context("could not allocate string")?;
        let default = namespace
            .get(scope, key.into())
            .context("could not read default export")?;

        let function = v8::Local::<v8::Function>::try_from(default)
            .context("default export is not a function")?;

        let context = v8::Global::new(scope, context);
        let function = v8::Global::new(scope, function);

        Ok(Script { context, function })
    }

    pub fn run(&mut self, script: &Script, root: JsAnyRoot) -> Result<JsSyntaxNode> {
        // Create a new `HandleScope` to manage the garbage collection of values accessed from Rust
        let scope = &mut v8::HandleScope::new(&mut self.isolate);

        // Create a new `ContextScope` to sandbox everything from now on to a specific context
        let context = v8::Local::new(scope, &script.context);
        let scope = &mut v8::ContextScope::new(scope, context);

        // Create a new `TryCatch` scope to catch JS exceptions in the following code
        let scope = &mut v8::TryCatch::new(scope);

        let function = v8::Local::new(scope, &script.function);
        let recv = v8::undefined(scope);
        let root = root.to_v8(scope)?;
        let result = function.call(scope, recv.into(), &[root]);

        if let Some(err) = scope.exception() {
            let err = err.to_string(scope).context("could not read string")?;
            bail!("JS Exception: {}", err.to_rust_string_lossy(scope));
        }

        let result = result.context("transform function did not return")?;

        let result = if let Ok(result) = Ref::<JsSyntaxNode>::from_v8(scope, result) {
            result.clone()
        } else {
            let result = unsafe { take_native::<BatchMutation<JsLanguage>>(result) };
            if let Ok(result) = result {
                result.commit()
            } else {
                bail!("transform function did not return a SyntaxNode or BatchMutation object")
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use rome_js_parser::{parse, FileId};
    use rome_js_syntax::SourceType;

    use super::Instance;

    #[test]
    fn test_run() {
        const INPUT: &str = "function test() {}";
        const OUTPUT: &str = "function TEST() {}";

        let module = parse(INPUT, FileId::zero(), SourceType::js_module());
        let root = module.tree();

        const CODE: &str = "export default function(root) {
    const mutation = new JsBatchMutation(root);

    for(const old_token of root.descendants_tokens()) {
        if(old_token.kind() === JsSyntaxKind.IDENT) {
            const new_text = old_token.text_trimmed().toUpperCase();
            const new_token = make.ident(new_text);
            mutation.replace_element(old_token, new_token);
        }
    }

    return mutation;
}";

        // Create a V8 instance
        let mut isolate = Instance::new().unwrap();

        // Load the script
        let script = isolate.load("test.mjs", CODE).unwrap();

        // Run the script over the provided node
        let result = isolate.run(&script, root).unwrap();

        assert_eq!(result.to_string(), OUTPUT);
    }
}
