use std::cell::RefCell;
use std::rc::Rc;

use boa_engine::module::SyntheticModuleInitializer;
use boa_engine::object::FunctionObjectBuilder;
use boa_engine::{Context, JsNativeError, JsValue, Module, NativeFunction, js_string};

use biome_analyze::RuleDiagnostic;
use biome_diagnostics::{Severity, category};
use biome_text_size::TextRange;

pub(crate) struct JsPluginApi {
    diagnostics: Rc<RefCell<Vec<RuleDiagnostic>>>,
}

impl JsPluginApi {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(crate) fn create_module(&self, context: &mut Context) -> Module {
        let diagnostics = self.diagnostics.clone();

        // SAFETY: The closure doesn't capture any GC-managed values.
        let register_diagnostic = FunctionObjectBuilder::new(context.realm(), unsafe {
            NativeFunction::from_closure(move |_this, args, _context| {
                let [JsValue::String(severity), JsValue::String(message)] = args else {
                    return Err(JsNativeError::typ()
                        .with_message("registerDiagnostic() expects two string arguments")
                        .into());
                };

                let severity =
                    match severity.to_std_string_lossy().as_str() {
                        "fatal" => Severity::Fatal,
                        "error" => Severity::Error,
                        "warning" => Severity::Warning,
                        "information" => Severity::Information,
                        "hint" => Severity::Hint,
                        _ => return Err(JsNativeError::typ()
                            .with_message(
                                "Unexpected severity, expected one of: fatal, error, warning, information, hint",
                            )
                            .into()),
                    };

                let diagnostic = RuleDiagnostic::new(
                    category!("plugin"),
                    None::<TextRange>, // TODO: retrieve a span from the AST
                    message.to_std_string_lossy(),
                )
                .with_severity(severity);

                diagnostics.borrow_mut().push(diagnostic);

                Ok(JsValue::undefined())
            })
        })
        .length(2)
        .name("registerDiagnostic")
        .build();

        // TODO: auto-generate AST classes and insert into the runtime
        // TODO: more runtime APIs?

        Module::synthetic(
            &[js_string!("registerDiagnostic")],
            SyntheticModuleInitializer::from_copy_closure_with_captures(
                |module, fns, _| {
                    module.set_export(&js_string!("registerDiagnostic"), fns.0.clone().into())
                },
                (register_diagnostic,),
            ),
            None,
            None,
            context,
        )
    }

    pub(crate) fn pull_diagnostics(&self) -> Vec<RuleDiagnostic> {
        std::mem::take(&mut self.diagnostics.borrow_mut())
    }
}
