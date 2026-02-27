use biome_analyze::{
    AddVisitor, ExtendedConfigurationProvider, FromServices, Phase, Phases, QueryKey, Queryable,
    RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ConfigurationSourceService(Option<Arc<dyn ExtendedConfigurationProvider>>);

impl ConfigurationSourceService {
    pub(crate) fn any_extended_starts_with_catch_all(&self) -> bool {
        self.0
            .as_ref()
            .is_some_and(|provider| provider.any_extended_starts_with_catch_all())
    }
}

impl FromServices for ConfigurationSourceService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let source: &Option<Arc<dyn ExtendedConfigurationProvider>> =
            services.get_service().ok_or_else(|| {
                ServicesDiagnostic::new(rule_key.rule_name(), &["ExtendedConfigurationProvider"])
            })?;

        Ok(Self(source.clone()))
    }
}

impl Phase for ConfigurationSourceService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the configuration source** and matches on specific [AstNode] types.
#[derive(Clone)]
pub struct ConfigSource<N>(pub N);

impl<N> Queryable for ConfigSource<N>
where
    N: AstNode<Language = JsonLanguage> + 'static,
{
    type Input = JsonSyntaxNode;
    type Output = N;

    type Language = JsonLanguage;
    type Services = ConfigurationSourceService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsonLanguage>, _: &JsonRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
