use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsRoot, JsCallArgumentList,
    JsClassDeclaration, JsClassExpression, JsLanguage, JsObjectExpression, JsReferenceIdentifier,
    JsSyntaxNode,
};
use biome_js_type_info::InferredType;
use biome_module_graph::{
    ModuleDb, ModuleInfo,
    type_inference::{
        ArrayOfPromisesClassificationRequest, CallableMemberRequest,
        ExpectedCallArgumentTypeRequest, ExpectedConstructorArgumentTypeRequest,
        FunctionReturnTypeRequest, MemberReturnTypeRequest, NormalizedBindingTypeRequest,
        NormalizedExpressionTypeRequest, PromiseClassificationRequest,
        PromiseReturningFunctionClassificationRequest, TypeInferenceArgument, TypeInferenceCaller,
        TypeInferenceClassification, TypeInferenceRequest, TypeInferenceSource,
        execute_type_inference_request,
    },
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct TypedModule {
    db: Rc<dyn ModuleDb>,
    module: ModuleInfo,
}

impl TypedModule {
    pub(crate) fn new(db: Rc<dyn ModuleDb>, module: ModuleInfo) -> Self {
        Self { db, module }
    }
}

/// Service for use with type inference rules.
///
/// This service retrieves inferred types for arbitrary expressions or function
/// definitions from the module graph.
#[derive(Clone)]
pub struct TypedService {
    module: Option<TypedModule>,
    model: Option<SemanticModel>,
    caller: TypeInferenceCaller,
}

impl TypedService {
    fn execute_request<'db, R>(&'db self, typed_module: &'db TypedModule, request: R) -> R::Output
    where
        R: TypeInferenceRequest<'db>,
    {
        execute_type_inference_request(typed_module.db.as_ref(), self.caller, request)
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
        Some(binding.tree().syntax().text_trimmed_range())
    }

    /// Returns the Salsa-inferred type for an expression.
    pub fn type_of_expression<'db>(
        &'db self,
        expression: &AnyJsExpression,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let ty = self.execute_request(
            typed_module,
            NormalizedExpressionTypeRequest::new(typed_module.module, expression.range()),
        )?;

        Some(InferredType::new(typed_module.db.as_ref(), ty))
    }

    /// Classifies an expression as a Promise without resolving unrelated members.
    ///
    /// Returns `Indeterminate` when type inference is unavailable or inconclusive.
    pub fn classify_expression_as_promise(
        &self,
        expression: &AnyJsExpression,
    ) -> TypeInferenceClassification {
        let Some(typed_module) = self.module.as_ref() else {
            return TypeInferenceClassification::Indeterminate;
        };
        self.execute_request(
            typed_module,
            PromiseClassificationRequest::new(typed_module.module, expression.range()),
        )
    }

    /// Classifies an expression as an array of Promises without normalizing
    /// unrelated nested types. Returns `Indeterminate` when type inference is
    /// unavailable or inconclusive.
    pub fn classify_expression_as_array_of_promises(
        &self,
        expression: &AnyJsExpression,
    ) -> TypeInferenceClassification {
        let Some(typed_module) = self.module.as_ref() else {
            return TypeInferenceClassification::Indeterminate;
        };
        self.execute_request(
            typed_module,
            ArrayOfPromisesClassificationRequest::new(typed_module.module, expression.range()),
        )
    }

    /// Classifies an expression as callable with a Promise return without
    /// normalizing callable parameters or unrelated nested types. Returns
    /// `Indeterminate` when type inference is unavailable or inconclusive.
    pub fn classify_expression_as_promise_returning_function(
        &self,
        expression: &AnyJsExpression,
    ) -> TypeInferenceClassification {
        let Some(typed_module) = self.module.as_ref() else {
            return TypeInferenceClassification::Indeterminate;
        };
        self.execute_request(
            typed_module,
            PromiseReturningFunctionClassificationRequest::new(
                typed_module.module,
                expression.range(),
            ),
        )
    }

    /// Returns the Salsa-inferred type for a named value visible at `range`.
    pub fn type_of_named_value<'db>(
        &'db self,
        range: TextRange,
        name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let binding = self.named_value_binding_range(range, name)?;
        let ty = self.execute_request(
            typed_module,
            NormalizedBindingTypeRequest::new(typed_module.module, range, binding),
        )?;

        Some(InferredType::new(typed_module.db.as_ref(), ty))
    }

    /// Returns the normalized Salsa-inferred return type for a function.
    pub fn return_type_of_function<'db>(
        &'db self,
        function: &AnyJsFunction,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let source = match function {
            AnyJsFunction::JsArrowFunctionExpression(expression) => {
                TypeInferenceSource::Expression(expression.range())
            }
            AnyJsFunction::JsFunctionDeclaration(declaration) => {
                let name = declaration
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?;
                TypeInferenceSource::Binding(
                    self.named_value_binding_range(function.range(), name.text_trimmed())?,
                )
            }
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                if let Some(name) = declaration
                    .id()
                    .and_then(|id| id.as_js_identifier_binding().cloned())
                    .and_then(|identifier| identifier.name_token().ok())
                {
                    TypeInferenceSource::Binding(
                        self.named_value_binding_range(function.range(), name.text_trimmed())?,
                    )
                } else {
                    TypeInferenceSource::DefaultExport
                }
            }
            AnyJsFunction::JsFunctionExpression(expression) => {
                TypeInferenceSource::Expression(expression.range())
            }
        };
        let ty = self.execute_request(
            typed_module,
            FunctionReturnTypeRequest::new(typed_module.module, function.range(), source),
        )?;

        Some(InferredType::new(typed_module.db.as_ref(), ty))
    }

    /// Returns the normalized Salsa-inferred return type for a class or object member.
    pub fn return_type_of_member<'db>(
        &'db self,
        member_syntax: &JsSyntaxNode,
        member_name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let parent = member_syntax.ancestors().find_map(|ancestor| {
            if let Some(class) = JsClassDeclaration::cast(ancestor.clone()) {
                let binding_range = class
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .text_trimmed_range();
                return Some(TypeInferenceSource::Binding(binding_range));
            }
            if let Some(class) = JsClassExpression::cast(ancestor.clone()) {
                return Some(TypeInferenceSource::Expression(class.range()));
            }
            if let Some(object) = JsObjectExpression::cast(ancestor) {
                return Some(TypeInferenceSource::Expression(object.range()));
            }
            None
        })?;
        let ty = self.execute_request(
            typed_module,
            MemberReturnTypeRequest::new(
                typed_module.module,
                member_syntax.text_trimmed_range(),
                parent,
                member_name,
            ),
        )?;

        Some(InferredType::new(typed_module.db.as_ref(), ty))
    }

    /// Classifies whether an expression has a callable member with the given name.
    ///
    /// Returns `Indeterminate` when type inference is unavailable or inconclusive.
    pub fn classify_callable_member(
        &self,
        expression: &AnyJsExpression,
        name: &str,
    ) -> TypeInferenceClassification {
        let Some(typed_module) = self.module.as_ref() else {
            return TypeInferenceClassification::Indeterminate;
        };
        self.execute_request(
            typed_module,
            CallableMemberRequest::new(typed_module.module, expression.range(), name),
        )
    }

    /// Returns the expected type for a call or constructor argument, selecting
    /// overloads using the other arguments in the same argument list.
    pub fn expected_argument_type<'db>(
        &'db self,
        callee: &AnyJsExpression,
        arguments: &JsCallArgumentList,
        argument_index: usize,
        is_constructor: bool,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let arguments = arguments
            .iter()
            .map(|argument| {
                let argument = argument.ok()?;
                let (expression, is_spread) = match argument {
                    AnyJsCallArgument::AnyJsExpression(expression) => (expression, false),
                    AnyJsCallArgument::JsSpread(spread) => (spread.argument().ok()?, true),
                };
                Some(TypeInferenceArgument::new(expression.range(), is_spread))
            })
            .collect::<Option<Vec<_>>>()?
            .into_boxed_slice();
        let origin = arguments.get(argument_index)?.range();
        let ty = if is_constructor {
            self.execute_request(
                typed_module,
                ExpectedConstructorArgumentTypeRequest::new(
                    typed_module.module,
                    origin,
                    callee.range(),
                    arguments,
                    argument_index,
                ),
            )?
        } else {
            self.execute_request(
                typed_module,
                ExpectedCallArgumentTypeRequest::new(
                    typed_module.module,
                    origin,
                    callee.range(),
                    arguments,
                    argument_index,
                ),
            )?
        };

        Some(InferredType::new(typed_module.db.as_ref(), ty))
    }

    pub fn has_binding(&self, reference: &JsReferenceIdentifier) -> bool {
        self.model
            .as_ref()
            .is_some_and(|model| model.binding(reference).is_some())
    }
}

impl FromServices for TypedService {
    fn from_services(
        rule_key: &RuleKey,
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        if cfg!(debug_assertions) {
            let has_project_domain = rule_metadata
                .domains
                .iter()
                .any(|d| d == &RuleDomain::Types);
            if !has_project_domain {
                panic!("The rule {rule_key} uses TypedService, but it is not in the Types domain.");
            }
        }

        let module = services
            .get_service::<Option<TypedModule>>()
            .cloned()
            .flatten();
        let model = services.get_service::<SemanticModel>().cloned();
        let caller = TypeInferenceCaller::new(rule_key.group(), rule_key.rule_name());
        Ok(Self {
            module,
            model,
            caller,
        })
    }
}

impl Phase for TypedService {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules that wish to perform type inference on
/// nodes.
#[derive(Clone)]
pub struct Typed<N>(N);

impl<N> QueryMatch for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N> Queryable for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TypedService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
