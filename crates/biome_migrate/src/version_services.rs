use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, Queryable,
    RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct VersionServices {
    version: Arc<TheVersion>,
}

#[derive(Debug, Clone)]
pub(crate) struct TheVersion(pub(crate) String);

impl VersionServices {
    pub fn version(&self) -> &str {
        self.version.0.as_str()
    }
}

impl FromServices for VersionServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let version: &Arc<TheVersion> = services
            .get_service()
            .ok_or_else(|| MissingServicesDiagnostic::new(rule_key.rule_name(), &["TheVersion"]))?;
        Ok(Self {
            version: version.clone(),
        })
    }
}

impl Phase for VersionServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub(crate) struct Version<N>(pub N);

impl<N> Queryable for Version<N>
where
    N: AstNode<Language = JsonLanguage> + 'static,
{
    type Input = JsonSyntaxNode;
    type Output = N;

    type Language = JsonLanguage;
    type Services = VersionServices;

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
