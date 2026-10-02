use crate::ast::JsAstNode;
use crate::mutation::{JsMutation, create_mutation};
use crate::token::factory_token;
use biome_analyze::{PluginActionData, PluginDiagnosticEntry, RuleDiagnostic};
use biome_diagnostics::{Applicability, Severity, category};
use biome_js_syntax::JsSyntaxNode;
use boa_engine::module::SyntheticModuleInitializer;
use boa_engine::object::builtins::JsArray;
use boa_engine::object::{FunctionObjectBuilder, ObjectInitializer};
use boa_engine::property::Attribute;
use boa_engine::{
    Context, JsNativeError, JsResult, JsString, JsValue, Module, NativeFunction, js_string,
};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct JsPluginApi {
    diagnostics: Rc<RefCell<Vec<PluginDiagnosticEntry>>>,
    source: Rc<RefCell<Option<JsSyntaxNode>>>,
}

impl JsPluginApi {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: Rc::new(RefCell::new(Vec::new())),
            source: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn create_module(&self, context: &mut Context) -> Module {
        let diagnostics = self.diagnostics.clone();
        let source = self.source.clone();

        // SAFETY: The closure doesn't capture any GC-managed values.
        let register_diagnostic = FunctionObjectBuilder::new(context.realm(), unsafe {
            NativeFunction::from_closure(move |_this, args, context| {
                let (node, severity, message, fix) = match args {
                    [node, severity, message] => (node, severity, message, None),
                    [node, severity, message, fix] => (node, severity, message, Some(fix)),
                    _ => return Err(JsNativeError::typ()
                        .with_message(
                            "registerDiagnostic() requires a node, severity, and message, with an optional fourth fix argument. Call registerDiagnostic(node, severity, message, fix).",
                        )
                        .into()),
                };

                let Some(node) = JsAstNode::from_value(node) else {
                    return Err(JsNativeError::typ()
                        .with_message(
                            "The first argument to registerDiagnostic() is not a Biome node. Pass the node supplied to run() or a node reached through its fields or traversal methods.",
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
                                "The diagnostic severity is not supported. Use \"fatal\", \"error\", \"warning\", \"information\", or \"hint\" as the second argument to registerDiagnostic().",
                            )
                            .into()),
                    };

                let diagnostic = RuleDiagnostic::new(
                    category!("plugin"),
                    node.node.text_trimmed_range(),
                    message.to_string(context)?.to_std_string_lossy(),
                )
                .with_severity(severity);

                let action = match fix.filter(|fix| !fix.is_undefined()) {
                    Some(fix) => {
                        let source = source.borrow().clone().ok_or_else(|| JsNativeError::typ()
                            .with_message("A code fix can only be reported while Biome is analyzing source code. Call registerDiagnostic() with the fix from run()."))?;
                        Self::code_fix(node, fix, &source, context)?
                    }
                    None => None,
                };
                diagnostics.borrow_mut().push(PluginDiagnosticEntry { diagnostic, action });

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

        let semantic = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(Self::semantic_query),
        )
        .length(1)
        .name("semantic")
        .build();

        let define_rule = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(Self::define_rule),
        )
        .length(1)
        .name("defineRule")
        .build();

        let create_mutation = FunctionObjectBuilder::new(
            context.realm(),
            NativeFunction::from_fn_ptr(create_mutation),
        )
        .length(1)
        .name("createMutation")
        .build();
        let factory = ObjectInitializer::new(context)
            .function(
                NativeFunction::from_fn_ptr(factory_token),
                js_string!("token"),
                1,
            )
            .build();

        Module::synthetic(
            &[
                js_string!("registerDiagnostic"),
                js_string!("ast"),
                js_string!("semantic"),
                js_string!("defineRule"),
                js_string!("createMutation"),
                js_string!("factory"),
            ],
            SyntheticModuleInitializer::from_copy_closure_with_captures(
                |module,
                 (register_diagnostic, ast, semantic, define_rule, create_mutation, factory),
                 _| {
                    module.set_export(
                        &js_string!("registerDiagnostic"),
                        register_diagnostic.clone().into(),
                    )?;
                    module.set_export(&js_string!("ast"), ast.clone().into())?;
                    module.set_export(&js_string!("semantic"), semantic.clone().into())?;
                    module.set_export(&js_string!("defineRule"), define_rule.clone().into())?;
                    module.set_export(
                        &js_string!("createMutation"),
                        create_mutation.clone().into(),
                    )?;
                    module.set_export(&js_string!("factory"), factory.clone().into())
                },
                (
                    register_diagnostic,
                    ast,
                    semantic,
                    define_rule,
                    create_mutation,
                    factory,
                ),
            ),
            None,
            None,
            context,
        )
    }

    /// Implements `ast(...kinds)`: builds an AST query object from syntax kind names.
    fn ast_query(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Self::node_query("ast", args, context)
    }

    fn semantic_query(
        _this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::node_query("semantic", args, context)
    }

    fn node_query(query_type: &str, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        if args.is_empty() {
            return Err(JsNativeError::typ()
                .with_message(format!("{query_type}() requires at least one node kind. Pass a kind from JsNodeByKind, such as {query_type}(\"JS_CALL_EXPRESSION\")."))
                .into());
        }

        let mut kinds = Vec::with_capacity(args.len());
        for arg in args {
            let Some(kind) = arg.as_string() else {
                return Err(JsNativeError::typ()
                    .with_message(format!("{query_type}() requires node kind names as strings. Pass names such as \"JS_CALL_EXPRESSION\", not node objects."))
                    .into());
            };
            if JsAstNode::syntax_kind_from_ast_name(&kind.to_std_string_lossy()).is_none() {
                return Err(JsNativeError::typ()
                    .with_message(format!(
                        "Unknown syntax kind {:?}. Pass a node kind listed in JsNodeByKind, such as \"JS_CALL_EXPRESSION\".",
                        kind.to_std_string_lossy(),
                    ))
                    .into());
            }
            kinds.push(JsValue::from(kind));
        }

        let kinds = JsArray::from_iter(kinds, context);
        let query = ObjectInitializer::new(context)
            .property(
                js_string!("type"),
                JsString::from(query_type),
                Attribute::ENUMERABLE,
            )
            .property(js_string!("kinds"), kinds, Attribute::ENUMERABLE)
            .build();

        Ok(query.into())
    }

    /// Implements `defineRule(rule)`: validates the shape of the rule and returns it as-is.
    /// Rules are collected from the module exports after the plugin is evaluated.
    fn define_rule(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let [rule] = args else {
            return Err(JsNativeError::typ()
                .with_message("defineRule() requires exactly one rule object. Pass an object with query and run properties.")
                .into());
        };

        let Some(object) = rule.as_object() else {
            return Err(JsNativeError::typ()
                .with_message("The argument to defineRule() is not an object. Pass an object with query and run properties.")
                .into());
        };

        if object
            .get(js_string!("query"), context)?
            .as_object()
            .is_none()
        {
            return Err(JsNativeError::typ()
                .with_message(
                    "The rule's query property must be a query object. Set it to ast(...) or semantic(...) with the node kinds the rule should inspect.",
                )
                .into());
        }

        if object
            .get(js_string!("run"), context)?
            .as_function()
            .is_none()
        {
            return Err(JsNativeError::typ()
                .with_message("The rule's run property must be a function. Add run(node) to inspect matching nodes and report diagnostics.")
                .into());
        }

        Ok(rule.clone())
    }

    fn code_fix(
        node: JsAstNode,
        fix: &JsValue,
        source: &JsSyntaxNode,
        context: &mut Context,
    ) -> JsResult<Option<PluginActionData>> {
        let fix = fix
            .as_object()
            .ok_or_else(|| JsNativeError::typ().with_message("The fix argument to registerDiagnostic() is not an object. Pass an object with mutation, message, and kind properties, or omit the fix argument."))?;
        let mutation = fix.get(js_string!("mutation"), context)?;
        let message = fix.get(js_string!("message"), context)?;
        let kind = fix.get(js_string!("kind"), context)?;
        let message = message
            .as_string()
            .ok_or_else(|| {
                JsNativeError::typ().with_message("The fix.message property must be a string. Set it to a description of the change, such as \"Replace var with let.\".")
            })?
            .to_std_string()
            .map_err(|_| {
                JsNativeError::typ().with_message("The fix.message property contains an incomplete Unicode character. Check its \\u escapes and supply complete characters.")
            })?;
        let kind = kind.as_string().ok_or_else(|| {
            JsNativeError::typ().with_message(
                "The fix.kind property must be a string. Set it to \"safe\" or \"unsafe\".",
            )
        })?;
        let applicability = if kind == "safe" {
            Applicability::Always
        } else if kind == "unsafe" {
            Applicability::MaybeIncorrect
        } else {
            return Err(JsNativeError::typ()
                .with_message(format!(
                    "Unknown fix kind {:?}. Set fix.kind to \"safe\" or \"unsafe\".",
                    kind.to_std_string_lossy()
                ))
                .into());
        };
        if node.node.ancestors().last().as_ref() != Some(source) {
            return Err(JsNativeError::typ()
                .with_message("The diagnostic's node does not belong to the source being analyzed. Pass a node from that source to registerDiagnostic().")
                .into());
        }

        // Property getters may execute JavaScript. Consume the batch only after validation.
        let batch = JsMutation::take(&mutation, source)?;
        let Some((source_range, text_edit)) = batch.to_text_range_and_edit() else {
            return Ok(None);
        };
        let source = source.text_with_trivia().to_string();
        if text_edit.new_string(&source) == source {
            return Ok(None);
        }
        Ok(Some(PluginActionData {
            source_range,
            text_edit,
            message,
            applicability,
        }))
    }

    pub(crate) fn pull_diagnostics(&self) -> Vec<PluginDiagnosticEntry> {
        std::mem::take(&mut self.diagnostics.borrow_mut())
    }

    pub(crate) fn set_source(&self, source: Option<JsSyntaxNode>) {
        *self.source.borrow_mut() = source;
    }
}
