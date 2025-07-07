use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsBinding, AnyJsClass, AnyJsClassMemberName, AnyJsExpression,
    AnyJsFunction, AnyJsObjectMemberName, AnyJsRoot, JsLanguage, JsObjectExpression, JsSyntaxNode,
};
use biome_js_type_info::Type;
use biome_module_graph::ModuleResolver;
use biome_rowan::{AstNode, TextRange};
use std::sync::Arc;

/// Service for use with type inference rules.
#[derive(Clone, Debug)]
pub struct TypedService {
    resolver: Option<Arc<ModuleResolver>>,
}

impl TypedService {
    pub fn type_of_expression(&self, expr: &AnyJsExpression) -> Type {
        self.resolver
            .as_ref()
            .map(|resolver| resolver.resolved_type_of_expression(expr))
            .unwrap_or_default()
    }

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
                    self.resolver.as_ref().map(|resolver| {
                        resolver.resolved_type_of_named_value(function.range(), name.text())
                    })
                })
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(_decl) => self
                .resolver
                .as_ref()
                .and_then(|resolver| resolver.resolved_type_of_default_export())
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsFunctionExpression(expr.clone()))
            }
        }
    }

    pub fn type_of_function_like(&self, function: &AnyFunctionLike) -> Option<Type> {
        match function {
            AnyFunctionLike::AnyJsFunction(function) => Some(self.type_of_function(function)),
            AnyFunctionLike::JsConstructorClassMember(member) => {
                let class_ty = self.get_type_of_enclosing_class(member.syntax())?;
                let constructor = class_ty.own_members().find(|member| {
                    // TODO: Accurately handle overloads
                    member.is_constructor()
                })?;
                self.resolver
                    .as_ref()
                    .map(|resolver| resolver.resolved_type_for_reference(&constructor.ty))
            }
            AnyFunctionLike::JsMethodObjectMember(member) => {
                let name = member
                    .name()
                    .ok()
                    .as_ref()
                    .and_then(AnyJsObjectMemberName::as_js_literal_member_name)
                    .and_then(|name| name.value().ok())?;
                let name = name.text_trimmed();

                let object = member
                    .syntax()
                    .ancestors()
                    .skip(1)
                    .find_map(JsObjectExpression::cast)?;

                let object_ty =
                    self.type_of_expression(&AnyJsExpression::JsObjectExpression(object));
                let member = object_ty.own_members().find(|member| {
                    // TODO: Accurately handle overloads
                    member.has_name(name)
                })?;
                self.resolver
                    .as_ref()
                    .map(|resolver| resolver.resolved_type_for_reference(&member.ty))
            }
            AnyFunctionLike::JsMethodClassMember(member) => {
                let name = member
                    .name()
                    .ok()
                    .as_ref()
                    .and_then(AnyJsClassMemberName::as_js_literal_member_name)
                    .and_then(|name| name.value().ok())?;
                let name = name.text_trimmed();

                let class_ty = self.get_type_of_enclosing_class(member.syntax())?;
                let member = class_ty
                    .own_members()
                    .find(|member| member.has_name(name))?;
                self.resolver
                    .as_ref()
                    .map(|resolver| resolver.resolved_type_for_reference(&member.ty))
            }
        }
    }

    fn get_type_of_enclosing_class(&self, node: &JsSyntaxNode) -> Option<Type> {
        let class = node.ancestors().skip(1).find_map(AnyJsClass::cast)?;
        let class_ty = match class {
            AnyJsClass::JsClassDeclaration(decl) => {
                let binding = decl.id().ok()?;
                let name = binding.as_js_identifier_binding()?.name_token().ok()?;
                self.resolver.as_ref().map(|resolver| {
                    resolver.resolved_type_of_named_value(node.text_trimmed_range(), name.text())
                })?
            }
            AnyJsClass::JsClassExportDefaultDeclaration(_decl) => self
                .resolver
                .as_ref()
                .and_then(|resolver| resolver.resolved_type_of_default_export())?,
            AnyJsClass::JsClassExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsClassExpression(expr))
            }
        };
        Some(class_ty)
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
                .any(|d| d == &RuleDomain::Project);
            if !has_project_domain {
                panic!(
                    "The rule {rule_key} uses TypedService, but it is not in the project domain."
                );
            }
        }

        let resolver: Option<&Option<Arc<ModuleResolver>>> = services.get_service();
        let resolver = resolver.and_then(|resolver| resolver.as_ref().map(Arc::clone));
        Ok(Self { resolver })
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
