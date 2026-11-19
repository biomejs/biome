use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_aria::AriaRoles;
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AriaServices {
    pub(crate) roles: Arc<AriaRoles>,
}

impl AriaServices {
    pub fn aria_roles(&self) -> &AriaRoles {
        &self.roles
    }
}

impl FromServices for AriaServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let roles: &Arc<AriaRoles> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["AriaRoles"]))?;
        Ok(Self {
            roles: roles.clone(),
        })
    }
}

impl Phase for AriaServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

#[derive(Clone)]
pub struct Aria<N>(pub N);

impl<N> Queryable for Aria<N>
where
    N: AstNode<Language = HtmlLanguage> + 'static,
{
    type Input = HtmlSyntaxNode;
    type Output = N;

    type Language = HtmlLanguage;
    type Services = AriaServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<HtmlLanguage>, _: &HtmlRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
