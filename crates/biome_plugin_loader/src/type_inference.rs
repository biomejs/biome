use std::rc::Rc;

use biome_analyze::ServiceBag;
use biome_js_runtime::{
    TypeClassification, TypeClassificationResult, TypeInferenceProvider, TypePredicate, TypeView,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsFunction, JsCallExpression, JsClassDeclaration, JsClassExpression,
    JsNewExpression, JsObjectExpression, JsSyntaxNode,
};
use biome_js_type_info::InferredType;
use biome_module_graph::type_inference::{
    ArrayOfPromisesClassificationRequest, ExpectedCallArgumentTypeRequest,
    ExpectedConstructorArgumentTypeRequest, FunctionReturnTypeRequest, MemberReturnTypeRequest,
    NormalizedBindingTypeRequest, NormalizedExpressionTypeRequest, PromiseClassificationRequest,
    PromiseReturningFunctionClassificationRequest, ThenableClassificationRequest,
    TypeInferenceArgument, TypeInferenceCaller, TypeInferenceClassification, TypeInferenceSource,
    execute_type_inference_request,
};
use biome_module_graph::{ModuleDb, ModuleInfo};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use camino::Utf8Path;

/// Builds a provider from the services of the current analysis.
///
/// Returns `None` when the module graph wasn't built with type inference, so
/// the caller can report that the `types` domain must be enabled.
pub(crate) fn provider(
    services: &ServiceBag,
    path: &Utf8Path,
) -> Option<Rc<dyn TypeInferenceProvider>> {
    let db = services.get_service::<Rc<dyn ModuleDb>>()?.clone();
    let module = db.module_for_path(path)?;
    if !module.kind(&*db).as_js_module_info()?.has_type_inference() {
        return None;
    }
    let model = services.get_service::<SemanticModel>().cloned();
    Some(Rc::new(JsTypeInferenceProvider {
        db,
        module,
        model,
        caller: TypeInferenceCaller::new("plugin", "js_plugin"),
    }))
}

struct JsTypeInferenceProvider {
    db: Rc<dyn ModuleDb>,
    module: ModuleInfo,
    model: Option<SemanticModel>,
    caller: TypeInferenceCaller,
}

impl JsTypeInferenceProvider {
    fn view(&self, locator: TypeLocator) -> Option<Rc<dyn TypeView>> {
        let view = LazyTypeView {
            db: Rc::clone(&self.db),
            module: self.module,
            caller: self.caller,
            locator,
        };
        // Only return a handle when the type resolves, so JavaScript receives
        // `undefined` for unknown or unavailable types.
        view.resolve()?;
        Some(Rc::new(view))
    }

    fn named_value_binding_range(&self, range: TextRange, name: &str) -> Option<TextRange> {
        let model = self.model.as_ref()?;
        let mut scope = model.scope_for_range(range);
        let binding = loop {
            if let Some(binding) = scope.get_binding(name) {
                break binding;
            }
            scope = scope.parent()?;
        };
        Some(binding.syntax().text_trimmed_range())
    }
}

impl TypeInferenceProvider for JsTypeInferenceProvider {
    fn type_of_expression(&self, expression: &JsSyntaxNode) -> Option<Rc<dyn TypeView>> {
        self.view(TypeLocator::Expression(expression.text_trimmed_range()))
    }

    fn type_of_named_value(&self, node: &JsSyntaxNode, name: &str) -> Option<Rc<dyn TypeView>> {
        let origin = node.text_trimmed_range();
        let binding = self.named_value_binding_range(origin, name)?;
        self.view(TypeLocator::Binding { origin, binding })
    }

    fn return_type_of_function(&self, function: &JsSyntaxNode) -> Option<Rc<dyn TypeView>> {
        let function = AnyJsFunction::cast_ref(function)?;
        let origin = function.range();
        let source = match &function {
            AnyJsFunction::JsArrowFunctionExpression(expression) => {
                FunctionSource::Expression(expression.range())
            }
            AnyJsFunction::JsFunctionDeclaration(declaration) => {
                let name = declaration
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?;
                FunctionSource::Binding(
                    self.named_value_binding_range(function.range(), name.text_trimmed())?,
                )
            }
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                if let Some(name) = declaration
                    .id()
                    .and_then(|id| id.as_js_identifier_binding().cloned())
                    .and_then(|identifier| identifier.name_token().ok())
                {
                    FunctionSource::Binding(
                        self.named_value_binding_range(function.range(), name.text_trimmed())?,
                    )
                } else {
                    FunctionSource::DefaultExport
                }
            }
            AnyJsFunction::JsFunctionExpression(expression) => {
                FunctionSource::Expression(expression.range())
            }
        };
        self.view(TypeLocator::Function { origin, source })
    }

    fn return_type_of_member(&self, member: &JsSyntaxNode, name: &str) -> Option<Rc<dyn TypeView>> {
        let parent = member.ancestors().find_map(|ancestor| {
            if let Some(class) = JsClassDeclaration::cast(ancestor.clone()) {
                let binding_range = class
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .text_trimmed_range();
                return Some(FunctionSource::Binding(binding_range));
            }
            if let Some(class) = JsClassExpression::cast(ancestor.clone()) {
                return Some(FunctionSource::Expression(class.range()));
            }
            if let Some(object) = JsObjectExpression::cast(ancestor) {
                return Some(FunctionSource::Expression(object.range()));
            }
            None
        })?;
        self.view(TypeLocator::Member {
            origin: member.text_trimmed_range(),
            parent,
            name: name.to_string(),
        })
    }

    fn expected_argument_type(
        &self,
        call: &JsSyntaxNode,
        index: usize,
    ) -> Option<Rc<dyn TypeView>> {
        let (callee, arguments, is_constructor) =
            if let Some(call) = JsCallExpression::cast_ref(call) {
                (call.callee().ok()?, call.arguments().ok()?, false)
            } else {
                let new = JsNewExpression::cast_ref(call)?;
                (new.callee().ok()?, new.arguments()?, true)
            };

        let arguments = arguments
            .args()
            .iter()
            .map(|argument| {
                let argument = argument.ok()?;
                let (expression, is_spread) = match argument {
                    AnyJsCallArgument::AnyJsExpression(expression) => (expression, false),
                    AnyJsCallArgument::JsSpread(spread) => (spread.argument().ok()?, true),
                };
                Some((expression.range(), is_spread))
            })
            .collect::<Option<Vec<_>>>()?;
        let origin = arguments.get(index)?.0;

        self.view(TypeLocator::ExpectedArgument {
            origin,
            callee: callee.range(),
            arguments,
            index,
            is_constructor,
        })
    }

    fn classify(
        &self,
        expression: &JsSyntaxNode,
        classification: TypeClassification,
    ) -> TypeClassificationResult {
        let range = expression.text_trimmed_range();
        let db = self.db.as_ref();
        let caller = self.caller;
        let result = match classification {
            TypeClassification::Promise => execute_type_inference_request(
                db,
                caller,
                PromiseClassificationRequest::new(self.module, range),
            ),
            TypeClassification::Thenable => execute_type_inference_request(
                db,
                caller,
                ThenableClassificationRequest::new(self.module, range),
            ),
            TypeClassification::ArrayOfPromises => execute_type_inference_request(
                db,
                caller,
                ArrayOfPromisesClassificationRequest::new(self.module, range),
            ),
            TypeClassification::PromiseReturningFunction => execute_type_inference_request(
                db,
                caller,
                PromiseReturningFunctionClassificationRequest::new(self.module, range),
            ),
        };

        match result {
            TypeInferenceClassification::Match => TypeClassificationResult::Match,
            TypeInferenceClassification::NoMatch => TypeClassificationResult::NoMatch,
            TypeInferenceClassification::Indeterminate => TypeClassificationResult::Indeterminate,
        }
    }
}

#[derive(Clone, Copy)]
enum FunctionSource {
    Expression(TextRange),
    Binding(TextRange),
    DefaultExport,
}

impl FunctionSource {
    fn to_request_source(self) -> TypeInferenceSource {
        match self {
            Self::Expression(range) => TypeInferenceSource::Expression(range),
            Self::Binding(range) => TypeInferenceSource::Binding(range),
            Self::DefaultExport => TypeInferenceSource::DefaultExport,
        }
    }
}

enum TypeLocator {
    Expression(TextRange),
    Binding {
        origin: TextRange,
        binding: TextRange,
    },
    Function {
        origin: TextRange,
        source: FunctionSource,
    },
    Member {
        origin: TextRange,
        parent: FunctionSource,
        name: String,
    },
    ExpectedArgument {
        origin: TextRange,
        callee: TextRange,
        arguments: Vec<(TextRange, bool)>,
        index: usize,
        is_constructor: bool,
    },
}

struct LazyTypeView {
    db: Rc<dyn ModuleDb>,
    module: ModuleInfo,
    caller: TypeInferenceCaller,
    locator: TypeLocator,
}

impl LazyTypeView {
    fn resolve(&self) -> Option<InferredType<'_>> {
        let db = self.db.as_ref();
        let ty = match &self.locator {
            TypeLocator::Expression(range) => execute_type_inference_request(
                db,
                self.caller,
                NormalizedExpressionTypeRequest::new(self.module, *range),
            )?,
            TypeLocator::Binding { origin, binding } => execute_type_inference_request(
                db,
                self.caller,
                NormalizedBindingTypeRequest::new(self.module, *origin, *binding),
            )?,
            TypeLocator::Function { origin, source } => execute_type_inference_request(
                db,
                self.caller,
                FunctionReturnTypeRequest::new(self.module, *origin, source.to_request_source()),
            )?,
            TypeLocator::Member {
                origin,
                parent,
                name,
            } => execute_type_inference_request(
                db,
                self.caller,
                MemberReturnTypeRequest::new(
                    self.module,
                    *origin,
                    parent.to_request_source(),
                    name,
                ),
            )?,
            TypeLocator::ExpectedArgument {
                origin,
                callee,
                arguments,
                index,
                is_constructor,
            } => {
                let arguments: Box<[_]> = arguments
                    .iter()
                    .map(|(range, is_spread)| TypeInferenceArgument::new(*range, *is_spread))
                    .collect();
                if *is_constructor {
                    execute_type_inference_request(
                        db,
                        self.caller,
                        ExpectedConstructorArgumentTypeRequest::new(
                            self.module,
                            *origin,
                            *callee,
                            arguments,
                            *index,
                        ),
                    )?
                } else {
                    execute_type_inference_request(
                        db,
                        self.caller,
                        ExpectedCallArgumentTypeRequest::new(
                            self.module,
                            *origin,
                            *callee,
                            arguments,
                            *index,
                        ),
                    )?
                }
            }
        };
        Some(InferredType::new(db, ty))
    }
}

impl TypeView for LazyTypeView {
    fn display(&self) -> String {
        self.resolve()
            .map_or_else(|| "unknown".to_string(), |ty| ty.to_string())
    }

    fn kind(&self) -> &'static str {
        self.resolve().map_or("unknown", |ty| ty.type_description())
    }

    fn evaluate(&self, predicate: TypePredicate) -> Option<bool> {
        let ty = self.resolve()?;
        Some(match predicate {
            TypePredicate::Any => ty.type_description() == "any",
            TypePredicate::Unknown => ty.type_description() == "unknown",
            TypePredicate::Never => ty.type_description() == "never",
            TypePredicate::Primitive => ty.is_primitive(),
            TypePredicate::ObjectLike => ty.is_object_like(),
            TypePredicate::Array => ty.is_array(),
            TypePredicate::Tuple => ty.type_description() == "tuple",
            TypePredicate::Function => ty.is_function(),
            TypePredicate::Callable => return ty.is_callable(),
            TypePredicate::PromiseInstance => return ty.is_promise_instance(),
            TypePredicate::ArrayOfPromise => return ty.is_array_of_promise(),
            TypePredicate::Nullish => ty.is_nullish(),
            TypePredicate::AlwaysTruthy => ty.is_always_truthy(),
            TypePredicate::AlwaysFalsy => ty.is_always_falsy(),
            TypePredicate::StringLike => ty.is_all_string_like(),
            TypePredicate::NumberLike => ty.is_all_number_like(),
            TypePredicate::BooleanLike => ty.is_all_boolean_like(),
            TypePredicate::BigIntLike => ty.is_all_bigint_like(),
        })
    }
}
