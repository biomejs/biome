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
    InferredType, TypeResolverLevel,
    resolved::{
        InferredCallArgumentType, InferredFunctionParameter, InferredLocalTypeHandle,
        InferredLocalTypeId, InferredReturnType, InferredTypeData,
    },
};
use biome_module_graph::{
    InferredModuleTypes, JsOwnExport, ModuleDb, ModuleInfo, ModuleInfoKind, NormalizeTypeInput,
    infer_call_argument_type, infer_module_types, infer_module_types_bottom_up, normalize_type,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use std::{cell::OnceCell, rc::Rc, sync::Arc};

#[derive(Clone)]
pub(crate) struct TypedModule {
    db: Rc<dyn ModuleDb>,
    module: ModuleInfo,
    types_warmed: Rc<OnceCell<bool>>,
}

impl TypedModule {
    pub(crate) fn new(db: Rc<dyn ModuleDb>, module: ModuleInfo) -> Self {
        Self {
            db,
            module,
            types_warmed: Rc::new(OnceCell::new()),
        }
    }

    fn inferred_types<'db>(&'db self) -> Option<Arc<InferredModuleTypes<'db>>> {
        let warmed = *self
            .types_warmed
            .get_or_init(|| infer_module_types_bottom_up(self.db.as_ref(), self.module).is_some());
        warmed.then(|| infer_module_types(self.db.as_ref(), self.module))?
    }
}

/// Service for use with type inference rules.
///
/// This service retrieves database-inferred types from the module graph.
/// Methods returning [`Option<InferredType>`] return `None` when the specific
/// type could not be inferred.
#[derive(Clone)]
pub struct TypedService {
    module: TypedModule,
    model: SemanticModel,
}

impl TypedService {
    fn normalized_inferred_type<'db>(
        &'db self,
        ty: InferredTypeData<'db>,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
        Some(InferredType::new(db, ty))
    }

    /// Returns the inferred type for an expression.
    pub fn type_of_expression<'db>(
        &'db self,
        expression: &AnyJsExpression,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
        let ty = inferred.expressions.get(&expression.range()).copied()?;
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));

        Some(InferredType::new(db, ty))
    }

    /// Returns the inferred type of a named value visible at `range`.
    pub fn type_of_named_value<'db>(
        &'db self,
        range: TextRange,
        name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let model = &self.model;
        let mut scope = model.scope_for_range(range);
        let binding = loop {
            if let Some(binding) = scope.get_binding(name) {
                break binding;
            }
            scope = scope.parent()?;
        };

        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
        let ty = inferred
            .binding_type_data
            .get(&binding.tree().syntax().text_trimmed_range())?
            .ty;
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));

        Some(InferredType::new(db, ty))
    }

    /// Returns the normalized inferred type for a function.
    pub fn type_of_function<'db>(&'db self, function: &AnyJsFunction) -> Option<InferredType<'db>> {
        self.normalized_inferred_type(self.inferred_function_data(function)?)
    }

    /// Returns the normalized database-inferred return type for a function.
    pub fn inferred_return_type_of_function<'db>(
        &'db self,
        function: &AnyJsFunction,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let function_ty = self.inferred_function_data(function)?;
        let function_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, function_ty),
        );
        let function = function_ty.callable_function(db)?;
        let InferredReturnType::Type(return_ty) = function.return_type(db) else {
            return None;
        };
        self.normalized_inferred_type(*return_ty)
    }

    fn inferred_function_data<'db>(
        &'db self,
        function: &AnyJsFunction,
    ) -> Option<InferredTypeData<'db>> {
        let typed_module = &self.module;
        let inferred = typed_module.inferred_types()?;
        match function {
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
        }
    }

    /// Returns the normalized inferred type for a class or object member.
    pub fn type_of_member<'db>(
        &'db self,
        member_syntax: &JsSyntaxNode,
        member_name: &str,
    ) -> Option<InferredType<'db>> {
        self.normalized_inferred_type(self.inferred_member_data(member_syntax, member_name)?)
    }

    /// Returns the normalized database-inferred return type for a class or object member.
    pub fn inferred_return_type_of_member<'db>(
        &'db self,
        member_syntax: &JsSyntaxNode,
        member_name: &str,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let member_ty = self.inferred_member_data(member_syntax, member_name)?;
        let member_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, member_ty),
        );
        let return_ty = member_ty.callable_function(db).and_then(|function| {
            let InferredReturnType::Type(return_ty) = function.return_type(db) else {
                return None;
            };
            Some(*return_ty)
        });
        self.normalized_inferred_type(return_ty.unwrap_or(member_ty))
    }

    fn inferred_member_data<'db>(
        &'db self,
        member_syntax: &JsSyntaxNode,
        member_name: &str,
    ) -> Option<InferredTypeData<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
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
        inferred.find_member_type(db, parent_ty, member_name)
    }

    fn inferred_default_export_data<'db>(&'db self) -> Option<InferredTypeData<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
        let ModuleInfoKind::Js(js_info) = typed_module.module.kind(db) else {
            return None;
        };
        let own_export = js_info.exports.get("default")?.as_own_export()?;
        let ty = match own_export {
            JsOwnExport::Binding(range) => inferred.binding_type_data.get(range)?.ty,
            JsOwnExport::Type(resolved) if resolved.level() == TypeResolverLevel::Thin => {
                let type_id = InferredLocalTypeId::new(resolved.index());
                if inferred.named_type_ids.contains(&type_id) {
                    InferredTypeData::Local(InferredLocalTypeHandle::new(
                        db,
                        inferred.module_key,
                        type_id,
                    ))
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
        let typed_module = &self.module;
        let model = &self.model;
        let mut scope = model.scope_for_range(range);
        let binding = loop {
            if let Some(binding) = scope.get_binding(name) {
                break binding;
            }
            scope = scope.parent()?;
        };
        typed_module
            .inferred_types()?
            .binding_type_data
            .get(&binding.tree().syntax().text_trimmed_range())
            .map(|data| data.ty)
    }

    /// Returns whether an expression has a callable member with the given name.
    ///
    /// Returns `None` when the expression or member type is unavailable or
    /// indeterminate.
    pub fn inferred_expression_has_callable_member(
        &self,
        expression: &AnyJsExpression,
        name: &str,
    ) -> Option<bool> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
        let ty = inferred.expressions.get(&expression.range()).copied()?;
        let ty = normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
        let Some(member_ty) = inferred.find_member_type(db, ty, name) else {
            return Some(false);
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
        arguments: &JsCallArgumentList,
        argument_index: usize,
        is_constructor: bool,
    ) -> Option<InferredType<'db>> {
        let typed_module = &self.module;
        let db = typed_module.db.as_ref();
        let inferred = typed_module.inferred_types()?;
        let callee_ty = inferred.expressions.get(&callee.range()).copied()?;
        let callee_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, callee_ty),
        );
        let argument_ty = if is_constructor {
            constructor_argument_type(db, callee_ty, argument_index)?
        } else {
            let arguments = arguments
                .iter()
                .map(|argument| {
                    let argument = argument.ok()?;
                    let (expression, is_spread) = match argument {
                        AnyJsCallArgument::AnyJsExpression(expression) => (expression, false),
                        AnyJsCallArgument::JsSpread(spread) => (spread.argument().ok()?, true),
                    };
                    let ty = inferred.expressions.get(&expression.range()).copied()?;
                    let ty =
                        normalize_type(db, NormalizeTypeInput::new(db, typed_module.module, ty));
                    Some(if is_spread {
                        InferredCallArgumentType::Spread(ty)
                    } else {
                        InferredCallArgumentType::Argument(ty)
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            infer_call_argument_type(db, callee_ty, &arguments, argument_index)?
        };
        let argument_ty = normalize_type(
            db,
            NormalizeTypeInput::new(db, typed_module.module, argument_ty),
        );

        Some(InferredType::new(db, argument_ty))
    }

    pub fn has_binding(&self, reference: &JsReferenceIdentifier) -> bool {
        self.model.binding(reference).is_some()
    }
}

fn is_callable_inferred_type(db: &dyn ModuleDb, ty: InferredTypeData) -> Option<bool> {
    match ty {
        InferredTypeData::Unknown
        | InferredTypeData::AnyKeyword
        | InferredTypeData::UnknownKeyword => None,
        InferredTypeData::Union(union) => {
            let mut indeterminate = false;
            for ty in union.types(db) {
                match is_callable_inferred_type(db, *ty) {
                    Some(true) => return Some(true),
                    Some(false) => {}
                    None => indeterminate = true,
                }
            }
            (!indeterminate).then_some(false)
        }
        ty => Some(ty.callable_function(db).is_some()),
    }
}

fn call_argument_type<'db>(
    db: &'db dyn ModuleDb,
    ty: InferredTypeData<'db>,
    argument_index: usize,
) -> Option<InferredTypeData<'db>> {
    if let Some(function) = ty.callable_function(db) {
        return function_parameter_type(function.parameters(db), argument_index);
    }

    match ty {
        InferredTypeData::Interface(interface) => interface
            .members(db)
            .iter()
            .filter(|member| member.kind.is_call_signature())
            .find_map(|member| call_argument_type(db, member.ty, argument_index)),
        InferredTypeData::Object(object) => object
            .members(db)
            .iter()
            .filter(|member| member.kind.is_call_signature())
            .find_map(|member| call_argument_type(db, member.ty, argument_index)),
        InferredTypeData::InstanceOf(instance) => {
            call_argument_type(db, instance.ty(db), argument_index)
        }
        InferredTypeData::TypeofType(typeof_type) => {
            call_argument_type(db, typeof_type.ty(db), argument_index)
        }
        InferredTypeData::TypeofValue(typeof_value) => {
            call_argument_type(db, typeof_value.ty(db), argument_index)
        }
        InferredTypeData::Union(union) => union
            .types(db)
            .iter()
            .find_map(|ty| call_argument_type(db, *ty, argument_index)),
        _ => None,
    }
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
            .and_then(Option::as_ref)
            .cloned()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["TypedModule"]))?;
        let model = services
            .get_service::<SemanticModel>()
            .cloned()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;
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
