use std::cell::RefCell;
use std::rc::Rc;

use boa_engine::module::SyntheticModuleInitializer;
use boa_engine::object::FunctionObjectBuilder;
use boa_engine::{Context, JsNativeError, JsValue, Module, NativeFunction, js_string};

use biome_analyze::RuleDiagnostic;
use biome_diagnostics::{Severity, category};

use crate::ast::JsAstNode;

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
            NativeFunction::from_closure(move |_this, args, context| {
                let [node, severity, message] = args else {
                    return Err(JsNativeError::typ()
                        .with_message(
                            "registerDiagnostic() expects an AST node, severity, and message",
                        )
                        .into());
                };

                let Some(range) = JsAstNode::text_range(node) else {
                    return Err(JsNativeError::typ()
                        .with_message(
                            "registerDiagnostic() expects an AST node as its first argument",
                        )
                        .into());
                };

                let severity =
                    match severity.to_string(context)?.to_std_string_lossy().as_str() {
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
                    range,
                    message.to_string(context)?.to_std_string_lossy(),
                )
                .with_severity(severity);

                diagnostics.borrow_mut().push(diagnostic);

                Ok(JsValue::undefined())
            })
        })
        .length(3)
        .name("registerDiagnostic")
        .build();

        // TODO: more runtime APIs?

        Module::synthetic(
            &[js_string!("registerDiagnostic")],
            SyntheticModuleInitializer::from_copy_closure_with_captures(
                |module, register_diagnostic, _| {
                    module.set_export(
                        &js_string!("registerDiagnostic"),
                        register_diagnostic.clone().into(),
                    )
                },
                register_diagnostic,
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
