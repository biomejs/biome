use std::cell::RefCell;
use std::rc::Rc;

use boa_engine::module::SyntheticModuleInitializer;
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{FunctionObjectBuilder, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{Context, JsNativeError, JsResult, JsValue, Module, NativeFunction, js_string};

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

        let ast = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(Self::ast_query),
        )
        .length(1)
        .name("ast")
        .build();

        let define_rule = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(Self::define_rule),
        )
        .length(1)
        .name("defineRule")
        .build();

        // TODO: more runtime APIs?

        Module::synthetic(
            &[
                js_string!("registerDiagnostic"),
                js_string!("ast"),
                js_string!("defineRule"),
            ],
            SyntheticModuleInitializer::from_copy_closure_with_captures(
                |module, (register_diagnostic, ast, define_rule), _| {
                    module.set_export(
                        &js_string!("registerDiagnostic"),
                        register_diagnostic.clone().into(),
                    )?;
                    module.set_export(&js_string!("ast"), ast.clone().into())?;
                    module.set_export(&js_string!("defineRule"), define_rule.clone().into())
                },
                (register_diagnostic, ast, define_rule),
            ),
            None,
            None,
            context,
        )
    }

    /// Implements `ast(...kinds)`: builds an AST query object from syntax kind names.
    fn ast_query(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(JsNativeError::typ()
                .with_message("ast() expects at least one syntax kind name")
                .into());
        }

        let mut kinds = Vec::with_capacity(args.len());
        for arg in args {
            let Some(kind) = arg.as_string() else {
                return Err(JsNativeError::typ()
                    .with_message("ast() expects syntax kind names as strings")
                    .into());
            };
            if JsAstNode::syntax_kind_from_ast_name(&kind.to_std_string_lossy()).is_none() {
                return Err(JsNativeError::typ()
                    .with_message(format!(
                        "Unknown syntax kind passed to ast(): {}",
                        kind.to_std_string_lossy(),
                    ))
                    .into());
            }
            kinds.push(JsValue::from(kind));
        }

        let kinds = JsArray::from_iter(kinds, context);
        let query = ObjectInitializer::new(context)
            .property(js_string!("type"), js_string!("ast"), Attribute::ENUMERABLE)
            .property(js_string!("kinds"), kinds, Attribute::ENUMERABLE)
            .build();

        Ok(query.into())
    }

    /// Implements `defineRule(rule)`: validates the shape of the rule and returns it as-is.
    /// Rules are collected from the module exports after the plugin is evaluated.
    fn define_rule(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let [rule] = args else {
            return Err(JsNativeError::typ()
                .with_message("defineRule() expects a single rule object")
                .into());
        };

        let Some(object) = rule.as_object() else {
            return Err(JsNativeError::typ()
                .with_message("defineRule() expects a rule object")
                .into());
        };

        if object
            .get(js_string!("query"), context)?
            .as_object()
            .is_none()
        {
            return Err(JsNativeError::typ()
                .with_message(
                    "defineRule() expects a query created with a query builder like ast()",
                )
                .into());
        }

        if object
            .get(js_string!("run"), context)?
            .as_function()
            .is_none()
        {
            return Err(JsNativeError::typ()
                .with_message("defineRule() expects a run() function")
                .into());
        }

        Ok(rule.clone())
    }

    pub(crate) fn pull_diagnostics(&self) -> Vec<RuleDiagnostic> {
        std::mem::take(&mut self.diagnostics.borrow_mut())
    }
}
