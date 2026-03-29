use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode};
use biome_rowan::AstNode;
use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// Project-scoped GraphQL metadata that can be populated by callers.
///
/// The initial indexed payload is operation names so rules can start consuming
/// cross-file information before the full schema service lands.
#[derive(Debug, Clone, Default)]
pub struct GraphqlProjectIndex {
    operation_names: FxHashMap<Box<str>, Vec<Utf8PathBuf>>,
}

impl GraphqlProjectIndex {
    pub fn insert_operation_name(
        &mut self,
        name: impl Into<Box<str>>,
        file_path: impl Into<Utf8PathBuf>,
    ) {
        let file_path = file_path.into();
        let entries = self.operation_names.entry(name.into()).or_default();
        if !entries.contains(&file_path) {
            entries.push(file_path);
        }
    }

    pub fn operation_files(&self, name: &str) -> &[Utf8PathBuf] {
        self.operation_names
            .get(name)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphqlAnalyzerServices {
    project_index: Arc<GraphqlProjectIndex>,
}

impl GraphqlAnalyzerServices {
    pub fn with_project_index(mut self, project_index: Arc<GraphqlProjectIndex>) -> Self {
        self.project_index = project_index;
        self
    }

    pub(crate) fn project_index(self) -> Arc<GraphqlProjectIndex> {
        self.project_index
    }
}

#[derive(Debug, Clone)]
pub struct GraphqlProjectIndexService(Arc<GraphqlProjectIndex>);

impl GraphqlProjectIndexService {
    pub fn project_index(&self) -> &GraphqlProjectIndex {
        self.0.as_ref()
    }
}

impl FromServices for GraphqlProjectIndexService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let project_index: &Arc<GraphqlProjectIndex> = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["GraphqlProjectIndex"])
        })?;

        Ok(Self(project_index.clone()))
    }
}

impl Phase for GraphqlProjectIndexService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

#[derive(Clone)]
pub struct ProjectGraphqlRoot;

impl Queryable for ProjectGraphqlRoot {
    type Input = GraphqlSyntaxNode;
    type Output = GraphqlRoot;

    type Language = GraphqlLanguage;
    type Services = GraphqlProjectIndexService;

    fn build_visitor(analyzer: &mut impl AddVisitor<GraphqlLanguage>, _: &GraphqlRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(GraphqlRoot::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        GraphqlRoot::unwrap_cast(node.clone())
    }
}
