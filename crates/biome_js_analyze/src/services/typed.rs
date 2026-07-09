use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsRoot, JsClassDeclaration, JsClassExpression,
    JsLanguage, JsObjectExpression, JsReferenceIdentifier, JsSyntaxNode,
};
use biome_js_type_info::Type;
use biome_module_graph::{
    ModuleDb, ModuleInfo, ModuleInfoKind, ModuleResolver, infer_module_types_bottom_up,
};
use biome_rowan::{AstNode, TextRange};
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
