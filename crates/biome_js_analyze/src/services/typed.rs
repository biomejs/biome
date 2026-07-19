use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsRoot, JsCallArgumentList,
    JsClassDeclaration, JsClassExpression, JsLanguage, JsObjectExpression, JsReferenceIdentifier,
    JsSyntaxNode,
};
use biome_js_type_info::{
    InferredType, Type, TypeResolverLevel,
    interned_types::{
        CallArgumentType as InferredCallArgumentType,
        FunctionParameter as InferredFunctionParameter, LocalTypeHandle, LocalTypeId,
        ReturnType as InferredReturnType, TypeData as InferredTypeData,
    },
};
use biome_module_graph::{
    CallArgumentTypeInput, JsOwnExport, ModuleDb, ModuleInfo, ModuleInfoKind, ModuleResolver,
    NormalizeTypeInput, infer_call_argument_type, infer_constructor_argument_type,
    infer_module_types_bottom_up, normalize_type,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use std::rc::Rc;
use std::sync::Arc;

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
/// This service is used for retrieving [`Type`] instances for arbitrary
/// expressions or function definitions from the module graph.
#[derive(Clone)]
pub struct TypedService {
    module: Option<TypedModule>,
    model: Option<SemanticModel>,
}

impl TypedService {
    fn normalized_inferred_type<'db>(
        &'db self,
        ty: InferredTypeData<'db>,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
        Some(InferredType::new(db, ty))
    }

    /// Returns the Salsa-inferred type for an expression.
    pub fn inferred_type_of_expression<'db>(
        &'db self,
        expression: &AnyJsExpression,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let ty = inferred.expressions.get(&expression.range()).copied()?;
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));

        Some(InferredType::new(db, ty))
    }

    /// Returns the Salsa-inferred type for a named value visible at `range`.
    pub fn inferred_type_of_named_value<'db>(
        &'db self,
        range: TextRange,
        name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let model = self.model.as_ref()?;
        let mut scope = model.scope_for_range(range);
        let binding = loop {
            if let Some(binding) = scope.get_binding(name) {
                break binding;
            }
            scope = scope.parent()?;
        };

        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let ty = inferred
            .binding_type_data
            .get(&binding.tree().syntax().text_trimmed_range())?
            .ty;
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));

        Some(InferredType::new(db, ty))
    }

    /// Returns the normalized Salsa-inferred return type for a function.
    pub fn inferred_return_type_of_function<'db>(
        &'db self,
        function: &AnyJsFunction,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let function_ty = match function {
            AnyJsFunction::JsArrowFunctionExpression(expression) => {
                inferred.expressions.get(&expression.range()).copied()
            }
            AnyJsFunction::JsFunctionDeclaration(declaration) => declaration
                .id()
                .ok()
                .as_ref()
                .and_then(AnyJsBinding::as_js_identifier_binding)
                .and_then(|identifier| identifier.name_token().ok())
                .and_then(|name| {
                    self.inferred_named_value_data(function.range(), name.text_trimmed())
                }),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                if let Some(name) = declaration
                    .id()
                    .and_then(|id| id.as_js_identifier_binding().cloned())
                    .and_then(|identifier| identifier.name_token().ok())
                {
                    self.inferred_named_value_data(function.range(), name.text_trimmed())
                } else {
                    self.inferred_default_export_data()
                }
            }
            AnyJsFunction::JsFunctionExpression(expression) => {
                inferred.expressions.get(&expression.range()).copied()
            }
        }?;
        let function = function_ty.callable_function(db)?;
        let InferredReturnType::Type(return_ty) = function.return_type(db) else {
            return None;
        };
        self.normalized_inferred_type(*return_ty)
    }

    /// Returns the normalized Salsa-inferred return type for a class or object member.
    pub fn inferred_return_type_of_member<'db>(
        &'db self,
        member_syntax: &JsSyntaxNode,
        member_name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let parent_ty = member_syntax.ancestors().find_map(|ancestor| {
            if let Some(class) = JsClassDeclaration::cast(ancestor.clone()) {
                let binding_range = class
                    .id()
                    .ok()?
                    .as_js_identifier_binding()?
                    .name_token()
                    .ok()?
                    .text_trimmed_range();
                return inferred
                    .binding_type_data
                    .get(&binding_range)
                    .map(|data| data.ty);
            }
            if let Some(class) = JsClassExpression::cast(ancestor.clone()) {
                return inferred.expressions.get(&class.range()).copied();
            }
            if let Some(object) = JsObjectExpression::cast(ancestor) {
                return inferred.expressions.get(&object.range()).copied();
            }
            None
        })?;
        let parent_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, parent_ty),
        );
        let member_ty = inferred.find_member_type(db, parent_ty, member_name)?;
        let return_ty = member_ty.callable_function(db).and_then(|function| {
            let InferredReturnType::Type(return_ty) = function.return_type(db) else {
                return None;
            };
            Some(*return_ty)
        });
        self.normalized_inferred_type(return_ty.unwrap_or(member_ty))
    }

    fn inferred_default_export_data<'db>(&'db self) -> Option<InferredTypeData<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let ModuleInfoKind::Js(js_info) = typed_module.module.kind(db) else {
            return None;
        };
        let own_export = js_info.exports.get("default")?.as_own_export()?;
        let ty = match own_export {
            JsOwnExport::Binding(range) => inferred.binding_type_data.get(range)?.ty,
            JsOwnExport::Type(resolved) if resolved.level() == TypeResolverLevel::Thin => {
                let type_id = LocalTypeId::new(resolved.index());
                if inferred.named_type_ids.contains(&type_id) {
                    InferredTypeData::Local(LocalTypeHandle::new(db, inferred.module_key, type_id))
                } else {
                    *inferred.types.get(resolved.index())?
                }
            }
            JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => return None,
        };
        Some(ty)
    }

    fn inferred_named_value_data<'db>(
        &'db self,
        range: TextRange,
        name: &str,
    ) -> Option<InferredTypeData<'db>> {
        let typed_module = self.module.as_ref()?;
        let model = self.model.as_ref()?;
        let mut scope = model.scope_for_range(range);
        let binding = loop {
            if let Some(binding) = scope.get_binding(name) {
                break binding;
            }
            scope = scope.parent()?;
        };
        infer_module_types_bottom_up(typed_module.db.as_ref(), typed_module.module)?
            .binding_type_data
            .get(&binding.tree().syntax().text_trimmed_range())
            .map(|data| data.ty)
    }

    /// Returns whether an expression has a callable member with the given name.
    pub fn inferred_expression_has_callable_member(
        &self,
        expression: &AnyJsExpression,
        name: &str,
    ) -> bool {
        let Some(typed_module) = self.module.as_ref() else {
            return false;
        };
        let db = typed_module.db.as_ref();
        let Some(inferred) = infer_module_types_bottom_up(db, typed_module.module) else {
            return false;
        };
        let Some(ty) = inferred.expressions.get(&expression.range()).copied() else {
            return false;
        };
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
        let Some(member_ty) = inferred.find_member_type(db, ty, name) else {
            return false;
        };
        let member_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, member_ty),
        );

        is_callable_inferred_type(db, member_ty)
    }

    /// Returns the expected type for a call or constructor argument.
    pub fn inferred_expected_argument_type<'db>(
        &'db self,
        callee: &AnyJsExpression,
        argument_index: usize,
        is_constructor: bool,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let callee_ty = inferred.expressions.get(&callee.range()).copied()?;
        let callee_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, callee_ty),
        );
        let argument_ty = if is_constructor {
            constructor_argument_type(db, callee_ty, argument_index)?
        } else {
            call_argument_type(db, callee_ty, argument_index)?
        };
        let argument_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, argument_ty),
        );

        Some(InferredType::new(db, argument_ty))
    }

    /// Returns the expected type for a call or constructor argument, selecting
    /// overloads using the other arguments in the same argument list.
    pub fn inferred_expected_argument_type_for_arguments<'db>(
        &'db self,
        callee: &AnyJsExpression,
        arguments: &JsCallArgumentList,
        argument_index: usize,
        is_constructor: bool,
    ) -> Option<InferredType<'db>> {
        let typed_module = self.module.as_ref()?;
        let db = typed_module.db.as_ref();
        let inferred = infer_module_types_bottom_up(db, typed_module.module)?;
        let callee_ty = inferred.expressions.get(&callee.range()).copied()?;
        let callee_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, callee_ty),
        );
        let arguments = arguments
            .iter()
            .map(|argument| {
                let argument = argument.ok()?;
                let (expression, is_spread) = match argument {
                    AnyJsCallArgument::AnyJsExpression(expression) => (expression, false),
                    AnyJsCallArgument::JsSpread(spread) => (spread.argument().ok()?, true),
                };
                let ty = inferred.expressions.get(&expression.range()).copied()?;
                let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
                Some(if is_spread {
                    InferredCallArgumentType::Spread(ty)
                } else {
                    InferredCallArgumentType::Argument(ty)
                })
            })
            .collect::<Option<Vec<_>>>()?;
        let input =
            CallArgumentTypeInput::new(db, callee_ty, arguments.into_boxed_slice(), argument_index);
        let argument_ty = if is_constructor {
            infer_constructor_argument_type(db, input)?
        } else {
            infer_call_argument_type(db, input)?
        };
        let argument_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, argument_ty),
        );

        Some(InferredType::new(db, argument_ty))
    }

    #[expect(
        clippy::arc_with_non_send_sync,
        reason = "The legacy ModuleResolver and Type APIs require Arc while this migration keeps them in place."
    )]
    fn resolver(&self) -> Option<Arc<ModuleResolver>> {
        let typed_module = self.module.as_ref()?;
        // NOTE: commented, no need to do useless computation. Comment this out once we're ready to migrate to the new engine.
        // let _ = infer_module_types_bottom_up(typed_module.db.as_ref(), typed_module.module);
        let ModuleInfoKind::Js(module_info) = typed_module.module.kind(typed_module.db.as_ref())
        else {
            return None;
        };

        Some(Arc::new(ModuleResolver::for_module(
            module_info.clone(),
            typed_module.db.clone(),
        )))
    }

    /// Returns the [`Type`] for the given `expression`.
    pub fn type_of_expression(&self, expression: &AnyJsExpression) -> Type {
        self.resolver()
            .map(|resolver| resolver.resolved_type_of_expression(expression))
            .unwrap_or_default()
    }

    /// Returns the [`Type`] of the value with the given `name`, as defined
    /// in the scope that contains `range`.
    pub fn type_of_named_value(&self, range: TextRange, name: &str) -> Type {
        self.resolver()
            .map(|resolver| resolver.resolved_type_of_named_value(range, name))
            .unwrap_or_default()
    }

    /// Returns the [`Type`] for the given `function`.
    pub fn type_of_function(&self, function: &AnyJsFunction) -> Type {
        match function {
            AnyJsFunction::JsArrowFunctionExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsArrowFunctionExpression(expr.clone()))
            }
            AnyJsFunction::JsFunctionDeclaration(decl) => decl
                .id()
                .ok()
                .as_ref()
                .and_then(AnyJsBinding::as_js_identifier_binding)
                .and_then(|identifier| identifier.name_token().ok())
                .and_then(|name| {
                    self.resolver().map(|resolver| {
                        resolver.resolved_type_of_named_value(function.range(), name.text())
                    })
                })
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(_decl) => self
                .resolver()
                .and_then(|resolver| resolver.resolved_type_of_default_export())
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsFunctionExpression(expr.clone()))
            }
        }
    }

    /// Returns the [`Type`] for a class/object member by navigating to the
    /// parent and looking up the member by name.
    pub fn type_of_member(&self, member_syntax: &JsSyntaxNode, member_name: &str) -> Type {
        let parent_type = member_syntax
            .ancestors()
            .find_map(|ancestor| {
                if let Some(class) = JsClassDeclaration::cast(ancestor.clone()) {
                    return class
                        .id()
                        .ok()
                        .and_then(|id| id.as_js_identifier_binding().cloned())
                        .and_then(|id| id.name_token().ok())
                        .map(|name| {
                            let trimmed = name.token_text_trimmed();
                            self.type_of_named_value(name.text_trimmed_range(), trimmed.text())
                        });
                }
                if let Some(class_expr) = JsClassExpression::cast(ancestor.clone()) {
                    return Some(
                        self.type_of_expression(&AnyJsExpression::JsClassExpression(class_expr)),
                    );
                }
                if let Some(obj_expr) = JsObjectExpression::cast(ancestor.clone()) {
                    return Some(
                        self.type_of_expression(&AnyJsExpression::JsObjectExpression(obj_expr)),
                    );
                }
                None
            })
            .unwrap_or_default();

        parent_type
            .find_member_type(member_name)
            .unwrap_or_default()
    }

    pub fn has_binding(&self, reference: &JsReferenceIdentifier) -> bool {
        self.model
            .as_ref()
            .is_some_and(|model| model.binding(reference).is_some())
    }
}

fn is_callable_inferred_type(db: &dyn ModuleDb, ty: InferredTypeData) -> bool {
    match ty {
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .any(|ty| is_callable_inferred_type(db, *ty)),
        ty => ty.callable_function(db).is_some(),
    }
}

fn call_argument_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let function = ty.callable_function(db)?;
    function_parameter_type(function.parameters(db), argument_index)
}

fn constructor_argument_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    let ty = match ty {
        InferredTypeData::InstanceOf(instance) => instance.ty(db),
        ty => ty,
    };
    let InferredTypeData::Class(class) = ty else {
        return None;
    };

    class.members(db).iter().find_map(|member| {
        if !member.kind.is_constructor() {
            return None;
        }
        match member.ty {
            InferredTypeData::Constructor(constructor) => constructor
                .parameters(db)
                .get(argument_index)
                .map(|parameter| parameter.parameter.ty()),
            ty => call_argument_type(db, ty, argument_index),
        }
    })
}

fn function_parameter_type<'db>(
    parameters: &[InferredFunctionParameter<'db>],
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    parameters
        .get(argument_index)
        .or_else(|| parameters.last().filter(|parameter| parameter.is_rest()))
        .map(InferredFunctionParameter::ty)
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
        Ok(Self { module, model })
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
