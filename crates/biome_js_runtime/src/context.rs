use crate::JsModuleLoader;
use crate::ast::JsAstNode;
use crate::kind_set::{JsNodeKindSet, JsTokenKindSet};
use crate::mutation::JsMutation;
use crate::plugin_api::{JsPluginApi, boolean_property};
use crate::rule_context::create_rule_context;
use crate::semantic::register_semantic;
use crate::source::read_module_source;
use crate::token::JsAstToken;
use crate::types::{TYPE_INFERENCE_UNAVAILABLE, TypeInferenceProvider, register_types};
use biome_analyze::{PluginDiagnosticEntry, ServiceBag};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxElement, JsSyntaxKind, JsSyntaxNode};
use biome_languages::JsFileSource;
use biome_resolver::FsWithResolverProxy;
use biome_rowan::AstNode;
use boa_engine::builtins::promise::PromiseState;
use boa_engine::object::builtins::{JsArray, JsFunction};
use boa_engine::property::PropertyKey;
use boa_engine::{
    Context, JsError, JsNativeError, JsResult, JsValue, Module, NativeFunction, Source, js_string,
};
use camino::Utf8Path;
use std::rc::Rc;
use std::sync::Arc;

use boa_engine::context::time::StdClock as Clock;

pub struct JsExecContext {
    ctx: Context,
    fs: Arc<dyn FsWithResolverProxy>,
    api: JsPluginApi,
}

/// A lint rule declared by a plugin with `defineRule()` and collected from its module exports.
///
/// Like the values created by [`JsExecContext::create_js_ast`], a rule is bound to the context
/// that loaded it: it must not outlive it, nor be passed to another [`JsExecContext`].
pub struct JsPluginRule {
    /// The name of the export the rule was assigned to.
    pub name: String,

    /// The syntax kinds the rule queries.
    pub kinds: Vec<JsSyntaxKind>,

    pub requires_semantic: bool,

    /// Whether the rule requests inferred type information.
    pub requires_types: bool,

    /// The `run()` function of the rule, called with every node or token matching the query.
    pub run: JsFunction,

    /// The Grit pattern used to select the nodes passed to `run`, if any.
    ///
    /// When set, `kinds` contains the file root kinds, because the pattern is
    /// executed once per file rather than once per node kind.
    pub grit: Option<String>,
}

impl JsExecContext {
    pub fn new(fs: Arc<dyn FsWithResolverProxy>) -> JsResult<Self> {
        let module_loader = Rc::new(JsModuleLoader::new(fs.clone()));
        let api = JsPluginApi::new();
        let mut ctx = Context::builder()
            .clock(Rc::new(Clock::default()))
            .module_loader(Rc::clone(&module_loader))
            .build()?;

        JsAstNode::register(&mut ctx)?;
        JsAstToken::register(&mut ctx)?;
        JsNodeKindSet::<JsLanguage>::register(&mut ctx)?;
        JsTokenKindSet::<JsLanguage>::register(&mut ctx)?;
        JsMutation::register(&mut ctx)?;
        register_semantic(&mut ctx)?;
        register_types(&mut ctx)?;

        module_loader.register_module(
            js_string!("@biomejs/runtime/plugin"),
            api.create_module(&mut ctx),
        );

        Ok(Self { ctx, fs, api })
    }

    #[inline]
    pub fn pull_diagnostics(&mut self) -> Vec<PluginDiagnosticEntry> {
        self.api.pull_diagnostics()
    }

    pub fn import_module(&mut self, path: impl AsRef<Utf8Path>) -> JsResult<Module> {
        let ctx = &mut self.ctx;
        let path = path.as_ref();
        let source = read_module_source(&self.fs, path)?;
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
            )?
            .then(
                Some(
                    NativeFunction::from_copy_closure_with_captures(
                        |_, _, module, context| Ok(module.evaluate(context)?.into()),
                        module.clone(),
                    )
                    .to_js_function(ctx.realm()),
                ),
                None,
                ctx,
            )?;

        loop {
            match promise_result.state() {
                PromiseState::Pending => {
                    // Drive the job queue until the promise settles.
                    let _ = ctx.run_jobs();
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

    /// Collects the rules the plugin module exports, i.e. every export that looks like an object
    /// created with `defineRule()`. Other exports are ignored, so plugins can export helpers.
    pub fn load_rules(&mut self, module: &Module) -> JsResult<Vec<JsPluginRule>> {
        let ctx = &mut self.ctx;
        let namespace = module.namespace(ctx);
        let mut rules = Vec::new();

        for key in namespace.own_property_keys(ctx)? {
            let PropertyKey::String(export_name) = &key else {
                continue;
            };
            let name = export_name.to_std_string_lossy();

            let Some(rule) = namespace.get(key.clone(), ctx)?.as_object() else {
                continue;
            };
            let query = rule.get(js_string!("query"), ctx)?;
            let run = rule.get(js_string!("run"), ctx)?;
            let (Some(query), Some(run)) = (query.as_object(), run.as_function()) else {
                continue;
            };

            let query_type = query.get(js_string!("type"), ctx)?;
            let query_type = query_type.as_string().map(|ty| ty.to_std_string_lossy());
            let requires_types = boolean_property(&query, "requiresTypes", ctx)?;
            let requires_semantic = match query_type.as_deref() {
                Some("ast") => boolean_property(&query, "requiresSemantic", ctx)? || requires_types,
                Some("semantic") => true,
                Some("grit") => {
                    boolean_property(&query, "requiresSemantic", ctx)? || requires_types
                }
                _ => {
                    return Err(JsNativeError::typ()
                        .with_message(format!(
                            "Rule {name} has an unsupported query type. Use ast(...), semantic(...), or grit(...)."
                        ))
                        .into());
                }
            };
            let grit = match query_type.as_deref() {
                Some("grit") => {
                    let pattern = query.get(js_string!("pattern"), ctx)?;
                    let Some(pattern) = pattern.as_string() else {
                        return Err(JsNativeError::typ()
                            .with_message(format!(
                                "Rule {name} has an invalid Grit query. Pass a Grit pattern string to grit()."
                            ))
                            .into());
                    };
                    Some(pattern.to_std_string_lossy())
                }
                _ => None,
            };

            let rule_kinds = if grit.is_some() {
                AnyJsRoot::KIND_SET.iter().collect()
            } else {
                let kinds = JsArray::from_object(
                    query
                        .get(js_string!("kinds"), ctx)?
                        .as_object()
                        .ok_or_else(|| {
                            JsNativeError::typ()
                                .with_message(format!("Rule {name} has an invalid AST query"))
                        })?,
                )?;

                let mut rule_kinds = Vec::new();
                for index in 0..kinds.length(ctx)? {
                    let kind = kinds.at(index as i64, ctx)?;
                    let kind = kind
                        .as_string()
                        .map(|kind| kind.to_std_string_lossy())
                        .and_then(|kind| JsAstNode::syntax_kind_from_ast_name(&kind))
                        .ok_or_else(|| {
                            JsNativeError::typ().with_message(format!(
                                "Rule {name} queries an unknown syntax kind: {}",
                                kind.display()
                            ))
                        })?;
                    rule_kinds.push(kind);
                }
                rule_kinds
            };

            rules.push(JsPluginRule {
                name,
                kinds: rule_kinds,
                requires_semantic,
                requires_types,
                run,
                grit,
            });
        }

        Ok(rules)
    }

    /// Wraps `node` in the AST bindings exposed to the plugins, so it can be passed to a plugin
    /// function as an argument.
    ///
    /// The returned value is bound to this context: it must not outlive it, nor be passed to
    /// another [`JsExecContext`].
    pub fn create_js_ast(&mut self, node: JsSyntaxNode) -> JsValue {
        self.create_js_element(JsSyntaxElement::Node(node))
    }

    /// Wraps a node or a token in the AST bindings exposed to the plugins.
    ///
    /// The returned value is bound to this context: it must not outlive it, nor
    /// be passed to another [`JsExecContext`].
    pub fn create_js_element(&mut self, element: JsSyntaxElement) -> JsValue {
        let root = match &element {
            JsSyntaxElement::Node(node) => node.ancestors().last(),
            JsSyntaxElement::Token(token) => {
                token.parent().and_then(|parent| parent.ancestors().last())
            }
        };
        self.api.set_source(root);
        JsAstNode::from_element(element, &mut self.ctx)
    }

    pub fn call_function(
        &mut self,
        function: &JsFunction,
        this: &JsValue,
        args: &[JsValue],
    ) -> JsResult<JsValue> {
        let result = function.call(this, args, &mut self.ctx);
        self.api.set_source(None);
        result
    }

    /// Creates file metadata whose values remain associated with this invocation's source.
    pub fn create_rule_context(&mut self, path: &Utf8Path, source_type: JsFileSource) -> JsValue {
        create_rule_context(path, source_type, None, None, &mut self.ctx)
    }

    pub fn call_rule(
        &mut self,
        rule: &JsPluginRule,
        element: JsSyntaxElement,
        path: &Utf8Path,
        source_type: JsFileSource,
        services: &ServiceBag,
        type_inference: Option<Rc<dyn TypeInferenceProvider>>,
    ) -> JsResult<JsValue> {
        let model = if rule.requires_semantic {
            Some(services.get_service::<SemanticModel>().ok_or_else(|| {
                JsNativeError::typ().with_message("The semantic model is unavailable. Supply a SemanticModel service when analyzing semantic plugin rules.")
            })?)
        } else {
            None
        };
        let types = if rule.requires_types {
            Some(type_inference.ok_or_else(|| {
                JsNativeError::typ().with_message(format!(
                    "{TYPE_INFERENCE_UNAVAILABLE}. Enable the `types` domain in the Biome configuration so files are indexed with type information."
                ))
            })?)
        } else {
            None
        };
        let ast = self.create_js_element(element);
        let context = create_rule_context(path, source_type, model, types, &mut self.ctx);
        self.call_function(&rule.run, &JsValue::undefined(), &[ast, context])
    }
}
