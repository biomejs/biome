use std::sync::Arc;

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_package::TurboJson;
use biome_rowan::AstNode;

use crate::services::semantic::SemanticModelBuilderVisitor;

/// Holds all turbo.json(c) configurations that apply to a file.
///
/// In a Turborepo monorepo, environment variables can be declared in:
/// 1. The root `turbo.json` at the repository root
/// 2. A package-level `turbo.json` in the package directory
///
/// Both are checked when validating environment variable declarations.
#[derive(Debug, Clone)]
pub struct TurborepoServices {
    /// All turbo.json configurations that apply to the current file,
    /// ordered from closest (package-level) to furthest (root).
    pub(crate) turborepo_configs: Vec<Arc<TurboJson>>,

    /// Semantic model for resolving bindings.
    model: SemanticModel,
}

impl TurborepoServices {
    /// Checks if the given environment variable is declared in any turbo.json.
    ///
    /// This checks all turbo.json files in the hierarchy (package-level and root).
    /// The env var is considered declared if it appears in ANY of the configs.
    pub fn is_env_var_declared(&self, env_var: &str) -> bool {
        self.turborepo_configs
            .iter()
            .any(|config| config.is_env_var_declared(env_var))
    }

    /// Returns whether any turbo.json file was found for the current file.
    pub fn has_turborepo_config(&self) -> bool {
        !self.turborepo_configs.is_empty()
    }

    /// Returns a reference to the semantic model.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for TurborepoServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let turborepo_configs: &Vec<Arc<TurboJson>> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["TurboJson"]))?;

        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;

        Ok(Self {
            turborepo_configs: turborepo_configs.clone(),
            model: model.clone(),
        })
    }
}

impl Phase for TurborepoServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules **that use the turbo.json config** and matches on specific [AstNode] types.
/// Also provides access to the semantic model for resolving bindings.
#[derive(Clone)]
pub struct Turborepo<N>(pub N);

impl<N> Queryable for Turborepo<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TurborepoServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::json::deserialize_from_json_str;

    fn create_turbo_json(json: &str) -> TurboJson {
        let (turbo_json, _diagnostics) =
            deserialize_from_json_str::<TurboJson>(json, Default::default(), "turbo.json")
                .consume();
        turbo_json.unwrap_or_default()
    }

    #[test]
    fn test_monorepo_env_var_in_root_only() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR", "SHARED_VAR"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["PACKAGE_VAR"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // ROOT_VAR is declared in root turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
        // PACKAGE_VAR is declared in package turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("PACKAGE_VAR")));
        // SHARED_VAR is declared in root turbo.json
        assert!(configs.iter().any(|c| c.is_env_var_declared("SHARED_VAR")));
        // UNDECLARED is not in any config
        assert!(!configs.iter().any(|c| c.is_env_var_declared("UNDECLARED")));
    }

    #[test]
    fn test_monorepo_env_var_in_package_task() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);
        let package_turbo =
            create_turbo_json(r#"{ "tasks": { "build": { "env": ["BUILD_OUTPUT"] } } }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // BUILD_OUTPUT is declared in package turbo.json's task
        assert!(
            configs
                .iter()
                .any(|c| c.is_env_var_declared("BUILD_OUTPUT"))
        );
        // ROOT_VAR is in root only
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
    }

    #[test]
    fn test_monorepo_wildcard_inheritance() {
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["NEXT_PUBLIC_*"] }"#);
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["DATABASE_URL"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // Wildcard from root applies
        assert!(
            configs
                .iter()
                .any(|c| c.is_env_var_declared("NEXT_PUBLIC_API_URL"))
        );
        // Specific var from package applies
        assert!(
            configs
                .iter()
                .any(|c| c.is_env_var_declared("DATABASE_URL"))
        );
        // Non-matching var is undeclared
        assert!(!configs.iter().any(|c| c.is_env_var_declared("ACME_SECRET")));
    }

    #[test]
    fn test_monorepo_negation_scope() {
        // Root allows all vars
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["*"] }"#);
        // Package excludes ACME_SECRET
        let package_turbo = create_turbo_json(r#"{ "globalEnv": ["*", "!ACME_SECRET"] }"#);

        let configs = vec![Arc::new(package_turbo), Arc::new(root_turbo)];

        // ACME_SECRET is excluded in package but allowed in root
        // Since we use `any()`, if ANY config allows it, it's considered declared
        // This matches the current implementation behavior
        assert!(configs.iter().any(|c| c.is_env_var_declared("ACME_SECRET")));

        // Regular vars are allowed by both
        assert!(configs.iter().any(|c| c.is_env_var_declared("ACME_TOKEN")));
    }

    #[test]
    fn test_monorepo_empty_configs() {
        let configs: Vec<Arc<TurboJson>> = vec![];

        // No configs means no turbo.json found
        assert!(configs.is_empty());
        assert!(!configs.iter().any(|c| c.is_env_var_declared("ANY_VAR")));
    }

    #[test]
    fn test_monorepo_multiple_levels() {
        // Simulates: root -> packages/shared -> packages/shared/utils
        let root_turbo = create_turbo_json(r#"{ "globalEnv": ["ROOT_VAR"] }"#);
        let shared_turbo = create_turbo_json(r#"{ "globalEnv": ["SHARED_VAR"] }"#);
        let utils_turbo = create_turbo_json(r#"{ "globalEnv": ["UTILS_VAR"] }"#);

        // Closest to furthest
        let configs = vec![
            Arc::new(utils_turbo),
            Arc::new(shared_turbo),
            Arc::new(root_turbo),
        ];

        // All vars from all levels should be accessible
        assert!(configs.iter().any(|c| c.is_env_var_declared("ROOT_VAR")));
        assert!(configs.iter().any(|c| c.is_env_var_declared("SHARED_VAR")));
        assert!(configs.iter().any(|c| c.is_env_var_declared("UTILS_VAR")));
        assert!(!configs.iter().any(|c| c.is_env_var_declared("UNDECLARED")));
    }
}
