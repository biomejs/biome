use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_configuration::{ConfigurationSource, ExtendedConfigurationIterator};
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ConfigurationSourceService(Option<Arc<ConfigurationSource>>);

impl ConfigurationSourceService {
    pub(crate) fn extends(&self) -> Option<ExtendedConfigurationIterator<'_>> {
        self.0
            .as_ref()
            .map(|source| source.extended_configurations())
    }
}

impl FromServices for ConfigurationSourceService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let source: &Option<Arc<ConfigurationSource>> =
            services.get_service().ok_or_else(|| {
                ServicesDiagnostic::new(rule_key.rule_name(), &["ConfigurationSource"])
            })?;

        Ok(Self(source.clone()))
    }
}

impl Phase for ConfigurationSourceService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the package manifest** and matches on specific [AstNode] types.
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
